#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SizeConstraintSetSizeConstraint {
    /// Type of comparison you want to perform, such as `EQ`, `NE`, `LT`, or `GT`. Please refer to the [documentation](https://docs.aws.amazon.com/waf/latest/APIReference/API_wafRegional_SizeConstraint.html) for a complete list of supported values.
    #[builder(into)]
    #[serde(rename = "comparisonOperator")]
    pub r#comparison_operator: Box<String>,
    /// Parameter that specifies where in a web request to look for the size constraint.
    #[builder(into)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Box<super::super::types::waf::SizeConstraintSetSizeConstraintFieldToMatch>,
    /// Size in bytes that you want to compare against the size of the specified `field_to_match`. Valid values for `size` are between 0 and 21474836480 bytes (0 and 20 GB).
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<i32>,
    /// Parameter is used to eliminate unusual formatting that attackers may use in web requests to bypass AWS WAF. When a transformation is specified, AWS WAF performs the transformation on the `field_to_match` before inspecting the request for a match. Some examples of supported transformations are `CMD_LINE`, `HTML_ENTITY_DECODE`, and `NONE`. You can find a complete list of supported values in the [AWS WAF API Reference](http://docs.aws.amazon.com/waf/latest/APIReference/API_SizeConstraint.html#WAF-Type-SizeConstraint-TextTransformation).
    /// **Note:** If you choose `BODY` as the `type`, you must also choose `NONE` because CloudFront only forwards the first 8192 bytes for inspection.
    #[builder(into)]
    #[serde(rename = "textTransformation")]
    pub r#text_transformation: Box<String>,
}
