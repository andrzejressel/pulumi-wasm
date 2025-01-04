#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSharedImageIdentifier {
    /// The Offer Name for this Shared Image.
    #[builder(into)]
    #[serde(rename = "offer")]
    pub r#offer: Box<String>,
    /// (Optional) The Purchase Plan Publisher for this Gallery Image.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
    /// The Name of the SKU for this Gallery Image.
    #[builder(into)]
    #[serde(rename = "sku")]
    pub r#sku: Box<String>,
}
