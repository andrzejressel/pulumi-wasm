#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSecurityPolicyRuleRedirectOption {
    /// Target for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// Type of the redirect action. Available options: EXTERNAL_302: Must specify the corresponding target field in config. GOOGLE_RECAPTCHA: Cannot specify target field in config.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
