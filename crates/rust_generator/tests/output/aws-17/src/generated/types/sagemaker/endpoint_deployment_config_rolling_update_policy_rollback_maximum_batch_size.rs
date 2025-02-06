#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointDeploymentConfigRollingUpdatePolicyRollbackMaximumBatchSize {
    /// Specifies the endpoint capacity type. Valid values are: `INSTANCE_COUNT`, or `CAPACITY_PERCENT`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// Defines the capacity size, either as a number of instances or a capacity percentage.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<i32>,
}
