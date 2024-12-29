#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StateMachineLoggingConfiguration {
    /// Determines whether execution data is included in your log. When set to `false`, data is excluded.
    #[builder(into, default)]
    #[serde(rename = "includeExecutionData")]
    pub r#include_execution_data: Box<Option<bool>>,
    /// Defines which category of execution history events are logged. Valid values: `ALL`, `ERROR`, `FATAL`, `OFF`
    #[builder(into, default)]
    #[serde(rename = "level")]
    pub r#level: Box<Option<String>>,
    /// Amazon Resource Name (ARN) of a CloudWatch log group. Make sure the State Machine has the correct IAM policies for logging. The ARN must end with `:*`
    #[builder(into, default)]
    #[serde(rename = "logDestination")]
    pub r#log_destination: Box<Option<String>>,
}
