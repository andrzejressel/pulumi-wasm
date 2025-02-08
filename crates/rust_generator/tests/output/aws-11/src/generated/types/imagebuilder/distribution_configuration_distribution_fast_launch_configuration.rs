#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DistributionConfigurationDistributionFastLaunchConfiguration {
    /// The owner account ID for the fast-launch enabled Windows AMI.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<String>,
    /// A Boolean that represents the current state of faster launching for the Windows AMI. Set to `true` to start using Windows faster launching, or `false` to stop using it.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Configuration block for the launch template that the fast-launch enabled Windows AMI uses when it launches Windows instances to create pre-provisioned snapshots. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "launchTemplate")]
    pub r#launch_template: Box<Option<super::super::types::imagebuilder::DistributionConfigurationDistributionFastLaunchConfigurationLaunchTemplate>>,
    /// The maximum number of parallel instances that are launched for creating resources.
    #[builder(into, default)]
    #[serde(rename = "maxParallelLaunches")]
    pub r#max_parallel_launches: Box<Option<i32>>,
    /// Configuration block for managing the number of snapshots that are created from pre-provisioned instances for the Windows AMI when faster launching is enabled. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "snapshotConfiguration")]
    pub r#snapshot_configuration: Box<Option<super::super::types::imagebuilder::DistributionConfigurationDistributionFastLaunchConfigurationSnapshotConfiguration>>,
}
