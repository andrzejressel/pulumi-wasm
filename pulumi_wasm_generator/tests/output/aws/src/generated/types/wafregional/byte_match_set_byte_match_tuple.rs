#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ByteMatchSetByteMatchTuple {
    /// Settings for the ByteMatchTuple. FieldToMatch documented below.
    #[builder(into)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Box<super::super::types::wafregional::ByteMatchSetByteMatchTupleFieldToMatch>,
    /// Within the portion of a web request that you want to search.
    #[builder(into)]
    #[serde(rename = "positionalConstraint")]
    pub r#positional_constraint: Box<String>,
    /// The value that you want AWS WAF to search for. The maximum length of the value is 50 bytes.
    #[builder(into, default)]
    #[serde(rename = "targetString")]
    pub r#target_string: Box<Option<String>>,
    /// The formatting way for web request.
    /// 
    /// FieldToMatch(field_to_match) support following:
    #[builder(into)]
    #[serde(rename = "textTransformation")]
    pub r#text_transformation: Box<String>,
}
