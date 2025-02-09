/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:glue:CatalogDatabase
///     properties:
///       name: example
///   exampleCatalogTable:
///     type: aws:glue:CatalogTable
///     name: example
///     properties:
///       name: example
///       databaseName: ${example.name}
///       owner: my_owner
///       retention: 1
///       tableType: VIRTUAL_VIEW
///       viewExpandedText: view_expanded_text_1
///       viewOriginalText: view_original_text_1
///       storageDescriptor:
///         bucketColumns:
///           - bucket_column_1
///         compressed: false
///         inputFormat: SequenceFileInputFormat
///         location: my_location
///         numberOfBuckets: 1
///         outputFormat: SequenceFileInputFormat
///         storedAsSubDirectories: false
///         parameters:
///           param1: param1_val
///         columns:
///           - name: my_column_1
///             type: int
///             comment: my_column1_comment
///           - name: my_column_2
///             type: string
///             comment: my_column2_comment
///         serDeInfo:
///           name: ser_de_name
///           parameters:
///             param1: param_val_1
///           serializationLibrary: org.apache.hadoop.hive.serde2.columnar.ColumnarSerDe
///         sortColumns:
///           - column: my_column_1
///             sortOrder: 1
///         skewedInfo:
///           skewedColumnNames:
///             - my_column_1
///           skewedColumnValueLocationMaps:
///             my_column_1: my_column_1_val_loc_map
///           skewedColumnValues:
///             - skewed_val_1
///       partitionKeys:
///         - name: my_column_1
///           type: int
///           comment: my_column_1_comment
///         - name: my_column_2
///           type: string
///           comment: my_column_2_comment
///       parameters:
///         param1: param1_val
///   examplePartitionIndex:
///     type: aws:glue:PartitionIndex
///     name: example
///     properties:
///       databaseName: ${example.name}
///       tableName: ${exampleCatalogTable.name}
///       partitionIndex:
///         indexName: example
///         keys:
///           - my_column_1
///           - my_column_2
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Partition Indexes using the catalog ID (usually AWS account ID), database name, table name, and index name. For example:
///
/// ```sh
/// $ pulumi import aws:glue/partitionIndex:PartitionIndex example 123456789012:MyDatabase:MyTable:index-name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod partition_index {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PartitionIndexArgs {
        /// The catalog ID where the table resides.
        #[builder(into, default)]
        pub catalog_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the metadata database where the table metadata resides. For Hive compatibility, this must be all lowercase.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block for a partition index. See `partition_index` below.
        #[builder(into)]
        pub partition_index: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::glue::PartitionIndexPartitionIndex,
        >,
        /// Name of the table. For Hive compatibility, this must be entirely lowercase.
        #[builder(into)]
        pub table_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PartitionIndexResult {
        /// The catalog ID where the table resides.
        pub catalog_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the metadata database where the table metadata resides. For Hive compatibility, this must be all lowercase.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for a partition index. See `partition_index` below.
        pub partition_index: pulumi_gestalt_rust::Output<
            super::super::types::glue::PartitionIndexPartitionIndex,
        >,
        /// Name of the table. For Hive compatibility, this must be entirely lowercase.
        pub table_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PartitionIndexArgs,
    ) -> PartitionIndexResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let catalog_id_binding = args.catalog_id.get_output(context);
        let database_name_binding = args.database_name.get_output(context);
        let partition_index_binding = args.partition_index.get_output(context);
        let table_name_binding = args.table_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/partitionIndex:PartitionIndex".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "partitionIndex".into(),
                    value: partition_index_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableName".into(),
                    value: table_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PartitionIndexResult {
            catalog_id: o.get_field("catalogId"),
            database_name: o.get_field("databaseName"),
            partition_index: o.get_field("partitionIndex"),
            table_name: o.get_field("tableName"),
        }
    }
}
