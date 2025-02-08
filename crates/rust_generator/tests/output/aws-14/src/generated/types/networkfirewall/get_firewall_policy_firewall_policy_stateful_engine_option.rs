#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFirewallPolicyFirewallPolicyStatefulEngineOption {
    #[builder(into)]
    #[serde(rename = "ruleOrder")]
    pub r#rule_order: Box<String>,
    #[builder(into)]
    #[serde(rename = "streamExceptionPolicy")]
    pub r#stream_exception_policy: Box<String>,
}
