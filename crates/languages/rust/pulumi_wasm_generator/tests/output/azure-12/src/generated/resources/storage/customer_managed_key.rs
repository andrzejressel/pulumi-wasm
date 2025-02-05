/// Manages a Customer Managed Key for a Storage Account.
///
/// > **NOTE:** It's possible to define a Customer Managed Key both within the `azure.storage.Account` resource via the `customer_managed_key` block and by using the `azure.storage.CustomerManagedKey` resource. However it's not possible to use both methods to manage a Customer Managed Key for a Storage Account, since there'll be conflicts.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: examplekv
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: standard
///       purgeProtectionEnabled: true
///   storage:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${exampleAccount.identity.principalId}
///       secretPermissions:
///         - Get
///       keyPermissions:
///         - Get
///         - UnwrapKey
///         - WrapKey
///   client:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${current.objectId}
///       secretPermissions:
///         - Get
///       keyPermissions:
///         - Get
///         - Create
///         - Delete
///         - List
///         - Restore
///         - Recover
///         - UnwrapKey
///         - WrapKey
///         - Purge
///         - Encrypt
///         - Decrypt
///         - Sign
///         - Verify
///         - GetRotationPolicy
///         - SetRotationPolicy
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: tfex-key
///       keyVaultId: ${exampleKeyVault.id}
///       keyType: RSA
///       keySize: 2048
///       keyOpts:
///         - decrypt
///         - encrypt
///         - sign
///         - unwrapKey
///         - verify
///         - wrapKey
///     options:
///       dependsOn:
///         - ${client}
///         - ${storage}
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestor
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: GRS
///       identity:
///         type: SystemAssigned
///   exampleCustomerManagedKey:
///     type: azure:storage:CustomerManagedKey
///     name: example
///     properties:
///       storageAccountId: ${exampleAccount.id}
///       keyVaultId: ${exampleKeyVault.id}
///       keyName: ${exampleKey.name}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Customer Managed Keys for a Storage Account can be imported using the `resource id` of the Storage Account, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/customerManagedKey:CustomerManagedKey example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.Storage/storageAccounts/myaccount
/// ```
///
pub mod customer_managed_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomerManagedKeyArgs {
        /// The Client ID of the multi-tenant application to be used in conjunction with the user-assigned identity for cross-tenant customer-managed-keys server-side encryption on the storage account.
        #[builder(into, default)]
        pub federated_identity_client_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of Key Vault Key.
        #[builder(into)]
        pub key_name: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub key_vault_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// URI pointing at the Key Vault. Required when using `federated_identity_client_id`. Exactly one of `managed_hsm_key_id`, `key_vault_id`, or `key_vault_uri` must be specified.
        #[builder(into, default)]
        pub key_vault_uri: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The version of Key Vault Key. Remove or omit this argument to enable Automatic Key Rotation.
        #[builder(into, default)]
        pub key_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key ID of a key in a managed HSM.  Exactly one of `managed_hsm_key_id`, `key_vault_id`, or `key_vault_uri` must be specified.
        #[builder(into, default)]
        pub managed_hsm_key_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Storage Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of a user assigned identity.
        #[builder(into, default)]
        pub user_assigned_identity_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CustomerManagedKeyResult {
        /// The Client ID of the multi-tenant application to be used in conjunction with the user-assigned identity for cross-tenant customer-managed-keys server-side encryption on the storage account.
        pub federated_identity_client_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of Key Vault Key.
        pub key_name: pulumi_wasm_rust::Output<String>,
        pub key_vault_id: pulumi_wasm_rust::Output<Option<String>>,
        /// URI pointing at the Key Vault. Required when using `federated_identity_client_id`. Exactly one of `managed_hsm_key_id`, `key_vault_id`, or `key_vault_uri` must be specified.
        pub key_vault_uri: pulumi_wasm_rust::Output<String>,
        /// The version of Key Vault Key. Remove or omit this argument to enable Automatic Key Rotation.
        pub key_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Key ID of a key in a managed HSM.  Exactly one of `managed_hsm_key_id`, `key_vault_id`, or `key_vault_uri` must be specified.
        pub managed_hsm_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Storage Account. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The ID of a user assigned identity.
        pub user_assigned_identity_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CustomerManagedKeyArgs,
    ) -> CustomerManagedKeyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let federated_identity_client_id_binding = args
            .federated_identity_client_id
            .get_output(context)
            .get_inner();
        let key_name_binding = args.key_name.get_output(context).get_inner();
        let key_vault_id_binding = args.key_vault_id.get_output(context).get_inner();
        let key_vault_uri_binding = args.key_vault_uri.get_output(context).get_inner();
        let key_version_binding = args.key_version.get_output(context).get_inner();
        let managed_hsm_key_id_binding = args
            .managed_hsm_key_id
            .get_output(context)
            .get_inner();
        let storage_account_id_binding = args
            .storage_account_id
            .get_output(context)
            .get_inner();
        let user_assigned_identity_id_binding = args
            .user_assigned_identity_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/customerManagedKey:CustomerManagedKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "federatedIdentityClientId".into(),
                    value: &federated_identity_client_id_binding,
                },
                register_interface::ObjectField {
                    name: "keyName".into(),
                    value: &key_name_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultUri".into(),
                    value: &key_vault_uri_binding,
                },
                register_interface::ObjectField {
                    name: "keyVersion".into(),
                    value: &key_version_binding,
                },
                register_interface::ObjectField {
                    name: "managedHsmKeyId".into(),
                    value: &managed_hsm_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "userAssignedIdentityId".into(),
                    value: &user_assigned_identity_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CustomerManagedKeyResult {
            federated_identity_client_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("federatedIdentityClientId"),
            ),
            key_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyName"),
            ),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyVaultId"),
            ),
            key_vault_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyVaultUri"),
            ),
            key_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyVersion"),
            ),
            managed_hsm_key_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managedHsmKeyId"),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
            user_assigned_identity_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userAssignedIdentityId"),
            ),
        }
    }
}
