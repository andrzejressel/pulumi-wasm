#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetVolumeGroupSapHanaVolume {
    /// The ID of the Capacity Pool.
    #[builder(into)]
    #[serde(rename = "capacityPoolId")]
    pub r#capacity_pool_id: Box<String>,
    /// A `data_protection_replication` block as defined below.
    #[builder(into)]
    #[serde(rename = "dataProtectionReplications")]
    pub r#data_protection_replications: Box<Vec<super::super::types::netapp::GetVolumeGroupSapHanaVolumeDataProtectionReplication>>,
    /// A `data_protection_snapshot_policy` block as defined below.
    #[builder(into)]
    #[serde(rename = "dataProtectionSnapshotPolicies")]
    pub r#data_protection_snapshot_policies: Box<Vec<super::super::types::netapp::GetVolumeGroupSapHanaVolumeDataProtectionSnapshotPolicy>>,
    /// A `export_policy_rule` block as defined below.
    #[builder(into)]
    #[serde(rename = "exportPolicyRules")]
    pub r#export_policy_rules: Box<Vec<super::super::types::netapp::GetVolumeGroupSapHanaVolumeExportPolicyRule>>,
    /// Volume ID.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// A `mount_ip_addresses` block as defined below.
    #[builder(into)]
    #[serde(rename = "mountIpAddresses")]
    pub r#mount_ip_addresses: Box<Vec<String>>,
    /// The name of this Application Volume Group for SAP HANA application.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A `protocols` block as defined below.
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Box<Vec<String>>,
    /// The ID of the proximity placement group.
    #[builder(into)]
    #[serde(rename = "proximityPlacementGroupId")]
    pub r#proximity_placement_group_id: Box<String>,
    /// Volume security style.
    #[builder(into)]
    #[serde(rename = "securityStyle")]
    pub r#security_style: Box<String>,
    /// The target performance of the file system.
    #[builder(into)]
    #[serde(rename = "serviceLevel")]
    pub r#service_level: Box<String>,
    /// Is the .snapshot (NFS clients) path of a volume visible?
    #[builder(into)]
    #[serde(rename = "snapshotDirectoryVisible")]
    pub r#snapshot_directory_visible: Box<bool>,
    /// The maximum Storage Quota allowed for a file system in Gigabytes.
    #[builder(into)]
    #[serde(rename = "storageQuotaInGb")]
    pub r#storage_quota_in_gb: Box<i32>,
    /// The ID of the Subnet the NetApp Volume resides in.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
    /// A mapping of tags assigned to the Application Volume Group.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<std::collections::HashMap<String, String>>,
    /// Throughput of this volume in Mibps.
    #[builder(into)]
    #[serde(rename = "throughputInMibps")]
    pub r#throughput_in_mibps: Box<f64>,
    /// A unique file path for the volume.
    #[builder(into)]
    #[serde(rename = "volumePath")]
    pub r#volume_path: Box<String>,
    /// Volume spec name.
    #[builder(into)]
    #[serde(rename = "volumeSpecName")]
    pub r#volume_spec_name: Box<String>,
}
