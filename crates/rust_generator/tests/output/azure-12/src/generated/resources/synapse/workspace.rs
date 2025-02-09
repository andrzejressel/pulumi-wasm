/// Manages a Synapse Workspace.
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
/// ```
///
/// ### Creating A Workspace With Customer Managed Key And Azure AD Admin
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
///       name: workspaceencryptionkey
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
///   exampleWorkspaceAadAdmin:
///     type: azure:synapse:WorkspaceAadAdmin
///     name: example
///     properties:
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       login: AzureAD Admin
///       objectId: 00000000-0000-0000-0000-000000000000
///       tenantId: 00000000-0000-0000-0000-000000000000
///     options:
///       dependsOn:
///         - ${exampleWorkspaceKey}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Synapse Workspace can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/workspace:Workspace example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Synapse/workspaces/workspace1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceArgs {
        /// An `azure_devops_repo` block as defined below.
        #[builder(into, default)]
        pub azure_devops_repo: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::synapse::WorkspaceAzureDevopsRepo>,
        >,
        /// Is Azure Active Directory Authentication the only way to authenticate with resources inside this synapse Workspace. Defaults to `false`.
        #[builder(into, default)]
        pub azuread_authentication_only: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Subnet ID used for computes in workspace Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub compute_subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `customer_managed_key` block as defined below.
        #[builder(into, default)]
        pub customer_managed_key: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::synapse::WorkspaceCustomerManagedKey>,
        >,
        /// Is data exfiltration protection enabled in this workspace? If set to `true`, `managed_virtual_network_enabled` must also be set to `true`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub data_exfiltration_protection_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A `github_repo` block as defined below.
        #[builder(into, default)]
        pub github_repo: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::synapse::WorkspaceGithubRepo>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::synapse::WorkspaceIdentity>,
        >,
        /// Allowed AAD Tenant Ids For Linking.
        #[builder(into, default)]
        pub linking_allowed_for_aad_tenant_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies the Azure Region where the synapse Workspace should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Workspace managed resource group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub managed_resource_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Is Virtual Network enabled for all computes in this workspace? Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub managed_virtual_network_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the name which should be used for this synapse Workspace. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether public network access is allowed for the Cognitive Account. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The ID of purview account.
        #[builder(into, default)]
        pub purview_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group where the synapse Workspace should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies The login name of the SQL administrator. Changing this forces a new resource to be created. If this is not provided `customer_managed_key` must be provided.
        #[builder(into, default)]
        pub sql_administrator_login: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Password associated with the `sql_administrator_login` for the SQL administrator. If this is not provided `customer_managed_key` must be provided.
        #[builder(into, default)]
        pub sql_administrator_login_password: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Are pipelines (running as workspace's system assigned identity) allowed to access SQL pools?
        #[builder(into, default)]
        pub sql_identity_control_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the ID of storage data lake gen2 filesystem resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_data_lake_gen2_filesystem_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// A mapping of tags which should be assigned to the Synapse Workspace.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkspaceResult {
        /// An `azure_devops_repo` block as defined below.
        pub azure_devops_repo: pulumi_gestalt_rust::Output<
            Option<super::super::types::synapse::WorkspaceAzureDevopsRepo>,
        >,
        /// Is Azure Active Directory Authentication the only way to authenticate with resources inside this synapse Workspace. Defaults to `false`.
        pub azuread_authentication_only: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Subnet ID used for computes in workspace Changing this forces a new resource to be created.
        pub compute_subnet_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of Connectivity endpoints for this Synapse Workspace.
        pub connectivity_endpoints: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A `customer_managed_key` block as defined below.
        pub customer_managed_key: pulumi_gestalt_rust::Output<
            Option<super::super::types::synapse::WorkspaceCustomerManagedKey>,
        >,
        /// Is data exfiltration protection enabled in this workspace? If set to `true`, `managed_virtual_network_enabled` must also be set to `true`. Changing this forces a new resource to be created.
        pub data_exfiltration_protection_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// A `github_repo` block as defined below.
        pub github_repo: pulumi_gestalt_rust::Output<
            Option<super::super::types::synapse::WorkspaceGithubRepo>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::synapse::WorkspaceIdentity>,
        >,
        /// Allowed AAD Tenant Ids For Linking.
        pub linking_allowed_for_aad_tenant_ids: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// Specifies the Azure Region where the synapse Workspace should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Workspace managed resource group. Changing this forces a new resource to be created.
        pub managed_resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Is Virtual Network enabled for all computes in this workspace? Changing this forces a new resource to be created.
        pub managed_virtual_network_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name which should be used for this synapse Workspace. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether public network access is allowed for the Cognitive Account. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of purview account.
        pub purview_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Resource Group where the synapse Workspace should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies The login name of the SQL administrator. Changing this forces a new resource to be created. If this is not provided `customer_managed_key` must be provided.
        pub sql_administrator_login: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Password associated with the `sql_administrator_login` for the SQL administrator. If this is not provided `customer_managed_key` must be provided.
        pub sql_administrator_login_password: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Are pipelines (running as workspace's system assigned identity) allowed to access SQL pools?
        pub sql_identity_control_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the ID of storage data lake gen2 filesystem resource. Changing this forces a new resource to be created.
        pub storage_data_lake_gen2_filesystem_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Synapse Workspace.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WorkspaceArgs,
    ) -> WorkspaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let azure_devops_repo_binding_1 = args.azure_devops_repo.get_output(context);
        let azure_devops_repo_binding = azure_devops_repo_binding_1.get_inner();
        let azuread_authentication_only_binding_1 = args
            .azuread_authentication_only
            .get_output(context);
        let azuread_authentication_only_binding = azuread_authentication_only_binding_1
            .get_inner();
        let compute_subnet_id_binding_1 = args.compute_subnet_id.get_output(context);
        let compute_subnet_id_binding = compute_subnet_id_binding_1.get_inner();
        let customer_managed_key_binding_1 = args
            .customer_managed_key
            .get_output(context);
        let customer_managed_key_binding = customer_managed_key_binding_1.get_inner();
        let data_exfiltration_protection_enabled_binding_1 = args
            .data_exfiltration_protection_enabled
            .get_output(context);
        let data_exfiltration_protection_enabled_binding = data_exfiltration_protection_enabled_binding_1
            .get_inner();
        let github_repo_binding_1 = args.github_repo.get_output(context);
        let github_repo_binding = github_repo_binding_1.get_inner();
        let identity_binding_1 = args.identity.get_output(context);
        let identity_binding = identity_binding_1.get_inner();
        let linking_allowed_for_aad_tenant_ids_binding_1 = args
            .linking_allowed_for_aad_tenant_ids
            .get_output(context);
        let linking_allowed_for_aad_tenant_ids_binding = linking_allowed_for_aad_tenant_ids_binding_1
            .get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let managed_resource_group_name_binding_1 = args
            .managed_resource_group_name
            .get_output(context);
        let managed_resource_group_name_binding = managed_resource_group_name_binding_1
            .get_inner();
        let managed_virtual_network_enabled_binding_1 = args
            .managed_virtual_network_enabled
            .get_output(context);
        let managed_virtual_network_enabled_binding = managed_virtual_network_enabled_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let public_network_access_enabled_binding_1 = args
            .public_network_access_enabled
            .get_output(context);
        let public_network_access_enabled_binding = public_network_access_enabled_binding_1
            .get_inner();
        let purview_id_binding_1 = args.purview_id.get_output(context);
        let purview_id_binding = purview_id_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let sql_administrator_login_binding_1 = args
            .sql_administrator_login
            .get_output(context);
        let sql_administrator_login_binding = sql_administrator_login_binding_1
            .get_inner();
        let sql_administrator_login_password_binding_1 = args
            .sql_administrator_login_password
            .get_output(context);
        let sql_administrator_login_password_binding = sql_administrator_login_password_binding_1
            .get_inner();
        let sql_identity_control_enabled_binding_1 = args
            .sql_identity_control_enabled
            .get_output(context);
        let sql_identity_control_enabled_binding = sql_identity_control_enabled_binding_1
            .get_inner();
        let storage_data_lake_gen2_filesystem_id_binding_1 = args
            .storage_data_lake_gen2_filesystem_id
            .get_output(context);
        let storage_data_lake_gen2_filesystem_id_binding = storage_data_lake_gen2_filesystem_id_binding_1
            .get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:synapse/workspace:Workspace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "azureDevopsRepo".into(),
                    value: &azure_devops_repo_binding,
                },
                register_interface::ObjectField {
                    name: "azureadAuthenticationOnly".into(),
                    value: &azuread_authentication_only_binding,
                },
                register_interface::ObjectField {
                    name: "computeSubnetId".into(),
                    value: &compute_subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "customerManagedKey".into(),
                    value: &customer_managed_key_binding,
                },
                register_interface::ObjectField {
                    name: "dataExfiltrationProtectionEnabled".into(),
                    value: &data_exfiltration_protection_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "githubRepo".into(),
                    value: &github_repo_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "linkingAllowedForAadTenantIds".into(),
                    value: &linking_allowed_for_aad_tenant_ids_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managedResourceGroupName".into(),
                    value: &managed_resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "managedVirtualNetworkEnabled".into(),
                    value: &managed_virtual_network_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "purviewId".into(),
                    value: &purview_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sqlAdministratorLogin".into(),
                    value: &sql_administrator_login_binding,
                },
                register_interface::ObjectField {
                    name: "sqlAdministratorLoginPassword".into(),
                    value: &sql_administrator_login_password_binding,
                },
                register_interface::ObjectField {
                    name: "sqlIdentityControlEnabled".into(),
                    value: &sql_identity_control_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "storageDataLakeGen2FilesystemId".into(),
                    value: &storage_data_lake_gen2_filesystem_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkspaceResult {
            azure_devops_repo: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azureDevopsRepo"),
            ),
            azuread_authentication_only: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azureadAuthenticationOnly"),
            ),
            compute_subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("computeSubnetId"),
            ),
            connectivity_endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectivityEndpoints"),
            ),
            customer_managed_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customerManagedKey"),
            ),
            data_exfiltration_protection_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataExfiltrationProtectionEnabled"),
            ),
            github_repo: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("githubRepo"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            linking_allowed_for_aad_tenant_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("linkingAllowedForAadTenantIds"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            managed_resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedResourceGroupName"),
            ),
            managed_virtual_network_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedVirtualNetworkEnabled"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            public_network_access_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            purview_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("purviewId"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sql_administrator_login: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sqlAdministratorLogin"),
            ),
            sql_administrator_login_password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sqlAdministratorLoginPassword"),
            ),
            sql_identity_control_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sqlIdentityControlEnabled"),
            ),
            storage_data_lake_gen2_filesystem_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageDataLakeGen2FilesystemId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
