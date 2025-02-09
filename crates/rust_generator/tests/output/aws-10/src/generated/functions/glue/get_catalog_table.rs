#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetCatalogTableArgs,
    ) -> GetCatalogTableResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let catalog_id_binding = args.catalog_id.get_output(context);
        let database_name_binding = args.database_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let query_as_of_time_binding = args.query_as_of_time.get_output(context);
        let transaction_id_binding = args.transaction_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:glue/getCatalogTable:getCatalogTable".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "catalogId".into(),
                    value: catalog_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseName".into(),
                    value: database_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queryAsOfTime".into(),
                    value: query_as_of_time_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transactionId".into(),
                    value: transaction_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCatalogTableResult {
            arn: o.get_field("arn"),
            catalog_id: o.get_field("catalogId"),
            database_name: o.get_field("databaseName"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            owner: o.get_field("owner"),
            parameters: o.get_field("parameters"),
            partition_indices: o.get_field("partitionIndices"),
            partition_keys: o.get_field("partitionKeys"),
            query_as_of_time: o.get_field("queryAsOfTime"),
            retention: o.get_field("retention"),
            storage_descriptors: o.get_field("storageDescriptors"),
            table_type: o.get_field("tableType"),
            target_tables: o.get_field("targetTables"),
            transaction_id: o.get_field("transactionId"),
            view_expanded_text: o.get_field("viewExpandedText"),
            view_original_text: o.get_field("viewOriginalText"),
        }
    }
}
