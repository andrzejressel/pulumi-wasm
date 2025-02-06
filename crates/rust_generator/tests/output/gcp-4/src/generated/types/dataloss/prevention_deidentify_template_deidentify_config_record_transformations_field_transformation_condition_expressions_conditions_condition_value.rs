#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationConditionExpressionsConditionsConditionValue {
    /// A boolean value.
    #[builder(into, default)]
    #[serde(rename = "booleanValue")]
    pub r#boolean_value: Box<Option<bool>>,
    /// Represents a whole or partial calendar date.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "dateValue")]
    pub r#date_value: Box<Option<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationConditionExpressionsConditionsConditionValueDateValue>>,
    /// Represents a day of the week.
    /// Possible values are: `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`, `SUNDAY`.
    #[builder(into, default)]
    #[serde(rename = "dayOfWeekValue")]
    pub r#day_of_week_value: Box<Option<String>>,
    /// A float value.
    #[builder(into, default)]
    #[serde(rename = "floatValue")]
    pub r#float_value: Box<Option<f64>>,
    /// An integer value (int64 format)
    #[builder(into, default)]
    #[serde(rename = "integerValue")]
    pub r#integer_value: Box<Option<String>>,
    /// A string value.
    #[builder(into, default)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Box<Option<String>>,
    /// Represents a time of day.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "timeValue")]
    pub r#time_value: Box<Option<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationConditionExpressionsConditionsConditionValueTimeValue>>,
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into, default)]
    #[serde(rename = "timestampValue")]
    pub r#timestamp_value: Box<Option<String>>,
}
