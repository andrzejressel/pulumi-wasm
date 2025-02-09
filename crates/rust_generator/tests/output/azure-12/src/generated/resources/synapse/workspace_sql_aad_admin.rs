/// Manages an Azure Active Directory SQL Administrator setting for a Synapse Workspace
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
///       name: workspace-encryption-key
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
///       identity:
///         type: SystemAssigned
///       tags:
///         Env: production
///   exampleWorkspaceSqlAadAdmin:
///     type: azure:synapse:WorkspaceSqlAadAdmin
///     name: example
///     properties:
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       login: AzureAD Admin
///       objectId: ${current.objectId}
///       tenantId: ${current.tenantId}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Synapse Workspace Azure AD Administrator can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/workspaceSqlAadAdmin:WorkspaceSqlAadAdmin example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Synapse/workspaces/workspace1/sqlAdministrators/activeDirectory
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace_sql_aad_admin {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceSqlAadAdminArgs {
        /// The login name of the Azure AD Administrator of this Synapse Workspace.
        #[builder(into)]
        pub login: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The object id of the Azure AD Administrator of this Synapse Workspace.
        #[builder(into)]
        pub object_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Synapse Workspace where the Azure AD Administrator should be configured.
        #[builder(into)]
        pub synapse_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The tenant id of the Azure AD Administrator of this Synapse Workspace.
        #[builder(into)]
        pub tenant_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceSqlAadAdminResult {
        /// The login name of the Azure AD Administrator of this Synapse Workspace.
        pub login: pulumi_gestalt_rust::Output<String>,
        /// The object id of the Azure AD Administrator of this Synapse Workspace.
        pub object_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Synapse Workspace where the Azure AD Administrator should be configured.
        pub synapse_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The tenant id of the Azure AD Administrator of this Synapse Workspace.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceSqlAadAdminArgs,
    ) -> WorkspaceSqlAadAdminResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let login_binding = args.login.get_output(context);
        let object_id_binding = args.object_id.get_output(context);
        let synapse_workspace_id_binding = args.synapse_workspace_id.get_output(context);
        let tenant_id_binding = args.tenant_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:synapse/workspaceSqlAadAdmin:WorkspaceSqlAadAdmin".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "login".into(),
                    value: login_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "objectId".into(),
                    value: object_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "synapseWorkspaceId".into(),
                    value: synapse_workspace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenantId".into(),
                    value: tenant_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkspaceSqlAadAdminResult {
            login: o.get_field("login"),
            object_id: o.get_field("objectId"),
            synapse_workspace_id: o.get_field("synapseWorkspaceId"),
            tenant_id: o.get_field("tenantId"),
        }
    }
}
