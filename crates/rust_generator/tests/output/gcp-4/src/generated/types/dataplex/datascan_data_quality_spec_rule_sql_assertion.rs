#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DatascanDataQualitySpecRuleSqlAssertion {
    /// The SQL statement.
    #[builder(into)]
    #[serde(rename = "sqlStatement")]
    pub r#sql_statement: Box<String>,
}
