#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeploymentConfigZonalConfigMinimumHealthyHostsPerZone {
    /// The type can either be `FLEET_PERCENT` or `HOST_COUNT`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
    /// The value when the type is `FLEET_PERCENT` represents the minimum number of healthy instances as a percentage of the total number of instances in the Availability Zone during a deployment. If you specify FLEET_PERCENT, at the start of the deployment, AWS CodeDeploy converts the percentage to the equivalent number of instance and rounds up fractional instances. When the type is `HOST_COUNT`, the value represents the minimum number of healthy instances in the Availability Zone as an absolute value.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<i32>>,
}
