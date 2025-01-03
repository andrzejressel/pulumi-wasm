#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ZeroTrustAccessApplicationTargetCriteriaTargetAttribute {
    /// The key of the attribute.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The values of the attribute.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
