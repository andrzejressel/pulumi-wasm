#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualMachineSqlInstance {
    /// Specifies if the SQL Server is optimized for adhoc workloads. Possible values are `true` and `false`. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "adhocWorkloadsOptimizationEnabled")]
    pub r#adhoc_workloads_optimization_enabled: Box<Option<bool>>,
    /// Collation of the SQL Server. Defaults to `SQL_Latin1_General_CP1_CI_AS`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "collation")]
    pub r#collation: Box<Option<String>>,
    /// Specifies if Instant File Initialization is enabled for the SQL Server. Possible values are `true` and `false`. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "instantFileInitializationEnabled")]
    pub r#instant_file_initialization_enabled: Box<Option<bool>>,
    /// Specifies if Lock Pages in Memory is enabled for the SQL Server. Possible values are `true` and `false`. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "lockPagesInMemoryEnabled")]
    pub r#lock_pages_in_memory_enabled: Box<Option<bool>>,
    /// Maximum Degree of Parallelism of the SQL Server. Possible values are between `0` and `32767`. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "maxDop")]
    pub r#max_dop: Box<Option<i32>>,
    /// Maximum amount memory that SQL Server Memory Manager can allocate to the SQL Server process. Possible values are between `128` and `2147483647` Defaults to `2147483647`.
    #[builder(into, default)]
    #[serde(rename = "maxServerMemoryMb")]
    pub r#max_server_memory_mb: Box<Option<i32>>,
    /// Minimum amount memory that SQL Server Memory Manager can allocate to the SQL Server process. Possible values are between `0` and `2147483647` Defaults to `0`.
    /// 
    /// > **NOTE:** `max_server_memory_mb` must be greater than or equal to `min_server_memory_mb`
    #[builder(into, default)]
    #[serde(rename = "minServerMemoryMb")]
    pub r#min_server_memory_mb: Box<Option<i32>>,
}
