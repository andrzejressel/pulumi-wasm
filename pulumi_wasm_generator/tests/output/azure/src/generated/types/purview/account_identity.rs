#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountIdentity {
    /// Specifies a list of User Assigned Managed Identity IDs to be assigned to this Purview Account.
    /// 
    /// > **NOTE:** This is required when `type` is set to `UserAssigned`.
    #[builder(into, default)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Option<Vec<String>>>,
    /// The Principal ID associated with this Managed Service Identity.
    #[builder(into, default)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<Option<String>>,
    /// The Tenant ID associated with this Managed Service Identity.
    #[builder(into, default)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<Option<String>>,
    /// Specifies the type of Managed Service Identity that should be configured on this Purview Account. Possible values are `UserAssigned` and `SystemAssigned`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}