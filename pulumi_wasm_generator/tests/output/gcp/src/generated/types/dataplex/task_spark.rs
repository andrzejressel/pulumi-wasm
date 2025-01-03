#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TaskSpark {
    /// Cloud Storage URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip.
    #[builder(into, default)]
    #[serde(rename = "archiveUris")]
    pub r#archive_uris: Box<Option<Vec<String>>>,
    /// Cloud Storage URIs of files to be placed in the working directory of each executor.
    #[builder(into, default)]
    #[serde(rename = "fileUris")]
    pub r#file_uris: Box<Option<Vec<String>>>,
    /// Infrastructure specification for the execution.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "infrastructureSpec")]
    pub r#infrastructure_spec: Box<Option<super::super::types::dataplex::TaskSparkInfrastructureSpec>>,
    /// The name of the driver's main class. The jar file that contains the class must be in the default CLASSPATH or specified in jar_file_uris. The execution args are passed in as a sequence of named process arguments (--key=value).
    #[builder(into, default)]
    #[serde(rename = "mainClass")]
    pub r#main_class: Box<Option<String>>,
    /// The Cloud Storage URI of the jar file that contains the main class. The execution args are passed in as a sequence of named process arguments (--key=value).
    #[builder(into, default)]
    #[serde(rename = "mainJarFileUri")]
    pub r#main_jar_file_uri: Box<Option<String>>,
    /// The Gcloud Storage URI of the main Python file to use as the driver. Must be a .py file. The execution args are passed in as a sequence of named process arguments (--key=value).
    #[builder(into, default)]
    #[serde(rename = "pythonScriptFile")]
    pub r#python_script_file: Box<Option<String>>,
    /// The query text. The execution args are used to declare a set of script variables (set key='value';).
    #[builder(into, default)]
    #[serde(rename = "sqlScript")]
    pub r#sql_script: Box<Option<String>>,
    /// A reference to a query file. This can be the Cloud Storage URI of the query file or it can the path to a SqlScript Content. The execution args are used to declare a set of script variables (set key='value';).
    #[builder(into, default)]
    #[serde(rename = "sqlScriptFile")]
    pub r#sql_script_file: Box<Option<String>>,
}
