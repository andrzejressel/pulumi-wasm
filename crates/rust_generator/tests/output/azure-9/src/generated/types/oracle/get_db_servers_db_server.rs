#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDbServersDbServer {
    /// The list of [OCIDs](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the Autonomous Virtual Machines associated with the DB Server.
    #[builder(into)]
    #[serde(rename = "autonomousVirtualMachineDs")]
    pub r#autonomous_virtual_machine_ds: Box<Vec<String>>,
    /// The list of [OCIDs](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the Autonomous VM Clusters associated with the DB Server.
    #[builder(into)]
    #[serde(rename = "autonomousVmClusterIds")]
    pub r#autonomous_vm_cluster_ids: Box<Vec<String>>,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the compartment.
    #[builder(into)]
    #[serde(rename = "compartmentId")]
    pub r#compartment_id: Box<String>,
    /// The number of CPU cores enabled on the DB Server.
    #[builder(into)]
    #[serde(rename = "cpuCoreCount")]
    pub r#cpu_core_count: Box<i32>,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the Db nodes associated with the DB Server.
    #[builder(into)]
    #[serde(rename = "dbNodeIds")]
    pub r#db_node_ids: Box<Vec<String>>,
    /// The allocated local node storage in GBs on the DB Server.
    #[builder(into)]
    #[serde(rename = "dbNodeStorageSizeInGbs")]
    pub r#db_node_storage_size_in_gbs: Box<i32>,
    /// The user-friendly name for the DB Server. The name does not need to be unique.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the Exadata infrastructure.
    #[builder(into)]
    #[serde(rename = "exadataInfrastructureId")]
    pub r#exadata_infrastructure_id: Box<String>,
    /// Additional information about the current lifecycle state.
    #[builder(into)]
    #[serde(rename = "lifecycleDetails")]
    pub r#lifecycle_details: Box<String>,
    /// The current state of the DB Server.
    #[builder(into)]
    #[serde(rename = "lifecycleState")]
    pub r#lifecycle_state: Box<String>,
    /// The total number of CPU cores available.
    #[builder(into)]
    #[serde(rename = "maxCpuCount")]
    pub r#max_cpu_count: Box<i32>,
    /// The total local node storage available in GBs.
    #[builder(into)]
    #[serde(rename = "maxDbNodeStorageInGbs")]
    pub r#max_db_node_storage_in_gbs: Box<i32>,
    /// The total memory available in GBs.
    #[builder(into)]
    #[serde(rename = "maxMemoryInGbs")]
    pub r#max_memory_in_gbs: Box<i32>,
    /// The allocated memory in GBs on the DB Server.
    #[builder(into)]
    #[serde(rename = "memorySizeInGbs")]
    pub r#memory_size_in_gbs: Box<i32>,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the DB Server.
    #[builder(into)]
    #[serde(rename = "ocid")]
    pub r#ocid: Box<String>,
    /// The shape of the DB Server. The shape determines the amount of CPU, storage, and memory resources available.
    #[builder(into)]
    #[serde(rename = "shape")]
    pub r#shape: Box<String>,
    /// The date and time that the DB Server was created.
    #[builder(into)]
    #[serde(rename = "timeCreated")]
    pub r#time_created: Box<String>,
    /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the VM Clusters associated with the DB Server.
    #[builder(into)]
    #[serde(rename = "vmClusterIds")]
    pub r#vm_cluster_ids: Box<Vec<String>>,
}
