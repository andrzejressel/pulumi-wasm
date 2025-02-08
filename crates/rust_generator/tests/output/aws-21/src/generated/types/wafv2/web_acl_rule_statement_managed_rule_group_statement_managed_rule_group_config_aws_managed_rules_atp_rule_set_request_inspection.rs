#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetRequestInspection {
    /// Details about your login page password field. See `password_field` for more details.
    #[builder(into)]
    #[serde(rename = "passwordField")]
    pub r#password_field: Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetRequestInspectionPasswordField>,
    /// The payload type for your login endpoint, either JSON or form encoded.
    #[builder(into)]
    #[serde(rename = "payloadType")]
    pub r#payload_type: Box<String>,
    /// Details about your login page username field. See `username_field` for more details.
    #[builder(into)]
    #[serde(rename = "usernameField")]
    pub r#username_field: Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetRequestInspectionUsernameField>,
}
