#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VolumeGroupSapHanaVolumeDataProtectionSnapshotPolicy {
    /// Resource ID of the snapshot policy to apply to the volume.
    #[builder(into)]
    #[serde(rename = "snapshotPolicyId")]
    pub r#snapshot_policy_id: Box<String>,
}
