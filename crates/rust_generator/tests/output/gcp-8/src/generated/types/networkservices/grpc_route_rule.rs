#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GrpcRouteRule {
    /// Required. A detailed rule defining how to route traffic.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<super::super::types::networkservices::GrpcRouteRuleAction>>,
    /// Matches define conditions used for matching the rule against incoming gRPC requests.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "matches")]
    pub r#matches: Box<Option<Vec<super::super::types::networkservices::GrpcRouteRuleMatch>>>,
}
