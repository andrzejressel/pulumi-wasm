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
pub mod linked_service_kusto {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceKustoArgs {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to Azure Kusto Linked Service:
        #[builder(into, default)]
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Linked Service.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub integration_runtime_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Kusto Database Name.
        #[builder(into)]
        pub kusto_database_name: pulumi_wasm_rust::Output<String>,
        /// The URI of the Kusto Cluster endpoint.
        #[builder(into)]
        pub kusto_endpoint: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of parameters to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The service principal id in which to authenticate against the Kusto Database.
        #[builder(into, default)]
        pub service_principal_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The service principal key in which to authenticate against the Kusto Database.
        #[builder(into, default)]
        pub service_principal_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The service principal tenant id or name in which to authenticate against the Kusto Database.
        ///
        /// > **NOTE** If `service_principal_id` is used, `service_principal_key` and `tenant` is also required.
        ///
        /// > **NOTE** One of Managed Identity authentication and Service Principal authentication must be set.
        #[builder(into, default)]
        pub tenant: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to use the Data Factory's managed identity to authenticate against the Kusto Database.
        #[builder(into, default)]
        pub use_managed_identity: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceKustoResult {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to Azure Kusto Linked Service:
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Linked Service.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        pub integration_runtime_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Kusto Database Name.
        pub kusto_database_name: pulumi_wasm_rust::Output<String>,
        /// The URI of the Kusto Cluster endpoint.
        pub kusto_endpoint: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Linked Service.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The service principal id in which to authenticate against the Kusto Database.
        pub service_principal_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The service principal key in which to authenticate against the Kusto Database.
        pub service_principal_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The service principal tenant id or name in which to authenticate against the Kusto Database.
        ///
        /// > **NOTE** If `service_principal_id` is used, `service_principal_key` and `tenant` is also required.
        ///
        /// > **NOTE** One of Managed Identity authentication and Service Principal authentication must be set.
        pub tenant: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to use the Data Factory's managed identity to authenticate against the Kusto Database.
        pub use_managed_identity: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LinkedServiceKustoArgs) -> LinkedServiceKustoResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_properties_binding = args.additional_properties.get_inner();
        let annotations_binding = args.annotations.get_inner();
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let description_binding = args.description.get_inner();
        let integration_runtime_name_binding = args.integration_runtime_name.get_inner();
        let kusto_database_name_binding = args.kusto_database_name.get_inner();
        let kusto_endpoint_binding = args.kusto_endpoint.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let service_principal_id_binding = args.service_principal_id.get_inner();
        let service_principal_key_binding = args.service_principal_key.get_inner();
        let tenant_binding = args.tenant.get_inner();
        let use_managed_identity_binding = args.use_managed_identity.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/linkedServiceKusto:LinkedServiceKusto".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalProperties".into(),
                    value: &additional_properties_binding,
                },
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "integrationRuntimeName".into(),
                    value: &integration_runtime_name_binding,
                },
                register_interface::ObjectField {
                    name: "kustoDatabaseName".into(),
                    value: &kusto_database_name_binding,
                },
                register_interface::ObjectField {
                    name: "kustoEndpoint".into(),
                    value: &kusto_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "servicePrincipalId".into(),
                    value: &service_principal_id_binding,
                },
                register_interface::ObjectField {
                    name: "servicePrincipalKey".into(),
                    value: &service_principal_key_binding,
                },
                register_interface::ObjectField {
                    name: "tenant".into(),
                    value: &tenant_binding,
                },
                register_interface::ObjectField {
                    name: "useManagedIdentity".into(),
                    value: &use_managed_identity_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalProperties".into(),
                },
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "dataFactoryId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "integrationRuntimeName".into(),
                },
                register_interface::ResultField {
                    name: "kustoDatabaseName".into(),
                },
                register_interface::ResultField {
                    name: "kustoEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "servicePrincipalId".into(),
                },
                register_interface::ResultField {
                    name: "servicePrincipalKey".into(),
                },
                register_interface::ResultField {
                    name: "tenant".into(),
                },
                register_interface::ResultField {
                    name: "useManagedIdentity".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LinkedServiceKustoResult {
            additional_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalProperties").unwrap(),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFactoryId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            integration_runtime_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationRuntimeName").unwrap(),
            ),
            kusto_database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kustoDatabaseName").unwrap(),
            ),
            kusto_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kustoEndpoint").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            service_principal_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicePrincipalId").unwrap(),
            ),
            service_principal_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicePrincipalKey").unwrap(),
            ),
            tenant: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenant").unwrap(),
            ),
            use_managed_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("useManagedIdentity").unwrap(),
            ),
        }
    }
}
