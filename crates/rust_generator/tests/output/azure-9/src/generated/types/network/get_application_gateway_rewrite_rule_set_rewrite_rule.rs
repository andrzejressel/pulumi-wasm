#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetApplicationGatewayRewriteRuleSetRewriteRule {
    /// One or more `condition` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Box<Vec<super::super::types::network::GetApplicationGatewayRewriteRuleSetRewriteRuleCondition>>,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// One or more `request_header_configuration` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "requestHeaderConfigurations")]
    pub r#request_header_configurations: Box<Vec<super::super::types::network::GetApplicationGatewayRewriteRuleSetRewriteRuleRequestHeaderConfiguration>>,
    /// One or more `response_header_configuration` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "responseHeaderConfigurations")]
    pub r#response_header_configurations: Box<Vec<super::super::types::network::GetApplicationGatewayRewriteRuleSetRewriteRuleResponseHeaderConfiguration>>,
    /// Rule sequence of the Rewrite Rule that determines the order of execution in a set.
    #[builder(into)]
    #[serde(rename = "ruleSequence")]
    pub r#rule_sequence: Box<i32>,
    /// One `url` block as defined below
    #[builder(into)]
    #[serde(rename = "urls")]
    pub r#urls: Box<Vec<super::super::types::network::GetApplicationGatewayRewriteRuleSetRewriteRuleUrl>>,
}
