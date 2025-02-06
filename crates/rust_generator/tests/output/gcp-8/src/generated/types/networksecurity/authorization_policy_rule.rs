#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthorizationPolicyRule {
    /// List of attributes for the traffic destination. All of the destinations must match. A destination is a match if a request matches all the specified hosts, ports, methods and headers.
    /// If not set, the action specified in the 'action' field will be applied without any rule checks for the destination.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "destinations")]
    pub r#destinations: Box<Option<Vec<super::super::types::networksecurity::AuthorizationPolicyRuleDestination>>>,
    /// List of attributes for the traffic source. All of the sources must match. A source is a match if both principals and ipBlocks match.
    /// If not set, the action specified in the 'action' field will be applied without any rule checks for the source.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sources")]
    pub r#sources: Box<Option<Vec<super::super::types::networksecurity::AuthorizationPolicyRuleSource>>>,
}
