#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomLayerLoadBasedAutoScaling {
    /// The downscaling settings, as defined below, used for load-based autoscaling
    #[builder(into, default)]
    #[serde(rename = "downscaling")]
    pub r#downscaling: Box<Option<super::super::types::opsworks::CustomLayerLoadBasedAutoScalingDownscaling>>,
    /// Whether load-based auto scaling is enabled for the layer.
    #[builder(into, default)]
    #[serde(rename = "enable")]
    pub r#enable: Box<Option<bool>>,
    /// The upscaling settings, as defined below, used for load-based autoscaling
    #[builder(into, default)]
    #[serde(rename = "upscaling")]
    pub r#upscaling: Box<Option<super::super::types::opsworks::CustomLayerLoadBasedAutoScalingUpscaling>>,
}