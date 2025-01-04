#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNetworkPacketCoreControlPlaneIdentity {
    /// A list of User Assigned Managed Identity IDs assigned to this resource.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Vec<String>>,
    /// The platform type where the packet core is deployed.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
