use crate::{arc, define_cls, define_obj_type, mlc, ns, objc};

define_obj_type!(ActivationDescriptor(ns::Id));
impl ActivationDescriptor {
    define_cls!(MLC_ACTIVATION_DESCRIPTOR);

    #[objc::msg_send(activationType)]
    pub fn activation_type(&self) -> mlc::ActivationType;

    #[objc::msg_send(a)]
    pub fn a(&self) -> f32;

    #[objc::msg_send(b)]
    pub fn b(&self) -> f32;

    #[objc::msg_send(c)]
    pub fn c(&self) -> f32;

    #[objc::cls_msg_send(descriptorWithType:)]
    pub fn with_type_ar(activation_type: mlc::ActivationType) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_type(activation_type: mlc::ActivationType) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(descriptorWithType:a:)]
    pub fn with_type_a_ar(activation_type: mlc::ActivationType, a: f32) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_type_a(activation_type: mlc::ActivationType, a: f32) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(descriptorWithType:a:b:)]
    pub fn with_type_ab_ar(
        activation_type: mlc::ActivationType,
        a: f32,
        b: f32,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_type_ab(
        activation_type: mlc::ActivationType,
        a: f32,
        b: f32,
    ) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(descriptorWithType:a:b:c:)]
    pub fn with_type_abc_ar(
        activation_type: mlc::ActivationType,
        a: f32,
        b: f32,
        c: f32,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_type_abc(
        activation_type: mlc::ActivationType,
        a: f32,
        b: f32,
        c: f32,
    ) -> Option<arc::R<Self>>;
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_ACTIVATION_DESCRIPTOR: &'static objc::Class<ActivationDescriptor>;
}

#[cfg(test)]
mod tests {
    use crate::mlc;

    #[test]
    fn basics() {
        let desc = mlc::ActivationDescriptor::with_type(mlc::ActivationType::ReLU).unwrap();
        assert_eq!(desc.a(), 0.0);
        assert_eq!(desc.b(), 1.0);
        assert_eq!(desc.c(), 1.0);
    }
}
