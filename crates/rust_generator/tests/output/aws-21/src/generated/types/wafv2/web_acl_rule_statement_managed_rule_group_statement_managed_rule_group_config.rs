#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfig {
    /// Additional configuration for using the Account Creation Fraud Prevention managed rule group. Use this to specify information such as the registration page of your application and the type of content to accept or reject from the client.
    #[builder(into, default)]
    #[serde(rename = "awsManagedRulesAcfpRuleSet")]
    pub r#aws_managed_rules_acfp_rule_set: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSet>>,
    /// Additional configuration for using the Account Takeover Protection managed rule group. Use this to specify information such as the sign-in page of your application and the type of content to accept or reject from the client.
    #[builder(into, default)]
    #[serde(rename = "awsManagedRulesAtpRuleSet")]
    pub r#aws_managed_rules_atp_rule_set: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSet>>,
    /// Additional configuration for using the Bot Control managed rule group. Use this to specify the inspection level that you want to use. See `aws_managed_rules_bot_control_rule_set` for more details
    #[builder(into, default)]
    #[serde(rename = "awsManagedRulesBotControlRuleSet")]
    pub r#aws_managed_rules_bot_control_rule_set: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesBotControlRuleSet>>,
    /// The path of the login endpoint for your application.
    #[builder(into, default)]
    #[serde(rename = "loginPath")]
    pub r#login_path: Box<Option<String>>,
    /// Details about your login page password field. See `password_field` for more details.
    #[builder(into, default)]
    #[serde(rename = "passwordField")]
    pub r#password_field: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigPasswordField>>,
    /// The payload type for your login endpoint, either JSON or form encoded.
    #[builder(into, default)]
    #[serde(rename = "payloadType")]
    pub r#payload_type: Box<Option<String>>,
    /// Details about your login page username field. See `username_field` for more details.
    #[builder(into, default)]
    #[serde(rename = "usernameField")]
    pub r#username_field: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigUsernameField>>,
}
