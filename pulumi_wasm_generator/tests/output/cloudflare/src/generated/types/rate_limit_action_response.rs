#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RateLimitActionResponse {
    /// The body to return, the content here should conform to the `content_type`.
    #[builder(into)]
    #[serde(rename = "body")]
    pub r#body: Box<String>,
    /// The content-type of the body. Available values: `text/plain`, `text/xml`, `application/json`.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<String>,
}