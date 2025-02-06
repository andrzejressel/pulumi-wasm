#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ByteMatchSetByteMatchTupleFieldToMatch {
    /// When `type` is `HEADER`, enter the name of the header that you want to search, e.g., `User-Agent` or `Referer`.
    /// If `type` is any other value, omit this field.
    #[builder(into, default)]
    #[serde(rename = "data")]
    pub r#data: Box<Option<String>>,
    /// The part of the web request that you want AWS WAF to search for a specified string.
    /// e.g., `HEADER`, `METHOD` or `BODY`.
    /// See [docs](http://docs.aws.amazon.com/waf/latest/APIReference/API_FieldToMatch.html)
    /// for all supported values.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
