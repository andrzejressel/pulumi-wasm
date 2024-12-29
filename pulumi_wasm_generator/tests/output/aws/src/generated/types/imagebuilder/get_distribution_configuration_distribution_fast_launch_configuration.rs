#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDistributionConfigurationDistributionFastLaunchConfiguration {
    /// The account ID that this configuration applies to.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<String>,
    /// A Boolean that represents the current state of faster launching for the Windows AMI.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Nested list of launch templates that the fast-launch enabled Windows AMI uses when it launches Windows instances to create pre-provisioned snapshots.
    #[builder(into)]
    #[serde(rename = "launchTemplates")]
    pub r#launch_templates: Box<Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionFastLaunchConfigurationLaunchTemplate>>,
    /// The maximum number of parallel instances that are launched for creating resources.
    #[builder(into)]
    #[serde(rename = "maxParallelLaunches")]
    pub r#max_parallel_launches: Box<i32>,
    /// Nested list of configurations for managing the number of snapshots that are created from pre-provisioned instances for the Windows AMI when faster launching is enabled.
    #[builder(into)]
    #[serde(rename = "snapshotConfigurations")]
    pub r#snapshot_configurations: Box<Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionFastLaunchConfigurationSnapshotConfiguration>>,
}
