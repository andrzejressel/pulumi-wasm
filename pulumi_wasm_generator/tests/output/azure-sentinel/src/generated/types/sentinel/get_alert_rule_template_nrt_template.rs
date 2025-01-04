#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAlertRuleTemplateNrtTemplate {
    /// The description of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The query of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: Box<String>,
    /// The alert severity of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: Box<String>,
    /// A list of categories of attacks by which to classify the rule.
    #[builder(into)]
    #[serde(rename = "tactics")]
    pub r#tactics: Box<Vec<String>>,
}
