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
pub mod workspace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceArgs {
        /// The ID of the Application Insights associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_insights_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the container registry associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `admin_enabled` should be `true` in order to associate the Container Registry to this Machine Learning Workspace.
        #[builder(into, default)]
        pub container_registry_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of this Machine Learning Workspace.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// An `encryption` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub encryption: pulumi_wasm_rust::Output<
            Option<super::super::types::machinelearning::WorkspaceEncryption>,
        >,
        /// A `feature_store` block as defined below.
        #[builder(into, default)]
        pub feature_store: pulumi_wasm_rust::Output<
            Option<super::super::types::machinelearning::WorkspaceFeatureStore>,
        >,
        /// Display name for this Machine Learning Workspace.
        #[builder(into, default)]
        pub friendly_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Flag to signal High Business Impact (HBI) data in the workspace and reduce diagnostic data collected by the service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub high_business_impact: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into)]
        pub identity: pulumi_wasm_rust::Output<
            super::super::types::machinelearning::WorkspaceIdentity,
        >,
        /// The compute name for image build of the Machine Learning Workspace.
        #[builder(into, default)]
        pub image_build_compute_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of key vault associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// The type of the Workspace. Possible values are `Default`, `FeatureStore`. Defaults to `Default`
        #[builder(into, default)]
        pub kind: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the Machine Learning Workspace should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// A `managed_network` block as defined below.
        #[builder(into, default)]
        pub managed_network: pulumi_wasm_rust::Output<
            Option<super::super::types::machinelearning::WorkspaceManagedNetwork>,
        >,
        /// Specifies the name of the Machine Learning Workspace. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The user assigned identity id that represents the workspace identity.
        #[builder(into, default)]
        pub primary_user_assigned_identity: pulumi_wasm_rust::Output<Option<String>>,
        /// Enable public access when this Machine Learning Workspace is behind VNet. Defaults to `true`.
        ///
        /// > **NOTE:** `public_access_behind_virtual_network_enabled` is deprecated and will be removed in favour of the property `public_network_access_enabled`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the Resource Group in which the Machine Learning Workspace should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `serverless_compute` block as defined below.
        #[builder(into, default)]
        pub serverless_compute: pulumi_wasm_rust::Output<
            Option<super::super::types::machinelearning::WorkspaceServerlessCompute>,
        >,
        /// SKU/edition of the Machine Learning Workspace, possible values are `Free`, `Basic`, `Standard` and `Premium`. Defaults to `Basic`.
        #[builder(into, default)]
        pub sku_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Storage Account associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `account_tier` cannot be `Premium` in order to associate the Storage Account to this Machine Learning Workspace.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Enable V1 API features, enabling `v1_legacy_mode` may prevent you from using features provided by the v2 API. Defaults to `false`.
        #[builder(into, default)]
        pub v1_legacy_mode_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceResult {
        /// The ID of the Application Insights associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        pub application_insights_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the container registry associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `admin_enabled` should be `true` in order to associate the Container Registry to this Machine Learning Workspace.
        pub container_registry_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of this Machine Learning Workspace.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The url for the discovery service to identify regional endpoints for machine learning experimentation services.
        pub discovery_url: pulumi_wasm_rust::Output<String>,
        /// An `encryption` block as defined below. Changing this forces a new resource to be created.
        pub encryption: pulumi_wasm_rust::Output<
            Option<super::super::types::machinelearning::WorkspaceEncryption>,
        >,
        /// A `feature_store` block as defined below.
        pub feature_store: pulumi_wasm_rust::Output<
            Option<super::super::types::machinelearning::WorkspaceFeatureStore>,
        >,
        /// Display name for this Machine Learning Workspace.
        pub friendly_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Flag to signal High Business Impact (HBI) data in the workspace and reduce diagnostic data collected by the service. Changing this forces a new resource to be created.
        pub high_business_impact: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            super::super::types::machinelearning::WorkspaceIdentity,
        >,
        /// The compute name for image build of the Machine Learning Workspace.
        pub image_build_compute_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of key vault associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// The type of the Workspace. Possible values are `Default`, `FeatureStore`. Defaults to `Default`
        pub kind: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the Machine Learning Workspace should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `managed_network` block as defined below.
        pub managed_network: pulumi_wasm_rust::Output<
            super::super::types::machinelearning::WorkspaceManagedNetwork,
        >,
        /// Specifies the name of the Machine Learning Workspace. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The user assigned identity id that represents the workspace identity.
        pub primary_user_assigned_identity: pulumi_wasm_rust::Output<Option<String>>,
        /// Enable public access when this Machine Learning Workspace is behind VNet. Defaults to `true`.
        ///
        /// > **NOTE:** `public_access_behind_virtual_network_enabled` is deprecated and will be removed in favour of the property `public_network_access_enabled`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the Resource Group in which the Machine Learning Workspace should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `serverless_compute` block as defined below.
        pub serverless_compute: pulumi_wasm_rust::Output<
            Option<super::super::types::machinelearning::WorkspaceServerlessCompute>,
        >,
        /// SKU/edition of the Machine Learning Workspace, possible values are `Free`, `Basic`, `Standard` and `Premium`. Defaults to `Basic`.
        pub sku_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Storage Account associated with this Machine Learning Workspace. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `account_tier` cannot be `Premium` in order to associate the Storage Account to this Machine Learning Workspace.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Enable V1 API features, enabling `v1_legacy_mode` may prevent you from using features provided by the v2 API. Defaults to `false`.
        pub v1_legacy_mode_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The immutable id associated with this workspace.
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkspaceArgs) -> WorkspaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_insights_id_binding = args.application_insights_id.get_inner();
        let container_registry_id_binding = args.container_registry_id.get_inner();
        let description_binding = args.description.get_inner();
        let encryption_binding = args.encryption.get_inner();
        let feature_store_binding = args.feature_store.get_inner();
        let friendly_name_binding = args.friendly_name.get_inner();
        let high_business_impact_binding = args.high_business_impact.get_inner();
        let identity_binding = args.identity.get_inner();
        let image_build_compute_name_binding = args.image_build_compute_name.get_inner();
        let key_vault_id_binding = args.key_vault_id.get_inner();
        let kind_binding = args.kind.get_inner();
        let location_binding = args.location.get_inner();
        let managed_network_binding = args.managed_network.get_inner();
        let name_binding = args.name.get_inner();
        let primary_user_assigned_identity_binding = args
            .primary_user_assigned_identity
            .get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let serverless_compute_binding = args.serverless_compute.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let v1_legacy_mode_enabled_binding = args.v1_legacy_mode_enabled.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:machinelearning/workspace:Workspace".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationInsightsId".into(),
                    value: &application_insights_id_binding,
                },
                register_interface::ObjectField {
                    name: "containerRegistryId".into(),
                    value: &container_registry_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "encryption".into(),
                    value: &encryption_binding,
                },
                register_interface::ObjectField {
                    name: "featureStore".into(),
                    value: &feature_store_binding,
                },
                register_interface::ObjectField {
                    name: "friendlyName".into(),
                    value: &friendly_name_binding,
                },
                register_interface::ObjectField {
                    name: "highBusinessImpact".into(),
                    value: &high_business_impact_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "imageBuildComputeName".into(),
                    value: &image_build_compute_name_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding,
                },
                register_interface::ObjectField {
                    name: "kind".into(),
                    value: &kind_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managedNetwork".into(),
                    value: &managed_network_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "primaryUserAssignedIdentity".into(),
                    value: &primary_user_assigned_identity_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serverlessCompute".into(),
                    value: &serverless_compute_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "v1LegacyModeEnabled".into(),
                    value: &v1_legacy_mode_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationInsightsId".into(),
                },
                register_interface::ResultField {
                    name: "containerRegistryId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "discoveryUrl".into(),
                },
                register_interface::ResultField {
                    name: "encryption".into(),
                },
                register_interface::ResultField {
                    name: "featureStore".into(),
                },
                register_interface::ResultField {
                    name: "friendlyName".into(),
                },
                register_interface::ResultField {
                    name: "highBusinessImpact".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "imageBuildComputeName".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultId".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managedNetwork".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "primaryUserAssignedIdentity".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serverlessCompute".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "v1LegacyModeEnabled".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkspaceResult {
            application_insights_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationInsightsId").unwrap(),
            ),
            container_registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerRegistryId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            discovery_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("discoveryUrl").unwrap(),
            ),
            encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryption").unwrap(),
            ),
            feature_store: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("featureStore").unwrap(),
            ),
            friendly_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("friendlyName").unwrap(),
            ),
            high_business_impact: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("highBusinessImpact").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            image_build_compute_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageBuildComputeName").unwrap(),
            ),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultId").unwrap(),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            managed_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedNetwork").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            primary_user_assigned_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryUserAssignedIdentity").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            serverless_compute: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverlessCompute").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            v1_legacy_mode_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("v1LegacyModeEnabled").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}