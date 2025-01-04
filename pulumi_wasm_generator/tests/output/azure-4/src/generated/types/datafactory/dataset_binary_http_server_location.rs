#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatasetBinaryHttpServerLocation {
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
    /// The base URL to the web server hosting the file.
    #[builder(into)]
    #[serde(rename = "relativeUrl")]
    pub r#relative_url: Box<String>,
}
