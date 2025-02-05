#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScaleSetIdentity {
    /// Specifies a list of user managed identity ids to be assigned to the VMSS. Required if `type` is `UserAssigned`.
    /// 
    /// ```yaml
    /// resources:
    ///   example:
    ///     type: azure:compute:ScaleSet
    ///     properties:
    ///       name: vm-scaleset
    ///       resourceGroupName: ${exampleAzurermResourceGroup.name}
    ///       location: ${exampleAzurermResourceGroup.location}
    ///       sku:
    ///         name: ${vmSku}
    ///         tier: Standard
    ///         capacity: ${instanceCount}
    ///       identity:
    ///         type: SystemAssigned
    ///       extensions:
    ///         - name: MSILinuxExtension
    ///           publisher: Microsoft.ManagedIdentity
    ///           type: ManagedIdentityExtensionForLinux
    ///           typeHandlerVersion: '1.0'
    ///           settings: '{"port": 50342}'
    /// outputs:
    ///   principalId: ${example.identity.principalId}
    /// ```
    #[builder(into, default)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<Option<String>>,
    /// Specifies the identity type to be assigned to the scale set. Allowable values are `SystemAssigned` and `UserAssigned`. For the `SystemAssigned` identity the scale set's Service Principal ID (SPN) can be retrieved after the scale set has been created. See [documentation](https://docs.microsoft.com/azure/active-directory/managed-service-identity/overview) for more information. Possible values are `SystemAssigned`, `UserAssigned` and `SystemAssigned, UserAssigned`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
