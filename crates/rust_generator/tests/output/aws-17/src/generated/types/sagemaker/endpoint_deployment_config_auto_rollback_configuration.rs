#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointDeploymentConfigAutoRollbackConfiguration {
    /// List of CloudWatch alarms in your account that are configured to monitor metrics on an endpoint. If any alarms are tripped during a deployment, SageMaker rolls back the deployment. See Alarms.
    #[builder(into, default)]
    #[serde(rename = "alarms")]
    pub r#alarms: Box<Option<Vec<super::super::types::sagemaker::EndpointDeploymentConfigAutoRollbackConfigurationAlarm>>>,
}
