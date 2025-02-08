#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CloudVmClusterProperties {
    /// OCI Cluster name.
    #[builder(into, default)]
    #[serde(rename = "clusterName")]
    pub r#cluster_name: Box<Option<String>>,
    /// (Output)
    /// Compartment ID of cluster.
    #[builder(into, default)]
    #[serde(rename = "compartmentId")]
    pub r#compartment_id: Box<Option<String>>,
    /// Number of enabled CPU cores.
    #[builder(into)]
    #[serde(rename = "cpuCoreCount")]
    pub r#cpu_core_count: Box<i32>,
    /// The data disk group size to be allocated in TBs.
    #[builder(into, default)]
    #[serde(rename = "dataStorageSizeTb")]
    pub r#data_storage_size_tb: Box<Option<f64>>,
    /// Local storage per VM
    #[builder(into, default)]
    #[serde(rename = "dbNodeStorageSizeGb")]
    pub r#db_node_storage_size_gb: Box<Option<i32>>,
    /// OCID of database servers.
    #[builder(into, default)]
    #[serde(rename = "dbServerOcids")]
    pub r#db_server_ocids: Box<Option<Vec<String>>>,
    /// Data collection options for diagnostics.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "diagnosticsDataCollectionOptions")]
    pub r#diagnostics_data_collection_options: Box<Option<super::super::types::oracledatabase::CloudVmClusterPropertiesDiagnosticsDataCollectionOptions>>,
    /// The type of redundancy.
    /// Possible values:
    /// DISK_REDUNDANCY_UNSPECIFIED
    /// HIGH
    /// NORMAL
    #[builder(into, default)]
    #[serde(rename = "diskRedundancy")]
    pub r#disk_redundancy: Box<Option<String>>,
    /// (Output)
    /// DNS listener IP.
    #[builder(into, default)]
    #[serde(rename = "dnsListenerIp")]
    pub r#dns_listener_ip: Box<Option<String>>,
    /// (Output)
    /// Parent DNS domain where SCAN DNS and hosts names are qualified.
    /// ex: ocispdelegated.ocisp10jvnet.oraclevcn.com
    #[builder(into, default)]
    #[serde(rename = "domain")]
    pub r#domain: Box<Option<String>>,
    /// Grid Infrastructure Version.
    #[builder(into, default)]
    #[serde(rename = "giVersion")]
    pub r#gi_version: Box<Option<String>>,
    /// (Output)
    /// host name without domain.
    /// format: "-" with some suffix.
    /// ex: sp2-yi0xq where "sp2" is the hostname_prefix.
    #[builder(into, default)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<Option<String>>,
    /// Prefix for VM cluster host names.
    #[builder(into, default)]
    #[serde(rename = "hostnamePrefix")]
    pub r#hostname_prefix: Box<Option<String>>,
    /// License type of VM Cluster.
    /// Possible values:
    /// LICENSE_TYPE_UNSPECIFIED
    /// LICENSE_INCLUDED
    /// BRING_YOUR_OWN_LICENSE
    #[builder(into)]
    #[serde(rename = "licenseType")]
    pub r#license_type: Box<String>,
    /// Use local backup.
    #[builder(into, default)]
    #[serde(rename = "localBackupEnabled")]
    pub r#local_backup_enabled: Box<Option<bool>>,
    /// Memory allocated in GBs.
    #[builder(into, default)]
    #[serde(rename = "memorySizeGb")]
    pub r#memory_size_gb: Box<Option<i32>>,
    /// Number of database servers.
    #[builder(into, default)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Box<Option<i32>>,
    /// (Output)
    /// Deep link to the OCI console to view this resource.
    #[builder(into, default)]
    #[serde(rename = "ociUrl")]
    pub r#oci_url: Box<Option<String>>,
    /// (Output)
    /// Oracle Cloud Infrastructure ID of VM Cluster.
    #[builder(into, default)]
    #[serde(rename = "ocid")]
    pub r#ocid: Box<Option<String>>,
    /// OCPU count per VM. Minimum is 0.1.
    #[builder(into, default)]
    #[serde(rename = "ocpuCount")]
    pub r#ocpu_count: Box<Option<f64>>,
    /// (Output)
    /// SCAN DNS name.
    /// ex: sp2-yi0xq-scan.ocispdelegated.ocisp10jvnet.oraclevcn.com
    #[builder(into, default)]
    #[serde(rename = "scanDns")]
    pub r#scan_dns: Box<Option<String>>,
    /// (Output)
    /// OCID of scan DNS record.
    #[builder(into, default)]
    #[serde(rename = "scanDnsRecordId")]
    pub r#scan_dns_record_id: Box<Option<String>>,
    /// (Output)
    /// OCIDs of scan IPs.
    #[builder(into, default)]
    #[serde(rename = "scanIpIds")]
    pub r#scan_ip_ids: Box<Option<Vec<String>>>,
    /// (Output)
    /// SCAN listener port - TCP
    #[builder(into, default)]
    #[serde(rename = "scanListenerPortTcp")]
    pub r#scan_listener_port_tcp: Box<Option<i32>>,
    /// (Output)
    /// SCAN listener port - TLS
    #[builder(into, default)]
    #[serde(rename = "scanListenerPortTcpSsl")]
    pub r#scan_listener_port_tcp_ssl: Box<Option<i32>>,
    /// (Output)
    /// Shape of VM Cluster.
    #[builder(into, default)]
    #[serde(rename = "shape")]
    pub r#shape: Box<Option<String>>,
    /// Use exadata sparse snapshots.
    #[builder(into, default)]
    #[serde(rename = "sparseDiskgroupEnabled")]
    pub r#sparse_diskgroup_enabled: Box<Option<bool>>,
    /// SSH public keys to be stored with cluster.
    #[builder(into, default)]
    #[serde(rename = "sshPublicKeys")]
    pub r#ssh_public_keys: Box<Option<Vec<String>>>,
    /// (Output)
    /// State of the cluster.
    /// Possible values:
    /// STATE_UNSPECIFIED
    /// PROVISIONING
    /// AVAILABLE
    /// UPDATING
    /// TERMINATING
    /// TERMINATED
    /// FAILED
    /// MAINTENANCE_IN_PROGRESS
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// (Output)
    /// The storage allocation for the disk group, in gigabytes (GB).
    #[builder(into, default)]
    #[serde(rename = "storageSizeGb")]
    pub r#storage_size_gb: Box<Option<i32>>,
    /// (Output)
    /// Operating system version of the image.
    #[builder(into, default)]
    #[serde(rename = "systemVersion")]
    pub r#system_version: Box<Option<String>>,
    /// Represents a time zone from the
    /// [IANA Time Zone Database](https://www.iana.org/time-zones).
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<Option<super::super::types::oracledatabase::CloudVmClusterPropertiesTimeZone>>,
}
