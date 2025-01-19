/// Manages Synapse Workspace keys
///
/// > **Note:** Keys that are actively protecting a workspace cannot be deleted. When the keys resource is deleted, if the key is inactive it will be deleted, if it is active it will not be deleted.
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestorageacc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       accountKind: StorageV2
///       isHnsEnabled: 'true'
///   exampleDataLakeGen2Filesystem:
///     type: azure:storage:DataLakeGen2Filesystem
///     name: example
///     properties:
///       name: example
///       storageAccountId: ${exampleAccount.id}
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: standard
///       purgeProtectionEnabled: true
///   deployer:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${current.objectId}
///       keyPermissions:
///         - Create
///         - Get
///         - Delete
///         - Purge
///         - GetRotationPolicy
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: workspaceEncryptionKey
///       keyVaultId: ${exampleKeyVault.id}
///       keyType: RSA
///       keySize: 2048
///       keyOpts:
///         - unwrapKey
///         - wrapKey
///     options:
///       dependsOn:
///         - ${deployer}
///   exampleWorkspace:
///     type: azure:synapse:Workspace
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       storageDataLakeGen2FilesystemId: ${exampleDataLakeGen2Filesystem.id}
///       sqlAdministratorLogin: sqladminuser
///       sqlAdministratorLoginPassword: H@Sh1CoR3!
///       customerManagedKey:
///         keyVersionlessId: ${exampleKey.versionlessId}
///         keyName: enckey
///       identity:
///         type: SystemAssigned
///       tags:
///         Env: production
///   workspacePolicy:
///     type: azure:keyvault:AccessPolicy
///     name: workspace_policy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${exampleWorkspace.identity.tenantId}
///       objectId: ${exampleWorkspace.identity.principalId}
///       keyPermissions:
///         - Get
///         - WrapKey
///         - UnwrapKey
///   exampleWorkspaceKey:
///     type: azure:synapse:WorkspaceKey
///     name: example
///     properties:
///       customerManagedKeyVersionlessId: ${exampleKey.versionlessId}
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       active: true
///       customerManagedKeyName: enckey
///     options:
///       dependsOn:
///         - ${workspacePolicy}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Synapse Workspace Keys can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/workspaceKey:WorkspaceKey example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Synapse/workspaces/workspace1/keys/key1
/// ```
///
pub mod workspace_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceKeyArgs {
        /// Specifies if the workspace should be encrypted with this key.
        ///
        /// > **Note:** Only one key can actively encrypt a workspace. When performing a key rotation, setting a new key as the active key will disable existing keys.
        #[builder(into)]
        pub active: pulumi_wasm_rust::Output<bool>,
        /// Specifies the name of the workspace key. Should match the name of the key in the synapse workspace.
        #[builder(into)]
        pub customer_managed_key_name: pulumi_wasm_rust::Output<String>,
        /// The Azure Key Vault Key Versionless ID to be used as the Customer Managed Key (CMK) for double encryption
        #[builder(into, default)]
        pub customer_managed_key_versionless_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The ID of the Synapse Workspace where the encryption key should be configured.
        #[builder(into)]
        pub synapse_workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceKeyResult {
        /// Specifies if the workspace should be encrypted with this key.
        ///
        /// > **Note:** Only one key can actively encrypt a workspace. When performing a key rotation, setting a new key as the active key will disable existing keys.
        pub active: pulumi_wasm_rust::Output<bool>,
        /// Specifies the name of the workspace key. Should match the name of the key in the synapse workspace.
        pub customer_managed_key_name: pulumi_wasm_rust::Output<String>,
        /// The Azure Key Vault Key Versionless ID to be used as the Customer Managed Key (CMK) for double encryption
        pub customer_managed_key_versionless_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The ID of the Synapse Workspace where the encryption key should be configured.
        pub synapse_workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkspaceKeyArgs) -> WorkspaceKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let active_binding = args.active.get_inner();
        let customer_managed_key_name_binding = args
            .customer_managed_key_name
            .get_inner();
        let customer_managed_key_versionless_id_binding = args
            .customer_managed_key_versionless_id
            .get_inner();
        let synapse_workspace_id_binding = args.synapse_workspace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:synapse/workspaceKey:WorkspaceKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "active".into(),
                    value: &active_binding,
                },
                register_interface::ObjectField {
                    name: "customerManagedKeyName".into(),
                    value: &customer_managed_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "customerManagedKeyVersionlessId".into(),
                    value: &customer_managed_key_versionless_id_binding,
                },
                register_interface::ObjectField {
                    name: "synapseWorkspaceId".into(),
                    value: &synapse_workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "active".into(),
                },
                register_interface::ResultField {
                    name: "customerManagedKeyName".into(),
                },
                register_interface::ResultField {
                    name: "customerManagedKeyVersionlessId".into(),
                },
                register_interface::ResultField {
                    name: "synapseWorkspaceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkspaceKeyResult {
            active: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("active").unwrap(),
            ),
            customer_managed_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerManagedKeyName").unwrap(),
            ),
            customer_managed_key_versionless_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerManagedKeyVersionlessId").unwrap(),
            ),
            synapse_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("synapseWorkspaceId").unwrap(),
            ),
        }
    }
}
