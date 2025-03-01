use crate::{arc, cm, define_obj_type, ns, objc, sn};

define_obj_type!(
    #[doc(alias = "SNClassification")]
    pub Classification(ns::Id)
);

impl Classification {
    #[objc::msg_send(identifier)]
    pub fn id(&self) -> arc::R<ns::String>;

    #[objc::msg_send(confidence)]
    pub fn confidence(&self) -> f64;
}

define_obj_type!(
    #[doc(alias = "SNClassificationResult")]
    pub ClassificationResult(sn::Result),
    SN_CLASSIFICATION_RESULT
);

impl ClassificationResult {
    #[objc::msg_send(classifications)]
    pub fn classifications(&self) -> arc::R<ns::Array<Classification>>;

    #[objc::msg_send(timeRange)]
    pub fn time_range(&self) -> cm::TimeRange;

    #[objc::msg_send(classificationForIdentifier:)]
    pub fn classification_for_id(&self, id: &ns::String) -> Option<arc::R<Classification>>;
}

unsafe extern "C" {
    static SN_CLASSIFICATION_RESULT: &'static objc::Class<ClassificationResult>;
}
