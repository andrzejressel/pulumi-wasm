#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EntryBigqueryTableSpec {
    /// (Output)
    /// The table source type.
    #[builder(into, default)]
    #[serde(rename = "tableSourceType")]
    pub r#table_source_type: Box<Option<String>>,
    /// (Output)
    /// Spec of a BigQuery table. This field should only be populated if tableSourceType is BIGQUERY_TABLE.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "tableSpecs")]
    pub r#table_specs: Box<Option<Vec<super::super::types::datacatalog::EntryBigqueryTableSpecTableSpec>>>,
    /// (Output)
    /// Table view specification. This field should only be populated if tableSourceType is BIGQUERY_VIEW.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "viewSpecs")]
    pub r#view_specs: Box<Option<Vec<super::super::types::datacatalog::EntryBigqueryTableSpecViewSpec>>>,
}
