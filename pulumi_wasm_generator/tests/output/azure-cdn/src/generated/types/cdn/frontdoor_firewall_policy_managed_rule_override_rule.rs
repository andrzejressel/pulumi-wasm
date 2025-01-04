#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrontdoorFirewallPolicyManagedRuleOverrideRule {
    /// The action to be applied when the managed rule matches or when the anomaly score is 5 or greater. Possible values for DRS `1.1` and below are `Allow`, `Log`, `Block`, and `Redirect`. For DRS `2.0` and above the possible values are `Log` or `AnomalyScoring`.
    /// 
    /// ->**NOTE:** Please see the DRS [product documentation](https://learn.microsoft.com/azure/web-application-firewall/afds/waf-front-door-drs?tabs=drs20#anomaly-scoring-mode) for more information.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// Is the managed rule override enabled or disabled. Defaults to `false`
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// One or more `exclusion` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Box<Option<Vec<super::super::types::cdn::FrontdoorFirewallPolicyManagedRuleOverrideRuleExclusion>>>,
    /// Identifier for the managed rule.
    #[builder(into)]
    #[serde(rename = "ruleId")]
    pub r#rule_id: Box<String>,
}
