#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationTargetTargetDocumentAttributeValue {
    /// A date expressed as an ISO 8601 string. It is important for the time zone to be included in the ISO 8601 date-time format. As of this writing only UTC is supported. For example, `2012-03-25T12:30:10+00:00`.
    #[builder(into, default)]
    #[serde(rename = "dateValue")]
    pub r#date_value: Box<Option<String>>,
    /// A long integer value.
    #[builder(into, default)]
    #[serde(rename = "longValue")]
    pub r#long_value: Box<Option<i32>>,
    /// A list of strings.
    #[builder(into, default)]
    #[serde(rename = "stringListValues")]
    pub r#string_list_values: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Box<Option<String>>,
}
