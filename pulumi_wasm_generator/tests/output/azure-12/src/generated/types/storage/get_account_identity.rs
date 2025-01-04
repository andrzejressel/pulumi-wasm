#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAccountIdentity {
    /// A list of User Assigned Managed Identity IDs assigned with the Identity of this Storage Account.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Vec<String>>,
    /// The Principal ID for the Service Principal associated with the Identity of this Storage Account.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The Tenant ID for the Service Principal associated with the Identity of this Storage Account.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
    /// The type of Managed Service Identity that is configured on this Storage Account
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
