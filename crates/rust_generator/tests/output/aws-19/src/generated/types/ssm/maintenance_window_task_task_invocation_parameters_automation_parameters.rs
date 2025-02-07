#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MaintenanceWindowTaskTaskInvocationParametersAutomationParameters {
    /// The version of an Automation document to use during task execution.
    #[builder(into, default)]
    #[serde(rename = "documentVersion")]
    pub r#document_version: Box<Option<String>>,
    /// The parameters for the RUN_COMMAND task execution. Documented below.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<Vec<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersAutomationParametersParameter>>>,
}
