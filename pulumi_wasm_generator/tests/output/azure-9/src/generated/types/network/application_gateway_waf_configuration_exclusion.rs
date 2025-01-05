#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationGatewayWafConfigurationExclusion {
    /// Match variable of the exclusion rule to exclude header, cookie or GET arguments. Possible values are `RequestArgKeys`, `RequestArgNames`, `RequestArgValues`, `RequestCookieKeys`, `RequestCookieNames`, `RequestCookieValues`, `RequestHeaderKeys`, `RequestHeaderNames` and `RequestHeaderValues`
    #[builder(into)]
    #[serde(rename = "matchVariable")]
    pub r#match_variable: Box<String>,
    /// String value which will be used for the filter operation. If empty will exclude all traffic on this `match_variable`
    #[builder(into, default)]
    #[serde(rename = "selector")]
    pub r#selector: Box<Option<String>>,
    /// Operator which will be used to search in the variable content. Possible values are `Contains`, `EndsWith`, `Equals`, `EqualsAny` and `StartsWith`. If empty will exclude all traffic on this `match_variable`
    #[builder(into, default)]
    #[serde(rename = "selectorMatchOperator")]
    pub r#selector_match_operator: Box<Option<String>>,
}
