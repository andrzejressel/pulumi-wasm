/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleInsights:
///     type: azure:appinsights:Insights
///     name: example
///     properties:
///       name: workspace-example-ai
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationType: web
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: workspaceexamplekeyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: workspacestorageaccount
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       accountTier: Standard
///       accountReplicationType: GRS
///   exampleWorkspace:
///     type: azure:machinelearning:Workspace
///     name: example
///     properties:
///       name: example-workspace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationInsightsId: ${exampleInsights.id}
///       keyVaultId: ${exampleKeyVault.id}
///       storageAccountId: ${exampleAccount.id}
///       identity:
///         type: SystemAssigned
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
///
/// ### With Data Encryption
///
/// > **NOTE:** The Key Vault must enable purge protection.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleInsights:
///     type: azure:appinsights:Insights
///     name: example
///     properties:
///       name: workspace-example-ai
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationType: web
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: workspaceexamplekeyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///       purgeProtectionEnabled: true
///   exampleAccessPolicy:
///     type: azure:keyvault:AccessPolicy
///     name: example
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: workspacestorageaccount
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       accountTier: Standard
///       accountReplicationType: GRS
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: workspaceexamplekeyvaultkey
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
///         - ${exampleKeyVault}
///         - ${exampleAccessPolicy}
///   exampleWorkspace:
///     type: azure:machinelearning:Workspace
///     name: example
///     properties:
///       name: example-workspace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationInsightsId: ${exampleInsights.id}
///       keyVaultId: ${exampleKeyVault.id}
///       storageAccountId: ${exampleAccount.id}
///       identity:
///         type: SystemAssigned
///       encryption:
///         keyVaultId: ${exampleKeyVault.id}
///         keyId: ${exampleKey.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
///
/// ### With User Assigned Identity And Data Encryption
///
/// > **NOTE:** The Key Vault must enable purge protection.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleInsights:
///     type: azure:appinsights:Insights
///     name: example
///     properties:
///       name: example-ai
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationType: web
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestorageaccount
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       accountTier: Standard
///       accountReplicationType: GRS
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: example-keyvalut
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///       purgeProtectionEnabled: true
///   exampleUserAssignedIdentity:
///     type: azure:authorization:UserAssignedIdentity
///     name: example
///     properties:
///       name: example-identity
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   example-identity:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${exampleUserAssignedIdentity.principalId}
///       keyPermissions:
///         - WrapKey
///         - UnwrapKey
///         - Get
///         - Recover
///       secretPermissions:
///         - Get
///         - List
///         - Set
///         - Delete
///         - Recover
///         - Backup
///         - Restore
///   example-sp:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${current.objectId}
///       keyPermissions:
///         - Get
///         - Create
///         - Recover
///         - Delete
///         - Purge
///         - GetRotationPolicy
///   example-cosmosdb:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${test.objectId}
///       keyPermissions:
///         - Get
///         - Recover
///         - UnwrapKey
///         - WrapKey
///     options:
///       dependsOn:
///         - ${test}
///         - ${current}
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: example-keyvaultkey
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
///         - ${exampleKeyVault}
///         - ${["example-sp"]}
///   example-role1:
///     type: azure:authorization:Assignment
///     properties:
///       scope: ${exampleKeyVault.id}
///       roleDefinitionName: Contributor
///       principalId: ${exampleUserAssignedIdentity.principalId}
///   example-role2:
///     type: azure:authorization:Assignment
///     properties:
///       scope: ${exampleAccount.id}
///       roleDefinitionName: Storage Blob Data Contributor
///       principalId: ${exampleUserAssignedIdentity.principalId}
///   example-role3:
///     type: azure:authorization:Assignment
///     properties:
///       scope: ${exampleAccount.id}
///       roleDefinitionName: Contributor
///       principalId: ${exampleUserAssignedIdentity.principalId}
///   example-role4:
///     type: azure:authorization:Assignment
///     properties:
///       scope: ${exampleInsights.id}
///       roleDefinitionName: Contributor
///       principalId: ${exampleUserAssignedIdentity.principalId}
///   exampleWorkspace:
///     type: azure:machinelearning:Workspace
///     name: example
///     properties:
///       name: example-workspace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationInsightsId: ${exampleInsights.id}
///       keyVaultId: ${exampleKeyVault.id}
///       storageAccountId: ${exampleAccount.id}
///       highBusinessImpact: true
///       primaryUserAssignedIdentity: ${exampleUserAssignedIdentity.id}
///       identity:
///         type: UserAssigned
///         identityIds:
///           - ${exampleUserAssignedIdentity.id}
///       encryption:
///         userAssignedIdentityId: ${exampleUserAssignedIdentity.id}
///         keyVaultId: ${exampleKeyVault.id}
///         keyId: ${exampleKey.id}
///     options:
///       dependsOn:
///         - ${["example-role1"]}
///         - ${["example-role2"]}
///         - ${["example-role3"]}
///         - ${["example-role4"]}
///         - ${["example-cosmosdb"]}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   test:
///     fn::invoke:
///       function: azuread:getServicePrincipal
///       arguments:
///         displayName: Azure Cosmos DB
/// ```
///
/// ## Import
///
/// Machine Learning Workspace can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:machinelearning/workspace:Workspace example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.MachineLearningServices/workspaces/workspace1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceArgs {
        /// The ID of the Application Insights associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_insights_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the container registry associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `admin_enabled` should be `true` in order to associate the Container Registry to this Machine Learning Workspace.
        #[builder(into, default)]
        pub container_registry_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description of this Machine Learning Workspace.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `encryption` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub encryption: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::machinelearning::WorkspaceEncryption>,
        >,
        /// A `feature_store` block as defined below.
        #[builder(into, default)]
        pub feature_store: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::machinelearning::WorkspaceFeatureStore>,
        >,
        /// Display name for this Machine Learning Workspace.
        #[builder(into, default)]
        pub friendly_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Flag to signal High Business Impact (HBI) data in the workspace and reduce diagnostic data collected by the service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub high_business_impact: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::machinelearning::WorkspaceIdentity,
        >,
        /// The compute name for image build of the Machine Learning Workspace.
        #[builder(into, default)]
        pub image_build_compute_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of key vault associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub key_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of the Workspace. Possible values are `Default`, `FeatureStore`. Defaults to `Default`
        #[builder(into, default)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the supported Azure location where the Machine Learning Workspace should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `managed_network` block as defined below.
        #[builder(into, default)]
        pub managed_network: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::machinelearning::WorkspaceManagedNetwork>,
        >,
        /// Specifies the name of the Machine Learning Workspace. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user assigned identity id that represents the workspace identity.
        #[builder(into, default)]
        pub primary_user_assigned_identity: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Enable public access when this Machine Learning Workspace is behind VNet. Defaults to `true`.
        ///
        /// > **NOTE:** `public_access_behind_virtual_network_enabled` is deprecated and will be removed in favour of the property `public_network_access_enabled`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the name of the Resource Group in which the Machine Learning Workspace should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `serverless_compute` block as defined below.
        #[builder(into, default)]
        pub serverless_compute: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::machinelearning::WorkspaceServerlessCompute>,
        >,
        /// SKU/edition of the Machine Learning Workspace, possible values are `Free`, `Basic`, `Standard` and `Premium`. Defaults to `Basic`.
        #[builder(into, default)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Storage Account associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `account_tier` cannot be `Premium` in order to associate the Storage Account to this Machine Learning Workspace.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Enable V1 API features, enabling `v1_legacy_mode` may prevent you from using features provided by the v2 API. Defaults to `false`.
        #[builder(into, default)]
        pub v1_legacy_mode_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceResult {
        /// The ID of the Application Insights associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        pub application_insights_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the container registry associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `admin_enabled` should be `true` in order to associate the Container Registry to this Machine Learning Workspace.
        pub container_registry_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description of this Machine Learning Workspace.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The url for the discovery service to identify regional endpoints for machine learning experimentation services.
        pub discovery_url: pulumi_gestalt_rust::Output<String>,
        /// An `encryption` block as defined below. Changing this forces a new resource to be created.
        pub encryption: pulumi_gestalt_rust::Output<
            Option<super::super::types::machinelearning::WorkspaceEncryption>,
        >,
        /// A `feature_store` block as defined below.
        pub feature_store: pulumi_gestalt_rust::Output<
            Option<super::super::types::machinelearning::WorkspaceFeatureStore>,
        >,
        /// Display name for this Machine Learning Workspace.
        pub friendly_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Flag to signal High Business Impact (HBI) data in the workspace and reduce diagnostic data collected by the service. Changing this forces a new resource to be created.
        pub high_business_impact: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            super::super::types::machinelearning::WorkspaceIdentity,
        >,
        /// The compute name for image build of the Machine Learning Workspace.
        pub image_build_compute_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of key vault associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        pub key_vault_id: pulumi_gestalt_rust::Output<String>,
        /// The type of the Workspace. Possible values are `Default`, `FeatureStore`. Defaults to `Default`
        pub kind: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the Machine Learning Workspace should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `managed_network` block as defined below.
        pub managed_network: pulumi_gestalt_rust::Output<
            super::super::types::machinelearning::WorkspaceManagedNetwork,
        >,
        /// Specifies the name of the Machine Learning Workspace. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The user assigned identity id that represents the workspace identity.
        pub primary_user_assigned_identity: pulumi_gestalt_rust::Output<Option<String>>,
        /// Enable public access when this Machine Learning Workspace is behind VNet. Defaults to `true`.
        ///
        /// > **NOTE:** `public_access_behind_virtual_network_enabled` is deprecated and will be removed in favour of the property `public_network_access_enabled`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the Resource Group in which the Machine Learning Workspace should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `serverless_compute` block as defined below.
        pub serverless_compute: pulumi_gestalt_rust::Output<
            Option<super::super::types::machinelearning::WorkspaceServerlessCompute>,
        >,
        /// SKU/edition of the Machine Learning Workspace, possible values are `Free`, `Basic`, `Standard` and `Premium`. Defaults to `Basic`.
        pub sku_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Storage Account associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `account_tier` cannot be `Premium` in order to associate the Storage Account to this Machine Learning Workspace.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Enable V1 API features, enabling `v1_legacy_mode` may prevent you from using features provided by the v2 API. Defaults to `false`.
        pub v1_legacy_mode_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The immutable id associated with this workspace.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceArgs,
    ) -> WorkspaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_insights_id_binding = args
            .application_insights_id
            .get_output(context);
        let container_registry_id_binding = args
            .container_registry_id
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let encryption_binding = args.encryption.get_output(context);
        let feature_store_binding = args.feature_store.get_output(context);
        let friendly_name_binding = args.friendly_name.get_output(context);
        let high_business_impact_binding = args.high_business_impact.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let image_build_compute_name_binding = args
            .image_build_compute_name
            .get_output(context);
        let key_vault_id_binding = args.key_vault_id.get_output(context);
        let kind_binding = args.kind.get_output(context);
        let location_binding = args.location.get_output(context);
        let managed_network_binding = args.managed_network.get_output(context);
        let name_binding = args.name.get_output(context);
        let primary_user_assigned_identity_binding = args
            .primary_user_assigned_identity
            .get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let serverless_compute_binding = args.serverless_compute.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let v1_legacy_mode_enabled_binding = args
            .v1_legacy_mode_enabled
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:machinelearning/workspace:Workspace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationInsightsId".into(),
                    value: &application_insights_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerRegistryId".into(),
                    value: &container_registry_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryption".into(),
                    value: &encryption_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "featureStore".into(),
                    value: &feature_store_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "friendlyName".into(),
                    value: &friendly_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "highBusinessImpact".into(),
                    value: &high_business_impact_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageBuildComputeName".into(),
                    value: &image_build_compute_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kind".into(),
                    value: &kind_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedNetwork".into(),
                    value: &managed_network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "primaryUserAssignedIdentity".into(),
                    value: &primary_user_assigned_identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverlessCompute".into(),
                    value: &serverless_compute_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "v1LegacyModeEnabled".into(),
                    value: &v1_legacy_mode_enabled_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkspaceResult {
            application_insights_id: o.get_field("applicationInsightsId"),
            container_registry_id: o.get_field("containerRegistryId"),
            description: o.get_field("description"),
            discovery_url: o.get_field("discoveryUrl"),
            encryption: o.get_field("encryption"),
            feature_store: o.get_field("featureStore"),
            friendly_name: o.get_field("friendlyName"),
            high_business_impact: o.get_field("highBusinessImpact"),
            identity: o.get_field("identity"),
            image_build_compute_name: o.get_field("imageBuildComputeName"),
            key_vault_id: o.get_field("keyVaultId"),
            kind: o.get_field("kind"),
            location: o.get_field("location"),
            managed_network: o.get_field("managedNetwork"),
            name: o.get_field("name"),
            primary_user_assigned_identity: o.get_field("primaryUserAssignedIdentity"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            serverless_compute: o.get_field("serverlessCompute"),
            sku_name: o.get_field("skuName"),
            storage_account_id: o.get_field("storageAccountId"),
            tags: o.get_field("tags"),
            v1_legacy_mode_enabled: o.get_field("v1LegacyModeEnabled"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
