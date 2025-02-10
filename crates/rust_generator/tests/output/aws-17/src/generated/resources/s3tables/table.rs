/// Resource for managing an Amazon S3 Tables Table.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = table::create(
///         "example",
///         TableArgs::builder()
///             .format("ICEBERG")
///             .name("example-table")
///             .namespace("${exampleNamespace}")
///             .table_bucket_arn("${exampleNamespace.tableBucketArn}")
///             .build_struct(),
///     );
///     let exampleNamespace = namespace::create(
///         "exampleNamespace",
///         NamespaceArgs::builder()
///             .namespace("example-namespace")
///             .table_bucket_arn("${exampleTableBucket.arn}")
///             .build_struct(),
///     );
///     let exampleTableBucket = table_bucket::create(
///         "exampleTableBucket",
///         TableBucketArgs::builder().name("example-bucket").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Tables Table using the `table_bucket_arn`, the value of `namespace`, and the value of `name`, separated by a semicolon (`;`). For example:
///
/// ```sh
/// $ pulumi import aws:s3tables/table:Table example 'arn:aws:s3tables:us-west-2:123456789012:bucket/example-bucket;example-namespace;example-table'
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableArgs {
        /// Format of the table.
        /// Must be `ICEBERG`.
        #[builder(into)]
        pub format: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A single table bucket maintenance configuration block.
        /// See `maintenance_configuration` below
        #[builder(into, default)]
        pub maintenance_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3tables::TableMaintenanceConfiguration>,
        >,
        /// Name of the table.
        /// Must be between 1 and 255 characters in length.
        /// Can consist of lowercase letters, numbers, and underscores, and must begin and end with a lowercase letter or number.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the namespace for this table.
        /// Must be between 1 and 255 characters in length.
        /// Can consist of lowercase letters, numbers, and underscores, and must begin and end with a lowercase letter or number.
        #[builder(into)]
        pub namespace: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN referencing the Table Bucket that contains this Namespace.
        ///
        /// The following argument is optional:
        #[builder(into)]
        pub table_bucket_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TableResult {
        /// ARN of the table.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date and time when the namespace was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Account ID of the account that created the namespace.
        pub created_by: pulumi_gestalt_rust::Output<String>,
        /// Format of the table.
        /// Must be `ICEBERG`.
        pub format: pulumi_gestalt_rust::Output<String>,
        /// A single table bucket maintenance configuration block.
        /// See `maintenance_configuration` below
        pub maintenance_configuration: pulumi_gestalt_rust::Output<
            super::super::types::s3tables::TableMaintenanceConfiguration,
        >,
        /// Location of table metadata.
        pub metadata_location: pulumi_gestalt_rust::Output<String>,
        /// Date and time when the namespace was last modified.
        pub modified_at: pulumi_gestalt_rust::Output<String>,
        /// Account ID of the account that last modified the namespace.
        pub modified_by: pulumi_gestalt_rust::Output<String>,
        /// Name of the table.
        /// Must be between 1 and 255 characters in length.
        /// Can consist of lowercase letters, numbers, and underscores, and must begin and end with a lowercase letter or number.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Name of the namespace for this table.
        /// Must be between 1 and 255 characters in length.
        /// Can consist of lowercase letters, numbers, and underscores, and must begin and end with a lowercase letter or number.
        pub namespace: pulumi_gestalt_rust::Output<String>,
        /// Account ID of the account that owns the namespace.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
        /// ARN referencing the Table Bucket that contains this Namespace.
        ///
        /// The following argument is optional:
        pub table_bucket_arn: pulumi_gestalt_rust::Output<String>,
        /// Type of the table.
        /// One of `customer` or `aws`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Identifier for the current version of table data.
        pub version_token: pulumi_gestalt_rust::Output<String>,
        /// S3 URI pointing to the S3 Bucket that contains the table data.
        pub warehouse_location: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TableArgs,
    ) -> TableResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let format_binding = args.format.get_output(context);
        let maintenance_configuration_binding = args
            .maintenance_configuration
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let namespace_binding = args.namespace.get_output(context);
        let table_bucket_arn_binding = args.table_bucket_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3tables/table:Table".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "format".into(),
                    value: format_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceConfiguration".into(),
                    value: maintenance_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespace".into(),
                    value: namespace_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableBucketArn".into(),
                    value: table_bucket_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TableResult {
            arn: o.get_field("arn"),
            created_at: o.get_field("createdAt"),
            created_by: o.get_field("createdBy"),
            format: o.get_field("format"),
            maintenance_configuration: o.get_field("maintenanceConfiguration"),
            metadata_location: o.get_field("metadataLocation"),
            modified_at: o.get_field("modifiedAt"),
            modified_by: o.get_field("modifiedBy"),
            name: o.get_field("name"),
            namespace: o.get_field("namespace"),
            owner_account_id: o.get_field("ownerAccountId"),
            table_bucket_arn: o.get_field("tableBucketArn"),
            type_: o.get_field("type"),
            version_token: o.get_field("versionToken"),
            warehouse_location: o.get_field("warehouseLocation"),
        }
    }
}
