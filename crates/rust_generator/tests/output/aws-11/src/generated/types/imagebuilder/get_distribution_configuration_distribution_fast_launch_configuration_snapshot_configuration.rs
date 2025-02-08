#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDistributionConfigurationDistributionFastLaunchConfigurationSnapshotConfiguration {
    /// The number of pre-provisioned snapshots to keep on hand for a fast-launch enabled Windows AMI.
    #[builder(into)]
    #[serde(rename = "targetResourceCount")]
    pub r#target_resource_count: Box<i32>,
}
