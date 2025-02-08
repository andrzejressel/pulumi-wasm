#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TableView {
    /// A query that BigQuery executes when the view is referenced.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: Box<String>,
    /// Specifies whether to use BigQuery's legacy SQL for this view.
    /// The default value is true. If set to false, the view will use BigQuery's standard SQL.
    #[builder(into, default)]
    #[serde(rename = "useLegacySql")]
    pub r#use_legacy_sql: Box<Option<bool>>,
}
