#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrontdoorFirewallPolicyManagedRule {
    /// The action to perform for all DRS rules when the managed rule is matched or when the anomaly score is 5 or greater depending on which version of the DRS you are using. Possible values include `Allow`, `Log`, `Block`, and `Redirect`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// One or more `exclusion` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Box<Option<Vec<super::super::types::cdn::FrontdoorFirewallPolicyManagedRuleExclusion>>>,
    /// One or more `override` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<Vec<super::super::types::cdn::FrontdoorFirewallPolicyManagedRuleOverride>>>,
    /// The name of the managed rule to use with this resource. Possible values include `DefaultRuleSet`, `Microsoft_DefaultRuleSet`, `BotProtection` or `Microsoft_BotManagerRuleSet`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// The version of the managed rule to use with this resource. Possible values depends on which DRS type you are using, for the `DefaultRuleSet` type the possible values include `1.0` or `preview-0.1`. For `Microsoft_DefaultRuleSet` the possible values include `1.1`, `2.0` or `2.1`. For `BotProtection` the value must be `preview-0.1` and for `Microsoft_BotManagerRuleSet` the value must be `1.0`.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
