#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleCaptchaConfig {
    /// Defines custom immunity time. See `immunity_time_property` below for details.
    #[builder(into, default)]
    #[serde(rename = "immunityTimeProperty")]
    pub r#immunity_time_property: Box<Option<super::super::types::wafv2::WebAclRuleCaptchaConfigImmunityTimeProperty>>,
}