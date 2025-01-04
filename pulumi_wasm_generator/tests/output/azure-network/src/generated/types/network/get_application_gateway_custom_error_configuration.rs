#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApplicationGatewayCustomErrorConfiguration {
    /// Error page URL of the application gateway custom error.
    #[builder(into)]
    #[serde(rename = "customErrorPageUrl")]
    pub r#custom_error_page_url: Box<String>,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Status code of the application gateway custom error.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<String>,
}
