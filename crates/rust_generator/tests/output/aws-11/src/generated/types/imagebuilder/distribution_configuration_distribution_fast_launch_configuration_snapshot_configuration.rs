#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DistributionConfigurationDistributionFastLaunchConfigurationSnapshotConfiguration {
    /// The number of pre-provisioned snapshots to keep on hand for a fast-launch enabled Windows AMI.
    #[builder(into, default)]
    #[serde(rename = "targetResourceCount")]
    pub r#target_resource_count: Box<Option<i32>>,
}
