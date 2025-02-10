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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceKeyArgs {
        /// Specifies if the workspace should be encrypted with this key.
        ///
        /// > **Note:** Only one key can actively encrypt a workspace. When performing a key rotation, setting a new key as the active key will disable existing keys.
        #[builder(into)]
        pub active: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Specifies the name of the workspace key. Should match the name of the key in the synapse workspace.
        #[builder(into)]
        pub customer_managed_key_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Key Vault Key Versionless ID to be used as the Customer Managed Key (CMK) for double encryption
        #[builder(into, default)]
        pub customer_managed_key_versionless_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the Synapse Workspace where the encryption key should be configured.
        #[builder(into)]
        pub synapse_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceKeyResult {
        /// Specifies if the workspace should be encrypted with this key.
        ///
        /// > **Note:** Only one key can actively encrypt a workspace. When performing a key rotation, setting a new key as the active key will disable existing keys.
        pub active: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the name of the workspace key. Should match the name of the key in the synapse workspace.
        pub customer_managed_key_name: pulumi_gestalt_rust::Output<String>,
        /// The Azure Key Vault Key Versionless ID to be used as the Customer Managed Key (CMK) for double encryption
        pub customer_managed_key_versionless_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The ID of the Synapse Workspace where the encryption key should be configured.
        pub synapse_workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceKeyArgs,
    ) -> WorkspaceKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let active_binding = args.active.get_output(context);
        let customer_managed_key_name_binding = args
            .customer_managed_key_name
            .get_output(context);
        let customer_managed_key_versionless_id_binding = args
            .customer_managed_key_versionless_id
            .get_output(context);
        let synapse_workspace_id_binding = args.synapse_workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:synapse/workspaceKey:WorkspaceKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "active".into(),
                    value: active_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerManagedKeyName".into(),
                    value: customer_managed_key_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerManagedKeyVersionlessId".into(),
                    value: customer_managed_key_versionless_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "synapseWorkspaceId".into(),
                    value: synapse_workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkspaceKeyResult {
            active: o.get_field("active"),
            customer_managed_key_name: o.get_field("customerManagedKeyName"),
            customer_managed_key_versionless_id: o
                .get_field("customerManagedKeyVersionlessId"),
            synapse_workspace_id: o.get_field("synapseWorkspaceId"),
        }
    }
}
