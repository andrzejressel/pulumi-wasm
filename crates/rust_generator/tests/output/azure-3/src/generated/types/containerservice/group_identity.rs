#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupIdentity {
    /// Specifies a list of User Assigned Managed Identity IDs to be assigned to this Container Group.
    /// 
    /// > **NOTE:** This is required when `type` is set to `UserAssigned` or `SystemAssigned, UserAssigned`.
    /// 
    /// > **NOTE:** Currently you can't use a managed identity in a container group deployed to a virtual network.
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
    /// Specifies the type of Managed Service Identity that should be configured on this Container Group. Possible values are `SystemAssigned`, `UserAssigned`, `SystemAssigned, UserAssigned` (to enable both).
    /// 
    /// > **NOTE:** When `type` is set to `SystemAssigned`, the identity of the Principal ID can be retrieved after the container group has been created. See [documentation](https://docs.microsoft.com/azure/active-directory/managed-service-identity/overview) for more information.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
