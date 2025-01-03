#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookies {
    /// List of item names, such as cookies, headers, or query strings.
    #[builder(into, default)]
    #[serde(rename = "items")]
    pub r#items: Box<Option<Vec<String>>>,
}
