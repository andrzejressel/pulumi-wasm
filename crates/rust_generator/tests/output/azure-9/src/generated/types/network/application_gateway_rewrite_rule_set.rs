#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationGatewayRewriteRuleSet {
    /// The ID of the Rewrite Rule Set
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Unique name of the rewrite rule set block
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// One or more `rewrite_rule` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "rewriteRules")]
    pub r#rewrite_rules: Box<Option<Vec<super::super::types::network::ApplicationGatewayRewriteRuleSetRewriteRule>>>,
}
