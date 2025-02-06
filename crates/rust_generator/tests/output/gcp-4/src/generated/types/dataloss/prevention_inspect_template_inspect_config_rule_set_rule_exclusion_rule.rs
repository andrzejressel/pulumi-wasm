#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRule {
    /// Dictionary which defines the rule.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "dictionary")]
    pub r#dictionary: Box<Option<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleDictionary>>,
    /// Drop if the hotword rule is contained in the proximate context.
    /// For tabular data, the context includes the column name.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "excludeByHotword")]
    pub r#exclude_by_hotword: Box<Option<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleExcludeByHotword>>,
    /// Set of infoTypes for which findings would affect this rule.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "excludeInfoTypes")]
    pub r#exclude_info_types: Box<Option<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleExcludeInfoTypes>>,
    /// How the rule is applied. See the documentation for more information: https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#MatchingType
    /// Possible values are: `MATCHING_TYPE_FULL_MATCH`, `MATCHING_TYPE_PARTIAL_MATCH`, `MATCHING_TYPE_INVERSE_MATCH`.
    #[builder(into)]
    #[serde(rename = "matchingType")]
    pub r#matching_type: Box<String>,
    /// Regular expression which defines the rule.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "regex")]
    pub r#regex: Box<Option<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleRegex>>,
}
