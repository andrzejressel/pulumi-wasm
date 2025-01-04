#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ByteMatchSetByteMatchTupleFieldToMatch {
    /// When the value of Type is HEADER, enter the name of the header that you want AWS WAF to search, for example, User-Agent or Referer. If the value of Type is any other value, omit Data.
    #[builder(into, default)]
    #[serde(rename = "data")]
    pub r#data: Box<Option<String>>,
    /// The part of the web request that you want AWS WAF to search for a specified string.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
