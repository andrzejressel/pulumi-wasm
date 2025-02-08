#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowTemplateJobHadoopJob {
    /// HCFS URIs of archives to be extracted in the working directory of Hadoop drivers and tasks. Supported file types: .jar, .tar, .tar.gz, .tgz, or .zip.
    #[builder(into, default)]
    #[serde(rename = "archiveUris")]
    pub r#archive_uris: Box<Option<Vec<String>>>,
    /// The arguments to pass to the driver. Do not include arguments, such as `-libjars` or `-Dfoo=bar`, that can be set as job properties, since a collision may occur that causes an incorrect job submission.
    #[builder(into, default)]
    #[serde(rename = "args")]
    pub r#args: Box<Option<Vec<String>>>,
    /// HCFS (Hadoop Compatible Filesystem) URIs of files to be copied to the working directory of Hadoop drivers and distributed tasks. Useful for naively parallel tasks.
    #[builder(into, default)]
    #[serde(rename = "fileUris")]
    pub r#file_uris: Box<Option<Vec<String>>>,
    /// Jar file URIs to add to the CLASSPATHs of the Hadoop driver and tasks.
    #[builder(into, default)]
    #[serde(rename = "jarFileUris")]
    pub r#jar_file_uris: Box<Option<Vec<String>>>,
    /// The runtime log config for job execution.
    #[builder(into, default)]
    #[serde(rename = "loggingConfig")]
    pub r#logging_config: Box<Option<super::super::types::dataproc::WorkflowTemplateJobHadoopJobLoggingConfig>>,
    /// The name of the driver's main class. The jar file containing the class must be in the default CLASSPATH or specified in `jar_file_uris`.
    #[builder(into, default)]
    #[serde(rename = "mainClass")]
    pub r#main_class: Box<Option<String>>,
    /// The HCFS URI of the jar file containing the main class. Examples: 'gs://foo-bucket/analytics-binaries/extract-useful-metrics-mr.jar' 'hdfs:/tmp/test-samples/custom-wordcount.jar' 'file:///home/usr/lib/hadoop-mapreduce/hadoop-mapreduce-examples.jar'
    #[builder(into, default)]
    #[serde(rename = "mainJarFileUri")]
    pub r#main_jar_file_uri: Box<Option<String>>,
    /// A mapping of property names to values, used to configure Hadoop. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site and classes in user code.
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
}
