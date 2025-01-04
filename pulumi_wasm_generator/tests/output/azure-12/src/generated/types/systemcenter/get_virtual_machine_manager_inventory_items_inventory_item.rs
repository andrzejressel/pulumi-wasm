#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualMachineManagerInventoryItemsInventoryItem {
    /// The ID of the System Center Virtual Machine Manager Inventory Item.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of the System Center Virtual Machine Manager Inventory Item.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The UUID of the System Center Virtual Machine Manager Inventory Item that is assigned by System Center Virtual Machine Manager.
    #[builder(into)]
    #[serde(rename = "uuid")]
    pub r#uuid: Box<String>,
}
