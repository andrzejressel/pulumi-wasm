#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointDeploymentConfigAutoRollbackConfigurationAlarm {
    /// The name of a CloudWatch alarm in your account.
    #[builder(into)]
    #[serde(rename = "alarmName")]
    pub r#alarm_name: Box<String>,
}
