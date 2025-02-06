#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TopicRuleErrorActionStepFunctions {
    /// The prefix used to generate, along with a UUID, the unique state machine execution name.
    #[builder(into, default)]
    #[serde(rename = "executionNamePrefix")]
    pub r#execution_name_prefix: Box<Option<String>>,
    /// The ARN of the IAM role that grants access to start execution of the state machine.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// The name of the Step Functions state machine whose execution will be started.
    #[builder(into)]
    #[serde(rename = "stateMachineName")]
    pub r#state_machine_name: Box<String>,
}
