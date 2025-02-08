#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EventTargetRedshiftTarget {
    /// The name of the database.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: Box<String>,
    /// The database user name.
    #[builder(into, default)]
    #[serde(rename = "dbUser")]
    pub r#db_user: Box<Option<String>>,
    /// The name or ARN of the secret that enables access to the database.
    #[builder(into, default)]
    #[serde(rename = "secretsManagerArn")]
    pub r#secrets_manager_arn: Box<Option<String>>,
    /// The SQL statement text to run.
    #[builder(into, default)]
    #[serde(rename = "sql")]
    pub r#sql: Box<Option<String>>,
    /// The name of the SQL statement.
    #[builder(into, default)]
    #[serde(rename = "statementName")]
    pub r#statement_name: Box<Option<String>>,
    /// Indicates whether to send an event back to EventBridge after the SQL statement runs.
    #[builder(into, default)]
    #[serde(rename = "withEvent")]
    pub r#with_event: Box<Option<bool>>,
}
