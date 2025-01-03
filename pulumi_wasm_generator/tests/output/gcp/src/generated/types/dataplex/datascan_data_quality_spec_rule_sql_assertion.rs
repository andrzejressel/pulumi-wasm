#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatascanDataQualitySpecRuleSqlAssertion {
    /// The SQL statement.
    #[builder(into)]
    #[serde(rename = "sqlStatement")]
    pub r#sql_statement: Box<String>,
}
