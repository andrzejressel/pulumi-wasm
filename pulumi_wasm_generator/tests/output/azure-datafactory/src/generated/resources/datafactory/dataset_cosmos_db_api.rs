/// Manages an Azure Cosmos DB SQL API Dataset inside an Azure Data Factory.
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
///       database: foo
///   exampleDatasetCosmosDBApi:
///     type: azure:datafactory:DatasetCosmosDBApi
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       linkedServiceName: ${exampleLinkedServiceCosmosDb.name}
///       collectionName: bar
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
/// Data Factory Datasets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/datasetCosmosDBApi:DatasetCosmosDBApi example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/datasets/example
/// ```
///
pub mod dataset_cosmos_db_api {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetCosmosDBApiArgs {
        /// A map of additional properties to associate with the Data Factory Dataset.
        ///
        /// The following supported arguments are specific to Azure Cosmos DB SQL API Dataset:
        #[builder(into, default)]
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The collection name of the Data Factory Dataset Azure Cosmos DB SQL API.
        #[builder(into, default)]
        pub collection_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Dataset.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        #[builder(into, default)]
        pub folder: pulumi_wasm_rust::Output<Option<String>>,
        /// The Data Factory Linked Service name in which to associate the Dataset with.
        #[builder(into)]
        pub linked_service_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Data Factory Dataset. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of parameters to associate with the Data Factory Dataset.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `schema_column` block as defined below.
        #[builder(into, default)]
        pub schema_columns: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::datafactory::DatasetCosmosDbApiSchemaColumn>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DatasetCosmosDBApiResult {
        /// A map of additional properties to associate with the Data Factory Dataset.
        ///
        /// The following supported arguments are specific to Azure Cosmos DB SQL API Dataset:
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The collection name of the Data Factory Dataset Azure Cosmos DB SQL API.
        pub collection_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Dataset.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        pub folder: pulumi_wasm_rust::Output<Option<String>>,
        /// The Data Factory Linked Service name in which to associate the Dataset with.
        pub linked_service_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Data Factory Dataset. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Dataset.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `schema_column` block as defined below.
        pub schema_columns: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::datafactory::DatasetCosmosDbApiSchemaColumn>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DatasetCosmosDBApiArgs) -> DatasetCosmosDBApiResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_properties_binding = args.additional_properties.get_inner();
        let annotations_binding = args.annotations.get_inner();
        let collection_name_binding = args.collection_name.get_inner();
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let description_binding = args.description.get_inner();
        let folder_binding = args.folder.get_inner();
        let linked_service_name_binding = args.linked_service_name.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let schema_columns_binding = args.schema_columns.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/datasetCosmosDBApi:DatasetCosmosDBApi".into(),
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
                    name: "collectionName".into(),
                    value: &collection_name_binding,
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
                    name: "folder".into(),
                    value: &folder_binding,
                },
                register_interface::ObjectField {
                    name: "linkedServiceName".into(),
                    value: &linked_service_name_binding,
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
                    name: "schemaColumns".into(),
                    value: &schema_columns_binding,
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
                    name: "collectionName".into(),
                },
                register_interface::ResultField {
                    name: "dataFactoryId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "folder".into(),
                },
                register_interface::ResultField {
                    name: "linkedServiceName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "schemaColumns".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DatasetCosmosDBApiResult {
            additional_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalProperties").unwrap(),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            collection_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("collectionName").unwrap(),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFactoryId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            folder: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("folder").unwrap(),
            ),
            linked_service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkedServiceName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            schema_columns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schemaColumns").unwrap(),
            ),
        }
    }
}