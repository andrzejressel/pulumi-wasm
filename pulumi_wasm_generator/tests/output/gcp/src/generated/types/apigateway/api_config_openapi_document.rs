#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApiConfigOpenapiDocument {
    /// The OpenAPI Specification document file.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "document")]
    pub r#document: Box<super::super::types::apigateway::ApiConfigOpenapiDocumentDocument>,
}
