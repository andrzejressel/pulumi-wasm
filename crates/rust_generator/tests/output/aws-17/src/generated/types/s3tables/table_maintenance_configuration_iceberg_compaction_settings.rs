#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TableMaintenanceConfigurationIcebergCompactionSettings {
    /// Data objects smaller than this size may be combined with others to improve query performance.
    /// Must be between `64` and `512`.
    #[builder(into)]
    #[serde(rename = "targetFileSizeMb")]
    pub r#target_file_size_mb: Box<f64>,
}
