#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CrawlerCatalogTarget {
    /// The name of the connection for an Amazon S3-backed Data Catalog table to be a target of the crawl when using a Catalog connection type paired with a `NETWORK` Connection type.
    #[builder(into, default)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: Box<Option<String>>,
    /// The name of the Glue database to be synchronized.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Box<String>,
    /// A valid Amazon SQS ARN.
    /// 
    /// > **Note:** `deletion_behavior` of catalog target doesn't support `DEPRECATE_IN_DATABASE`.
    /// 
    /// > **Note:** `configuration` for catalog target crawlers will have `{ ... "Grouping": { "TableGroupingPolicy": "CombineCompatibleSchemas"} }` by default.
    #[builder(into, default)]
    #[serde(rename = "dlqEventQueueArn")]
    pub r#dlq_event_queue_arn: Box<Option<String>>,
    /// A valid Amazon SQS ARN.
    #[builder(into, default)]
    #[serde(rename = "eventQueueArn")]
    pub r#event_queue_arn: Box<Option<String>>,
    /// A list of catalog tables to be synchronized.
    #[builder(into)]
    #[serde(rename = "tables")]
    pub r#tables: Box<Vec<String>>,
}
