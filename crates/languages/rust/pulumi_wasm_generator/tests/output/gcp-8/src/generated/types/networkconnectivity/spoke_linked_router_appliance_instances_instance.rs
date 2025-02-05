#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpokeLinkedRouterApplianceInstancesInstance {
    /// The IP address on the VM to use for peering.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<String>,
    /// The URI of the virtual machine resource
    #[builder(into)]
    #[serde(rename = "virtualMachine")]
    pub r#virtual_machine: Box<String>,
}
