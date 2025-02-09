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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TableBucketArgs,
    ) -> TableBucketResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let maintenance_configuration_binding_1 = args
            .maintenance_configuration
            .get_output(context);
        let maintenance_configuration_binding = maintenance_configuration_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3tables/tableBucket:TableBucket".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "maintenanceConfiguration".into(),
                    value: &maintenance_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TableBucketResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            created_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            maintenance_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenanceConfiguration"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerAccountId"),
            ),
        }
    }
}
