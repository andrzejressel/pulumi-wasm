#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleStatementRateBasedStatementScopeDownStatementIpSetReferenceStatement {
    /// The Amazon Resource Name (ARN) of the IP Set that this statement references.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// The configuration for inspecting IP addresses in an HTTP header that you specify, instead of using the IP address that's reported by the web request origin. See IPSet Forwarded IP Config below for more details.
    #[builder(into, default)]
    #[serde(rename = "ipSetForwardedIpConfig")]
    pub r#ip_set_forwarded_ip_config: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementIpSetReferenceStatementIpSetForwardedIpConfig>>,
}
