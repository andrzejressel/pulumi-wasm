#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleStatementRateBasedStatementForwardedIpConfig {
    /// Match status to assign to the web request if the request doesn't have a valid IP address in the specified position. Valid values include: `MATCH` or `NO_MATCH`.
    #[builder(into)]
    #[serde(rename = "fallbackBehavior")]
    pub r#fallback_behavior: Box<String>,
    /// Name of the HTTP header to use for the IP address.
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: Box<String>,
}
