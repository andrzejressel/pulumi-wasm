#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventSubscriptionAdvancedFilter {
    /// Compares a value of an event using a single boolean value.
    #[builder(into, default)]
    #[serde(rename = "boolEquals")]
    pub r#bool_equals: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterBoolEqual>>>,
    /// Evaluates if a value of an event isn't NULL or undefined.
    #[builder(into, default)]
    #[serde(rename = "isNotNulls")]
    pub r#is_not_nulls: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterIsNotNull>>>,
    /// Evaluates if a value of an event is NULL or undefined.
    /// 
    /// Each nested block consists of a key and a value(s) element.
    #[builder(into, default)]
    #[serde(rename = "isNullOrUndefineds")]
    pub r#is_null_or_undefineds: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterIsNullOrUndefined>>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into, default)]
    #[serde(rename = "numberGreaterThanOrEquals")]
    pub r#number_greater_than_or_equals: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterNumberGreaterThanOrEqual>>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into, default)]
    #[serde(rename = "numberGreaterThans")]
    pub r#number_greater_thans: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterNumberGreaterThan>>>,
    /// Compares a value of an event using multiple floating point number ranges.
    #[builder(into, default)]
    #[serde(rename = "numberInRanges")]
    pub r#number_in_ranges: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterNumberInRange>>>,
    /// Compares a value of an event using multiple floating point numbers.
    #[builder(into, default)]
    #[serde(rename = "numberIns")]
    pub r#number_ins: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterNumberIn>>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into, default)]
    #[serde(rename = "numberLessThanOrEquals")]
    pub r#number_less_than_or_equals: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterNumberLessThanOrEqual>>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into, default)]
    #[serde(rename = "numberLessThans")]
    pub r#number_less_thans: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterNumberLessThan>>>,
    /// Compares a value of an event using multiple floating point number ranges.
    #[builder(into, default)]
    #[serde(rename = "numberNotInRanges")]
    pub r#number_not_in_ranges: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterNumberNotInRange>>>,
    /// Compares a value of an event using multiple floating point numbers.
    #[builder(into, default)]
    #[serde(rename = "numberNotIns")]
    pub r#number_not_ins: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterNumberNotIn>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringBeginsWiths")]
    pub r#string_begins_withs: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterStringBeginsWith>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringContains")]
    pub r#string_contains: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterStringContain>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringEndsWiths")]
    pub r#string_ends_withs: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterStringEndsWith>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringIns")]
    pub r#string_ins: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterStringIn>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringNotBeginsWiths")]
    pub r#string_not_begins_withs: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterStringNotBeginsWith>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringNotContains")]
    pub r#string_not_contains: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterStringNotContain>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringNotEndsWiths")]
    pub r#string_not_ends_withs: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterStringNotEndsWith>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringNotIns")]
    pub r#string_not_ins: Box<Option<Vec<super::super::types::eventhub::EventSubscriptionAdvancedFilterStringNotIn>>>,
}
