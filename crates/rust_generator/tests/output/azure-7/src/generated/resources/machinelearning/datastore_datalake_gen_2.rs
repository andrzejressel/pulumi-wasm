/// Manages a Machine Learning Data Lake Gen2 DataStore.
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
///   exampleDatastoreDatalakeGen2:
///     type: azure:machinelearning:DatastoreDatalakeGen2
///     name: example
///     properties:
///       name: example-datastore
///       workspaceId: ${exampleWorkspace.id}
///       storageContainerId: ${exampleContainer.resourceManagerId}
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
/// $ pulumi import azure:machinelearning/datastoreDatalakeGen2:DatastoreDatalakeGen2 example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.MachineLearningServices/workspaces/mlw1/dataStores/datastore1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod datastore_datalake_gen_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatastoreDatalakeGen2Args {
        /// An URL used for authentication.
        #[builder(into, default)]
        pub authority_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The object ID of the Service Principal.
        #[builder(into, default)]
        pub client_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The secret of the Service Principal.
        #[builder(into, default)]
        pub client_secret: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Text used to describe the asset. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Machine Learning DataStore. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies which identity to use when retrieving data from the specified source. Defaults to `None`. Possible values are `None`, `WorkspaceSystemAssignedIdentity` and `WorkspaceUserAssignedIdentity`.
        #[builder(into, default)]
        pub service_data_identity: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Storage Account Container. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into)]
        pub storage_container_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Machine Learning DataStore. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Tenant which the Service Principal belongs to.
        #[builder(into, default)]
        pub tenant_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Machine Learning Workspace. Changing this forces a new Machine Learning DataStore to be created.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DatastoreDatalakeGen2Result {
        /// An URL used for authentication.
        pub authority_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// The object ID of the Service Principal.
        pub client_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The secret of the Service Principal.
        pub client_secret: pulumi_gestalt_rust::Output<Option<String>>,
        /// Text used to describe the asset. Changing this forces a new Machine Learning DataStore to be created.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates whether this Machines Learning DataStore is the default for the Workspace.
        pub is_default: pulumi_gestalt_rust::Output<bool>,
        /// The name of the Machine Learning DataStore. Changing this forces a new Machine Learning DataStore to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies which identity to use when retrieving data from the specified source. Defaults to `None`. Possible values are `None`, `WorkspaceSystemAssignedIdentity` and `WorkspaceUserAssignedIdentity`.
        pub service_data_identity: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Storage Account Container. Changing this forces a new Machine Learning DataStore to be created.
        pub storage_container_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Machine Learning DataStore. Changing this forces a new Machine Learning DataStore to be created.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Tenant which the Service Principal belongs to.
        pub tenant_id: pulumi_gestalt_rust::Output<Option<String>>,
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
        args: DatastoreDatalakeGen2Args,
    ) -> DatastoreDatalakeGen2Result {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authority_url_binding = args.authority_url.get_output(context);
        let client_id_binding = args.client_id.get_output(context);
        let client_secret_binding = args.client_secret.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let service_data_identity_binding = args
            .service_data_identity
            .get_output(context);
        let storage_container_id_binding = args.storage_container_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tenant_id_binding = args.tenant_id.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:machinelearning/datastoreDatalakeGen2:DatastoreDatalakeGen2"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorityUrl".into(),
                    value: authority_url_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientId".into(),
                    value: client_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientSecret".into(),
                    value: client_secret_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceDataIdentity".into(),
                    value: service_data_identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageContainerId".into(),
                    value: storage_container_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenantId".into(),
                    value: tenant_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatastoreDatalakeGen2Result {
            authority_url: o.get_field("authorityUrl"),
            client_id: o.get_field("clientId"),
            client_secret: o.get_field("clientSecret"),
            description: o.get_field("description"),
            is_default: o.get_field("isDefault"),
            name: o.get_field("name"),
            service_data_identity: o.get_field("serviceDataIdentity"),
            storage_container_id: o.get_field("storageContainerId"),
            tags: o.get_field("tags"),
            tenant_id: o.get_field("tenantId"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
