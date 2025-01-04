#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableMaterializedView {
    /// Allow non incremental materialized view definition.
    /// The default value is false.
    #[builder(into, default)]
    #[serde(rename = "allowNonIncrementalDefinition")]
    pub r#allow_non_incremental_definition: Box<Option<bool>>,
    /// Specifies whether to use BigQuery's automatic refresh for this materialized view when the base table is updated.
    /// The default value is true.
    #[builder(into, default)]
    #[serde(rename = "enableRefresh")]
    pub r#enable_refresh: Box<Option<bool>>,
    /// A query whose result is persisted.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: Box<String>,
    /// The maximum frequency at which this materialized view will be refreshed.
    /// The default value is 1800000
    #[builder(into, default)]
    #[serde(rename = "refreshIntervalMs")]
    pub r#refresh_interval_ms: Box<Option<i32>>,
}
