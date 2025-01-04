#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleCaptchaConfig {
    /// Defines custom immunity time. See Immunity Time Property below for details.
    #[builder(into, default)]
    #[serde(rename = "immunityTimeProperty")]
    pub r#immunity_time_property: Box<Option<super::super::types::wafv2::RuleGroupRuleCaptchaConfigImmunityTimeProperty>>,
}
