/// Manages a Key Vault Access Policy.
///
/// > **NOTE:** It's possible to define Key Vault Access Policies both within the `azure.keyvault.KeyVault` resource via the `access_policy` block and by using the `azure.keyvault.AccessPolicy` resource. However it's not possible to use both methods to manage Access Policies within a KeyVault, since there'll be conflicts.
///
/// > **NOTE:** Azure permits a maximum of 1024 Access Policies per Key Vault - [more information can be found in this document](https://docs.microsoft.com/azure/key-vault/key-vault-secure-your-key-vault#data-plane-access-control).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: examplekeyvault
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///   exampleAccessPolicy:
///     type: azure:keyvault:AccessPolicy
///     name: example
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${current.objectId}
///       keyPermissions:
///         - Get
///       secretPermissions:
///         - Get
///   example-principal:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${example.objectId}
///       keyPermissions:
///         - Get
///         - List
///         - Encrypt
///         - Decrypt
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   example:
///     fn::invoke:
///       function: azuread:getServicePrincipal
///       arguments:
///         displayName: example-app
/// ```
///
/// ## Import
///
/// Key Vault Access Policies can be imported using the Resource ID of the Key Vault, plus some additional metadata.
///
/// If both an `object_id` and `application_id` are specified, then the Access Policy can be imported using the following code:
///
/// ```sh
/// $ pulumi import azure:keyvault/accessPolicy:AccessPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.KeyVault/vaults/test-vault/objectId/11111111-1111-1111-1111-111111111111/applicationId/22222222-2222-2222-2222-222222222222
/// ```
///
/// where `11111111-1111-1111-1111-111111111111` is the `object_id` and `22222222-2222-2222-2222-222222222222` is the `application_id`.
///
/// ---
///
/// Access Policies with an `object_id` but no `application_id` can be imported using the following command:
///
/// ```sh
/// $ pulumi import azure:keyvault/accessPolicy:AccessPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.KeyVault/vaults/test-vault/objectId/11111111-1111-1111-1111-111111111111
/// ```
///
/// where `11111111-1111-1111-1111-111111111111` is the `object_id`.
///
pub mod access_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessPolicyArgs {
        /// The object ID of an Application in Azure Active Directory. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub application_id: pulumi_wasm_rust::Output<Option<String>>,
        /// List of certificate permissions, must be one or more from the following: `Backup`, `Create`, `Delete`, `DeleteIssuers`, `Get`, `GetIssuers`, `Import`, `List`, `ListIssuers`, `ManageContacts`, `ManageIssuers`, `Purge`, `Recover`, `Restore`, `SetIssuers` and `Update`.
        #[builder(into, default)]
        pub certificate_permissions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of key permissions, must be one or more from the following: `Backup`, `Create`, `Decrypt`, `Delete`, `Encrypt`, `Get`, `Import`, `List`, `Purge`, `Recover`, `Restore`, `Sign`, `UnwrapKey`, `Update`, `Verify`, `WrapKey`, `Release`, `Rotate`, `GetRotationPolicy` and `SetRotationPolicy`.
        #[builder(into, default)]
        pub key_permissions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the id of the Key Vault resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// The object ID of a user, service principal or security group in the Azure Active Directory tenant for the vault. The object ID of a service principal can be fetched from `azuread_service_principal.object_id`. The object ID must be unique for the list of access policies. Changing this forces a new resource to be created.
        #[builder(into)]
        pub object_id: pulumi_wasm_rust::Output<String>,
        /// List of secret permissions, must be one or more from the following: `Backup`, `Delete`, `Get`, `List`, `Purge`, `Recover`, `Restore` and `Set`.
        #[builder(into, default)]
        pub secret_permissions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of storage permissions, must be one or more from the following: `Backup`, `Delete`, `DeleteSAS`, `Get`, `GetSAS`, `List`, `ListSAS`, `Purge`, `Recover`, `RegenerateKey`, `Restore`, `Set`, `SetSAS` and `Update`.
        #[builder(into, default)]
        pub storage_permissions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Azure Active Directory tenant ID that should be used for authenticating requests to the key vault. Changing this forces a new resource to be created.
        #[builder(into)]
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AccessPolicyResult {
        /// The object ID of an Application in Azure Active Directory. Changing this forces a new resource to be created.
        pub application_id: pulumi_wasm_rust::Output<Option<String>>,
        /// List of certificate permissions, must be one or more from the following: `Backup`, `Create`, `Delete`, `DeleteIssuers`, `Get`, `GetIssuers`, `Import`, `List`, `ListIssuers`, `ManageContacts`, `ManageIssuers`, `Purge`, `Recover`, `Restore`, `SetIssuers` and `Update`.
        pub certificate_permissions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of key permissions, must be one or more from the following: `Backup`, `Create`, `Decrypt`, `Delete`, `Encrypt`, `Get`, `Import`, `List`, `Purge`, `Recover`, `Restore`, `Sign`, `UnwrapKey`, `Update`, `Verify`, `WrapKey`, `Release`, `Rotate`, `GetRotationPolicy` and `SetRotationPolicy`.
        pub key_permissions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the id of the Key Vault resource. Changing this forces a new resource to be created.
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// The object ID of a user, service principal or security group in the Azure Active Directory tenant for the vault. The object ID of a service principal can be fetched from `azuread_service_principal.object_id`. The object ID must be unique for the list of access policies. Changing this forces a new resource to be created.
        pub object_id: pulumi_wasm_rust::Output<String>,
        /// List of secret permissions, must be one or more from the following: `Backup`, `Delete`, `Get`, `List`, `Purge`, `Recover`, `Restore` and `Set`.
        pub secret_permissions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of storage permissions, must be one or more from the following: `Backup`, `Delete`, `DeleteSAS`, `Get`, `GetSAS`, `List`, `ListSAS`, `Purge`, `Recover`, `RegenerateKey`, `Restore`, `Set`, `SetSAS` and `Update`.
        pub storage_permissions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Azure Active Directory tenant ID that should be used for authenticating requests to the key vault. Changing this forces a new resource to be created.
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccessPolicyArgs) -> AccessPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_inner();
        let certificate_permissions_binding = args.certificate_permissions.get_inner();
        let key_permissions_binding = args.key_permissions.get_inner();
        let key_vault_id_binding = args.key_vault_id.get_inner();
        let object_id_binding = args.object_id.get_inner();
        let secret_permissions_binding = args.secret_permissions.get_inner();
        let storage_permissions_binding = args.storage_permissions.get_inner();
        let tenant_id_binding = args.tenant_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:keyvault/accessPolicy:AccessPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "certificatePermissions".into(),
                    value: &certificate_permissions_binding,
                },
                register_interface::ObjectField {
                    name: "keyPermissions".into(),
                    value: &key_permissions_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding,
                },
                register_interface::ObjectField {
                    name: "objectId".into(),
                    value: &object_id_binding,
                },
                register_interface::ObjectField {
                    name: "secretPermissions".into(),
                    value: &secret_permissions_binding,
                },
                register_interface::ObjectField {
                    name: "storagePermissions".into(),
                    value: &storage_permissions_binding,
                },
                register_interface::ObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "certificatePermissions".into(),
                },
                register_interface::ResultField {
                    name: "keyPermissions".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultId".into(),
                },
                register_interface::ResultField {
                    name: "objectId".into(),
                },
                register_interface::ResultField {
                    name: "secretPermissions".into(),
                },
                register_interface::ResultField {
                    name: "storagePermissions".into(),
                },
                register_interface::ResultField {
                    name: "tenantId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessPolicyResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            certificate_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificatePermissions").unwrap(),
            ),
            key_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyPermissions").unwrap(),
            ),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultId").unwrap(),
            ),
            object_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("objectId").unwrap(),
            ),
            secret_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretPermissions").unwrap(),
            ),
            storage_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storagePermissions").unwrap(),
            ),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantId").unwrap(),
            ),
        }
    }
}
