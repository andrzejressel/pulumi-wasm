#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ThreatIntelligenceIndicatorParsedPattern {
    /// The type key of parsed pattern.
    #[builder(into, default)]
    #[serde(rename = "patternTypeKey")]
    pub r#pattern_type_key: Box<Option<String>>,
    /// A `pattern_type_values` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "patternTypeValues")]
    pub r#pattern_type_values: Box<Option<Vec<super::super::types::sentinel::ThreatIntelligenceIndicatorParsedPatternPatternTypeValue>>>,
}
