#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDbNodesDbNode {
    /// Additional information about the planned maintenance.
    #[builder(into)]
    #[serde(rename = "additionalDetails")]
    pub r#additional_details: Box<String>,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the backup IP address associated with the database node. Use this OCID with either the [GetPrivateIp](https://docs.cloud.oracle.com/iaas/api/#/en/iaas/20160918/PrivateIp/GetPrivateIp) or the [GetPublicIpByPrivateIpId](https://docs.cloud.oracle.com/iaas/api/#/en/iaas/20160918/PublicIp/GetPublicIpByPrivateIpId) API to get the IP address needed to make a database connection.
    #[builder(into)]
    #[serde(rename = "backupIpId")]
    pub r#backup_ip_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "backupVnic2Id")]
    pub r#backup_vnic_2_id: Box<String>,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the backup VNIC.
    #[builder(into)]
    #[serde(rename = "backupVnicId")]
    pub r#backup_vnic_id: Box<String>,
    /// The number of CPU cores enabled on the DB node.
    #[builder(into)]
    #[serde(rename = "cpuCoreCount")]
    pub r#cpu_core_count: Box<i32>,
    /// The allocated local node storage in GBs on the DB node.
    #[builder(into)]
    #[serde(rename = "dbNodeStorageSizeInGbs")]
    pub r#db_node_storage_size_in_gbs: Box<i32>,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the ExaCC DB server associated with the database node.
    #[builder(into)]
    #[serde(rename = "dbServerId")]
    pub r#db_server_id: Box<String>,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the DB system.
    #[builder(into)]
    #[serde(rename = "dbSystemId")]
    pub r#db_system_id: Box<String>,
    /// The name of the Fault Domain the instance is contained in.
    #[builder(into)]
    #[serde(rename = "faultDomain")]
    pub r#fault_domain: Box<String>,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the host IP address associated with the database node. Use this OCID with either the [GetPrivateIp](https://docs.cloud.oracle.com/iaas/api/#/en/iaas/20160918/PrivateIp/GetPrivateIp) or the [GetPublicIpByPrivateIpId](https://docs.cloud.oracle.com/iaas/api/#/en/iaas/20160918/PublicIp/GetPublicIpByPrivateIpId) API to get the IP address needed to make a database connection.
    #[builder(into)]
    #[serde(rename = "hostIpId")]
    pub r#host_ip_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<String>,
    /// Information about the current lifecycle details.
    #[builder(into)]
    #[serde(rename = "lifecycleDetails")]
    pub r#lifecycle_details: Box<String>,
    /// Information about the current lifecycle state.
    #[builder(into)]
    #[serde(rename = "lifecycleState")]
    pub r#lifecycle_state: Box<String>,
    /// The type of database node maintenance.
    #[builder(into)]
    #[serde(rename = "maintenanceType")]
    pub r#maintenance_type: Box<String>,
    /// The allocated memory in GBs on the DB Node.
    #[builder(into)]
    #[serde(rename = "memorySizeInGbs")]
    pub r#memory_size_in_gbs: Box<i32>,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the DB node.
    #[builder(into)]
    #[serde(rename = "ocid")]
    pub r#ocid: Box<String>,
    /// The size (in GB) of the block storage volume allocation for the DB system. This attribute applies only for virtual machine DB systems.
    #[builder(into)]
    #[serde(rename = "softwareStorageSizeInGb")]
    pub r#software_storage_size_in_gb: Box<i32>,
    /// The date and time that the DB node was created.
    #[builder(into)]
    #[serde(rename = "timeCreated")]
    pub r#time_created: Box<String>,
    /// End date and time of maintenance window.
    #[builder(into)]
    #[serde(rename = "timeMaintenanceWindowEnd")]
    pub r#time_maintenance_window_end: Box<String>,
    /// Start date and time of maintenance window.
    #[builder(into)]
    #[serde(rename = "timeMaintenanceWindowStart")]
    pub r#time_maintenance_window_start: Box<String>,
    #[builder(into)]
    #[serde(rename = "vnic2Id")]
    pub r#vnic_2_id: Box<String>,
    /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the VNIC.
    #[builder(into)]
    #[serde(rename = "vnicId")]
    pub r#vnic_id: Box<String>,
}
