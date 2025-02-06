#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApplicationGatewayRewriteRuleSet {
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// One or more `rewrite_rule` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "rewriteRules")]
    pub r#rewrite_rules: Box<Vec<super::super::types::network::GetApplicationGatewayRewriteRuleSetRewriteRule>>,
}
