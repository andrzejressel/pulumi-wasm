#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BatchSparkSqlBatch {
    /// HCFS URIs of jar files to be added to the Spark CLASSPATH.
    #[builder(into, default)]
    #[serde(rename = "jarFileUris")]
    pub r#jar_file_uris: Box<Option<Vec<String>>>,
    /// The HCFS URI of the script that contains Spark SQL queries to execute.
    #[builder(into, default)]
    #[serde(rename = "queryFileUri")]
    pub r#query_file_uri: Box<Option<String>>,
    /// Mapping of query variable names to values (equivalent to the Spark SQL command: SET name="value";).
    #[builder(into, default)]
    #[serde(rename = "queryVariables")]
    pub r#query_variables: Box<Option<std::collections::HashMap<String, String>>>,
}
