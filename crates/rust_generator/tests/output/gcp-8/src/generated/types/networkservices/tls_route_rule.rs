#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TlsRouteRule {
    /// Required. A detailed rule defining how to route traffic.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<super::super::types::networkservices::TlsRouteRuleAction>,
    /// Matches define the predicate used to match requests to a given action.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "matches")]
    pub r#matches: Box<Vec<super::super::types::networkservices::TlsRouteRuleMatch>>,
}
