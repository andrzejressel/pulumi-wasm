#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesBotControlRuleSet {
    /// Applies only to the targeted inspection level. Determines whether to use machine learning (ML) to analyze your web traffic for bot-related activity. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enableMachineLearning")]
    pub r#enable_machine_learning: Box<Option<bool>>,
    /// The inspection level to use for the Bot Control rule group.
    #[builder(into)]
    #[serde(rename = "inspectionLevel")]
    pub r#inspection_level: Box<String>,
}