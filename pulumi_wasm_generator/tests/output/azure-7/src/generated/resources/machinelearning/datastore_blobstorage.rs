/// Manages a Machine Learning Blob Storage DataStore.
///
/// ## Example Usage
///
/// ### With Azure Blob
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
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: example-container
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: private
///   exampleDatastoreBlobstorage:
///     type: azure:machinelearning:DatastoreBlobstorage
///     name: example
///     properties:
///       name: example-datastore
///       workspaceId: ${exampleWorkspace.id}
///       storageContainerId: ${exampleContainer.resourceManagerId}
///       accountKey: ${exampleAccount.primaryAccessKey}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Machine Learning DataStores can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:machinelearning/datastoreBlobstorage:DatastoreBlobstorage example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.MachineLearningServices/workspaces/mlw1/dataStores/datastore1
/// ```
///
pub mod datastore_blobstorage {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatastoreBlobstorageArgs {
        /// The access key of the Storage Account. Conflicts with `shared_access_signature`.
        #[builder(into, default)]
        pub account_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Text used to describe the asset. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether this Machines Learning DataStore is the default for the Workspace. Defaults to `false`.
        ///
        /// > **Note:** `is_default` can only be set to `true` on update.
        #[builder(into, default)]
        pub is_default: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Machine Learning DataStore. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies which identity to use when retrieving data from the specified source. Defaults to `None`. Possible values are `None`, `WorkspaceSystemAssignedIdentity` and `WorkspaceUserAssignedIdentity`.
        #[builder(into, default)]
        pub service_data_auth_identity: pulumi_wasm_rust::Output<Option<String>>,
        /// The Shared Access Signature of the Storage Account. Conflicts with `account_key`.
        ///
        /// > **Note:**  One of `account_key` or `shared_access_signature` must be specified.
        #[builder(into, default)]
        pub shared_access_signature: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Storage Account Container. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into)]
        pub storage_container_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Machine Learning DataStore. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Machine Learning Workspace. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DatastoreBlobstorageResult {
        /// The access key of the Storage Account. Conflicts with `shared_access_signature`.
        pub account_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Text used to describe the asset. Changing this forces a new Machine Learning DataStore to be created.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether this Machines Learning DataStore is the default for the Workspace. Defaults to `false`.
        ///
        /// > **Note:** `is_default` can only be set to `true` on update.
        pub is_default: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Machine Learning DataStore. Changing this forces a new Machine Learning DataStore to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies which identity to use when retrieving data from the specified source. Defaults to `None`. Possible values are `None`, `WorkspaceSystemAssignedIdentity` and `WorkspaceUserAssignedIdentity`.
        pub service_data_auth_identity: pulumi_wasm_rust::Output<Option<String>>,
        /// The Shared Access Signature of the Storage Account. Conflicts with `account_key`.
        ///
        /// > **Note:**  One of `account_key` or `shared_access_signature` must be specified.
        pub shared_access_signature: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Storage Account Container. Changing this forces a new Machine Learning DataStore to be created.
        pub storage_container_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Machine Learning DataStore. Changing this forces a new Machine Learning DataStore to be created.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Machine Learning Workspace. Changing this forces a new Machine Learning DataStore to be created.
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DatastoreBlobstorageArgs,
    ) -> DatastoreBlobstorageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_key_binding = args.account_key.get_inner();
        let description_binding = args.description.get_inner();
        let is_default_binding = args.is_default.get_inner();
        let name_binding = args.name.get_inner();
        let service_data_auth_identity_binding = args
            .service_data_auth_identity
            .get_inner();
        let shared_access_signature_binding = args.shared_access_signature.get_inner();
        let storage_container_id_binding = args.storage_container_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:machinelearning/datastoreBlobstorage:DatastoreBlobstorage"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountKey".into(),
                    value: &account_key_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "isDefault".into(),
                    value: &is_default_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceDataAuthIdentity".into(),
                    value: &service_data_auth_identity_binding,
                },
                register_interface::ObjectField {
                    name: "sharedAccessSignature".into(),
                    value: &shared_access_signature_binding,
                },
                register_interface::ObjectField {
                    name: "storageContainerId".into(),
                    value: &storage_container_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountKey".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "isDefault".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "serviceDataAuthIdentity".into(),
                },
                register_interface::ResultField {
                    name: "sharedAccessSignature".into(),
                },
                register_interface::ResultField {
                    name: "storageContainerId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
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
        DatastoreBlobstorageResult {
            account_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountKey").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            is_default: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isDefault").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            service_data_auth_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceDataAuthIdentity").unwrap(),
            ),
            shared_access_signature: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedAccessSignature").unwrap(),
            ),
            storage_container_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageContainerId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}
