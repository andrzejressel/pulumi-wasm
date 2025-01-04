#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAppIdentity {
    /// A list of one or more Resource IDs for User Assigned Managed identities to assign. Required when `type` is set to `UserAssigned`.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
    /// The type of managed identity to assign. Possible values are `UserAssigned` and `SystemAssigned`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
