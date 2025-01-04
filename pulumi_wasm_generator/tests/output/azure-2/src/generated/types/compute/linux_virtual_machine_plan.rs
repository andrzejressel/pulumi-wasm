#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxVirtualMachinePlan {
    /// Specifies the Name of the Marketplace Image this Virtual Machine should be created from. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the Product of the Marketplace Image this Virtual Machine should be created from. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "product")]
    pub r#product: Box<String>,
    /// Specifies the Publisher of the Marketplace Image this Virtual Machine should be created from. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
}
