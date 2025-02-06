#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowTemplateJobSparkRJob {
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
    /// The runtime log config for job execution.
    #[builder(into, default)]
    #[serde(rename = "loggingConfig")]
    pub r#logging_config: Box<Option<super::super::types::dataproc::WorkflowTemplateJobSparkRJobLoggingConfig>>,
    /// Required. The HCFS URI of the main R file to use as the driver. Must be a .R file.
    #[builder(into)]
    #[serde(rename = "mainRFileUri")]
    pub r#main_r_file_uri: Box<String>,
    /// A mapping of property names to values, used to configure SparkR. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code.
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
}
