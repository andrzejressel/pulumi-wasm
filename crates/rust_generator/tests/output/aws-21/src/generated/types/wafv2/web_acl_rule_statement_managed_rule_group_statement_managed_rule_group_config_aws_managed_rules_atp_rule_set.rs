#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSet {
    /// Whether or not to allow the use of regular expressions in the login page path.
    #[builder(into, default)]
    #[serde(rename = "enableRegexInPath")]
    pub r#enable_regex_in_path: Box<Option<bool>>,
    /// The path of the login endpoint for your application.
    #[builder(into)]
    #[serde(rename = "loginPath")]
    pub r#login_path: Box<String>,
    /// The criteria for inspecting login requests, used by the ATP rule group to validate credentials usage. See `request_inspection` for more details.
    #[builder(into, default)]
    #[serde(rename = "requestInspection")]
    pub r#request_inspection: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetRequestInspection>>,
    /// The criteria for inspecting responses to login requests, used by the ATP rule group to track login failure rates. Note that Response Inspection is available only on web ACLs that protect CloudFront distributions. See `response_inspection` for more details.
    #[builder(into, default)]
    #[serde(rename = "responseInspection")]
    pub r#response_inspection: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetResponseInspection>>,
}
