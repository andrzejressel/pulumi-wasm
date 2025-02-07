#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EnterpriseKeyWafSettings {
    /// Supported WAF features. For more information, see https://cloud.google.com/recaptcha-enterprise/docs/usecase#comparison_of_features. Possible values: CHALLENGE_PAGE, SESSION_TOKEN, ACTION_TOKEN, EXPRESS
    #[builder(into)]
    #[serde(rename = "wafFeature")]
    pub r#waf_feature: Box<String>,
    /// The WAF service that uses this key. Possible values: CA, FASTLY
    #[builder(into)]
    #[serde(rename = "wafService")]
    pub r#waf_service: Box<String>,
}
