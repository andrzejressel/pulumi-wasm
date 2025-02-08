#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EndpointConfigurationDataCaptureConfigCaptureOption {
    /// Specifies the data to be captured. Should be one of `Input`, `Output` or `InputAndOutput`.
    #[builder(into)]
    #[serde(rename = "captureMode")]
    pub r#capture_mode: Box<String>,
}
