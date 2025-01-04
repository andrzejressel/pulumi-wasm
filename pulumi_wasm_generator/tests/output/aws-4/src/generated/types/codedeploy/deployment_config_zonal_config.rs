#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeploymentConfigZonalConfig {
    /// The period of time, in seconds, that CodeDeploy must wait after completing a deployment to the first Availability Zone. CodeDeploy will wait this amount of time before starting a deployment to the second Availability Zone. If you don't specify a value for `first_zone_monitor_duration_in_seconds`, then CodeDeploy uses the `monitor_duration_in_seconds` value for the first Availability Zone.
    #[builder(into, default)]
    #[serde(rename = "firstZoneMonitorDurationInSeconds")]
    pub r#first_zone_monitor_duration_in_seconds: Box<Option<i32>>,
    /// The number or percentage of instances that must remain available per Availability Zone during a deployment. If you don't specify a value under `minimum_healthy_hosts_per_zone`, then CodeDeploy uses a default value of 0 percent. This block is more documented below.
    #[builder(into, default)]
    #[serde(rename = "minimumHealthyHostsPerZone")]
    pub r#minimum_healthy_hosts_per_zone: Box<Option<super::super::types::codedeploy::DeploymentConfigZonalConfigMinimumHealthyHostsPerZone>>,
    /// The period of time, in seconds, that CodeDeploy must wait after completing a deployment to an Availability Zone. CodeDeploy will wait this amount of time before starting a deployment to the next Availability Zone. If you don't specify a `monitor_duration_in_seconds`, CodeDeploy starts deploying to the next Availability Zone immediately.
    #[builder(into, default)]
    #[serde(rename = "monitorDurationInSeconds")]
    pub r#monitor_duration_in_seconds: Box<Option<i32>>,
}
