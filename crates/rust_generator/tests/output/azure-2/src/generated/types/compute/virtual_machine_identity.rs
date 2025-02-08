#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VirtualMachineIdentity {
    /// Specifies a list of User Assigned Managed Identity IDs to be assigned to this Virtual Machine.
    /// 
    /// > **NOTE:** This is required when `type` is set to `UserAssigned` or `SystemAssigned, UserAssigned`.
    #[builder(into, default)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Option<Vec<String>>>,
    /// The Principal ID associated with this Managed Service Identity.
    #[builder(into, default)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<Option<String>>,
    /// Specifies the type of Managed Service Identity that should be configured on this Virtual Machine. Possible values are `SystemAssigned`, `UserAssigned`, `SystemAssigned, UserAssigned` (to enable both).
    /// 
    /// > **NOTE:** Managed Service Identity previously required the installation of a VM Extension, but this information [is now available via the Azure Instance Metadata Service](https://docs.microsoft.com/azure/active-directory/managed-service-identity/overview#how-does-it-work).
    /// 
    /// > **NOTE:** When `type` is set to `SystemAssigned`, identity the Principal ID can be retrieved after the virtual machine has been created. More details are available below. See [documentation](https://docs.microsoft.com/azure/active-directory/managed-service-identity/overview) for additional information.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
