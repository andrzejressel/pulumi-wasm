#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionInspectTemplateInspectConfigRuleSet {
    /// List of infoTypes this rule set is applied to.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "infoTypes")]
    pub r#info_types: Box<Vec<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetInfoType>>,
    /// Set of rules to be applied to infoTypes. The rules are applied in order.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Vec<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRule>>,
}
