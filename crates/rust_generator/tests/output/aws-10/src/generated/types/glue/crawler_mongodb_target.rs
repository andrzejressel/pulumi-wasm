#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CrawlerMongodbTarget {
    /// The name of the connection to use to connect to the Amazon DocumentDB or MongoDB target.
    #[builder(into)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: Box<String>,
    /// The path of the Amazon DocumentDB or MongoDB target (database/collection).
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// Indicates whether to scan all the records, or to sample rows from the table. Scanning all the records can take a long time when the table is not a high throughput table. Default value is `true`.
    #[builder(into, default)]
    #[serde(rename = "scanAll")]
    pub r#scan_all: Box<Option<bool>>,
}
