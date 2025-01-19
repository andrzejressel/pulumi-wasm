/// Provides a S3 bucket server-side encryption configuration resource.
///
/// > **NOTE:** Destroying an `aws.s3.BucketServerSideEncryptionConfigurationV2` resource resets the bucket to [Amazon S3 bucket default encryption](https://docs.aws.amazon.com/AmazonS3/latest/userguide/default-encryption-faq.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_server_side_encryption_configuration_v_2::create(
///         "example",
///         BucketServerSideEncryptionConfigurationV2Args::builder()
///             .bucket("${mybucket.id}")
///             .rules(
///                 vec![
///                     BucketServerSideEncryptionConfigurationV2Rule::builder()
///                     .applyServerSideEncryptionByDefault(BucketServerSideEncryptionConfigurationV2RuleApplyServerSideEncryptionByDefault::builder()
///                     .kmsMasterKeyId("${mykey.arn}").sseAlgorithm("aws:kms")
///                     .build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let mybucket = bucket_v_2::create(
///         "mybucket",
///         BucketV2Args::builder().bucket("mybucket").build_struct(),
///     );
///     let mykey = key::create(
///         "mykey",
///         KeyArgs::builder()
///             .deletion_window_in_days(10)
///             .description("This key is used to encrypt bucket objects")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// __Using `pulumi import` to import__ S3 bucket server-side encryption configuration using the `bucket` or using the `bucket` and `expected_bucket_owner` separated by a comma (`,`). For example:
///
/// If the owner (account ID) of the source bucket is the same account used to configure the AWS Provider, import using the `bucket`:
///
/// ```sh
/// $ pulumi import aws:s3/bucketServerSideEncryptionConfigurationV2:BucketServerSideEncryptionConfigurationV2 example bucket-name
/// ```
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// ```sh
/// $ pulumi import aws:s3/bucketServerSideEncryptionConfigurationV2:BucketServerSideEncryptionConfigurationV2 example bucket-name,123456789012
/// ```
pub mod bucket_server_side_encryption_configuration_v_2 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketServerSideEncryptionConfigurationV2Args {
        /// ID (name) of the bucket.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Account ID of the expected bucket owner.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of server-side encryption configuration rules. See below. Currently, only a single rule is supported.
        #[builder(into)]
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketServerSideEncryptionConfigurationV2Rule>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketServerSideEncryptionConfigurationV2Result {
        /// ID (name) of the bucket.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Account ID of the expected bucket owner.
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of server-side encryption configuration rules. See below. Currently, only a single rule is supported.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketServerSideEncryptionConfigurationV2Rule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BucketServerSideEncryptionConfigurationV2Args,
    ) -> BucketServerSideEncryptionConfigurationV2Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let expected_bucket_owner_binding = args.expected_bucket_owner.get_inner();
        let rules_binding = args.rules.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketServerSideEncryptionConfigurationV2:BucketServerSideEncryptionConfigurationV2"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "expectedBucketOwner".into(),
                    value: &expected_bucket_owner_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "expectedBucketOwner".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketServerSideEncryptionConfigurationV2Result {
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            expected_bucket_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expectedBucketOwner").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
        }
    }
}
