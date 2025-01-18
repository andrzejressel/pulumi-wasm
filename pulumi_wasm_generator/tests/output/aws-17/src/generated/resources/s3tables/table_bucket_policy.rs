/// Resource for managing an Amazon S3 Tables Table Bucket Policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   exampleTableBucketPolicy:
///     type: aws:s3tables:TableBucketPolicy
///     name: example
///     properties:
///       resourcePolicy: ${example.json}
///       tableBucketArn: ${exampleAwsS3tablesTableBucket.arn}
///   test:
///     type: aws:s3tables:TableBucket
///     properties:
///       name: example-bucket
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Tables Table Bucket Policy using the `table_bucket_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:s3tables/tableBucketPolicy:TableBucketPolicy example 'arn:aws:s3tables:us-west-2:123456789012:bucket/example-bucket;example-namespace'
/// ```
pub mod table_bucket_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableBucketPolicyArgs {
        /// Amazon Web Services resource-based policy document in JSON format.
        #[builder(into)]
        pub resource_policy: pulumi_wasm_rust::Output<String>,
        /// ARN referencing the Table Bucket that owns this policy.
        #[builder(into)]
        pub table_bucket_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TableBucketPolicyResult {
        /// Amazon Web Services resource-based policy document in JSON format.
        pub resource_policy: pulumi_wasm_rust::Output<String>,
        /// ARN referencing the Table Bucket that owns this policy.
        pub table_bucket_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TableBucketPolicyArgs) -> TableBucketPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let resource_policy_binding = args.resource_policy.get_inner();
        let table_bucket_arn_binding = args.table_bucket_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3tables/tableBucketPolicy:TableBucketPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "resourcePolicy".into(),
                    value: &resource_policy_binding,
                },
                register_interface::ObjectField {
                    name: "tableBucketArn".into(),
                    value: &table_bucket_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "resourcePolicy".into(),
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
        TableBucketPolicyResult {
            resource_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourcePolicy").unwrap(),
            ),
            table_bucket_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableBucketArn").unwrap(),
            ),
        }
    }
}
