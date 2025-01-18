/// Resource for managing an Amazon S3 Tables Namespace.
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
///     let example = namespace::create(
///         "example",
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
/// Using `pulumi import`, import S3 Tables Namespace using the `table_bucket_arn` and the value of `namespace`, separated by a semicolon (`;`). For example:
///
/// ```sh
/// $ pulumi import aws:s3tables/namespace:Namespace example 'arn:aws:s3tables:us-west-2:123456789012:bucket/example-bucket;example-namespace'
/// ```
pub mod namespace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceArgs {
        /// Name of the namespace.
        /// Must be between 1 and 255 characters in length.
        /// Can consist of lowercase letters, numbers, and underscores, and must begin and end with a lowercase letter or number.
        #[builder(into)]
        pub namespace: pulumi_wasm_rust::Output<String>,
        /// ARN referencing the Table Bucket that contains this Namespace.
        #[builder(into)]
        pub table_bucket_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NamespaceResult {
        /// Date and time when the namespace was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Account ID of the account that created the namespace.
        pub created_by: pulumi_wasm_rust::Output<String>,
        /// Name of the namespace.
        /// Must be between 1 and 255 characters in length.
        /// Can consist of lowercase letters, numbers, and underscores, and must begin and end with a lowercase letter or number.
        pub namespace: pulumi_wasm_rust::Output<String>,
        /// Account ID of the account that owns the namespace.
        pub owner_account_id: pulumi_wasm_rust::Output<String>,
        /// ARN referencing the Table Bucket that contains this Namespace.
        pub table_bucket_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NamespaceArgs) -> NamespaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let namespace_binding = args.namespace.get_inner();
        let table_bucket_arn_binding = args.table_bucket_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3tables/namespace:Namespace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding,
                },
                register_interface::ObjectField {
                    name: "tableBucketArn".into(),
                    value: &table_bucket_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "createdBy".into(),
                },
                register_interface::ResultField {
                    name: "namespace".into(),
                },
                register_interface::ResultField {
                    name: "ownerAccountId".into(),
                },
                register_interface::ResultField {
                    name: "tableBucketArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NamespaceResult {
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            created_by: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdBy").unwrap(),
            ),
            namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespace").unwrap(),
            ),
            owner_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerAccountId").unwrap(),
            ),
            table_bucket_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableBucketArn").unwrap(),
            ),
        }
    }
}
