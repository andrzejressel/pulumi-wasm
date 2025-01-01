#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobSparkConfig {
    /// HCFS URIs of archives to be extracted in the working directory of .jar, .tar, .tar.gz, .tgz, and .zip.
    #[builder(into, default)]
    #[serde(rename = "archiveUris")]
    pub r#archive_uris: Box<Option<Vec<String>>>,
    /// The arguments to pass to the driver.
    #[builder(into, default)]
    #[serde(rename = "args")]
    pub r#args: Box<Option<Vec<String>>>,
    /// HCFS URIs of files to be copied to the working directory of Spark drivers and distributed tasks. Useful for naively parallel tasks.
    #[builder(into, default)]
    #[serde(rename = "fileUris")]
    pub r#file_uris: Box<Option<Vec<String>>>,
    /// HCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks.
    #[builder(into, default)]
    #[serde(rename = "jarFileUris")]
    pub r#jar_file_uris: Box<Option<Vec<String>>>,
    /// The runtime logging config of the job
    #[builder(into, default)]
    #[serde(rename = "loggingConfig")]
    pub r#logging_config: Box<Option<super::super::types::dataproc::JobSparkConfigLoggingConfig>>,
    /// The class containing the main method of the driver. Must be in a
    /// provided jar or jar that is already on the classpath. Conflicts with `main_jar_file_uri`
    #[builder(into, default)]
    #[serde(rename = "mainClass")]
    pub r#main_class: Box<Option<String>>,
    /// The HCFS URI of jar file containing
    /// the driver jar. Conflicts with `main_class`
    #[builder(into, default)]
    #[serde(rename = "mainJarFileUri")]
    pub r#main_jar_file_uri: Box<Option<String>>,
    /// A mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in `/etc/spark/conf/spark-defaults.conf` and classes in user code.
    /// 
    /// * `logging_config.driver_log_levels`- (Required) The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
}
