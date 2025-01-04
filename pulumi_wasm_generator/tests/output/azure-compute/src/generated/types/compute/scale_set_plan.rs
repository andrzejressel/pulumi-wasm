#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScaleSetPlan {
    /// Specifies the name of the image from the marketplace.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the product of the image from the marketplace.
    #[builder(into)]
    #[serde(rename = "product")]
    pub r#product: Box<String>,
    /// Specifies the publisher of the image.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
}
