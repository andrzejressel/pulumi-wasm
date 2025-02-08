#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MaintenanceWindowTaskTaskInvocationParameters {
    /// The parameters for an AUTOMATION task type. Documented below.
    #[builder(into, default)]
    #[serde(rename = "automationParameters")]
    pub r#automation_parameters: Box<Option<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersAutomationParameters>>,
    /// The parameters for a LAMBDA task type. Documented below.
    #[builder(into, default)]
    #[serde(rename = "lambdaParameters")]
    pub r#lambda_parameters: Box<Option<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersLambdaParameters>>,
    /// The parameters for a RUN_COMMAND task type. Documented below.
    #[builder(into, default)]
    #[serde(rename = "runCommandParameters")]
    pub r#run_command_parameters: Box<Option<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersRunCommandParameters>>,
    /// The parameters for a STEP_FUNCTIONS task type. Documented below.
    #[builder(into, default)]
    #[serde(rename = "stepFunctionsParameters")]
    pub r#step_functions_parameters: Box<Option<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersStepFunctionsParameters>>,
}
