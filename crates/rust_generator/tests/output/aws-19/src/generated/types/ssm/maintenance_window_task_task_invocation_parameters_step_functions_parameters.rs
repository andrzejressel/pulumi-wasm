#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MaintenanceWindowTaskTaskInvocationParametersStepFunctionsParameters {
    /// The inputs for the STEP_FUNCTION task.
    #[builder(into, default)]
    #[serde(rename = "input")]
    pub r#input: Box<Option<String>>,
    /// The name of the STEP_FUNCTION task.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
