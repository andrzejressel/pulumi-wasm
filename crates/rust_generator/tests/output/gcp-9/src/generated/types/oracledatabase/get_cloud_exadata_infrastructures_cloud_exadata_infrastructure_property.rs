#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetCloudExadataInfrastructuresCloudExadataInfrastructureProperty {
    /// The requested number of additional storage servers activated for the
    /// Exadata Infrastructure.
    #[builder(into)]
    #[serde(rename = "activatedStorageCount")]
    pub r#activated_storage_count: Box<i32>,
    /// The requested number of additional storage servers for the Exadata
    /// Infrastructure.
    #[builder(into)]
    #[serde(rename = "additionalStorageCount")]
    pub r#additional_storage_count: Box<i32>,
    /// The available storage can be allocated to the Exadata Infrastructure
    /// resource, in gigabytes (GB).
    #[builder(into)]
    #[serde(rename = "availableStorageSizeGb")]
    pub r#available_storage_size_gb: Box<i32>,
    /// The number of compute servers for the Exadata Infrastructure.
    #[builder(into)]
    #[serde(rename = "computeCount")]
    pub r#compute_count: Box<i32>,
    /// The number of enabled CPU cores.
    #[builder(into)]
    #[serde(rename = "cpuCount")]
    pub r#cpu_count: Box<i32>,
    /// The list of customer contacts.
    #[builder(into)]
    #[serde(rename = "customerContacts")]
    pub r#customer_contacts: Box<Vec<super::super::types::oracledatabase::GetCloudExadataInfrastructuresCloudExadataInfrastructurePropertyCustomerContact>>,
    /// Size, in terabytes, of the DATA disk group.
    #[builder(into)]
    #[serde(rename = "dataStorageSizeTb")]
    pub r#data_storage_size_tb: Box<f64>,
    /// The local node storage allocated in GBs.
    #[builder(into)]
    #[serde(rename = "dbNodeStorageSizeGb")]
    pub r#db_node_storage_size_gb: Box<i32>,
    /// The software version of the database servers (dom0) in the Exadata
    /// Infrastructure.
    #[builder(into)]
    #[serde(rename = "dbServerVersion")]
    pub r#db_server_version: Box<String>,
    /// Maintenance window as defined by Oracle.
    /// https://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/MaintenanceWindow
    #[builder(into)]
    #[serde(rename = "maintenanceWindows")]
    pub r#maintenance_windows: Box<Vec<super::super::types::oracledatabase::GetCloudExadataInfrastructuresCloudExadataInfrastructurePropertyMaintenanceWindow>>,
    /// The total number of CPU cores available.
    #[builder(into)]
    #[serde(rename = "maxCpuCount")]
    pub r#max_cpu_count: Box<i32>,
    /// The total available DATA disk group size.
    #[builder(into)]
    #[serde(rename = "maxDataStorageTb")]
    pub r#max_data_storage_tb: Box<f64>,
    /// The total local node storage available in GBs.
    #[builder(into)]
    #[serde(rename = "maxDbNodeStorageSizeGb")]
    pub r#max_db_node_storage_size_gb: Box<i32>,
    /// The total memory available in GBs.
    #[builder(into)]
    #[serde(rename = "maxMemoryGb")]
    pub r#max_memory_gb: Box<i32>,
    /// The memory allocated in GBs.
    #[builder(into)]
    #[serde(rename = "memorySizeGb")]
    pub r#memory_size_gb: Box<i32>,
    /// The monthly software version of the database servers (dom0)
    /// in the Exadata Infrastructure. Example: 20.1.15
    #[builder(into)]
    #[serde(rename = "monthlyDbServerVersion")]
    pub r#monthly_db_server_version: Box<String>,
    /// The monthly software version of the storage servers (cells)
    /// in the Exadata Infrastructure. Example: 20.1.15
    #[builder(into)]
    #[serde(rename = "monthlyStorageServerVersion")]
    pub r#monthly_storage_server_version: Box<String>,
    /// The OCID of the next maintenance run.
    #[builder(into)]
    #[serde(rename = "nextMaintenanceRunId")]
    pub r#next_maintenance_run_id: Box<String>,
    /// The time when the next maintenance run will occur.
    #[builder(into)]
    #[serde(rename = "nextMaintenanceRunTime")]
    pub r#next_maintenance_run_time: Box<String>,
    /// The time when the next security maintenance run will occur.
    #[builder(into)]
    #[serde(rename = "nextSecurityMaintenanceRunTime")]
    pub r#next_security_maintenance_run_time: Box<String>,
    /// Deep link to the OCI console to view this resource.
    #[builder(into)]
    #[serde(rename = "ociUrl")]
    pub r#oci_url: Box<String>,
    /// OCID of created infra.
    /// https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm#Oracle
    #[builder(into)]
    #[serde(rename = "ocid")]
    pub r#ocid: Box<String>,
    /// The shape of the Exadata Infrastructure. The shape determines the
    /// amount of CPU, storage, and memory resources allocated to the instance.
    #[builder(into)]
    #[serde(rename = "shape")]
    pub r#shape: Box<String>,
    /// The current lifecycle state of the Exadata Infrastructure. 
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
    /// The number of Cloud Exadata storage servers for the Exadata Infrastructure.
    #[builder(into)]
    #[serde(rename = "storageCount")]
    pub r#storage_count: Box<i32>,
    /// The software version of the storage servers (cells) in the Exadata
    /// Infrastructure.
    #[builder(into)]
    #[serde(rename = "storageServerVersion")]
    pub r#storage_server_version: Box<String>,
    /// The total storage allocated to the Exadata Infrastructure
    /// resource, in gigabytes (GB).
    #[builder(into)]
    #[serde(rename = "totalStorageSizeGb")]
    pub r#total_storage_size_gb: Box<i32>,
}
