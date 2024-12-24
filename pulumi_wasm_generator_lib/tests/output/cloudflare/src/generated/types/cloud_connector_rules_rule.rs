#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct CloudConnectorRulesRule {
    /// Brief summary of the cloud connector rule and its intended use.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Whether the headers rule is active.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Criteria for an HTTP request to trigger the cloud connector rule. Uses the Firewall Rules expression language based on Wireshark display filters.
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    /// Cloud Connector Rule Parameters
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<super::types::CloudConnectorRulesRuleParameters>>,
    /// Type of provider. Available values: `aws_s3`, `cloudflare_r2`, `azure_storage`, `gcp_storage`
    #[builder(into)]
    #[serde(rename = "provider")]
    pub r#provider: Box<String>,
}
