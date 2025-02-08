#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettings {
    /// Data objects marked for deletion are deleted after this many days.
    /// Must be at least `1`.
    #[builder(into)]
    #[serde(rename = "nonCurrentDays")]
    pub r#non_current_days: Box<f64>,
    /// Unreferenced data objects are marked for deletion after this many days.
    /// Must be at least `1`.
    #[builder(into)]
    #[serde(rename = "unreferencedDays")]
    pub r#unreferenced_days: Box<f64>,
}
