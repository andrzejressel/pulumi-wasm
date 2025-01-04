#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVolumeGroupIdentity {
    /// A list of the User Assigned Identity IDs assigned to this Elastic SAN Volume Group.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Vec<String>>,
    /// The Principal ID associated with the Managed Service Identity assigned to this Elastic SAN Volume Group.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The Tenant ID associated with this Managed Service Identity assigned to this Elastic SAN Volume Group.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
    /// The type of Managed Identity assigned to this Elastic SAN Volume Group.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
