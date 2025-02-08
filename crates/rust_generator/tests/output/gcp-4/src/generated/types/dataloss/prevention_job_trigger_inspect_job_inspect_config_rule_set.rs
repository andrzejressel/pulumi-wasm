#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionJobTriggerInspectJobInspectConfigRuleSet {
    /// List of infoTypes this rule set is applied to.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "infoTypes")]
    pub r#info_types: Box<Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetInfoType>>>,
    /// Set of rules to be applied to infoTypes. The rules are applied in order.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRule>>,
}
