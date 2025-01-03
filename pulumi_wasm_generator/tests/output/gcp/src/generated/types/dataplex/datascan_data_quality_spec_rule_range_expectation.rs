#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatascanDataQualitySpecRuleRangeExpectation {
    /// The maximum column value allowed for a row to pass this validation. At least one of minValue and maxValue need to be provided.
    #[builder(into, default)]
    #[serde(rename = "maxValue")]
    pub r#max_value: Box<Option<String>>,
    /// The minimum column value allowed for a row to pass this validation. At least one of minValue and maxValue need to be provided.
    #[builder(into, default)]
    #[serde(rename = "minValue")]
    pub r#min_value: Box<Option<String>>,
    /// Whether each value needs to be strictly lesser than ('<') the maximum, or if equality is allowed.
    /// Only relevant if a maxValue has been defined. Default = false.
    #[builder(into, default)]
    #[serde(rename = "strictMaxEnabled")]
    pub r#strict_max_enabled: Box<Option<bool>>,
    /// Whether each value needs to be strictly greater than ('>') the minimum, or if equality is allowed.
    /// Only relevant if a minValue has been defined. Default = false.
    #[builder(into, default)]
    #[serde(rename = "strictMinEnabled")]
    pub r#strict_min_enabled: Box<Option<bool>>,
}
