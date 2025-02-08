#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionJobTriggerInspectJobInspectConfigRuleSetRule {
    /// The rule that specifies conditions when findings of infoTypes specified in InspectionRuleSet are removed from results.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "exclusionRule")]
    pub r#exclusion_rule: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRule>>,
    /// Hotword-based detection rule.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "hotwordRule")]
    pub r#hotword_rule: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleHotwordRule>>,
}
