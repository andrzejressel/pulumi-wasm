#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RepositoryWorkflowConfigInvocationConfigIncludedTarget {
    /// The action's database (Google Cloud project ID).
    #[builder(into, default)]
    #[serde(rename = "database")]
    pub r#database: Box<Option<String>>,
    /// The action's name, within database and schema.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The action's schema (BigQuery dataset ID), within database.
    #[builder(into, default)]
    #[serde(rename = "schema")]
    pub r#schema: Box<Option<String>>,
}
