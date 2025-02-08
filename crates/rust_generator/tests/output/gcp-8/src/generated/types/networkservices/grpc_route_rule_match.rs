#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GrpcRouteRuleMatch {
    /// Specifies a list of HTTP request headers to match against.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::networkservices::GrpcRouteRuleMatchHeader>>>,
    /// A gRPC method to match against. If this field is empty or omitted, will match all methods.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "method")]
    pub r#method: Box<Option<super::super::types::networkservices::GrpcRouteRuleMatchMethod>>,
}
