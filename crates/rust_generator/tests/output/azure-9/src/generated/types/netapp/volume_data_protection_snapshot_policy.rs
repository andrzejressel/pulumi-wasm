#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VolumeDataProtectionSnapshotPolicy {
    /// Resource ID of the snapshot policy to apply to the volume.
    /// 
    /// A full example of the `data_protection_snapshot_policy` attribute usage can be found in the `./examples/netapp/nfsv3_volume_with_snapshot_policy` directory within the GitHub Repository
    /// 
    /// > **NOTE:** `data_protection_snapshot_policy` block can be used alone or with data_protection_replication in the primary volume only, if enabling it in the secondary, an error will be thrown.
    #[builder(into)]
    #[serde(rename = "snapshotPolicyId")]
    pub r#snapshot_policy_id: Box<String>,
}
