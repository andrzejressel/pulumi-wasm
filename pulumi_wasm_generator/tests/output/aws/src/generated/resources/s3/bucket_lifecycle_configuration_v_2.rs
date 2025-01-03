/// Provides an independent configuration resource for S3 bucket [lifecycle configuration](https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-lifecycle-mgmt.html).
///
/// An S3 Lifecycle configuration consists of one or more Lifecycle rules. Each rule consists of the following:
///
/// * Rule metadata (`id` and `status`)
/// * Filter identifying objects to which the rule applies
/// * One or more transition or expiration actions
///
/// For more information see the Amazon S3 User Guide on [`Lifecycle Configuration Elements`](https://docs.aws.amazon.com/AmazonS3/latest/userguide/intro-lifecycle-rules.html).
///
/// > S3 Buckets only support a single lifecycle configuration. Declaring multiple `aws.s3.BucketLifecycleConfigurationV2` resources to the same S3 Bucket will cause a perpetual difference in configuration.
///
/// > Lifecycle configurations may take some time to fully propagate to all AWS S3 systems.
/// Running Pulumi operations shortly after creating a lifecycle configuration may result in changes that affect configuration idempotence.
/// See the Amazon S3 User Guide on [setting lifecycle configuration on a bucket](https://docs.aws.amazon.com/AmazonS3/latest/userguide/how-to-set-lifecycle-configuration-intro.html).
///
/// ## Example Usage
///
/// ### With neither a filter nor prefix specified
///
/// The Lifecycle rule applies to a subset of objects based on the key name prefix (`""`).
///
/// This configuration is intended to replicate the default behavior of the `lifecycle_rule`
/// parameter in the AWS Provider `aws.s3.BucketV2` resource prior to `v4.0`.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_lifecycle_configuration_v_2::create(
///         "example",
///         BucketLifecycleConfigurationV2Args::builder()
///             .bucket("${bucket.id}")
///             .rules(
///                 vec![
///                     BucketLifecycleConfigurationV2Rule::builder().id("rule-1")
///                     .status("Enabled").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Specifying an empty filter
///
/// The Lifecycle rule applies to all objects in the bucket.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_lifecycle_configuration_v_2::create(
///         "example",
///         BucketLifecycleConfigurationV2Args::builder()
///             .bucket("${bucket.id}")
///             .rules(
///                 vec![
///                     BucketLifecycleConfigurationV2Rule::builder()
///                     .filter(BucketLifecycleConfigurationV2RuleFilter::builder()
///                     .build_struct()).id("rule-1").status("Enabled").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Specifying a filter using key prefixes
///
/// The Lifecycle rule applies to a subset of objects based on the key name prefix (`logs/`).
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_lifecycle_configuration_v_2::create(
///         "example",
///         BucketLifecycleConfigurationV2Args::builder()
///             .bucket("${bucket.id}")
///             .rules(
///                 vec![
///                     BucketLifecycleConfigurationV2Rule::builder()
///                     .filter(BucketLifecycleConfigurationV2RuleFilter::builder()
///                     .prefix("logs/").build_struct()).id("rule-1").status("Enabled")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// If you want to apply a Lifecycle action to a subset of objects based on different key name prefixes, specify separate rules.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_lifecycle_configuration_v_2::create(
///         "example",
///         BucketLifecycleConfigurationV2Args::builder()
///             .bucket("${bucket.id}")
///             .rules(
///                 vec![
///                     BucketLifecycleConfigurationV2Rule::builder()
///                     .filter(BucketLifecycleConfigurationV2RuleFilter::builder()
///                     .prefix("logs/").build_struct()).id("rule-1").status("Enabled")
///                     .build_struct(), BucketLifecycleConfigurationV2Rule::builder()
///                     .filter(BucketLifecycleConfigurationV2RuleFilter::builder()
///                     .prefix("tmp/").build_struct()).id("rule-2").status("Enabled")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Specifying a filter based on an object tag
///
/// The Lifecycle rule specifies a filter based on a tag key and value. The rule then applies only to a subset of objects with the specific tag.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_lifecycle_configuration_v_2::create(
///         "example",
///         BucketLifecycleConfigurationV2Args::builder()
///             .bucket("${bucket.id}")
///             .rules(
///                 vec![
///                     BucketLifecycleConfigurationV2Rule::builder()
///                     .filter(BucketLifecycleConfigurationV2RuleFilter::builder()
///                     .tag(BucketLifecycleConfigurationV2RuleFilterTag::builder()
///                     .key("Name").value("Staging").build_struct()).build_struct())
///                     .id("rule-1").status("Enabled").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Specifying a filter based on multiple tags
///
/// The Lifecycle rule directs Amazon S3 to perform lifecycle actions on objects with two tags (with the specific tag keys and values). Notice `tags` is wrapped in the `and` configuration block.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketLifecycleConfigurationV2
///     properties:
///       bucket: ${bucket.id}
///       rules:
///         - id: rule-1
///           filter:
///             and:
///               tags:
///                 Key1: Value1
///                 Key2: Value2
///           status: Enabled
/// ```
///
/// ### Specifying a filter based on both prefix and one or more tags
///
/// The Lifecycle rule directs Amazon S3 to perform lifecycle actions on objects with the specified prefix and two tags (with the specific tag keys and values). Notice both `prefix` and `tags` are wrapped in the `and` configuration block.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketLifecycleConfigurationV2
///     properties:
///       bucket: ${bucket.id}
///       rules:
///         - id: rule-1
///           filter:
///             and:
///               prefix: logs/
///               tags:
///                 Key1: Value1
///                 Key2: Value2
///           status: Enabled
/// ```
///
/// ### Specifying a filter based on object size
///
/// Object size values are in bytes. Maximum filter size is 5TB. Amazon S3 applies a default behavior to your Lifecycle configuration that prevents objects smaller than 128 KB from being transitioned to any storage class. You can allow smaller objects to transition by adding a minimum size (`object_size_greater_than`) or a maximum size (`object_size_less_than`) filter that specifies a smaller size to the configuration. This example allows any object smaller than 128 KB to transition to the S3 Glacier Instant Retrieval storage class:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketLifecycleConfigurationV2
///     properties:
///       bucket: ${bucket.id}
///       rules:
///         - id: Allow small object transitions
///           filter:
///             objectSizeGreaterThan: 1
///           status: Enabled
///           transitions:
///             - days: 365
///               storageClass: GLACIER_IR
/// ```
///
/// ### Specifying a filter based on object size range and prefix
///
/// The `object_size_greater_than` must be less than the `object_size_less_than`. Notice both the object size range and prefix are wrapped in the `and` configuration block.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_lifecycle_configuration_v_2::create(
///         "example",
///         BucketLifecycleConfigurationV2Args::builder()
///             .bucket("${bucket.id}")
///             .rules(
///                 vec![
///                     BucketLifecycleConfigurationV2Rule::builder()
///                     .filter(BucketLifecycleConfigurationV2RuleFilter::builder()
///                     .and(BucketLifecycleConfigurationV2RuleFilterAnd::builder()
///                     .objectSizeGreaterThan(500).objectSizeLessThan(64000).prefix("logs/")
///                     .build_struct()).build_struct()).id("rule-1").status("Enabled")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Creating a Lifecycle Configuration for a bucket with versioning
///
/// ```yaml
/// resources:
///   bucket:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: my-bucket
///   bucketAcl:
///     type: aws:s3:BucketAclV2
///     name: bucket_acl
///     properties:
///       bucket: ${bucket.id}
///       acl: private
///   bucket-config:
///     type: aws:s3:BucketLifecycleConfigurationV2
///     properties:
///       bucket: ${bucket.id}
///       rules:
///         - id: log
///           expiration:
///             days: 90
///           filter:
///             and:
///               prefix: log/
///               tags:
///                 rule: log
///                 autoclean: 'true'
///           status: Enabled
///           transitions:
///             - days: 30
///               storageClass: STANDARD_IA
///             - days: 60
///               storageClass: GLACIER
///         - id: tmp
///           filter:
///             prefix: tmp/
///           expiration:
///             date: 2023-01-13T00:00:00Z
///           status: Enabled
///   versioningBucket:
///     type: aws:s3:BucketV2
///     name: versioning_bucket
///     properties:
///       bucket: my-versioning-bucket
///   versioningBucketAcl:
///     type: aws:s3:BucketAclV2
///     name: versioning_bucket_acl
///     properties:
///       bucket: ${versioningBucket.id}
///       acl: private
///   versioning:
///     type: aws:s3:BucketVersioningV2
///     properties:
///       bucket: ${versioningBucket.id}
///       versioningConfiguration:
///         status: Enabled
///   versioning-bucket-config:
///     type: aws:s3:BucketLifecycleConfigurationV2
///     properties:
///       bucket: ${versioningBucket.id}
///       rules:
///         - id: config
///           filter:
///             prefix: config/
///           noncurrentVersionExpiration:
///             noncurrentDays: 90
///           noncurrentVersionTransitions:
///             - noncurrentDays: 30
///               storageClass: STANDARD_IA
///             - noncurrentDays: 60
///               storageClass: GLACIER
///           status: Enabled
///     options:
///       dependsOn:
///         - ${versioning}
/// ```
///
/// ## Import
///
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// Using `pulumi import`, import an S3 bucket lifecycle configuration using the `bucket` or the `bucket` and `expected_bucket_owner` separated by a comma (`,`). For example:
///
/// If the owner (account ID) of the source bucket is the same account used to configure the AWS Provider, import using the `bucket`:
///
/// ```sh
/// $ pulumi import aws:s3/bucketLifecycleConfigurationV2:BucketLifecycleConfigurationV2 example bucket-name
/// ```
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// ```sh
/// $ pulumi import aws:s3/bucketLifecycleConfigurationV2:BucketLifecycleConfigurationV2 example bucket-name,123456789012
/// ```
pub mod bucket_lifecycle_configuration_v_2 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketLifecycleConfigurationV2Args {
        /// Name of the source S3 bucket you want Amazon S3 to monitor.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Account ID of the expected bucket owner. If the bucket is owned by a different account, the request will fail with an HTTP 403 (Access Denied) error.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// List of configuration blocks describing the rules managing the replication. See below.
        #[builder(into)]
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketLifecycleConfigurationV2Rule>,
        >,
        /// The default minimum object size behavior applied to the lifecycle configuration. Valid values: `all_storage_classes_128K` (default), `varies_by_storage_class`. To customize the minimum object size for any transition you can add a `filter` that specifies a custom `object_size_greater_than` or `object_size_less_than` value. Custom filters always take precedence over the default transition behavior.
        #[builder(into, default)]
        pub transition_default_minimum_object_size: pulumi_wasm_rust::Output<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketLifecycleConfigurationV2Result {
        /// Name of the source S3 bucket you want Amazon S3 to monitor.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Account ID of the expected bucket owner. If the bucket is owned by a different account, the request will fail with an HTTP 403 (Access Denied) error.
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// List of configuration blocks describing the rules managing the replication. See below.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketLifecycleConfigurationV2Rule>,
        >,
        /// The default minimum object size behavior applied to the lifecycle configuration. Valid values: `all_storage_classes_128K` (default), `varies_by_storage_class`. To customize the minimum object size for any transition you can add a `filter` that specifies a custom `object_size_greater_than` or `object_size_less_than` value. Custom filters always take precedence over the default transition behavior.
        pub transition_default_minimum_object_size: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BucketLifecycleConfigurationV2Args,
    ) -> BucketLifecycleConfigurationV2Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let expected_bucket_owner_binding = args.expected_bucket_owner.get_inner();
        let rules_binding = args.rules.get_inner();
        let transition_default_minimum_object_size_binding = args
            .transition_default_minimum_object_size
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketLifecycleConfigurationV2:BucketLifecycleConfigurationV2"
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
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "transitionDefaultMinimumObjectSize".into(),
                    value: &transition_default_minimum_object_size_binding,
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
                register_interface::ResultField {
                    name: "transitionDefaultMinimumObjectSize".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketLifecycleConfigurationV2Result {
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            expected_bucket_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expectedBucketOwner").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
            transition_default_minimum_object_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitionDefaultMinimumObjectSize").unwrap(),
            ),
        }
    }
}
