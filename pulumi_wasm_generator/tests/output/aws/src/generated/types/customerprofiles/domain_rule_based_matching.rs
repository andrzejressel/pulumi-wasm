#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainRuleBasedMatching {
    /// A block that configures information about the `AttributeTypesSelector` where the rule-based identity resolution uses to match profiles. Documented below.
    #[builder(into, default)]
    #[serde(rename = "attributeTypesSelector")]
    pub r#attribute_types_selector: Box<Option<super::super::types::customerprofiles::DomainRuleBasedMatchingAttributeTypesSelector>>,
    /// A block that specifies how the auto-merging process should resolve conflicts between different profiles. Documented below.
    #[builder(into, default)]
    #[serde(rename = "conflictResolution")]
    pub r#conflict_resolution: Box<Option<super::super::types::customerprofiles::DomainRuleBasedMatchingConflictResolution>>,
    /// The flag that enables the rule-based matching process of duplicate profiles.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// A block that specifies the configuration for exporting Identity Resolution results. Documented below.
    #[builder(into, default)]
    #[serde(rename = "exportingConfig")]
    pub r#exporting_config: Box<Option<super::super::types::customerprofiles::DomainRuleBasedMatchingExportingConfig>>,
    /// A block that configures how the rule-based matching process should match profiles. You can have up to 15 `rule` in the `natching_rules`. Documented below.
    #[builder(into, default)]
    #[serde(rename = "matchingRules")]
    pub r#matching_rules: Box<Option<Vec<super::super::types::customerprofiles::DomainRuleBasedMatchingMatchingRule>>>,
    /// Indicates the maximum allowed rule level for matching.
    #[builder(into, default)]
    #[serde(rename = "maxAllowedRuleLevelForMatching")]
    pub r#max_allowed_rule_level_for_matching: Box<Option<i32>>,
    /// Indicates the maximum allowed rule level for merging.
    #[builder(into, default)]
    #[serde(rename = "maxAllowedRuleLevelForMerging")]
    pub r#max_allowed_rule_level_for_merging: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
