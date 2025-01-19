pub mod get_catalog_table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCatalogTableArgs {
        /// ID of the Glue Catalog and database where the table metadata resides. If omitted, this defaults to the current AWS Account ID.
        #[builder(into, default)]
        pub catalog_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the metadata database where the table metadata resides.
        #[builder(into)]
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// Name of the table.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The time as of when to read the table contents. If not set, the most recent transaction commit time will be used. Cannot be specified along with `transaction_id`. Specified in RFC 3339 format, e.g. `2006-01-02T15:04:05Z07:00`.
        #[builder(into, default)]
        pub query_as_of_time: pulumi_wasm_rust::Output<Option<String>>,
        /// The transaction ID at which to read the table contents.
        #[builder(into, default)]
        pub transaction_id: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct GetCatalogTableResult {
        /// The ARN of the Glue Table.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ID of the Data Catalog in which the table resides.
        pub catalog_id: pulumi_wasm_rust::Output<String>,
        /// Name of the catalog database that contains the target table.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// Description of the table.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the target table.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Owner of the table.
        pub owner: pulumi_wasm_rust::Output<String>,
        /// Map of initialization parameters for the SerDe, in key-value form.
        pub parameters: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for a maximum of 3 partition indexes. See `partition_index` below.
        pub partition_indices: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::glue::GetCatalogTablePartitionIndex>,
        >,
        /// Configuration block of columns by which the table is partitioned. Only primitive types are supported as partition keys. See `partition_keys` below.
        pub partition_keys: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::glue::GetCatalogTablePartitionKey>,
        >,
        pub query_as_of_time: pulumi_wasm_rust::Output<Option<String>>,
        /// Retention time for this table.
        pub retention: pulumi_wasm_rust::Output<i32>,
        /// Configuration block for information about the physical storage of this table. For more information, refer to the [Glue Developer Guide](https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-catalog-tables.html#aws-glue-api-catalog-tables-StorageDescriptor). See `storage_descriptor` below.
        pub storage_descriptors: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::glue::GetCatalogTableStorageDescriptor>,
        >,
        /// Type of this table (EXTERNAL_TABLE, VIRTUAL_VIEW, etc.). While optional, some Athena DDL queries such as `ALTER TABLE` and `SHOW CREATE TABLE` will fail if this argument is empty.
        pub table_type: pulumi_wasm_rust::Output<String>,
        /// Configuration block of a target table for resource linking. See `target_table` below.
        pub target_tables: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::glue::GetCatalogTableTargetTable>,
        >,
        pub transaction_id: pulumi_wasm_rust::Output<Option<i32>>,
        /// If the table is a view, the expanded text of the view; otherwise null.
        pub view_expanded_text: pulumi_wasm_rust::Output<String>,
        /// If the table is a view, the original text of the view; otherwise null.
        pub view_original_text: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCatalogTableArgs) -> GetCatalogTableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_inner();
        let database_name_binding = args.database_name.get_inner();
        let name_binding = args.name.get_inner();
        let query_as_of_time_binding = args.query_as_of_time.get_inner();
        let transaction_id_binding = args.transaction_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:glue/getCatalogTable:getCatalogTable".into(),
            version: super::super::super::get_version(),
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "queryAsOfTime".into(),
                    value: &query_as_of_time_binding,
                },
                register_interface::ObjectField {
                    name: "transactionId".into(),
                    value: &transaction_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "catalogId".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "partitionIndices".into(),
                },
                register_interface::ResultField {
                    name: "partitionKeys".into(),
                },
                register_interface::ResultField {
                    name: "queryAsOfTime".into(),
                },
                register_interface::ResultField {
                    name: "retention".into(),
                },
                register_interface::ResultField {
                    name: "storageDescriptors".into(),
                },
                register_interface::ResultField {
                    name: "tableType".into(),
                },
                register_interface::ResultField {
                    name: "targetTables".into(),
                },
                register_interface::ResultField {
                    name: "transactionId".into(),
                },
                register_interface::ResultField {
                    name: "viewExpandedText".into(),
                },
                register_interface::ResultField {
                    name: "viewOriginalText".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCatalogTableResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            catalog_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogId").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            partition_indices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partitionIndices").unwrap(),
            ),
            partition_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partitionKeys").unwrap(),
            ),
            query_as_of_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queryAsOfTime").unwrap(),
            ),
            retention: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retention").unwrap(),
            ),
            storage_descriptors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageDescriptors").unwrap(),
            ),
            table_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableType").unwrap(),
            ),
            target_tables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetTables").unwrap(),
            ),
            transaction_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transactionId").unwrap(),
            ),
            view_expanded_text: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("viewExpandedText").unwrap(),
            ),
            view_original_text: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("viewOriginalText").unwrap(),
            ),
        }
    }
}
