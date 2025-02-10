/// Manages a Linked Service (connection) between a Kusto Cluster and Azure Data Factory.
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
///   exampleFactory:
///     type: azure:datafactory:Factory
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       identity:
///         type: SystemAssigned
///   exampleCluster:
///     type: azure:kusto:Cluster
///     name: example
///     properties:
///       name: kustocluster
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         name: Standard_D13_v2
///         capacity: 2
///   exampleDatabase:
///     type: azure:kusto:Database
///     name: example
///     properties:
///       name: my-kusto-database
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       clusterName: ${exampleCluster.name}
///   exampleLinkedServiceKusto:
///     type: azure:datafactory:LinkedServiceKusto
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       kustoEndpoint: ${exampleCluster.uri}
///       kustoDatabaseName: ${exampleDatabase.name}
///       useManagedIdentity: true
///   exampleDatabasePrincipalAssignment:
///     type: azure:kusto:DatabasePrincipalAssignment
///     name: example
///     properties:
///       name: KustoPrincipalAssignment
///       resourceGroupName: ${example.name}
///       clusterName: ${exampleCluster.name}
///       databaseName: ${exampleDatabase.name}
///       tenantId: ${exampleFactory.identity.tenantId}
///       principalId: ${exampleFactory.identity.principalId}
///       principalType: App
///       role: Viewer
/// ```
///
/// ## Import
///
/// Data Factory Linked Service's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/linkedServiceKusto:LinkedServiceKusto example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/linkedservices/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod linked_service_kusto {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceKustoArgs {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to Azure Kusto Linked Service:
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Linked Service.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub integration_runtime_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Kusto Database Name.
        #[builder(into)]
        pub kusto_database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The URI of the Kusto Cluster endpoint.
        #[builder(into)]
        pub kusto_endpoint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of parameters to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The service principal id in which to authenticate against the Kusto Database.
        #[builder(into, default)]
        pub service_principal_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The service principal key in which to authenticate against the Kusto Database.
        #[builder(into, default)]
        pub service_principal_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The service principal tenant id or name in which to authenticate against the Kusto Database.
        ///
        /// > **NOTE** If `service_principal_id` is used, `service_principal_key` and `tenant` is also required.
        ///
        /// > **NOTE** One of Managed Identity authentication and Service Principal authentication must be set.
        #[builder(into, default)]
        pub tenant: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to use the Data Factory's managed identity to authenticate against the Kusto Database.
        #[builder(into, default)]
        pub use_managed_identity: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceKustoResult {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to Azure Kusto Linked Service:
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Linked Service.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        pub integration_runtime_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Kusto Database Name.
        pub kusto_database_name: pulumi_gestalt_rust::Output<String>,
        /// The URI of the Kusto Cluster endpoint.
        pub kusto_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Linked Service.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The service principal id in which to authenticate against the Kusto Database.
        pub service_principal_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The service principal key in which to authenticate against the Kusto Database.
        pub service_principal_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The service principal tenant id or name in which to authenticate against the Kusto Database.
        ///
        /// > **NOTE** If `service_principal_id` is used, `service_principal_key` and `tenant` is also required.
        ///
        /// > **NOTE** One of Managed Identity authentication and Service Principal authentication must be set.
        pub tenant: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to use the Data Factory's managed identity to authenticate against the Kusto Database.
        pub use_managed_identity: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LinkedServiceKustoArgs,
    ) -> LinkedServiceKustoResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_properties_binding = args
            .additional_properties
            .get_output(context);
        let annotations_binding = args.annotations.get_output(context);
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let integration_runtime_name_binding = args
            .integration_runtime_name
            .get_output(context);
        let kusto_database_name_binding = args.kusto_database_name.get_output(context);
        let kusto_endpoint_binding = args.kusto_endpoint.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let service_principal_id_binding = args.service_principal_id.get_output(context);
        let service_principal_key_binding = args
            .service_principal_key
            .get_output(context);
        let tenant_binding = args.tenant.get_output(context);
        let use_managed_identity_binding = args.use_managed_identity.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/linkedServiceKusto:LinkedServiceKusto".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalProperties".into(),
                    value: additional_properties_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: annotations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataFactoryId".into(),
                    value: data_factory_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationRuntimeName".into(),
                    value: integration_runtime_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kustoDatabaseName".into(),
                    value: kusto_database_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kustoEndpoint".into(),
                    value: kusto_endpoint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "servicePrincipalId".into(),
                    value: service_principal_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "servicePrincipalKey".into(),
                    value: service_principal_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenant".into(),
                    value: tenant_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "useManagedIdentity".into(),
                    value: use_managed_identity_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LinkedServiceKustoResult {
            additional_properties: o.get_field("additionalProperties"),
            annotations: o.get_field("annotations"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            integration_runtime_name: o.get_field("integrationRuntimeName"),
            kusto_database_name: o.get_field("kustoDatabaseName"),
            kusto_endpoint: o.get_field("kustoEndpoint"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            service_principal_id: o.get_field("servicePrincipalId"),
            service_principal_key: o.get_field("servicePrincipalKey"),
            tenant: o.get_field("tenant"),
            use_managed_identity: o.get_field("useManagedIdentity"),
        }
    }
}
