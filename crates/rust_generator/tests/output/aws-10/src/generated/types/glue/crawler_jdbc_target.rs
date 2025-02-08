#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CrawlerJdbcTarget {
    /// The name of the connection to use to connect to the JDBC target.
    #[builder(into)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: Box<String>,
    /// Specify a value of `RAWTYPES` or `COMMENTS` to enable additional metadata intable responses. `RAWTYPES` provides the native-level datatype. `COMMENTS` provides comments associated with a column or table in the database.
    #[builder(into, default)]
    #[serde(rename = "enableAdditionalMetadatas")]
    pub r#enable_additional_metadatas: Box<Option<Vec<String>>>,
    /// A list of glob patterns used to exclude from the crawl.
    #[builder(into, default)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Box<Option<Vec<String>>>,
    /// The path of the JDBC target.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
