/// Provides an S3 bucket Object Lock configuration resource. For more information about Object Locking, go to [Using S3 Object Lock](https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-lock.html) in the Amazon S3 User Guide.
///
/// > This resource can be used enable Object Lock for **new** and **existing** buckets.
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ### Object Lock configuration for new or existing buckets
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("mybucket").build_struct(),
///     );
///     let exampleBucketObjectLockConfigurationV2 = bucket_object_lock_configuration_v_2::create(
///         "exampleBucketObjectLockConfigurationV2",
///         BucketObjectLockConfigurationV2Args::builder()
///             .bucket("${example.id}")
///             .rule(
///                 BucketObjectLockConfigurationV2Rule::builder()
///                     .defaultRetention(
///                         BucketObjectLockConfigurationV2RuleDefaultRetention::builder()
///                             .days(5)
///                             .mode("COMPLIANCE")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleBucketVersioningV2 = bucket_versioning_v_2::create(
///         "exampleBucketVersioningV2",
///         BucketVersioningV2Args::builder()
///             .bucket("${example.id}")
///             .versioning_configuration(
///                 BucketVersioningV2VersioningConfiguration::builder()
///                     .status("Enabled")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner`, separated by a comma (`,`). For example:
///
/// __Using `pulumi import`__, import an S3 bucket Object Lock Configuration using one of two forms. If the owner (account ID) of the source bucket is the same account used to configure the AWS Provider, import using the `bucket`. For example:
///
/// ```sh
/// $ pulumi import aws:s3/bucketObjectLockConfigurationV2:BucketObjectLockConfigurationV2 example bucket-name
/// ```
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner`, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:s3/bucketObjectLockConfigurationV2:BucketObjectLockConfigurationV2 example bucket-name,123456789012
/// ```
pub mod bucket_object_lock_configuration_v_2 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketObjectLockConfigurationV2Args {
        /// Name of the bucket.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Account ID of the expected bucket owner.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether this bucket has an Object Lock configuration enabled. Defaults to `Enabled`. Valid values: `Enabled`.
        #[builder(into, default)]
        pub object_lock_enabled: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for specifying the Object Lock rule for the specified object. See below.
        #[builder(into, default)]
        pub rule: pulumi_wasm_rust::Output<
            Option<super::super::types::s3::BucketObjectLockConfigurationV2Rule>,
        >,
        /// This argument is deprecated and no longer needed to enable Object Lock.
        /// To enable Object Lock for an existing bucket, you must first enable versioning on the bucket and then enable Object Lock. For more details on versioning, see the `aws.s3.BucketVersioningV2` resource.
        #[builder(into, default)]
        pub token: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BucketObjectLockConfigurationV2Result {
        /// Name of the bucket.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Account ID of the expected bucket owner.
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether this bucket has an Object Lock configuration enabled. Defaults to `Enabled`. Valid values: `Enabled`.
        pub object_lock_enabled: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for specifying the Object Lock rule for the specified object. See below.
        pub rule: pulumi_wasm_rust::Output<
            Option<super::super::types::s3::BucketObjectLockConfigurationV2Rule>,
        >,
        /// This argument is deprecated and no longer needed to enable Object Lock.
        /// To enable Object Lock for an existing bucket, you must first enable versioning on the bucket and then enable Object Lock. For more details on versioning, see the `aws.s3.BucketVersioningV2` resource.
        pub token: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BucketObjectLockConfigurationV2Args,
    ) -> BucketObjectLockConfigurationV2Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let expected_bucket_owner_binding = args.expected_bucket_owner.get_inner();
        let object_lock_enabled_binding = args.object_lock_enabled.get_inner();
        let rule_binding = args.rule.get_inner();
        let token_binding = args.token.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketObjectLockConfigurationV2:BucketObjectLockConfigurationV2"
                .into(),
            name: name.to_string(),
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
                    name: "objectLockEnabled".into(),
                    value: &object_lock_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "rule".into(),
                    value: &rule_binding,
                },
                register_interface::ObjectField {
                    name: "token".into(),
                    value: &token_binding,
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
                    name: "objectLockEnabled".into(),
                },
                register_interface::ResultField {
                    name: "rule".into(),
                },
                register_interface::ResultField {
                    name: "token".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketObjectLockConfigurationV2Result {
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            expected_bucket_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expectedBucketOwner").unwrap(),
            ),
            object_lock_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("objectLockEnabled").unwrap(),
            ),
            rule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rule").unwrap(),
            ),
            token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("token").unwrap(),
            ),
        }
    }
}