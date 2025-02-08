#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TaskDefinitionInferenceAccelerator {
    /// Elastic Inference accelerator device name. The deviceName must also be referenced in a container definition as a ResourceRequirement.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<String>,
    /// Elastic Inference accelerator type to use.
    #[builder(into)]
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<String>,
}
