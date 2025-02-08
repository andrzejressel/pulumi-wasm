#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MemcachedLayerLoadBasedAutoScaling {
    #[builder(into, default)]
    #[serde(rename = "downscaling")]
    pub r#downscaling: Box<Option<super::super::types::opsworks::MemcachedLayerLoadBasedAutoScalingDownscaling>>,
    #[builder(into, default)]
    #[serde(rename = "enable")]
    pub r#enable: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "upscaling")]
    pub r#upscaling: Box<Option<super::super::types::opsworks::MemcachedLayerLoadBasedAutoScalingUpscaling>>,
}
