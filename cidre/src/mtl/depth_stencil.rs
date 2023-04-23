use crate::{arc, define_mtl, define_obj_type, ns, objc};

#[doc(alias = "MTLCompareFunction")]
#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum CompareFn {
    /// A new value never passes the comparison test.
    Never = 0,
    /// A new value passes the comparison test if it is less than the existing value.
    Less = 1,
    /// A new value passes the comparison test if it is equal to the existing value.
    Equal = 2,
    /// A new value passes the comparison test if it is less than or equal to the existing value.
    LessEqual = 3,
    /// A new value passes the comparison test if it is greater than the existing value.
    Greater = 4,
    /// A new value passes the comparison test if it is not equal to the existing value.
    NotEqual = 5,
    /// A new value passes the comparison test if it is greater than or equal to the existing value.
    GreaterEqual = 6,
    /// A new value always passes the comparison test.
    Always = 7,
}

#[doc(alias = "MTLStencilOperation")]
#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum StencilOp {
    /// Keep the current stencil value.
    Keep = 0,
    /// Set the stencil value to zero.
    Zero = 1,
    /// Replace the stencil value with the stencil reference value, which is set by
    /// the set_stencil_reference_value method of mtl::RenderCmdEncoder.
    Replace = 2,
    /// If the current stencil value is not the maximum representable value, increase the stencil
    /// value by one. Otherwise, if the current stencil value is the maximum representable value,
    /// do not change the stencil value.
    IncrementClamp = 3,
    /// If the current stencil value is not zero, decrease the stencil value by one. Otherwise,
    /// if the current stencil value is zero, do not change the stencil value.
    DecrementClamp = 4,
    /// Perform a logical bitwise invert operation on the current stencil value.
    Invert = 5,
    /// If the current stencil value is not the maximum representable value, increase
    /// the stencil value by one. Otherwise, if the current stencil value is the maximum
    /// representable value, set the stencil value to zero.
    IncrementWrap = 6,
    /// If the current stencil value is not zero, decrease the stencil value by one.
    /// Otherwise, if the current stencil value is zero, set the stencil value to the maximum
    /// representable value.
    DecrementWrap = 7,
}

define_obj_type!(StencilDescriptor(ns::Id), MTL_STENCIL_DESCRIPTOR);

impl StencilDescriptor {
    /// The comparison that is performed between the masked reference value and a
    /// masked value in the stencil attachment.
    #[objc::msg_send(stencilCompareFunction)]
    pub fn compare_fn(&self) -> CompareFn;

    #[objc::msg_send(setStencilCompareFunction:)]
    pub fn set_compare_fn(&mut self, value: CompareFn);

    /// The operation that is performed to update the values in the stencil attachment
    /// when the stencil test fails.
    #[objc::msg_send(stencilFailureOperation)]
    pub fn failure_op(&self) -> StencilOp;

    #[objc::msg_send(setStencilFailureOperation:)]
    pub fn set_failure_op(&mut self, value: StencilOp);

    /// The operation that is performed to update the values in the stencil attachment
    /// when the stencil test passes, but the depth test fails.
    #[objc::msg_send(depthFailureOperation)]
    pub fn depth_failure_op(&self) -> StencilOp;

    #[objc::msg_send(setDepthFailureOperation:)]
    pub fn set_depth_failure_op(&mut self, value: StencilOp);

    /// The operation that is performed to update the values in the stencil attachment
    /// when both the stencil test and the depth test pass.
    #[objc::msg_send(depthStencilPassOperation)]
    pub fn depth_stencil_op(&self) -> StencilOp;

    #[objc::msg_send(setDepthStencilPassOperation:)]
    pub fn set_depth_stencil_op(&mut self, value: StencilOp);

    /// A bitmask that determines from which bits that stencil comparison tests can read.
    #[objc::msg_send(readMask)]
    pub fn read_mask(&self) -> u32;

    #[objc::msg_send(setReadMask:)]
    pub fn set_read_mask(&mut self, value: u32);

    /// A bitmask that determines to which bits that stencil operations can write.
    #[objc::msg_send(writeMask)]
    pub fn write_mask(&self) -> u32;

    #[objc::msg_send(setWriteMask:)]
    pub fn set_write_mask(&mut self, value: u32);
}

define_obj_type!(DepthStencilDescriptor(ns::Id), MTL_DEPTH_STENCIL_DESCRIPTOR);

impl DepthStencilDescriptor {
    /// Defaults to mtl::CompareFnAlways, which effectively skips the depth test
    #[objc::msg_send(depthCompareFunction)]
    pub fn compare_fn(&self) -> CompareFn;

    #[objc::msg_send(setDepthCompareFunction:)]
    pub fn set_compare_fn(&mut self, value: CompareFn);

    /// Defaults to false, so no depth writes are performed
    #[objc::msg_send(isDepthWriteEnabled)]
    pub fn is_depth_write_enabled(&self) -> bool;

    #[objc::msg_send(setDepthWriteEnabled:)]
    pub fn set_depth_write_enabled(&mut self, value: bool);

    #[objc::msg_send(frontFaceStencil)]
    pub fn front_face_stenil(&self) -> &StencilDescriptor;

    #[objc::msg_send(setFrontFaceStencil:)]
    fn _set_front_face_stencil(&mut self, value: Option<&StencilDescriptor>);

    #[inline]
    pub fn set_front_face_stencil(&mut self, value: &StencilDescriptor) {
        self._set_front_face_stencil(Some(value))
    }

    #[inline]
    pub fn reset_front_face_stencil(&mut self) {
        self._set_front_face_stencil(None)
    }

    #[objc::msg_send(backFaceStencil)]
    pub fn back_face_stencil(&self) -> &StencilDescriptor;

    #[objc::msg_send(setBackFaceStencil:)]
    fn _set_back_face_stencil(&mut self, value: Option<&StencilDescriptor>);

    #[inline]
    pub fn set_back_face_stencil(&mut self, value: &StencilDescriptor) {
        self._set_back_face_stencil(Some(value))
    }

    #[inline]
    pub fn reset_back_face_stencil(&mut self) {
        self._set_back_face_stencil(None)
    }

    define_mtl!(label, set_label);
}

define_obj_type!(State(ns::Id));

impl State {
    define_mtl!(label, device);
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_STENCIL_DESCRIPTOR: &'static objc::Class<StencilDescriptor>;
    static MTL_DEPTH_STENCIL_DESCRIPTOR: &'static objc::Class<DepthStencilDescriptor>;
}
