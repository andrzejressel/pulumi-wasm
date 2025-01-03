#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TcpRouteRule {
    /// A detailed rule defining how to route traffic.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<super::super::types::networkservices::TcpRouteRuleAction>,
    /// RouteMatch defines the predicate used to match requests to a given action. Multiple match types are "OR"ed for evaluation.
    /// If no routeMatch field is specified, this rule will unconditionally match traffic.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "matches")]
    pub r#matches: Box<Option<Vec<super::super::types::networkservices::TcpRouteRuleMatch>>>,
}
