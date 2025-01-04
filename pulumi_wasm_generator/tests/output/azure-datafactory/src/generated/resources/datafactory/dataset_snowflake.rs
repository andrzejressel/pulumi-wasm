/// Manages a Snowflake Dataset inside an Azure Data Factory.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleDatasetSnowflake = dataset_snowflake::create(
///         "exampleDatasetSnowflake",
///         DatasetSnowflakeArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .linked_service_name("${exampleLinkedServiceSnowflake.name}")
///             .name("example")
///             .schema_name("foo_schema")
///             .table_name("foo_table")
///             .build_struct(),
///     );
///     let exampleFactory = factory::create(
///         "exampleFactory",
///         FactoryArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLinkedServiceSnowflake = linked_service_snowflake::create(
///         "exampleLinkedServiceSnowflake",
///         LinkedServiceSnowflakeArgs::builder()
///             .connection_string(
///                 "jdbc:snowflake://account.region.snowflakecomputing.com/?user=user&db=db&warehouse=wh",
///             )
///             .data_factory_id("${exampleFactory.id}")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory Snowflake Datasets can be imported using the `resource id`,  e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/datasetSnowflake:DatasetSnowflake example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/datasets/example
/// ```
///
pub mod dataset_snowflake {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetSnowflakeArgs {
        /// A map of additional properties to associate with the Data Factory Dataset Snowflake.
        #[builder(into, default)]
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset Snowflake.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Dataset Snowflake.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        #[builder(into, default)]
        pub folder: pulumi_wasm_rust::Output<Option<String>>,
        /// The Data Factory Linked Service name in which to associate the Dataset with.
        #[builder(into)]
        pub linked_service_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Data Factory Dataset Snowflake. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of parameters to associate with the Data Factory Dataset Snowflake.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `schema_column` block as defined below.
        #[builder(into, default)]
        pub schema_columns: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::datafactory::DatasetSnowflakeSchemaColumn>>,
        >,
        /// The schema name of the Data Factory Dataset Snowflake.
        #[builder(into, default)]
        pub schema_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The table name of the Data Factory Dataset Snowflake.
        #[builder(into, default)]
        pub table_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DatasetSnowflakeResult {
        /// A map of additional properties to associate with the Data Factory Dataset Snowflake.
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset Snowflake.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Dataset Snowflake.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        pub folder: pulumi_wasm_rust::Output<Option<String>>,
        /// The Data Factory Linked Service name in which to associate the Dataset with.
        pub linked_service_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Data Factory Dataset Snowflake. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Dataset Snowflake.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `schema_column` block as defined below.
        pub schema_columns: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::datafactory::DatasetSnowflakeSchemaColumn>>,
        >,
        /// The schema name of the Data Factory Dataset Snowflake.
        pub schema_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The table name of the Data Factory Dataset Snowflake.
        pub table_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DatasetSnowflakeArgs) -> DatasetSnowflakeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_properties_binding = args.additional_properties.get_inner();
        let annotations_binding = args.annotations.get_inner();
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let description_binding = args.description.get_inner();
        let folder_binding = args.folder.get_inner();
        let linked_service_name_binding = args.linked_service_name.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let schema_columns_binding = args.schema_columns.get_inner();
        let schema_name_binding = args.schema_name.get_inner();
        let table_name_binding = args.table_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/datasetSnowflake:DatasetSnowflake".into(),
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
                register_interface::ObjectField {
                    name: "schemaName".into(),
                    value: &schema_name_binding,
                },
                register_interface::ObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding,
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
                register_interface::ResultField {
                    name: "schemaName".into(),
                },
                register_interface::ResultField {
                    name: "tableName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DatasetSnowflakeResult {
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
            schema_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schemaName").unwrap(),
            ),
            table_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableName").unwrap(),
            ),
        }
    }
}
