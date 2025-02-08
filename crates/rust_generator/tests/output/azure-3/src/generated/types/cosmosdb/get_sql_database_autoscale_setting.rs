#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSqlDatabaseAutoscaleSetting {
    /// The maximum throughput of the SQL database (RU/s).
    #[builder(into)]
    #[serde(rename = "maxThroughput")]
    pub r#max_throughput: Box<i32>,
}
