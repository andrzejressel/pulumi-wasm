#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VirtualMachineStorageConfiguration {
    /// A `storage_settings` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "dataSettings")]
    pub r#data_settings: Box<Option<super::super::types::mssql::VirtualMachineStorageConfigurationDataSettings>>,
    /// The type of disk configuration to apply to the SQL Server. Valid values include `NEW`, `EXTEND`, or `ADD`.
    #[builder(into)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Box<String>,
    /// A `storage_settings` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "logSettings")]
    pub r#log_settings: Box<Option<super::super::types::mssql::VirtualMachineStorageConfigurationLogSettings>>,
    /// The type of storage workload. Valid values include `GENERAL`, `OLTP`, or `DW`.
    #[builder(into)]
    #[serde(rename = "storageWorkloadType")]
    pub r#storage_workload_type: Box<String>,
    /// Specifies whether to set system databases (except tempDb) location to newly created data storage. Possible values are `true` and `false`. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "systemDbOnDataDiskEnabled")]
    pub r#system_db_on_data_disk_enabled: Box<Option<bool>>,
    /// An `temp_db_settings` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "tempDbSettings")]
    pub r#temp_db_settings: Box<Option<super::super::types::mssql::VirtualMachineStorageConfigurationTempDbSettings>>,
}
