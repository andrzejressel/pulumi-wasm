#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobPigConfig {
    /// Whether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries. Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "continueOnFailure")]
    pub r#continue_on_failure: Box<Option<bool>>,
    /// HCFS URIs of jar files to add to the CLASSPATH of the Pig Client and Hadoop MapReduce (MR) tasks. Can contain Pig UDFs.
    /// 
    /// * `logging_config.driver_log_levels`- (Required) The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'
    #[builder(into, default)]
    #[serde(rename = "jarFileUris")]
    pub r#jar_file_uris: Box<Option<Vec<String>>>,
    /// The runtime logging config of the job
    #[builder(into, default)]
    #[serde(rename = "loggingConfig")]
    pub r#logging_config: Box<Option<super::super::types::dataproc::JobPigConfigLoggingConfig>>,
    /// A mapping of property names to values, used to configure Pig. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in `/etc/hadoop/conf/*-site.xml`, `/etc/pig/conf/pig.properties`, and classes in user code.
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
    /// HCFS URI of file containing Hive script to execute as the job.
    /// Conflicts with `query_list`
    #[builder(into, default)]
    #[serde(rename = "queryFileUri")]
    pub r#query_file_uri: Box<Option<String>>,
    /// The list of Hive queries or statements to execute as part of the job.
    /// Conflicts with `query_file_uri`
    #[builder(into, default)]
    #[serde(rename = "queryLists")]
    pub r#query_lists: Box<Option<Vec<String>>>,
    /// Mapping of query variable names to values (equivalent to the Pig command: `name=[value]`).
    #[builder(into, default)]
    #[serde(rename = "scriptVariables")]
    pub r#script_variables: Box<Option<std::collections::HashMap<String, String>>>,
}
