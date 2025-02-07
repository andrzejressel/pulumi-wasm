#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ThreatIntelligenceIndicatorParsedPatternPatternTypeValue {
    /// The value of the parsed pattern type.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
    /// The type of the value of the parsed pattern type value.
    #[builder(into, default)]
    #[serde(rename = "valueType")]
    pub r#value_type: Box<Option<String>>,
}
