#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SharedImagePurchasePlan {
    /// The Purchase Plan Name for this Shared Image. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Purchase Plan Product for this Gallery Image. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "product")]
    pub r#product: Box<Option<String>>,
    /// The Purchase Plan Publisher for this Gallery Image. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<Option<String>>,
}