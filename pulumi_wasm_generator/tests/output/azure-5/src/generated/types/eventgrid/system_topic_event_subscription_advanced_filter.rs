#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SystemTopicEventSubscriptionAdvancedFilter {
    /// Compares a value of an event using a single boolean value.
    #[builder(into, default)]
    #[serde(rename = "boolEquals")]
    pub r#bool_equals: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterBoolEqual>>>,
    /// Evaluates if a value of an event isn't NULL or undefined.
    #[builder(into, default)]
    #[serde(rename = "isNotNulls")]
    pub r#is_not_nulls: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterIsNotNull>>>,
    /// Evaluates if a value of an event is NULL or undefined.
    /// 
    /// Each nested block consists of a key and a value(s) element.
    #[builder(into, default)]
    #[serde(rename = "isNullOrUndefineds")]
    pub r#is_null_or_undefineds: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterIsNullOrUndefined>>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into, default)]
    #[serde(rename = "numberGreaterThanOrEquals")]
    pub r#number_greater_than_or_equals: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThanOrEqual>>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into, default)]
    #[serde(rename = "numberGreaterThans")]
    pub r#number_greater_thans: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThan>>>,
    /// Compares a value of an event using multiple floating point number ranges.
    #[builder(into, default)]
    #[serde(rename = "numberInRanges")]
    pub r#number_in_ranges: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberInRange>>>,
    /// Compares a value of an event using multiple floating point numbers.
    #[builder(into, default)]
    #[serde(rename = "numberIns")]
    pub r#number_ins: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberIn>>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into, default)]
    #[serde(rename = "numberLessThanOrEquals")]
    pub r#number_less_than_or_equals: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberLessThanOrEqual>>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into, default)]
    #[serde(rename = "numberLessThans")]
    pub r#number_less_thans: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberLessThan>>>,
    /// Compares a value of an event using multiple floating point number ranges.
    #[builder(into, default)]
    #[serde(rename = "numberNotInRanges")]
    pub r#number_not_in_ranges: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberNotInRange>>>,
    /// Compares a value of an event using multiple floating point numbers.
    #[builder(into, default)]
    #[serde(rename = "numberNotIns")]
    pub r#number_not_ins: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberNotIn>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringBeginsWiths")]
    pub r#string_begins_withs: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringBeginsWith>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringContains")]
    pub r#string_contains: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringContain>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringEndsWiths")]
    pub r#string_ends_withs: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringEndsWith>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringIns")]
    pub r#string_ins: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringIn>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringNotBeginsWiths")]
    pub r#string_not_begins_withs: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringNotBeginsWith>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringNotContains")]
    pub r#string_not_contains: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringNotContain>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringNotEndsWiths")]
    pub r#string_not_ends_withs: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringNotEndsWith>>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into, default)]
    #[serde(rename = "stringNotIns")]
    pub r#string_not_ins: Box<Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringNotIn>>>,
}
