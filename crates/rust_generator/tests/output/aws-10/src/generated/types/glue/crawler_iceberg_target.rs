#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CrawlerIcebergTarget {
    /// The name of the connection to use to connect to the Iceberg target.
    #[builder(into, default)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: Box<Option<String>>,
    /// A list of glob patterns used to exclude from the crawl.
    #[builder(into, default)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Box<Option<Vec<String>>>,
    /// The maximum depth of Amazon S3 paths that the crawler can traverse to discover the Iceberg metadata folder in your Amazon S3 path. Used to limit the crawler run time. Valid values are between `1` and `20`.
    #[builder(into)]
    #[serde(rename = "maximumTraversalDepth")]
    pub r#maximum_traversal_depth: Box<i32>,
    /// One or more Amazon S3 paths that contains Iceberg metadata folders as s3://bucket/prefix.
    #[builder(into)]
    #[serde(rename = "paths")]
    pub r#paths: Box<Vec<String>>,
}
