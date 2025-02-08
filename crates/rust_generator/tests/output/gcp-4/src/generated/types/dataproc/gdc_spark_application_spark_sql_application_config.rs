#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GdcSparkApplicationSparkSqlApplicationConfig {
    /// HCFS URIs of jar files to be added to the Spark CLASSPATH.
    #[builder(into, default)]
    #[serde(rename = "jarFileUris")]
    pub r#jar_file_uris: Box<Option<Vec<String>>>,
    /// The HCFS URI of the script that contains SQL queries.
    #[builder(into, default)]
    #[serde(rename = "queryFileUri")]
    pub r#query_file_uri: Box<Option<String>>,
    /// Represents a list of queries.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "queryList")]
    pub r#query_list: Box<Option<super::super::types::dataproc::GdcSparkApplicationSparkSqlApplicationConfigQueryList>>,
    /// Mapping of query variable names to values (equivalent to the Spark SQL command: SET `name="value";`).
    #[builder(into, default)]
    #[serde(rename = "scriptVariables")]
    pub r#script_variables: Box<Option<std::collections::HashMap<String, String>>>,
}
