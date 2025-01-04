#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAppTemplateCustomScaleRule {
    #[builder(into)]
    #[serde(rename = "authentications")]
    pub r#authentications: Box<Vec<super::super::types::containerapp::GetAppTemplateCustomScaleRuleAuthentication>>,
    #[builder(into)]
    #[serde(rename = "customRuleType")]
    pub r#custom_rule_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<std::collections::HashMap<String, String>>,
    /// The name of the Container App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
