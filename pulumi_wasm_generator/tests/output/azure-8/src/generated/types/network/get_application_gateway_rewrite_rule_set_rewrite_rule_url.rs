#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApplicationGatewayRewriteRuleSetRewriteRuleUrl {
    /// The components used to rewrite the URL.
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Box<String>,
    /// The URL path to rewrite.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// The query string to rewrite.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Box<String>,
    /// Whether the URL path map is reevaluated after this rewrite has been applied.
    #[builder(into)]
    #[serde(rename = "reroute")]
    pub r#reroute: Box<bool>,
}
