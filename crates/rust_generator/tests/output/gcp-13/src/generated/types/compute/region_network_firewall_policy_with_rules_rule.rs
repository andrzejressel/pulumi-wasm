#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionNetworkFirewallPolicyWithRulesRule {
    /// The Action to perform when the client connection triggers the rule. Can currently be either
    /// "allow", "deny", "apply_security_profile_group" or "goto_next".
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// A description of the rule.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The direction in which this rule applies. If unspecified an INGRESS rule is created.
    /// Possible values are: `INGRESS`, `EGRESS`.
    #[builder(into, default)]
    #[serde(rename = "direction")]
    pub r#direction: Box<Option<String>>,
    /// Denotes whether the firewall policy rule is disabled. When set to true,
    /// the firewall policy rule is not enforced and traffic behaves as if it did
    /// not exist. If this is unspecified, the firewall policy rule will be
    /// enabled.
    #[builder(into, default)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
    /// Denotes whether to enable logging for a particular rule.
    /// If logging is enabled, logs will be exported to the
    /// configured export destination in Stackdriver.
    #[builder(into, default)]
    #[serde(rename = "enableLogging")]
    pub r#enable_logging: Box<Option<bool>>,
    /// A match condition that incoming traffic is evaluated against. If it evaluates to true, the corresponding 'action' is enforced.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: Box<super::super::types::compute::RegionNetworkFirewallPolicyWithRulesRuleMatch>,
    /// An integer indicating the priority of a rule in the list. The priority must be a value
    /// between 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the
    /// highest priority and 2147483647 is the lowest priority.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// An optional name for the rule. This field is not a unique identifier
    /// and can be updated.
    #[builder(into, default)]
    #[serde(rename = "ruleName")]
    pub r#rule_name: Box<Option<String>>,
    /// A fully-qualified URL of a SecurityProfile resource instance.
    /// Example:
    /// https://networksecurity.googleapis.com/v1/projects/{project}/locations/{location}/securityProfileGroups/my-security-profile-group
    /// Must be specified if action is 'apply_security_profile_group'.
    #[builder(into, default)]
    #[serde(rename = "securityProfileGroup")]
    pub r#security_profile_group: Box<Option<String>>,
    /// A list of secure tags that controls which instances the firewall rule
    /// applies to. If <code>targetSecureTag</code> are specified, then the
    /// firewall rule applies only to instances in the VPC network that have one
    /// of those EFFECTIVE secure tags, if all the target_secure_tag are in
    /// INEFFECTIVE state, then this rule will be ignored.
    /// <code>targetSecureTag</code> may not be set at the same time as
    /// <code>targetServiceAccounts</code>.
    /// If neither <code>targetServiceAccounts</code> nor
    /// <code>targetSecureTag</code> are specified, the firewall rule applies
    /// to all instances on the specified network.
    /// Maximum number of target label tags allowed is 256.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "targetSecureTags")]
    pub r#target_secure_tags: Box<Option<Vec<super::super::types::compute::RegionNetworkFirewallPolicyWithRulesRuleTargetSecureTag>>>,
    /// A list of service accounts indicating the sets of
    /// instances that are applied with this rule.
    #[builder(into, default)]
    #[serde(rename = "targetServiceAccounts")]
    pub r#target_service_accounts: Box<Option<Vec<String>>>,
    /// Boolean flag indicating if the traffic should be TLS decrypted.
    /// It can be set only if action = 'apply_security_profile_group' and cannot be set for other actions.
    #[builder(into, default)]
    #[serde(rename = "tlsInspect")]
    pub r#tls_inspect: Box<Option<bool>>,
}
