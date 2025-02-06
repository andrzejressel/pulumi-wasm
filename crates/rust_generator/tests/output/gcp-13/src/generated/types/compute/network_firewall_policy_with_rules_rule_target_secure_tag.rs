#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkFirewallPolicyWithRulesRuleTargetSecureTag {
    /// Name of the secure tag, created with TagManager's TagValue API.
    /// @pattern tagValues/[0-9]+
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// (Output)
    /// [Output Only] State of the secure tag, either `EFFECTIVE` or
    /// `INEFFECTIVE`. A secure tag is `INEFFECTIVE` when it is deleted
    /// or its network is deleted.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
