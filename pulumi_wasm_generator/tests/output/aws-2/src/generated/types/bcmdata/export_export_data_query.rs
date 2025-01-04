#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ExportExportDataQuery {
    /// Query statement.
    #[builder(into)]
    #[serde(rename = "queryStatement")]
    pub r#query_statement: Box<String>,
    /// Table configuration.
    #[builder(into, default)]
    #[serde(rename = "tableConfigurations")]
    pub r#table_configurations: Box<Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>>,
}
