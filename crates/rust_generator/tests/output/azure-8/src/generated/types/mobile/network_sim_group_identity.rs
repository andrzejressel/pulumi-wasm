#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkSimGroupIdentity {
    /// A list of IDs for User Assigned Managed Identity resources to be assigned.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Vec<String>>,
    /// Specifies the type of Managed Service Identity. Possible value is `UserAssigned`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
