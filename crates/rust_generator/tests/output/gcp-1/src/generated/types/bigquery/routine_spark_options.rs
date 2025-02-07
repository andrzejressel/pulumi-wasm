#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RoutineSparkOptions {
    /// Archive files to be extracted into the working directory of each executor. For more information about Apache Spark, see Apache Spark.
    #[builder(into, default)]
    #[serde(rename = "archiveUris")]
    pub r#archive_uris: Box<Option<Vec<String>>>,
    /// Fully qualified name of the user-provided Spark connection object.
    /// Format: "projects/{projectId}/locations/{locationId}/connections/{connectionId}"
    #[builder(into, default)]
    #[serde(rename = "connection")]
    pub r#connection: Box<Option<String>>,
    /// Custom container image for the runtime environment.
    #[builder(into, default)]
    #[serde(rename = "containerImage")]
    pub r#container_image: Box<Option<String>>,
    /// Files to be placed in the working directory of each executor. For more information about Apache Spark, see Apache Spark.
    #[builder(into, default)]
    #[serde(rename = "fileUris")]
    pub r#file_uris: Box<Option<Vec<String>>>,
    /// JARs to include on the driver and executor CLASSPATH. For more information about Apache Spark, see Apache Spark.
    #[builder(into, default)]
    #[serde(rename = "jarUris")]
    pub r#jar_uris: Box<Option<Vec<String>>>,
    /// The fully qualified name of a class in jarUris, for example, com.example.wordcount.
    /// Exactly one of mainClass and main_jar_uri field should be set for Java/Scala language type.
    #[builder(into, default)]
    #[serde(rename = "mainClass")]
    pub r#main_class: Box<Option<String>>,
    /// The main file/jar URI of the Spark application.
    /// Exactly one of the definitionBody field and the mainFileUri field must be set for Python.
    /// Exactly one of mainClass and mainFileUri field should be set for Java/Scala language type.
    #[builder(into, default)]
    #[serde(rename = "mainFileUri")]
    pub r#main_file_uri: Box<Option<String>>,
    /// Configuration properties as a set of key/value pairs, which will be passed on to the Spark application.
    /// For more information, see Apache Spark and the procedure option list.
    /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
    /// Python files to be placed on the PYTHONPATH for PySpark application. Supported file types: .py, .egg, and .zip. For more information about Apache Spark, see Apache Spark.
    #[builder(into, default)]
    #[serde(rename = "pyFileUris")]
    pub r#py_file_uris: Box<Option<Vec<String>>>,
    /// Runtime version. If not specified, the default runtime version is used.
    #[builder(into, default)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: Box<Option<String>>,
}
