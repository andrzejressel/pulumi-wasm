#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetIdentity {
    /// The list of the User Assigned Identity IDs that is assigned to this Load Test Service.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Vec<String>>,
    /// The Principal ID for the System-Assigned Managed Identity assigned to this Load Test Service.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The Tenant ID for the System-Assigned Managed Identity assigned to this Load Test Service.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
    /// Type of Managed Service Identity that is assigned to this Load Test Encryption.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
