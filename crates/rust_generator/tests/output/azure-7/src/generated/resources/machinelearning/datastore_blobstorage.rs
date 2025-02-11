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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod datastore_blobstorage {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatastoreBlobstorageArgs {
        /// The access key of the Storage Account. Conflicts with `shared_access_signature`.
        #[builder(into, default)]
        pub account_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Text used to describe the asset. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether this Machines Learning DataStore is the default for the Workspace. Defaults to `false`.
        ///
        /// > **Note:** `is_default` can only be set to `true` on update.
        #[builder(into, default)]
        pub is_default: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Machine Learning DataStore. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies which identity to use when retrieving data from the specified source. Defaults to `None`. Possible values are `None`, `WorkspaceSystemAssignedIdentity` and `WorkspaceUserAssignedIdentity`.
        #[builder(into, default)]
        pub service_data_auth_identity: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Shared Access Signature of the Storage Account. Conflicts with `account_key`.
        ///
        /// > **Note:**  One of `account_key` or `shared_access_signature` must be specified.
        #[builder(into, default)]
        pub shared_access_signature: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Storage Account Container. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into)]
        pub storage_container_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Machine Learning DataStore. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Machine Learning Workspace. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DatastoreBlobstorageResult {
        /// The access key of the Storage Account. Conflicts with `shared_access_signature`.
        pub account_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Text used to describe the asset. Changing this forces a new Machine Learning DataStore to be created.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether this Machines Learning DataStore is the default for the Workspace. Defaults to `false`.
        ///
        /// > **Note:** `is_default` can only be set to `true` on update.
        pub is_default: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Machine Learning DataStore. Changing this forces a new Machine Learning DataStore to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies which identity to use when retrieving data from the specified source. Defaults to `None`. Possible values are `None`, `WorkspaceSystemAssignedIdentity` and `WorkspaceUserAssignedIdentity`.
        pub service_data_auth_identity: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Shared Access Signature of the Storage Account. Conflicts with `account_key`.
        ///
        /// > **Note:**  One of `account_key` or `shared_access_signature` must be specified.
        pub shared_access_signature: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Storage Account Container. Changing this forces a new Machine Learning DataStore to be created.
        pub storage_container_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Machine Learning DataStore. Changing this forces a new Machine Learning DataStore to be created.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Machine Learning Workspace. Changing this forces a new Machine Learning DataStore to be created.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DatastoreBlobstorageArgs,
    ) -> DatastoreBlobstorageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_key_binding = args.account_key.get_output(context);
        let description_binding = args.description.get_output(context);
        let is_default_binding = args.is_default.get_output(context);
        let name_binding = args.name.get_output(context);
        let service_data_auth_identity_binding = args
            .service_data_auth_identity
            .get_output(context);
        let shared_access_signature_binding = args
            .shared_access_signature
            .get_output(context);
        let storage_container_id_binding = args.storage_container_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:machinelearning/datastoreBlobstorage:DatastoreBlobstorage"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountKey".into(),
                    value: &account_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isDefault".into(),
                    value: &is_default_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceDataAuthIdentity".into(),
                    value: &service_data_auth_identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sharedAccessSignature".into(),
                    value: &shared_access_signature_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageContainerId".into(),
                    value: &storage_container_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatastoreBlobstorageResult {
            account_key: o.get_field("accountKey"),
            description: o.get_field("description"),
            is_default: o.get_field("isDefault"),
            name: o.get_field("name"),
            service_data_auth_identity: o.get_field("serviceDataAuthIdentity"),
            shared_access_signature: o.get_field("sharedAccessSignature"),
            storage_container_id: o.get_field("storageContainerId"),
            tags: o.get_field("tags"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
