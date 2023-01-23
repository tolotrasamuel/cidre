use crate::{arc, define_obj_type, ns, objc, os};

use super::{mixer_node::MixerNode, ConnectionPoint, Format, InputNode, Node, NodeBus, OutputNode};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ManualRenderingError(pub os::Status);

impl ManualRenderingError {
    /// The operation cannot be performed because the engine is either not in manual
    /// rendering mode or the right variant of it.
    pub const INVALID_MODE: Self = Self(os::Status(-80800));

    /// The operation cannot be performed because the engine is initialized (i.e. not stopped).
    pub const INITIALIZED: Self = Self(os::Status(-80801));

    /// The operation cannot be performed because the engine is not running (i.e. not started).
    pub const NOT_RUNNING: Self = Self(os::Status(-80801));
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(isize)]
pub enum ManualRenderingStatus {
    /// An error occurred when rendering and no data was returned. See the returned error code
    /// for the description of the error.
    Error = -1,
    /// All of the requested data was returned successfully.
    Succes = 0,
    /// Applicable only to the input node, when it provides input data for rendering
    /// (see `AVAudioInputNode(setManualRenderingInputPCMFormat:inputBlock:)`).
    /// Indicates that not enough input data was returned by the input node to satisfy the
    /// render request at the current time. The output buffer may contain data rendered by other
    /// active sources in the engine's processing graph.
    InsufficientDataFromInputNode = 1,
    /// The operation could not be performed now, but the client could retry later if needed.
    /// This is usually to guard a realtime render operation (e.g. rendering through
    /// `manualRenderingBlock`) when a reconfiguration of the engine's internal state
    /// is in progress.
    CannotDoInCurrentContext = 2,
}

/// By default, the engine is connected to an audio device and automatically renders in realtime.
/// It can also be configured to operate in manual rendering mode, i.e. not connected to an
/// audio device and rendering in response to requests from the client.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(isize)]
pub enum ManualRenderingMode {
    /// The engine operates in an offline mode without any realtime constraints.
    ManualRenderingModeOffline = 0,
    /// The engine operates under realtime constraints, i.e. it will not make any blocking call
    ///	(e.g. calling libdispatch, blocking on a mutex, allocating memory etc.) while rendering.
    /// Note that only the block based render mechanism can be used in this mode
    /// (see `AVAudioEngine(manualRenderingBlock)`.
    ManualRenderingModeRealtime = 1,
}

define_obj_type!(Engine(ns::Id), AV_AUDIO_ENGINE);

/// An AVAudioEngine contains a group of connected AVAudioNodes ("nodes"), each of which performs
/// an audio signal generation, processing, or input/output task.
///
/// Nodes are created separately and attached to the engine.
///
/// The engine supports dynamic connection, disconnection and removal of nodes while running,
/// with only minor limitations:
/// - all dynamic reconnections must occur upstream of a mixer
/// - while removals of effects will normally result in the automatic connection of the adjacent
///   nodes, removal of a node which has differing input vs. output channel counts, or which
///   is a mixer, is likely to result in a broken graph.
///
/// By default, the engine is connected to an audio device and automatically renders in realtime.
/// It can also be configured to operate in manual rendering mode, i.e. not connected to an
/// audio device and rendering in response to requests from the client, normally at or
/// faster than realtime rate.
impl Engine {
    #[objc::msg_send(attachNode:)]
    pub fn attach_node(&self, node: &Node);

    #[objc::msg_send(detachNode:)]
    pub fn detach_node(&self, node: &Node);

    #[objc::msg_send(connect:to:fromBus:toBus:format:)]
    pub fn connect_node_to_node_bus_to_bus(
        &self,
        node_from: &Node,
        node_to: &Node,
        from_bus: NodeBus,
        to_bus: NodeBus,
        format: Option<&Format>,
    );

    #[objc::msg_send(connect:to:format:)]
    pub fn connect_node_to_node(&self, node_from: &Node, node_to: &Node, format: Option<&Format>);

    #[objc::msg_send(connect:toConnectionPoints:fromBus:format:)]
    pub fn connect_node_to_connection_points_from_bus(
        &self,
        node: &Node,
        connection_pods: &ns::Array<ConnectionPoint>,
        from_bus: NodeBus,
        format: Option<&Format>,
    );

    #[objc::msg_send(disconnectNodeInput:bus:)]
    pub fn disconnect_node_input_bus(&self, node: &Node, bus: NodeBus);

    #[objc::msg_send(disconnectNodeInput:)]
    pub fn disconnect_node_input(&self, node: &Node);

    #[objc::msg_send(disconnectNodeOutput:bus:)]
    pub fn disconnect_node_output_bus(&self, node: &Node, bus: NodeBus);

    #[objc::msg_send(disconnectNodeOutput:)]
    pub fn disconnect_node_output(&self, node: &Node);

    #[objc::msg_send(prepare)]
    pub fn prepare(&self);

    #[objc::msg_send(startAndReturnError:)]
    pub fn start_and_return_error<'ar>(&self, error: &mut Option<&'ar ns::Error>) -> bool;

    #[inline]
    pub fn start<'ar>(&self) -> Result<(), &'ar ns::Error> {
        unsafe {
            let mut error = None;
            let res = self.start_and_return_error(&mut error);
            if res {
                Ok(())
            } else {
                Err(error.unwrap_unchecked())
            }
        }
    }

    /// ```no_run
    /// use cidre::av;
    ///
    /// let engine = av::audio::Engine::new();
    /// let input_node = engine.input_node();
    /// let en = input_node.engine().expect("engine");
    /// ```
    #[objc::msg_send(inputNode)]
    pub fn input_node(&self) -> &InputNode;

    /// ```no_run
    /// use cidre::av;
    ///
    /// let engine = av::audio::Engine::new();
    /// let output_node = engine.output_node();
    ///
    /// ```
    #[objc::msg_send(outputNode)]
    pub fn output_node(&self) -> &OutputNode;

    #[objc::msg_send(mainMixerNode)]
    pub fn main_mixer_node(&self) -> &MixerNode;

    #[objc::msg_send(reset)]
    pub fn reset(&self);

    #[objc::msg_send(pause)]
    pub fn pause(&self);

    #[objc::msg_send(stop)]
    pub fn stop(&self);

    #[objc::msg_send(isRunning)]
    pub fn is_running(&self) -> bool;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_AUDIO_ENGINE: &'static objc::Class<Engine>;
}

#[cfg(test)]
mod tests {
    use crate::av;

    #[test]
    fn basics() {
        let engine = av::audio::Engine::new();
        assert!(!engine.is_running());
        let _output_node = engine.output_node();
        let input_node = engine.input_node();
        let _en = input_node.engine().expect("engine");
    }
}
