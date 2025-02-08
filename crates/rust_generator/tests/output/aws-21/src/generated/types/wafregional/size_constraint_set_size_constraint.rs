#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SizeConstraintSetSizeConstraint {
    /// The type of comparison you want to perform.
    /// e.g., `EQ`, `NE`, `LT`, `GT`.
    /// See [docs](https://docs.aws.amazon.com/waf/latest/APIReference/API_wafRegional_SizeConstraint.html) for all supported values.
    #[builder(into)]
    #[serde(rename = "comparisonOperator")]
    pub r#comparison_operator: Box<String>,
    /// Specifies where in a web request to look for the size constraint.
    #[builder(into)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Box<super::super::types::wafregional::SizeConstraintSetSizeConstraintFieldToMatch>,
    /// The size in bytes that you want to compare against the size of the specified `field_to_match`.
    /// Valid values are between 0 - 21474836480 bytes (0 - 20 GB).
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<i32>,
    /// Text transformations used to eliminate unusual formatting that attackers use in web requests in an effort to bypass AWS WAF.
    /// If you specify a transformation, AWS WAF performs the transformation on `field_to_match` before inspecting a request for a match.
    /// e.g., `CMD_LINE`, `HTML_ENTITY_DECODE` or `NONE`.
    /// See [docs](http://docs.aws.amazon.com/waf/latest/APIReference/API_SizeConstraint.html#WAF-Type-SizeConstraint-TextTransformation)
    /// for all supported values.
    /// **Note:** if you choose `BODY` as `type`, you must choose `NONE` because CloudFront forwards only the first 8192 bytes for inspection.
    #[builder(into)]
    #[serde(rename = "textTransformation")]
    pub r#text_transformation: Box<String>,
}
