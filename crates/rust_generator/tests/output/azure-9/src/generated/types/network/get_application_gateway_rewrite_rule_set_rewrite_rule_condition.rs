#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetApplicationGatewayRewriteRuleSetRewriteRuleCondition {
    /// Whether a case insensitive comparison is performed.
    #[builder(into)]
    #[serde(rename = "ignoreCase")]
    pub r#ignore_case: Box<bool>,
    /// Whether the result of the condition evaluation is negated.
    #[builder(into)]
    #[serde(rename = "negate")]
    pub r#negate: Box<bool>,
    /// The pattern, either fixed string or regular expression, that evaluates the truthfulness of the condition.
    #[builder(into)]
    #[serde(rename = "pattern")]
    pub r#pattern: Box<String>,
    /// The [variable](https://docs.microsoft.com/azure/application-gateway/rewrite-http-headers#server-variables) of the condition.
    #[builder(into)]
    #[serde(rename = "variable")]
    pub r#variable: Box<String>,
}
