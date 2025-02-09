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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod partition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PartitionArgs {
        /// ID of the Glue Catalog and database to create the table in. If omitted, this defaults to the AWS Account ID plus the database name.
        #[builder(into, default)]
        pub catalog_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the metadata database where the table metadata resides. For Hive compatibility, this must be all lowercase.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Properties associated with this table, as a list of key-value pairs.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The values that define the partition.
        #[builder(into)]
        pub partition_values: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A storage descriptor object containing information about the physical storage of this table. You can refer to the [Glue Developer Guide](https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-catalog-tables.html#aws-glue-api-catalog-tables-StorageDescriptor) for a full explanation of this object.
        #[builder(into, default)]
        pub storage_descriptor: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::PartitionStorageDescriptor>,
        >,
        #[builder(into)]
        pub table_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PartitionResult {
        /// ID of the Glue Catalog and database to create the table in. If omitted, this defaults to the AWS Account ID plus the database name.
        pub catalog_id: pulumi_gestalt_rust::Output<String>,
        /// The time at which the partition was created.
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// Name of the metadata database where the table metadata resides. For Hive compatibility, this must be all lowercase.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// The last time at which the partition was accessed.
        pub last_accessed_time: pulumi_gestalt_rust::Output<String>,
        /// The last time at which column statistics were computed for this partition.
        pub last_analyzed_time: pulumi_gestalt_rust::Output<String>,
        /// Properties associated with this table, as a list of key-value pairs.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The values that define the partition.
        pub partition_values: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A storage descriptor object containing information about the physical storage of this table. You can refer to the [Glue Developer Guide](https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-catalog-tables.html#aws-glue-api-catalog-tables-StorageDescriptor) for a full explanation of this object.
        pub storage_descriptor: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::PartitionStorageDescriptor>,
        >,
        pub table_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PartitionArgs,
    ) -> PartitionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let catalog_id_binding = args.catalog_id.get_output(context);
        let database_name_binding = args.database_name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let partition_values_binding = args.partition_values.get_output(context);
        let storage_descriptor_binding = args.storage_descriptor.get_output(context);
        let table_name_binding = args.table_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/partition:Partition".into(),
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
                    name: "parameters".into(),
                    value: parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partitionValues".into(),
                    value: partition_values_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageDescriptor".into(),
                    value: storage_descriptor_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableName".into(),
                    value: table_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PartitionResult {
            catalog_id: o.get_field("catalogId"),
            creation_time: o.get_field("creationTime"),
            database_name: o.get_field("databaseName"),
            last_accessed_time: o.get_field("lastAccessedTime"),
            last_analyzed_time: o.get_field("lastAnalyzedTime"),
            parameters: o.get_field("parameters"),
            partition_values: o.get_field("partitionValues"),
            storage_descriptor: o.get_field("storageDescriptor"),
            table_name: o.get_field("tableName"),
        }
    }
}
