#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RunBookDraftContentLinkHash {
    /// Specifies the hash algorithm used to hash the content.
    #[builder(into)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Box<String>,
    /// Specifies the expected hash value of the content.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
