#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContainerUpload {
    /// Literal string value to use as the object content, which will be uploaded as UTF-8-encoded text. Conflicts with `content_base64` & `source`
    #[builder(into, default)]
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// Base64-encoded data that will be decoded and uploaded as raw bytes for the object content. This allows safely uploading non-UTF8 binary data, but is recommended only for larger binary content such as the result of the `base64encode` interpolation function. See here for the reason. Conflicts with `content` & `source`
    #[builder(into, default)]
    #[serde(rename = "contentBase64")]
    pub r#content_base_64: Box<Option<String>>,
    /// If `true`, the file will be uploaded with user executable permission. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "executable")]
    pub r#executable: Box<Option<bool>>,
    /// Path to the file in the container where is upload goes to
    #[builder(into)]
    #[serde(rename = "file")]
    pub r#file: Box<String>,
    /// A filename that references a file which will be uploaded as the object content. This allows for large file uploads that do not get stored in state. Conflicts with `content` & `content_base64`
    #[builder(into, default)]
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
    /// If using `source`, this will force an update if the file content has updated but the filename has not.
    #[builder(into, default)]
    #[serde(rename = "sourceHash")]
    pub r#source_hash: Box<Option<String>>,
}
