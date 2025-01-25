/// Resource for managing an Amazon S3 Tables Table Bucket.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod table_bucket {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableBucketArgs {
        /// A single table bucket maintenance configuration block.
        /// See `maintenance_configuration` below
        #[builder(into, default)]
        pub maintenance_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::s3tables::TableBucketMaintenanceConfiguration>,
        >,
        /// Name of the table bucket.
        /// Must be between 3 and 63 characters in length.
        /// Can consist of lowercase letters, numbers, and hyphens, and must begin and end with a lowercase letter or number.
        /// A full list of bucket naming rules may be found in S3 Tables documentation.
        ///
        /// The following argument is optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TableBucketResult {
        /// ARN of the table bucket.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Date and time when the bucket was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// A single table bucket maintenance configuration block.
        /// See `maintenance_configuration` below
        pub maintenance_configuration: pulumi_wasm_rust::Output<
            super::super::types::s3tables::TableBucketMaintenanceConfiguration,
        >,
        /// Name of the table bucket.
        /// Must be between 3 and 63 characters in length.
        /// Can consist of lowercase letters, numbers, and hyphens, and must begin and end with a lowercase letter or number.
        /// A full list of bucket naming rules may be found in S3 Tables documentation.
        ///
        /// The following argument is optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Account ID of the account that owns the table bucket.
        pub owner_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TableBucketArgs,
    ) -> TableBucketResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let maintenance_configuration_binding = args
            .maintenance_configuration
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ownerAccountId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TableBucketResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            maintenance_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceConfiguration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerAccountId").unwrap(),
            ),
        }
    }
}
