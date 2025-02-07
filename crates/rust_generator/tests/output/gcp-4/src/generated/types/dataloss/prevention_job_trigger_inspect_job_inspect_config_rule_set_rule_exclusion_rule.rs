#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRule {
    /// Dictionary which defines the rule.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "dictionary")]
    pub r#dictionary: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleDictionary>>,
    /// Drop if the hotword rule is contained in the proximate context.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "excludeByHotword")]
    pub r#exclude_by_hotword: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleExcludeByHotword>>,
    /// Set of infoTypes for which findings would affect this rule.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "excludeInfoTypes")]
    pub r#exclude_info_types: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleExcludeInfoTypes>>,
    /// How the rule is applied. See the documentation for more information: https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#MatchingType
    /// Possible values are: `MATCHING_TYPE_FULL_MATCH`, `MATCHING_TYPE_PARTIAL_MATCH`, `MATCHING_TYPE_INVERSE_MATCH`.
    #[builder(into)]
    #[serde(rename = "matchingType")]
    pub r#matching_type: Box<String>,
    /// Regular expression which defines the rule.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "regex")]
    pub r#regex: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleRegex>>,
}
