#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuntimeVirtualMachineVirtualMachineConfigAcceleratorConfig {
    /// Count of cores of this accelerator.
    #[builder(into, default)]
    #[serde(rename = "coreCount")]
    pub r#core_count: Box<Option<i32>>,
    /// Accelerator model. For valid values, see
    /// `https://cloud.google.com/vertex-ai/docs/workbench/reference/
    /// rest/v1/projects.locations.runtimes#AcceleratorType`
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
