#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationGatewayRewriteRuleSetRewriteRuleCondition {
    /// Perform a case in-sensitive comparison. Defaults to `false`
    #[builder(into, default)]
    #[serde(rename = "ignoreCase")]
    pub r#ignore_case: Box<Option<bool>>,
    /// Negate the result of the condition evaluation. Defaults to `false`
    #[builder(into, default)]
    #[serde(rename = "negate")]
    pub r#negate: Box<Option<bool>>,
    /// The pattern, either fixed string or regular expression, that evaluates the truthfulness of the condition.
    #[builder(into)]
    #[serde(rename = "pattern")]
    pub r#pattern: Box<String>,
    /// The [variable](https://docs.microsoft.com/azure/application-gateway/rewrite-http-headers#server-variables) of the condition.
    #[builder(into)]
    #[serde(rename = "variable")]
    pub r#variable: Box<String>,
}
