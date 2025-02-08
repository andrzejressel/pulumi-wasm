#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetCloudVmClustersCloudVmClusterProperty {
    /// OCI Cluster name.
    #[builder(into)]
    #[serde(rename = "clusterName")]
    pub r#cluster_name: Box<String>,
    /// Compartment ID of cluster.
    #[builder(into)]
    #[serde(rename = "compartmentId")]
    pub r#compartment_id: Box<String>,
    /// Number of enabled CPU cores.
    #[builder(into)]
    #[serde(rename = "cpuCoreCount")]
    pub r#cpu_core_count: Box<i32>,
    /// The data disk group size to be allocated in TBs.
    #[builder(into)]
    #[serde(rename = "dataStorageSizeTb")]
    pub r#data_storage_size_tb: Box<f64>,
    /// Local storage per VM
    #[builder(into)]
    #[serde(rename = "dbNodeStorageSizeGb")]
    pub r#db_node_storage_size_gb: Box<i32>,
    /// OCID of database servers.
    #[builder(into)]
    #[serde(rename = "dbServerOcids")]
    pub r#db_server_ocids: Box<Vec<String>>,
    /// Data collection options for diagnostics.
    #[builder(into)]
    #[serde(rename = "diagnosticsDataCollectionOptions")]
    pub r#diagnostics_data_collection_options: Box<Vec<super::super::types::oracledatabase::GetCloudVmClustersCloudVmClusterPropertyDiagnosticsDataCollectionOption>>,
    /// The type of redundancy. 
    ///  Possible values:
    ///  DISK_REDUNDANCY_UNSPECIFIED
    /// HIGH
    /// NORMAL
    #[builder(into)]
    #[serde(rename = "diskRedundancy")]
    pub r#disk_redundancy: Box<String>,
    /// DNS listener IP.
    #[builder(into)]
    #[serde(rename = "dnsListenerIp")]
    pub r#dns_listener_ip: Box<String>,
    /// Parent DNS domain where SCAN DNS and hosts names are qualified.
    /// ex: ocispdelegated.ocisp10jvnet.oraclevcn.com
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: Box<String>,
    /// Grid Infrastructure Version.
    #[builder(into)]
    #[serde(rename = "giVersion")]
    pub r#gi_version: Box<String>,
    /// host name without domain.
    /// format: "-" with some suffix.
    /// ex: sp2-yi0xq where "sp2" is the hostname_prefix.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<String>,
    /// Prefix for VM cluster host names.
    #[builder(into)]
    #[serde(rename = "hostnamePrefix")]
    pub r#hostname_prefix: Box<String>,
    /// License type of VM Cluster. 
    ///  Possible values:
    ///  LICENSE_TYPE_UNSPECIFIED
    /// LICENSE_INCLUDED
    /// BRING_YOUR_OWN_LICENSE
    #[builder(into)]
    #[serde(rename = "licenseType")]
    pub r#license_type: Box<String>,
    /// Use local backup.
    #[builder(into)]
    #[serde(rename = "localBackupEnabled")]
    pub r#local_backup_enabled: Box<bool>,
    /// Memory allocated in GBs.
    #[builder(into)]
    #[serde(rename = "memorySizeGb")]
    pub r#memory_size_gb: Box<i32>,
    /// Number of database servers.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Box<i32>,
    /// Deep link to the OCI console to view this resource.
    #[builder(into)]
    #[serde(rename = "ociUrl")]
    pub r#oci_url: Box<String>,
    /// Oracle Cloud Infrastructure ID of VM Cluster.
    #[builder(into)]
    #[serde(rename = "ocid")]
    pub r#ocid: Box<String>,
    /// OCPU count per VM. Minimum is 0.1.
    #[builder(into)]
    #[serde(rename = "ocpuCount")]
    pub r#ocpu_count: Box<f64>,
    /// SCAN DNS name.
    /// ex: sp2-yi0xq-scan.ocispdelegated.ocisp10jvnet.oraclevcn.com
    #[builder(into)]
    #[serde(rename = "scanDns")]
    pub r#scan_dns: Box<String>,
    /// OCID of scan DNS record.
    #[builder(into)]
    #[serde(rename = "scanDnsRecordId")]
    pub r#scan_dns_record_id: Box<String>,
    /// OCIDs of scan IPs.
    #[builder(into)]
    #[serde(rename = "scanIpIds")]
    pub r#scan_ip_ids: Box<Vec<String>>,
    /// SCAN listener port - TCP
    #[builder(into)]
    #[serde(rename = "scanListenerPortTcp")]
    pub r#scan_listener_port_tcp: Box<i32>,
    /// SCAN listener port - TLS
    #[builder(into)]
    #[serde(rename = "scanListenerPortTcpSsl")]
    pub r#scan_listener_port_tcp_ssl: Box<i32>,
    /// Shape of VM Cluster.
    #[builder(into)]
    #[serde(rename = "shape")]
    pub r#shape: Box<String>,
    /// Use exadata sparse snapshots.
    #[builder(into)]
    #[serde(rename = "sparseDiskgroupEnabled")]
    pub r#sparse_diskgroup_enabled: Box<bool>,
    /// SSH public keys to be stored with cluster.
    #[builder(into)]
    #[serde(rename = "sshPublicKeys")]
    pub r#ssh_public_keys: Box<Vec<String>>,
    /// State of the cluster. 
    ///  Possible values:
    ///  STATE_UNSPECIFIED
    /// PROVISIONING
    /// AVAILABLE
    /// UPDATING
    /// TERMINATING
    /// TERMINATED
    /// FAILED
    /// MAINTENANCE_IN_PROGRESS
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
    /// The storage allocation for the disk group, in gigabytes (GB).
    #[builder(into)]
    #[serde(rename = "storageSizeGb")]
    pub r#storage_size_gb: Box<i32>,
    /// Operating system version of the image.
    #[builder(into)]
    #[serde(rename = "systemVersion")]
    pub r#system_version: Box<String>,
    /// Represents a time zone from the
    /// [IANA Time Zone Database](https://www.iana.org/time-zones).
    #[builder(into)]
    #[serde(rename = "timeZones")]
    pub r#time_zones: Box<Vec<super::super::types::oracledatabase::GetCloudVmClustersCloudVmClusterPropertyTimeZone>>,
}
