#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutomationRuleAction {
    /// A block that specifies that the automation rule action is an update to a finding field.  Documented below.
    #[builder(into, default)]
    #[serde(rename = "findingFieldsUpdate")]
    pub r#finding_fields_update: Box<Option<super::super::types::securityhub::AutomationRuleActionFindingFieldsUpdate>>,
    /// Specifies that the rule action should update the `Types` finding field. The `Types` finding field classifies findings in the format of namespace/category/classifier.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}