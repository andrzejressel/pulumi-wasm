#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ByteMatchSetByteMatchTuple {
    /// The part of a web request that you want to search, such as a specified header or a query string.
    #[builder(into)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Box<super::super::types::waf::ByteMatchSetByteMatchTupleFieldToMatch>,
    /// Within the portion of a web request that you want to search
    /// (for example, in the query string, if any), specify where you want to search.
    /// e.g., `CONTAINS`, `CONTAINS_WORD` or `EXACTLY`.
    /// See [docs](http://docs.aws.amazon.com/waf/latest/APIReference/API_ByteMatchTuple.html#WAF-Type-ByteMatchTuple-PositionalConstraint)
    /// for all supported values.
    #[builder(into)]
    #[serde(rename = "positionalConstraint")]
    pub r#positional_constraint: Box<String>,
    /// The value that you want to search for within the field specified by `field_to_match`, e.g., `badrefer1`.
    /// See [docs](https://docs.aws.amazon.com/waf/latest/APIReference/API_waf_ByteMatchTuple.html)
    /// for all supported values.
    #[builder(into, default)]
    #[serde(rename = "targetString")]
    pub r#target_string: Box<Option<String>>,
    /// Text transformations used to eliminate unusual formatting that attackers use in web requests in an effort to bypass AWS WAF.
    /// If you specify a transformation, AWS WAF performs the transformation on `target_string` before inspecting a request for a match.
    /// e.g., `CMD_LINE`, `HTML_ENTITY_DECODE` or `NONE`.
    /// See [docs](http://docs.aws.amazon.com/waf/latest/APIReference/API_ByteMatchTuple.html#WAF-Type-ByteMatchTuple-TextTransformation)
    /// for all supported values.
    #[builder(into)]
    #[serde(rename = "textTransformation")]
    pub r#text_transformation: Box<String>,
}
