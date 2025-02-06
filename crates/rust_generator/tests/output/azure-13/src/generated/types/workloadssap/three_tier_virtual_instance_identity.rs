#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ThreeTierVirtualInstanceIdentity {
    /// A list of User Assigned Managed Identity IDs to be assigned to this SAP Three Tier Virtual Instance.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Vec<String>>,
    /// The type of Managed Service Identity that should be configured on this SAP Three Tier Virtual Instance. Only possible value is `UserAssigned`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
