/// Manages a Linked Service (connection) between a CosmosDB and Azure Data Factory using SQL API.
///
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleFactory:
///     type: azure:datafactory:Factory
///     name: example
///     properties:
///       name: example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///   exampleLinkedServiceCosmosDb:
///     type: azure:datafactory:LinkedServiceCosmosDb
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       accountEndpoint: ${example.endpoint}
///       accountKey: ${example.primaryKey}
///       database: foo
/// variables:
///   example:
///     fn::invoke:
///       function: azure:cosmosdb:getAccount
///       arguments:
///         name: tfex-cosmosdb-account
///         resourceGroupName: tfex-cosmosdb-account-rg
/// ```
///
/// ## Import
///
/// Data Factory Linked Service's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/linkedServiceCosmosDb:LinkedServiceCosmosDb example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/linkedservices/example
/// ```
///
pub mod linked_service_cosmos_db {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceCosmosDbArgs {
        /// The endpoint of the Azure CosmosDB account. Required if `connection_string` is unspecified.
        #[builder(into, default)]
        pub account_endpoint: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The account key of the Azure Cosmos DB account. Required if `connection_string` is unspecified.
        #[builder(into, default)]
        pub account_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to CosmosDB Linked Service:
        #[builder(into, default)]
        pub additional_properties: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The connection string. Required if `account_endpoint`, `account_key`, and `database` are unspecified.
        #[builder(into, default)]
        pub connection_string: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the database. Required if `connection_string` is unspecified.
        #[builder(into, default)]
        pub database: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The description for the Data Factory Linked Service.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub integration_runtime_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of parameters to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceCosmosDbResult {
        /// The endpoint of the Azure CosmosDB account. Required if `connection_string` is unspecified.
        pub account_endpoint: pulumi_wasm_rust::Output<Option<String>>,
        /// The account key of the Azure Cosmos DB account. Required if `connection_string` is unspecified.
        pub account_key: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to CosmosDB Linked Service:
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The connection string. Required if `account_endpoint`, `account_key`, and `database` are unspecified.
        pub connection_string: pulumi_wasm_rust::Output<Option<String>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The name of the database. Required if `connection_string` is unspecified.
        pub database: pulumi_wasm_rust::Output<Option<String>>,
        /// The description for the Data Factory Linked Service.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        pub integration_runtime_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Linked Service.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LinkedServiceCosmosDbArgs,
    ) -> LinkedServiceCosmosDbResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_endpoint_binding = args
            .account_endpoint
            .get_output(context)
            .get_inner();
        let account_key_binding = args.account_key.get_output(context).get_inner();
        let additional_properties_binding = args
            .additional_properties
            .get_output(context)
            .get_inner();
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let connection_string_binding = args
            .connection_string
            .get_output(context)
            .get_inner();
        let data_factory_id_binding = args
            .data_factory_id
            .get_output(context)
            .get_inner();
        let database_binding = args.database.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let integration_runtime_name_binding = args
            .integration_runtime_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/linkedServiceCosmosDb:LinkedServiceCosmosDb"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountEndpoint".into(),
                    value: &account_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "accountKey".into(),
                    value: &account_key_binding,
                },
                register_interface::ObjectField {
                    name: "additionalProperties".into(),
                    value: &additional_properties_binding,
                },
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "connectionString".into(),
                    value: &connection_string_binding,
                },
                register_interface::ObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding,
                },
                register_interface::ObjectField {
                    name: "database".into(),
                    value: &database_binding,
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "accountKey".into(),
                },
                register_interface::ResultField {
                    name: "additionalProperties".into(),
                },
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "connectionString".into(),
                },
                register_interface::ResultField {
                    name: "dataFactoryId".into(),
                },
                register_interface::ResultField {
                    name: "database".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "integrationRuntimeName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LinkedServiceCosmosDbResult {
            account_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountEndpoint").unwrap(),
            ),
            account_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountKey").unwrap(),
            ),
            additional_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalProperties").unwrap(),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionString").unwrap(),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFactoryId").unwrap(),
            ),
            database: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("database").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            integration_runtime_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationRuntimeName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
        }
    }
}
