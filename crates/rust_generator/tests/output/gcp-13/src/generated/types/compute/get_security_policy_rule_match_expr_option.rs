#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSecurityPolicyRuleMatchExprOption {
    /// reCAPTCHA configuration options to be applied for the rule. If the rule does not evaluate reCAPTCHA tokens, this field has no effect.
    #[builder(into)]
    #[serde(rename = "recaptchaOptions")]
    pub r#recaptcha_options: Box<Vec<super::super::types::compute::GetSecurityPolicyRuleMatchExprOptionRecaptchaOption>>,
}
