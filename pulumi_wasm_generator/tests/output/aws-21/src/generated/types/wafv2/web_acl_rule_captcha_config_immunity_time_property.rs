#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleCaptchaConfigImmunityTimeProperty {
    /// The amount of time, in seconds, that a CAPTCHA or challenge timestamp is considered valid by AWS WAF. The default setting is 300.
    #[builder(into, default)]
    #[serde(rename = "immunityTime")]
    pub r#immunity_time: Box<Option<i32>>,
}
