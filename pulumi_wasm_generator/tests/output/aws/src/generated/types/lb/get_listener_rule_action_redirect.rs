#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetListenerRuleActionRedirect {
    /// The hostname.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The absolute path, starting with `/`.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// The port.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<String>,
    /// The protocol.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// The query parameters.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: Box<String>,
    /// The HTTP redirect code.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<String>,
}