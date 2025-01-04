#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApiConfigOpenapiDocumentDocument {
    /// Base64 encoded content of the file.
    #[builder(into)]
    #[serde(rename = "contents")]
    pub r#contents: Box<String>,
    /// The file path (full or relative path). This is typically the path of the file when it is uploaded.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
