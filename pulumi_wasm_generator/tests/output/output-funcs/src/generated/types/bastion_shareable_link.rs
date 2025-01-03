#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BastionShareableLink {
    /// Reference of the virtual machine resource.
    #[builder(into)]
    #[serde(rename = "vm")]
    pub r#vm: Box<String>,
}
