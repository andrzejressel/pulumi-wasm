/// Attaches a policy to an S3 bucket resource.
///
/// > Policies can be attached to both S3 general purpose buckets and S3 directory buckets.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: my-tf-test-bucket
///   allowAccessFromAnotherAccountBucketPolicy:
///     type: aws:s3:BucketPolicy
///     name: allow_access_from_another_account
///     properties:
///       bucket: ${example.id}
///       policy: ${allowAccessFromAnotherAccount.json}
/// variables:
///   allowAccessFromAnotherAccount:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - principals:
///               - type: AWS
///                 identifiers:
///                   - '123456789012'
///             actions:
///               - s3:GetObject
///               - s3:ListBucket
///             resources:
///               - ${example.arn}
///               - ${example.arn}/*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 bucket policies using the bucket name. For example:
///
/// ```sh
/// $ pulumi import aws:s3/bucketPolicy:BucketPolicy allow_access_from_another_account my-tf-test-bucket
/// ```
pub mod bucket_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketPolicyArgs {
        /// Name of the bucket to which to apply the policy.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Text of the policy. Although this is a bucket policy rather than an IAM policy, the `aws.iam.getPolicyDocument` data source may be used, so long as it specifies a principal. For more information about building AWS IAM policy documents, see the AWS IAM Policy Document Guide. Note: Bucket policies are limited to 20 KB in size.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct BucketPolicyResult {
        /// Name of the bucket to which to apply the policy.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Text of the policy. Although this is a bucket policy rather than an IAM policy, the `aws.iam.getPolicyDocument` data source may be used, so long as it specifies a principal. For more information about building AWS IAM policy documents, see the AWS IAM Policy Document Guide. Note: Bucket policies are limited to 20 KB in size.
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BucketPolicyArgs) -> BucketPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let policy_binding = args.policy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketPolicy:BucketPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketPolicyResult {
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
        }
    }
}
