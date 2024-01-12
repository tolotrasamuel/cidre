use crate::{arc, av, blocks, define_obj_type, ns, objc, sn};

define_obj_type!(
    pub AudioStreamAnalyzer(ns::Id),
    SN_AUDIO_STREAM_ANALYZER
);

unsafe impl Send for AudioStreamAnalyzer {}
unsafe impl Sync for AudioStreamAnalyzer {}

impl arc::A<AudioStreamAnalyzer> {
    #[objc::msg_send(initWithFormat:)]
    pub fn init_with_format(self, format: &av::AudioFormat) -> arc::R<AudioStreamAnalyzer>;
}

impl AudioStreamAnalyzer {
    /// Only PCM formats are supported.
    pub fn with_format(format: &av::AudioFormat) -> arc::R<Self> {
        Self::alloc().init_with_format(format)
    }

    #[objc::msg_send(addRequest:withObserver:error:)]
    pub unsafe fn add_request_with_observer_err<'ear, O: sn::ResultsObserving>(
        &mut self,
        request: &sn::Request,
        observer: &O,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn add_request_with_observer<'ear, O: sn::ResultsObserving>(
        &mut self,
        request: &sn::Request,
        observer: &O,
    ) -> Result<(), &'ear ns::Error> {
        ns::if_false(|err| unsafe { self.add_request_with_observer_err(request, observer, err) })
    }

    #[objc::msg_send(removeRequest:)]
    pub fn remove_request(&mut self, request: &sn::Request);

    #[objc::msg_send(removeAllRequests)]
    pub fn remove_all_requests(&mut self);

    #[objc::msg_send(analyzeAudioBuffer:atAudioFramePosition:)]
    pub fn analyze_audio_buf_at_pos(&mut self, audio_buf: &av::AudioBuf, pos: av::AudioFramePos);

    #[objc::msg_send(completeAnalysis)]
    pub fn complete_analysis(&mut self);
}

define_obj_type!(
    pub AudioFileAnalyzer(ns::Id),
    SN_AUDIO_FILE_ANALYZER
);

impl arc::A<AudioFileAnalyzer> {
    #[objc::msg_send(initWithURL:error:)]
    pub unsafe fn init_with_url_err<'ear>(
        self,
        url: &ns::Url,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<AudioFileAnalyzer>>;
}

impl AudioFileAnalyzer {
    pub fn with_url<'ear>(url: &ns::Url) -> Result<arc::R<Self>, &'ear ns::Error> {
        ns::if_none(|err| unsafe { Self::alloc().init_with_url_err(url, err) })
    }

    #[objc::msg_send(addRequest:withObserver:error:)]
    pub unsafe fn add_request_with_observer_err<'ear, O: sn::ResultsObserving>(
        &mut self,
        request: &sn::Request,
        observer: &O,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn add_request_with_observer<'ear, O: sn::ResultsObserving>(
        &mut self,
        request: &sn::Request,
        observer: &O,
    ) -> Result<(), &'ear ns::Error> {
        ns::if_false(|err| unsafe { self.add_request_with_observer_err(request, observer, err) })
    }

    #[objc::msg_send(removeRequest:)]
    pub fn remove_request(&mut self, request: &sn::Request);

    #[objc::msg_send(removeAllRequests)]
    pub fn remove_all_requests(&mut self);

    #[objc::msg_send(analyze)]
    pub fn analyze(&mut self);

    /// Analyzes the audio file asynchronously
    #[objc::msg_send(analyzeWithCompletionHandler:)]
    pub unsafe fn _analyze_with_ch(&mut self, handler: *mut std::ffi::c_void);

    pub fn analyze_with_ch<F>(&mut self, handler: &'static mut blocks::Block<F>)
    where
        F: FnOnce(bool),
    {
        unsafe { self._analyze_with_ch(handler.as_mut_ptr()) }
    }

    /// Analyzes the audio file asynchronously
    pub async fn analyze_with(&mut self) -> bool {
        let (future, block) = blocks::comp1();
        self.analyze_with_ch(block.escape());
        future.await
    }

    #[objc::msg_send(cancelAnalysis)]
    pub fn cancel_analysis(&mut self);
}

#[link(name = "sn", kind = "static")]
extern "C" {
    static SN_AUDIO_STREAM_ANALYZER: &'static objc::Class<AudioStreamAnalyzer>;
    static SN_AUDIO_FILE_ANALYZER: &'static objc::Class<AudioFileAnalyzer>;
}

#[cfg(test)]
mod tests {
    // use crate::{av, sn};

    #[test]
    fn basics() {
        // av::AudioFormat::standard_with_sample_rate_and_channel_layout(, )
        // sn::AudioStreamAnalyzer::with_format()
    }
}
