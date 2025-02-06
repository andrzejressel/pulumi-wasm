#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HttpRouteRuleActionRequestMirrorPolicy {
    /// The destination the requests will be mirrored to.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "destination")]
    pub r#destination: Box<Option<super::super::types::networkservices::HttpRouteRuleActionRequestMirrorPolicyDestination>>,
}
