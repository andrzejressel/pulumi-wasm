#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserAgentBlockingRuleConfiguration {
    /// The configuration target for this rule. You must set the target to ua for User Agent Blocking rules.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// The exact user agent string to match. This value will be compared to the received User-Agent HTTP header value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
