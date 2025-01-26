/// Manages a Machine Learning File Share DataStore.
///
/// ## Example Usage
///
/// ### With Azure File Share
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
///   exampleShare:
///     type: azure:storage:Share
///     name: example
///     properties:
///       name: example
///       storageAccountName: ${exampleAccount.name}
///       quota: 1
///   exampleDatastoreFileshare:
///     type: azure:machinelearning:DatastoreFileshare
///     name: example
///     properties:
///       name: example-datastore
///       workspaceId: ${exampleWorkspace.id}
///       storageFileshareId: ${exampleShare.resourceManagerId}
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
/// $ pulumi import azure:machinelearning/datastoreFileshare:DatastoreFileshare example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.MachineLearningServices/workspaces/mlw1/dataStores/datastore1
/// ```
///
pub mod datastore_fileshare {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatastoreFileshareArgs {
        /// The access key of the Storage Account. Conflicts with `shared_access_signature`.
        #[builder(into, default)]
        pub account_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Text used to describe the asset. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Machine Learning DataStore. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies which identity to use when retrieving data from the specified source. Defaults to `None`. Possible values are `None`, `WorkspaceSystemAssignedIdentity` and `WorkspaceUserAssignedIdentity`.
        #[builder(into, default)]
        pub service_data_identity: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Shared Access Signature of the Storage Account. Conflicts with `account_key`.
        #[builder(into, default)]
        pub shared_access_signature: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Storage Account File Share. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into)]
        pub storage_fileshare_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Machine Learning DataStore. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Machine Learning Workspace. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DatastoreFileshareResult {
        /// The access key of the Storage Account. Conflicts with `shared_access_signature`.
        pub account_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Text used to describe the asset. Changing this forces a new Machine Learning DataStore to be created.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicate whether this Machines Learning DataStore is the default for the Workspace.
        pub is_default: pulumi_wasm_rust::Output<bool>,
        /// The name of the Machine Learning DataStore. Changing this forces a new Machine Learning DataStore to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies which identity to use when retrieving data from the specified source. Defaults to `None`. Possible values are `None`, `WorkspaceSystemAssignedIdentity` and `WorkspaceUserAssignedIdentity`.
        pub service_data_identity: pulumi_wasm_rust::Output<Option<String>>,
        /// The Shared Access Signature of the Storage Account. Conflicts with `account_key`.
        pub shared_access_signature: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Storage Account File Share. Changing this forces a new Machine Learning DataStore to be created.
        pub storage_fileshare_id: pulumi_wasm_rust::Output<String>,
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
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DatastoreFileshareArgs,
    ) -> DatastoreFileshareResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_key_binding = args.account_key.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let service_data_identity_binding = args
            .service_data_identity
            .get_output(context)
            .get_inner();
        let shared_access_signature_binding = args
            .shared_access_signature
            .get_output(context)
            .get_inner();
        let storage_fileshare_id_binding = args
            .storage_fileshare_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let workspace_id_binding = args.workspace_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:machinelearning/datastoreFileshare:DatastoreFileshare".into(),
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceDataIdentity".into(),
                    value: &service_data_identity_binding,
                },
                register_interface::ObjectField {
                    name: "sharedAccessSignature".into(),
                    value: &shared_access_signature_binding,
                },
                register_interface::ObjectField {
                    name: "storageFileshareId".into(),
                    value: &storage_fileshare_id_binding,
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
                    name: "serviceDataIdentity".into(),
                },
                register_interface::ResultField {
                    name: "sharedAccessSignature".into(),
                },
                register_interface::ResultField {
                    name: "storageFileshareId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DatastoreFileshareResult {
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
            service_data_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceDataIdentity").unwrap(),
            ),
            shared_access_signature: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedAccessSignature").unwrap(),
            ),
            storage_fileshare_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageFileshareId").unwrap(),
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
