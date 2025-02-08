/// Provides a Glue Catalog Table Resource. You can refer to the [Glue Developer Guide](http://docs.aws.amazon.com/glue/latest/dg/populate-data-catalog.html) for a full explanation of the Glue Data Catalog functionality.
///
/// ## Example Usage
///
/// ### Basic Table
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let awsGlueCatalogTable = catalog_table::create(
///         "awsGlueCatalogTable",
///         CatalogTableArgs::builder()
///             .database_name("MyCatalogDatabase")
///             .name("MyCatalogTable")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Parquet Table for Athena
///
/// ```yaml
/// resources:
///   awsGlueCatalogTable:
///     type: aws:glue:CatalogTable
///     name: aws_glue_catalog_table
///     properties:
///       name: MyCatalogTable
///       databaseName: MyCatalogDatabase
///       tableType: EXTERNAL_TABLE
///       parameters:
///         EXTERNAL: TRUE
///         parquet.compression: SNAPPY
///       storageDescriptor:
///         location: s3://my-bucket/event-streams/my-stream
///         inputFormat: org.apache.hadoop.hive.ql.io.parquet.MapredParquetInputFormat
///         outputFormat: org.apache.hadoop.hive.ql.io.parquet.MapredParquetOutputFormat
///         serDeInfo:
///           name: my-stream
///           serializationLibrary: org.apache.hadoop.hive.ql.io.parquet.serde.ParquetHiveSerDe
///           parameters:
///             serialization.format: 1
///         columns:
///           - name: my_string
///             type: string
///           - name: my_double
///             type: double
///           - name: my_date
///             type: date
///             comment: ""
///           - name: my_bigint
///             type: bigint
///             comment: ""
///           - name: my_struct
///             type: struct<my_nested_string:string>
///             comment: ""
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Tables using the catalog ID (usually AWS account ID), database name, and table name. For example:
///
/// ```sh
/// $ pulumi import aws:glue/catalogTable:CatalogTable MyTable 123456789012:MyDatabase:MyTable
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod catalog_table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CatalogTableArgs {
        /// ID of the Glue Catalog and database to create the table in. If omitted, this defaults to the AWS Account ID plus the database name.
        #[builder(into, default)]
        pub catalog_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the metadata database where the table metadata resides. For Hive compatibility, this must be all lowercase.
        ///
        /// The follow arguments are optional:
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the table.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the table. For Hive compatibility, this must be entirely lowercase.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for open table formats. See `open_table_format_input` below.
        #[builder(into, default)]
        pub open_table_format_input: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::CatalogTableOpenTableFormatInput>,
        >,
        /// Owner of the table.
        #[builder(into, default)]
        pub owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Properties associated with this table, as a list of key-value pairs.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for a maximum of 3 partition indexes. See `partition_index` below.
        #[builder(into, default)]
        pub partition_indices: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::glue::CatalogTablePartitionIndex>>,
        >,
        /// Configuration block of columns by which the table is partitioned. Only primitive types are supported as partition keys. See `partition_keys` below.
        #[builder(into, default)]
        pub partition_keys: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::glue::CatalogTablePartitionKey>>,
        >,
        /// Retention time for this table.
        #[builder(into, default)]
        pub retention: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Configuration block for information about the physical storage of this table. For more information, refer to the [Glue Developer Guide](https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-catalog-tables.html#aws-glue-api-catalog-tables-StorageDescriptor). See `storage_descriptor` below.
        #[builder(into, default)]
        pub storage_descriptor: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::CatalogTableStorageDescriptor>,
        >,
        /// Type of this table (EXTERNAL_TABLE, VIRTUAL_VIEW, etc.). While optional, some Athena DDL queries such as `ALTER TABLE` and `SHOW CREATE TABLE` will fail if this argument is empty.
        #[builder(into, default)]
        pub table_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block of a target table for resource linking. See `target_table` below.
        #[builder(into, default)]
        pub target_table: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::CatalogTableTargetTable>,
        >,
        /// If the table is a view, the expanded text of the view; otherwise null.
        #[builder(into, default)]
        pub view_expanded_text: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If the table is a view, the original text of the view; otherwise null.
        #[builder(into, default)]
        pub view_original_text: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CatalogTableResult {
        /// The ARN of the Glue Table.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the Glue Catalog and database to create the table in. If omitted, this defaults to the AWS Account ID plus the database name.
        pub catalog_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the metadata database where the table metadata resides. For Hive compatibility, this must be all lowercase.
        ///
        /// The follow arguments are optional:
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// Description of the table.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the table. For Hive compatibility, this must be entirely lowercase.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for open table formats. See `open_table_format_input` below.
        pub open_table_format_input: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::CatalogTableOpenTableFormatInput>,
        >,
        /// Owner of the table.
        pub owner: pulumi_gestalt_rust::Output<Option<String>>,
        /// Properties associated with this table, as a list of key-value pairs.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for a maximum of 3 partition indexes. See `partition_index` below.
        pub partition_indices: pulumi_gestalt_rust::Output<
            Vec<super::super::types::glue::CatalogTablePartitionIndex>,
        >,
        /// Configuration block of columns by which the table is partitioned. Only primitive types are supported as partition keys. See `partition_keys` below.
        pub partition_keys: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::glue::CatalogTablePartitionKey>>,
        >,
        /// Retention time for this table.
        pub retention: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Configuration block for information about the physical storage of this table. For more information, refer to the [Glue Developer Guide](https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-catalog-tables.html#aws-glue-api-catalog-tables-StorageDescriptor). See `storage_descriptor` below.
        pub storage_descriptor: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::CatalogTableStorageDescriptor>,
        >,
        /// Type of this table (EXTERNAL_TABLE, VIRTUAL_VIEW, etc.). While optional, some Athena DDL queries such as `ALTER TABLE` and `SHOW CREATE TABLE` will fail if this argument is empty.
        pub table_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block of a target table for resource linking. See `target_table` below.
        pub target_table: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::CatalogTableTargetTable>,
        >,
        /// If the table is a view, the expanded text of the view; otherwise null.
        pub view_expanded_text: pulumi_gestalt_rust::Output<Option<String>>,
        /// If the table is a view, the original text of the view; otherwise null.
        pub view_original_text: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CatalogTableArgs,
    ) -> CatalogTableResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_output(context).get_inner();
        let database_name_binding = args.database_name.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let open_table_format_input_binding = args
            .open_table_format_input
            .get_output(context)
            .get_inner();
        let owner_binding = args.owner.get_output(context).get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let partition_indices_binding = args
            .partition_indices
            .get_output(context)
            .get_inner();
        let partition_keys_binding = args.partition_keys.get_output(context).get_inner();
        let retention_binding = args.retention.get_output(context).get_inner();
        let storage_descriptor_binding = args
            .storage_descriptor
            .get_output(context)
            .get_inner();
        let table_type_binding = args.table_type.get_output(context).get_inner();
        let target_table_binding = args.target_table.get_output(context).get_inner();
        let view_expanded_text_binding = args
            .view_expanded_text
            .get_output(context)
            .get_inner();
        let view_original_text_binding = args
            .view_original_text
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/catalogTable:CatalogTable".into(),
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
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "openTableFormatInput".into(),
                    value: &open_table_format_input_binding,
                },
                register_interface::ObjectField {
                    name: "owner".into(),
                    value: &owner_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "partitionIndices".into(),
                    value: &partition_indices_binding,
                },
                register_interface::ObjectField {
                    name: "partitionKeys".into(),
                    value: &partition_keys_binding,
                },
                register_interface::ObjectField {
                    name: "retention".into(),
                    value: &retention_binding,
                },
                register_interface::ObjectField {
                    name: "storageDescriptor".into(),
                    value: &storage_descriptor_binding,
                },
                register_interface::ObjectField {
                    name: "tableType".into(),
                    value: &table_type_binding,
                },
                register_interface::ObjectField {
                    name: "targetTable".into(),
                    value: &target_table_binding,
                },
                register_interface::ObjectField {
                    name: "viewExpandedText".into(),
                    value: &view_expanded_text_binding,
                },
                register_interface::ObjectField {
                    name: "viewOriginalText".into(),
                    value: &view_original_text_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CatalogTableResult {
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
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            open_table_format_input: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("openTableFormatInput"),
            ),
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
            retention: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retention"),
            ),
            storage_descriptor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageDescriptor"),
            ),
            table_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tableType"),
            ),
            target_table: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetTable"),
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
