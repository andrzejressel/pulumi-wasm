#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VolumeGroupSapHanaVolume {
    /// The ID of the Capacity Pool. Changing this forces a new Application Volume Group to be created and data will be lost.
    #[builder(into)]
    #[serde(rename = "capacityPoolId")]
    pub r#capacity_pool_id: Box<String>,
    /// A `data_protection_replication` block as defined below. Changing this forces a new Application Volume Group to be created and data will be lost.
    #[builder(into, default)]
    #[serde(rename = "dataProtectionReplication")]
    pub r#data_protection_replication: Box<Option<super::super::types::netapp::VolumeGroupSapHanaVolumeDataProtectionReplication>>,
    /// A `data_protection_snapshot_policy` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "dataProtectionSnapshotPolicy")]
    pub r#data_protection_snapshot_policy: Box<Option<super::super::types::netapp::VolumeGroupSapHanaVolumeDataProtectionSnapshotPolicy>>,
    /// One or more `export_policy_rule` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "exportPolicyRules")]
    pub r#export_policy_rules: Box<Vec<super::super::types::netapp::VolumeGroupSapHanaVolumeExportPolicyRule>>,
    /// The ID of the Application Volume Group.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "mountIpAddresses")]
    pub r#mount_ip_addresses: Box<Option<Vec<String>>>,
    /// The name which should be used for this volume. Changing this forces a new Application Volume Group to be created and data will be lost.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The target volume protocol expressed as a list. Changing this forces a new Application Volume Group to be created and data will be lost. Supported values for Application Volume Group include `NFSv3` or `NFSv4.1`, multi-protocol is not supported and there are certain rules on which protocol is supporteed per volume spec, please check [Configure application volume groups for the SAP HANA REST API](https://learn.microsoft.com/en-us/azure/azure-netapp-files/configure-application-volume-group-sap-hana-api) document for details.
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Box<String>,
    /// The ID of the proximity placement group. Changing this forces a new Application Volume Group to be created and data will be lost. For SAP-HANA application, it is required to have PPG enabled so Azure NetApp Files can pin the volumes next to your compute resources, please check [Requirements and considerations for application volume group for SAP HANA](https://learn.microsoft.com/en-us/azure/azure-netapp-files/application-volume-group-considerations) for details and other requirements.
    #[builder(into, default)]
    #[serde(rename = "proximityPlacementGroupId")]
    pub r#proximity_placement_group_id: Box<Option<String>>,
    /// Volume security style. Possible values are `ntfs` and `unix`. Changing this forces a new Application Volume Group to be created and data will be lost.
    #[builder(into)]
    #[serde(rename = "securityStyle")]
    pub r#security_style: Box<String>,
    /// Volume security style. Possible values are `Premium`, `Standard` and `Ultra`. Changing this forces a new Application Volume Group to be created and data will be lost.
    #[builder(into)]
    #[serde(rename = "serviceLevel")]
    pub r#service_level: Box<String>,
    /// Specifies whether the .snapshot (NFS clients) path of a volume is visible. Changing this forces a new Application Volume Group to be created and data will be lost.
    #[builder(into)]
    #[serde(rename = "snapshotDirectoryVisible")]
    pub r#snapshot_directory_visible: Box<bool>,
    /// The maximum Storage Quota allowed for a file system in Gigabytes.
    #[builder(into)]
    #[serde(rename = "storageQuotaInGb")]
    pub r#storage_quota_in_gb: Box<i32>,
    /// The ID of the Subnet the NetApp Volume resides in, which must have the `Microsoft.NetApp/volumes` delegation. Changing this forces a new Application Volume Group to be created and data will be lost.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
    /// A mapping of tags which should be assigned to the Application Volume Group.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// Throughput of this volume in Mibps.
    #[builder(into)]
    #[serde(rename = "throughputInMibps")]
    pub r#throughput_in_mibps: Box<f64>,
    /// A unique file path for the volume. Changing this forces a new Application Volume Group to be created and data will be lost.
    #[builder(into)]
    #[serde(rename = "volumePath")]
    pub r#volume_path: Box<String>,
    /// Volume specification name. Possible values are `data`, `log`, `shared`, `data-backup` and `log-backup`. Changing this forces a new Application Volume Group to be created and data will be lost.
    #[builder(into)]
    #[serde(rename = "volumeSpecName")]
    pub r#volume_spec_name: Box<String>,
}
