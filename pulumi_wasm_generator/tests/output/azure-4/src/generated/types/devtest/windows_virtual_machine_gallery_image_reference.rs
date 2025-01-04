#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsVirtualMachineGalleryImageReference {
    /// The Offer of the Gallery Image. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "offer")]
    pub r#offer: Box<String>,
    /// The Publisher of the Gallery Image. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
    /// The SKU of the Gallery Image. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sku")]
    pub r#sku: Box<String>,
    /// The Version of the Gallery Image. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
