#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CatalogTableOptimizerConfigurationOrphanFileDeletionConfigurationIcebergConfiguration {
    /// Specifies a directory in which to look for files. You may choose a sub-directory rather than the top-level table location. Defaults to the table's location.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// The number of days that orphan files should be retained before file deletion. Defaults to `3`.
    #[builder(into, default)]
    #[serde(rename = "orphanFileRetentionPeriodInDays")]
    pub r#orphan_file_retention_period_in_days: Box<Option<f64>>,
}
