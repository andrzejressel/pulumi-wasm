#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableMaintenanceConfigurationIcebergSnapshotManagement {
    /// Settings for snapshot management.
    /// See `iceberg_snapshot_management.settings` below
    #[builder(into)]
    #[serde(rename = "settings")]
    pub r#settings: Box<super::super::types::s3tables::TableMaintenanceConfigurationIcebergSnapshotManagementSettings>,
    /// Whether the configuration is enabled.
    /// Valid values are `enabled` and `disabled`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
