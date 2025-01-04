#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatasetDelimitedTextAzureBlobFsLocation {
    /// Is the `file_system` using dynamic expression, function or system variables? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "dynamicFileSystemEnabled")]
    pub r#dynamic_file_system_enabled: Box<Option<bool>>,
    /// Is the `filename` using dynamic expression, function or system variables? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "dynamicFilenameEnabled")]
    pub r#dynamic_filename_enabled: Box<Option<bool>>,
    /// Is the `path` using dynamic expression, function or system variables? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "dynamicPathEnabled")]
    pub r#dynamic_path_enabled: Box<Option<bool>>,
    /// The storage data lake gen2 file system on the Azure Blob Storage Account hosting the file.
    #[builder(into, default)]
    #[serde(rename = "fileSystem")]
    pub r#file_system: Box<Option<String>>,
    /// The filename of the file.
    #[builder(into, default)]
    #[serde(rename = "filename")]
    pub r#filename: Box<Option<String>>,
    /// The folder path to the file.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
}
