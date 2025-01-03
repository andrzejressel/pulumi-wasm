#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowTemplateJobPrestoJob {
    /// Presto client tags to attach to this query
    #[builder(into, default)]
    #[serde(rename = "clientTags")]
    pub r#client_tags: Box<Option<Vec<String>>>,
    /// Whether to continue executing queries if a query fails. The default value is `false`. Setting to `true` can be useful when executing independent parallel queries.
    #[builder(into, default)]
    #[serde(rename = "continueOnFailure")]
    pub r#continue_on_failure: Box<Option<bool>>,
    /// The runtime log config for job execution.
    #[builder(into, default)]
    #[serde(rename = "loggingConfig")]
    pub r#logging_config: Box<Option<super::super::types::dataproc::WorkflowTemplateJobPrestoJobLoggingConfig>>,
    /// The format in which query output will be displayed. See the Presto documentation for supported output formats
    #[builder(into, default)]
    #[serde(rename = "outputFormat")]
    pub r#output_format: Box<Option<String>>,
    /// A mapping of property names to values. Used to set Presto (https://prestodb.io/docs/current/sql/set-session.html) Equivalent to using the --session flag in the Presto CLI
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
    /// The HCFS URI of the script that contains SQL queries.
    #[builder(into, default)]
    #[serde(rename = "queryFileUri")]
    pub r#query_file_uri: Box<Option<String>>,
    /// A list of queries.
    #[builder(into, default)]
    #[serde(rename = "queryList")]
    pub r#query_list: Box<Option<super::super::types::dataproc::WorkflowTemplateJobPrestoJobQueryList>>,
}
