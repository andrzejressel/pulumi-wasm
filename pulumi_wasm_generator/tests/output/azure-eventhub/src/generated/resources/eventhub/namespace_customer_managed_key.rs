/// Manages a Customer Managed Key for a EventHub Namespace.
///
/// !> **Note:** In 2.x versions of the Azure Provider during deletion this resource will **delete and recreate the parent EventHub Namespace which may involve data loss** as it's not possible to remove the Customer Managed Key from the EventHub Namespace once it's been added. Version 3.0 of the Azure Provider will change this so that the Delete operation is a noop, requiring the parent EventHub Namespace is deleted/recreated to remove the Customer Managed Key.
///
/// ## Example Usage
///
/// ### With System Assigned Identity
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleCluster:
///     type: azure:eventhub:Cluster
///     name: example
///     properties:
///       name: example-cluster
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       skuName: Dedicated_1
///   exampleEventHubNamespace:
///     type: azure:eventhub:EventHubNamespace
///     name: example
///     properties:
///       name: example-namespace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///       dedicatedClusterId: ${exampleCluster.id}
///       identity:
///         type: SystemAssigned
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
///   exampleAccessPolicy:
///     type: azure:keyvault:AccessPolicy
///     name: example
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${exampleEventHubNamespace.identity.tenantId}
///       objectId: ${exampleEventHubNamespace.identity.principalId}
///       keyPermissions:
///         - Get
///         - UnwrapKey
///         - WrapKey
///   example2:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${current.objectId}
///       keyPermissions:
///         - Create
///         - Delete
///         - Get
///         - List
///         - Purge
///         - Recover
///         - GetRotationPolicy
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: examplekvkey
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
///         - ${exampleAccessPolicy}
///         - ${example2}
///   exampleNamespaceCustomerManagedKey:
///     type: azure:eventhub:NamespaceCustomerManagedKey
///     name: example
///     properties:
///       eventhubNamespaceId: ${exampleEventHubNamespace.id}
///       keyVaultKeyIds:
///         - ${exampleKey.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
///
/// ### With User Assigned Identity
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleCluster:
///     type: azure:eventhub:Cluster
///     name: example
///     properties:
///       name: example-cluster
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       skuName: Dedicated_1
///   exampleUserAssignedIdentity:
///     type: azure:authorization:UserAssignedIdentity
///     name: example
///     properties:
///       location: ${example.location}
///       name: example
///       resourceGroupName: ${example.name}
///   exampleEventHubNamespace:
///     type: azure:eventhub:EventHubNamespace
///     name: example
///     properties:
///       name: example-namespace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///       dedicatedClusterId: ${exampleCluster.id}
///       identity:
///         type: UserAssigned
///         identityIds:
///           - ${exampleUserAssignedIdentity.id}
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
///   exampleAccessPolicy:
///     type: azure:keyvault:AccessPolicy
///     name: example
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${test.tenantId}
///       objectId: ${test.principalId}
///       keyPermissions:
///         - Get
///         - UnwrapKey
///         - WrapKey
///   example2:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${current.objectId}
///       keyPermissions:
///         - Create
///         - Delete
///         - Get
///         - List
///         - Purge
///         - Recover
///         - GetRotationPolicy
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: examplekvkey
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
///         - ${exampleAccessPolicy}
///         - ${example2}
///   exampleNamespaceCustomerManagedKey:
///     type: azure:eventhub:NamespaceCustomerManagedKey
///     name: example
///     properties:
///       eventhubNamespaceId: ${exampleEventHubNamespace.id}
///       keyVaultKeyIds:
///         - ${exampleKey.id}
///       userAssignedIdentityId: ${exampleUserAssignedIdentity.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Customer Managed Keys for a EventHub Namespace can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventhub/namespaceCustomerManagedKey:NamespaceCustomerManagedKey example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1
/// ```
///
pub mod namespace_customer_managed_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceCustomerManagedKeyArgs {
        /// The ID of the EventHub Namespace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub eventhub_namespace_id: pulumi_wasm_rust::Output<String>,
        /// Whether to enable Infrastructure Encryption (Double Encryption). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub infrastructure_encryption_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The list of keys of Key Vault.
        #[builder(into)]
        pub key_vault_key_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of a User Managed Identity that will be used to access Key Vaults that contain the encryption keys.
        ///
        /// > **Note:** If using `user_assigned_identity_id`, ensure the User Assigned Identity is also assigned to the parent Event Hub.
        ///
        /// > **Note:** If using `user_assigned_identity_id`, make sure to assign the identity the appropriate permissions to access the Key Vault key. Failure to grant `Get, UnwrapKey, and WrapKey` will cause this resource to fail to apply.
        #[builder(into, default)]
        pub user_assigned_identity_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NamespaceCustomerManagedKeyResult {
        /// The ID of the EventHub Namespace. Changing this forces a new resource to be created.
        pub eventhub_namespace_id: pulumi_wasm_rust::Output<String>,
        /// Whether to enable Infrastructure Encryption (Double Encryption). Changing this forces a new resource to be created.
        pub infrastructure_encryption_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The list of keys of Key Vault.
        pub key_vault_key_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of a User Managed Identity that will be used to access Key Vaults that contain the encryption keys.
        ///
        /// > **Note:** If using `user_assigned_identity_id`, ensure the User Assigned Identity is also assigned to the parent Event Hub.
        ///
        /// > **Note:** If using `user_assigned_identity_id`, make sure to assign the identity the appropriate permissions to access the Key Vault key. Failure to grant `Get, UnwrapKey, and WrapKey` will cause this resource to fail to apply.
        pub user_assigned_identity_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NamespaceCustomerManagedKeyArgs,
    ) -> NamespaceCustomerManagedKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let eventhub_namespace_id_binding = args.eventhub_namespace_id.get_inner();
        let infrastructure_encryption_enabled_binding = args
            .infrastructure_encryption_enabled
            .get_inner();
        let key_vault_key_ids_binding = args.key_vault_key_ids.get_inner();
        let user_assigned_identity_id_binding = args
            .user_assigned_identity_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventhub/namespaceCustomerManagedKey:NamespaceCustomerManagedKey"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "eventhubNamespaceId".into(),
                    value: &eventhub_namespace_id_binding,
                },
                register_interface::ObjectField {
                    name: "infrastructureEncryptionEnabled".into(),
                    value: &infrastructure_encryption_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultKeyIds".into(),
                    value: &key_vault_key_ids_binding,
                },
                register_interface::ObjectField {
                    name: "userAssignedIdentityId".into(),
                    value: &user_assigned_identity_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "eventhubNamespaceId".into(),
                },
                register_interface::ResultField {
                    name: "infrastructureEncryptionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultKeyIds".into(),
                },
                register_interface::ResultField {
                    name: "userAssignedIdentityId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NamespaceCustomerManagedKeyResult {
            eventhub_namespace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventhubNamespaceId").unwrap(),
            ),
            infrastructure_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("infrastructureEncryptionEnabled").unwrap(),
            ),
            key_vault_key_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultKeyIds").unwrap(),
            ),
            user_assigned_identity_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userAssignedIdentityId").unwrap(),
            ),
        }
    }
}
