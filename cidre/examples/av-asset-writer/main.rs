use clap::Parser;
use std::{path::PathBuf, sync::Arc};

use cidre::{arc, av, blocks, cat, cf, cm, dispatch, ns};

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Encode wav file to aac file
    #[clap(alias = "e", alias = "enc")]
    Encode(EncodeArgs),

    /// Decode file to wav file
    #[clap(alias = "d", alias = "dec")]
    Decode(DecodeArgs),
}

impl Command {
    async fn run(&self) {
        match self {
            Self::Encode(args) => encode(args).await,
            Self::Decode(args) => decode(args).await,
        }
    }
}

#[tokio::main]
async fn main() {
    Cli::parse().command.run().await;
}

#[derive(clap::Args, Debug)]
struct EncodeArgs {
    #[arg(value_name = "WAV FILE")]
    src: PathBuf,

    #[arg(value_name = "AAC FILE")]
    dst: Option<PathBuf>,
}

async fn reader_and_output(
    path: &PathBuf,
) -> (arc::R<av::AssetReader>, arc::R<av::AssetReaderTrackOutput>) {
    let true = path.is_file() else {
        panic!("src file doesn't exists `{}`", path.to_string_lossy());
    };

    let src = cf::Url::with_path(path.as_path(), false).unwrap();

    let src_asset = av::UrlAsset::with_url(src.as_ns(), None).unwrap();
    let mut asset_reader = av::AssetReader::with_asset(&src_asset).unwrap();

    let tracks = src_asset
        .load_tracks_with_media_type(av::MediaType::audio())
        .await
        .unwrap();

    let mut track_output =
        av::AssetReaderTrackOutput::with_track(&tracks.get(0).unwrap(), None).unwrap();
    track_output.set_always_copies_sample_data(false);

    asset_reader.add_output(&track_output).unwrap();
    (asset_reader, track_output)
}

fn writer_and_input(
    path: &PathBuf,
    file_type: &av::FileType,
    reader: &mut av::AssetReader,
    output: &mut av::AssetReaderTrackOutput,
) -> (
    arc::R<av::AssetWriter>,
    arc::R<av::AssetWriterInput>,
    arc::R<cm::SampleBuf>,
) {
    if path.exists() {
        std::fs::remove_file(&path).unwrap();
    }

    let dst = cf::Url::with_path(path.as_path(), false).unwrap();

    let mut writer = av::AssetWriter::with_url_and_file_type(dst.as_ns(), file_type).unwrap();

    assert!(reader.start_reading().expect("failed to start reading"));
    let buf = output
        .next_sample_buf()
        .expect("Failed to copy next sample buf")
        .unwrap();
    let fd = buf.format_desc().unwrap();
    let src_asbd = fd.stream_basic_desc().unwrap();
    let desc = cm::AudioFormatDesc::with_asbd(&src_asbd).unwrap();

    let settings = if file_type == av::FileType::m4a() {
        ns::Dictionary::with_keys_values(
            &[
                av::audio::all_formats_keys::id(),
                av::audio::all_formats_keys::number_of_channels(),
            ],
            &[
                cat::AudioFormat::MPEG4_AAC.as_ref(),
                src_asbd.channels_per_frame.as_ref(),
            ],
        )
    } else {
        ns::Dictionary::with_keys_values(
            &[
                av::audio::all_formats_keys::id(),
                // av::audio::all_formats_keys::number_of_channels(),
                // av::audio::all_formats_keys::sample_rate(),
                // av::audio::linear_pcm_keys::bit_depth(),
                // av::audio::linear_pcm_keys::is_float(),
            ],
            &[
                cat::AudioFormat::LINEAR_PCM.as_ref(),
                // ns::Number::tagged_i16(src_asbd.channels_per_frame as _).as_ref(),
                // ns::Number::tagged_i16(src_asbd.sample_rate as _).as_ref(),
                // ns::Number::tagged_i16(32).as_ref(),
                // ns::Number::tagged_i16(0).as_ref(),
            ],
        )
    };

    let input = av::AssetWriterInput::with_media_type_output_settings_source_format_hint(
        av::MediaType::audio(),
        Some(settings.as_ref()),
        Some(&desc),
    )
    .unwrap();

    writer.add_input(&input).unwrap();

    (writer, input, buf)
}

fn write(
    reader: &mut av::AssetReader,
    writer: &mut av::AssetWriter,
    output: &mut av::AssetReaderTrackOutput,
    input: &mut av::AssetWriterInput,
    first_buf: &cm::SampleBuf,
) {
    let mut buf = first_buf.retained();
    writer.start_writing();
    writer.start_session_at_src_time(cm::Time::zero());

    let sema = Arc::new(dispatch::Semaphore::new(0));
    let queue = dispatch::Queue::serial_with_ar_pool();
    let sem = sema.clone();
    let mut inp = input.retained();
    let mut out = output.retained();

    let mut block = blocks::EscBlock::new0(move || {
        while inp.is_ready_for_more_media_data() {
            unsafe { inp.append_sample_buf_throws(&buf) };
            let Ok(Some(b)) = out.next_sample_buf() else {
                inp.mark_as_finished();
                sem.signal();
                break;
            };
            buf = b;
        }
    });

    input
        .request_media_data_when_ready_on_queue(&queue, &mut block)
        .unwrap();

    sema.wait_forever();

    writer.finish_writing();
    reader.cancel_reading();
}

async fn encode(args: &EncodeArgs) {
    let (mut reader, mut output) = reader_and_output(&args.src).await;

    let dst = match args.dst {
        Some(ref dst) => dst.clone(),
        None => args.src.with_extension("m4a"),
    };

    let (mut writer, mut input, mut buf) =
        writer_and_input(&dst, av::FileType::m4a(), &mut reader, &mut output);

    write(&mut reader, &mut writer, &mut output, &mut input, &mut buf);
}

#[derive(clap::Args, Debug)]
struct DecodeArgs {
    #[arg(value_name = "AUDIO FILE")]
    src: PathBuf,

    #[arg(value_name = "WAV FILE")]
    dst: Option<PathBuf>,
}

async fn decode(args: &DecodeArgs) {
    let (mut reader, mut output) = reader_and_output(&args.src).await;

    let dst = match args.dst {
        Some(ref dst) => dst.clone(),
        None => args.src.with_extension("wav"),
    };

    let (mut writer, mut input, mut buf) =
        writer_and_input(&dst, av::FileType::wav(), &mut reader, &mut output);

    write(&mut reader, &mut writer, &mut output, &mut input, &mut buf);
}
