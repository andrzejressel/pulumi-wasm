/// Resource for managing an Amazon S3 Tables Table Bucket.
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
///     let example = table_bucket::create(
///         "example",
///         TableBucketArgs::builder().name("example-bucket").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Tables Table Bucket using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:s3tables/tableBucket:TableBucket example arn:aws:s3tables:us-west-2:123456789012:bucket/example-bucket
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod table_bucket {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableBucketArgs {
        /// A single table bucket maintenance configuration block.
        /// See `maintenance_configuration` below
        #[builder(into, default)]
        pub maintenance_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3tables::TableBucketMaintenanceConfiguration>,
        >,
        /// Name of the table bucket.
        /// Must be between 3 and 63 characters in length.
        /// Can consist of lowercase letters, numbers, and hyphens, and must begin and end with a lowercase letter or number.
        /// A full list of bucket naming rules may be found in S3 Tables documentation.
        ///
        /// The following argument is optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TableBucketResult {
        /// ARN of the table bucket.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date and time when the bucket was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// A single table bucket maintenance configuration block.
        /// See `maintenance_configuration` below
        pub maintenance_configuration: pulumi_gestalt_rust::Output<
            super::super::types::s3tables::TableBucketMaintenanceConfiguration,
        >,
        /// Name of the table bucket.
        /// Must be between 3 and 63 characters in length.
        /// Can consist of lowercase letters, numbers, and hyphens, and must begin and end with a lowercase letter or number.
        /// A full list of bucket naming rules may be found in S3 Tables documentation.
        ///
        /// The following argument is optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Account ID of the account that owns the table bucket.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TableBucketArgs,
    ) -> TableBucketResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let maintenance_configuration_binding = args
            .maintenance_configuration
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3tables/tableBucket:TableBucket".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceConfiguration".into(),
                    value: maintenance_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TableBucketResult {
            arn: o.get_field("arn"),
            created_at: o.get_field("createdAt"),
            maintenance_configuration: o.get_field("maintenanceConfiguration"),
            name: o.get_field("name"),
            owner_account_id: o.get_field("ownerAccountId"),
        }
    }
}
