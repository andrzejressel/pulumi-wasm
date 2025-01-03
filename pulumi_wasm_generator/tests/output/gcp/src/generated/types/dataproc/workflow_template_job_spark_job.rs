#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowTemplateJobSparkJob {
    /// HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip.
    #[builder(into, default)]
    #[serde(rename = "archiveUris")]
    pub r#archive_uris: Box<Option<Vec<String>>>,
    /// The arguments to pass to the driver. Do not include arguments, such as `--conf`, that can be set as job properties, since a collision may occur that causes an incorrect job submission.
    #[builder(into, default)]
    #[serde(rename = "args")]
    pub r#args: Box<Option<Vec<String>>>,
    /// HCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks.
    #[builder(into, default)]
    #[serde(rename = "fileUris")]
    pub r#file_uris: Box<Option<Vec<String>>>,
    /// HCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks.
    #[builder(into, default)]
    #[serde(rename = "jarFileUris")]
    pub r#jar_file_uris: Box<Option<Vec<String>>>,
    /// The runtime log config for job execution.
    #[builder(into, default)]
    #[serde(rename = "loggingConfig")]
    pub r#logging_config: Box<Option<super::super::types::dataproc::WorkflowTemplateJobSparkJobLoggingConfig>>,
    /// The name of the driver's main class. The jar file that contains the class must be in the default CLASSPATH or specified in `jar_file_uris`.
    #[builder(into, default)]
    #[serde(rename = "mainClass")]
    pub r#main_class: Box<Option<String>>,
    /// The HCFS URI of the jar file that contains the main class.
    #[builder(into, default)]
    #[serde(rename = "mainJarFileUri")]
    pub r#main_jar_file_uri: Box<Option<String>>,
    /// A mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code.
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
}
