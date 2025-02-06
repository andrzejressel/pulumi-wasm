#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StaticWebLayerLoadBasedAutoScaling {
    #[builder(into, default)]
    #[serde(rename = "downscaling")]
    pub r#downscaling: Box<Option<super::super::types::opsworks::StaticWebLayerLoadBasedAutoScalingDownscaling>>,
    #[builder(into, default)]
    #[serde(rename = "enable")]
    pub r#enable: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "upscaling")]
    pub r#upscaling: Box<Option<super::super::types::opsworks::StaticWebLayerLoadBasedAutoScalingUpscaling>>,
}
