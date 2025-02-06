#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatasetJsonAzureBlobStorageLocation {
    /// The container on the Azure Blob Storage Account hosting the file.
    #[builder(into)]
    #[serde(rename = "container")]
    pub r#container: Box<String>,
    /// Is the `container` using dynamic expression, function or system variables? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "dynamicContainerEnabled")]
    pub r#dynamic_container_enabled: Box<Option<bool>>,
    /// Is the `filename` using dynamic expression, function or system variables? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "dynamicFilenameEnabled")]
    pub r#dynamic_filename_enabled: Box<Option<bool>>,
    /// Is the `path` using dynamic expression, function or system variables? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "dynamicPathEnabled")]
    pub r#dynamic_path_enabled: Box<Option<bool>>,
    /// The filename of the file on the web server.
    #[builder(into)]
    #[serde(rename = "filename")]
    pub r#filename: Box<String>,
    /// The folder path to the file on the web server.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
