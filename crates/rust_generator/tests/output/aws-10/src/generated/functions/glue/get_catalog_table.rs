#[allow(clippy::doc_lazy_continuation)]
pub mod get_catalog_table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCatalogTableArgs {
        /// ID of the Glue Catalog and database where the table metadata resides. If omitted, this defaults to the current AWS Account ID.
        #[builder(into, default)]
        pub catalog_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the metadata database where the table metadata resides.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the table.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The time as of when to read the table contents. If not set, the most recent transaction commit time will be used. Cannot be specified along with `transaction_id`. Specified in RFC 3339 format, e.g. `2006-01-02T15:04:05Z07:00`.
        #[builder(into, default)]
        pub query_as_of_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The transaction ID at which to read the table contents.
        #[builder(into, default)]
        pub transaction_id: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct GetCatalogTableResult {
        /// The ARN of the Glue Table.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the Data Catalog in which the table resides.
        pub catalog_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the catalog database that contains the target table.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// Description of the table.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the target table.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Owner of the table.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// Map of initialization parameters for the SerDe, in key-value form.
        pub parameters: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for a maximum of 3 partition indexes. See `partition_index` below.
        pub partition_indices: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::glue::GetCatalogTablePartitionIndex>,
        >,
        /// Configuration block of columns by which the table is partitioned. Only primitive types are supported as partition keys. See `partition_keys` below.
        pub partition_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::glue::GetCatalogTablePartitionKey>,
        >,
        pub query_as_of_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// Retention time for this table.
        pub retention: pulumi_gestalt_rust::Output<i32>,
        /// Configuration block for information about the physical storage of this table. For more information, refer to the [Glue Developer Guide](https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-catalog-tables.html#aws-glue-api-catalog-tables-StorageDescriptor). See `storage_descriptor` below.
        pub storage_descriptors: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::glue::GetCatalogTableStorageDescriptor>,
        >,
        /// Type of this table (EXTERNAL_TABLE, VIRTUAL_VIEW, etc.). While optional, some Athena DDL queries such as `ALTER TABLE` and `SHOW CREATE TABLE` will fail if this argument is empty.
        pub table_type: pulumi_gestalt_rust::Output<String>,
        /// Configuration block of a target table for resource linking. See `target_table` below.
        pub target_tables: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::glue::GetCatalogTableTargetTable>,
        >,
        pub transaction_id: pulumi_gestalt_rust::Output<Option<i32>>,
        /// If the table is a view, the expanded text of the view; otherwise null.
        pub view_expanded_text: pulumi_gestalt_rust::Output<String>,
        /// If the table is a view, the original text of the view; otherwise null.
        pub view_original_text: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetCatalogTableArgs,
    ) -> GetCatalogTableResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_output(context).get_inner();
        let database_name_binding = args.database_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let query_as_of_time_binding = args
            .query_as_of_time
            .get_output(context)
            .get_inner();
        let transaction_id_binding = args.transaction_id.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCatalogTableResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            catalog_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("catalogId"),
            ),
            database_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseName"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner: pulumi_gestalt_rust::__private::into_domain(o.extract_field("owner")),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            partition_indices: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("partitionIndices"),
            ),
            partition_keys: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("partitionKeys"),
            ),
            query_as_of_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("queryAsOfTime"),
            ),
            retention: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retention"),
            ),
            storage_descriptors: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageDescriptors"),
            ),
            table_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tableType"),
            ),
            target_tables: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetTables"),
            ),
            transaction_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transactionId"),
            ),
            view_expanded_text: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("viewExpandedText"),
            ),
            view_original_text: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("viewOriginalText"),
            ),
        }
    }
}
