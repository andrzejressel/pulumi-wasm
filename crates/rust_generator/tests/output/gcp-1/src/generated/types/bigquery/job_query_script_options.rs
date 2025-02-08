#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct JobQueryScriptOptions {
    /// Determines which statement in the script represents the "key result",
    /// used to populate the schema and query results of the script job.
    /// Possible values are: `LAST`, `FIRST_SELECT`.
    #[builder(into, default)]
    #[serde(rename = "keyResultStatement")]
    pub r#key_result_statement: Box<Option<String>>,
    /// Limit on the number of bytes billed per statement. Exceeding this budget results in an error.
    #[builder(into, default)]
    #[serde(rename = "statementByteBudget")]
    pub r#statement_byte_budget: Box<Option<String>>,
    /// Timeout period for each statement in a script.
    #[builder(into, default)]
    #[serde(rename = "statementTimeoutMs")]
    pub r#statement_timeout_ms: Box<Option<String>>,
}
