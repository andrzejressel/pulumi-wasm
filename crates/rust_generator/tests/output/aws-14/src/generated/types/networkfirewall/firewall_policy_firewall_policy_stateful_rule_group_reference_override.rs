#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallPolicyFirewallPolicyStatefulRuleGroupReferenceOverride {
    /// The action that changes the rule group from DROP to ALERT . This only applies to managed rule groups.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
}
