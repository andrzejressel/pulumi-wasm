/// Provides a Glue Partition Resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:glue:Partition
///     properties:
///       databaseName: some-database
///       tableName: some-table
///       values:
///         - some-value
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Partitions using the catalog ID (usually AWS account ID), database name, table name and partition values. For example:
///
/// ```sh
/// $ pulumi import aws:glue/partition:Partition part 123456789012:MyDatabase:MyTable:val1#val2
/// ```
pub mod partition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PartitionArgs {
        /// ID of the Glue Catalog and database to create the table in. If omitted, this defaults to the AWS Account ID plus the database name.
        #[builder(into, default)]
        pub catalog_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the metadata database where the table metadata resides. For Hive compatibility, this must be all lowercase.
        #[builder(into)]
        pub database_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Properties associated with this table, as a list of key-value pairs.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The values that define the partition.
        #[builder(into)]
        pub partition_values: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// A storage descriptor object containing information about the physical storage of this table. You can refer to the [Glue Developer Guide](https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-catalog-tables.html#aws-glue-api-catalog-tables-StorageDescriptor) for a full explanation of this object.
        #[builder(into, default)]
        pub storage_descriptor: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::glue::PartitionStorageDescriptor>,
        >,
        #[builder(into)]
        pub table_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PartitionResult {
        /// ID of the Glue Catalog and database to create the table in. If omitted, this defaults to the AWS Account ID plus the database name.
        pub catalog_id: pulumi_wasm_rust::Output<String>,
        /// The time at which the partition was created.
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// Name of the metadata database where the table metadata resides. For Hive compatibility, this must be all lowercase.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// The last time at which the partition was accessed.
        pub last_accessed_time: pulumi_wasm_rust::Output<String>,
        /// The last time at which column statistics were computed for this partition.
        pub last_analyzed_time: pulumi_wasm_rust::Output<String>,
        /// Properties associated with this table, as a list of key-value pairs.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The values that define the partition.
        pub partition_values: pulumi_wasm_rust::Output<Vec<String>>,
        /// A storage descriptor object containing information about the physical storage of this table. You can refer to the [Glue Developer Guide](https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-catalog-tables.html#aws-glue-api-catalog-tables-StorageDescriptor) for a full explanation of this object.
        pub storage_descriptor: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::PartitionStorageDescriptor>,
        >,
        pub table_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PartitionArgs,
    ) -> PartitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_output(context).get_inner();
        let database_name_binding = args.database_name.get_output(context).get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let partition_values_binding = args
            .partition_values
            .get_output(context)
            .get_inner();
        let storage_descriptor_binding = args
            .storage_descriptor
            .get_output(context)
            .get_inner();
        let table_name_binding = args.table_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/partition:Partition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding,
                },
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "partitionValues".into(),
                    value: &partition_values_binding,
                },
                register_interface::ObjectField {
                    name: "storageDescriptor".into(),
                    value: &storage_descriptor_binding,
                },
                register_interface::ObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "catalogId".into(),
                },
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "lastAccessedTime".into(),
                },
                register_interface::ResultField {
                    name: "lastAnalyzedTime".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "partitionValues".into(),
                },
                register_interface::ResultField {
                    name: "storageDescriptor".into(),
                },
                register_interface::ResultField {
                    name: "tableName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PartitionResult {
            catalog_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogId").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            last_accessed_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastAccessedTime").unwrap(),
            ),
            last_analyzed_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastAnalyzedTime").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            partition_values: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partitionValues").unwrap(),
            ),
            storage_descriptor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageDescriptor").unwrap(),
            ),
            table_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableName").unwrap(),
            ),
        }
    }
}
