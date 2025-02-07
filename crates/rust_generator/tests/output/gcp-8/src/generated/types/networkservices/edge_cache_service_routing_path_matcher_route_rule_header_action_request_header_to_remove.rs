#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderActionRequestHeaderToRemove {
    /// The name of the header to remove.
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: Box<String>,
}
