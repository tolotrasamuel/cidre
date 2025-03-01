use crate::{arc, define_cls, define_obj_type, mlc, ns, objc};

define_obj_type!(pub Desc(ns::Id));
impl Desc {
    define_cls!(MLC_MATMUL_DESCRIPTOR);

    #[objc::msg_send(alpha)]
    pub fn alpha(&self) -> f32;

    #[objc::msg_send(transposesX)]
    pub fn transposes_x(&self) -> bool;

    #[objc::msg_send(transposesY)]
    pub fn transposes_y(&self) -> bool;

    #[objc::msg_send(descriptor)]
    pub fn desc() -> arc::R<Self>;

    #[objc::msg_send(descriptorWithAlpha:transposesX:transposesY:)]
    pub fn with_alpha(alpha: f32, transposes_x: bool, transposes_y: bool) -> Option<arc::R<Self>>;

    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::desc()
    }
}

define_obj_type!(
    #[doc(alias = "MLCLayer")]
    pub Layer(mlc::Layer)
);

impl Layer {
    define_cls!(MLC_MATMUL_LAYER);

    #[objc::msg_send(layerWithDescriptor:)]
    pub fn with_desc(desc: &Desc) -> Option<arc::R<Self>>;

    #[objc::msg_send(descriptor)]
    pub fn desc(&self) -> &Desc;

    pub fn new() -> arc::R<Self> {
        Self::with_desc(&Desc::new()).unwrap()
    }
}

#[link(name = "mlc", kind = "static")]
unsafe extern "C" {
    static MLC_MATMUL_DESCRIPTOR: &'static objc::Class<Desc>;
    static MLC_MATMUL_LAYER: &'static objc::Class<Layer>;
}
