/// Provides a S3 bucket resource.
///
/// > This resource provides functionality for managing S3 general purpose buckets in an AWS Partition. To manage Amazon S3 Express directory buckets, use the `aws_directory_bucket` resource. To manage [S3 on Outposts](https://docs.aws.amazon.com/AmazonS3/latest/dev/S3onOutposts.html), use the `aws.s3control.Bucket` resource.
///
/// > Object Lock can be enabled by using the `object_lock_enable` attribute or by using the `aws.s3.BucketObjectLockConfigurationV2` resource. Please note, that by using the resource, Object Lock can be enabled/disabled without destroying and recreating the bucket.
///
/// ## Example Usage
///
/// ### Private Bucket With Tags
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: my-tf-test-bucket
///       tags:
///         Name: My bucket
///         Environment: Dev
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 bucket using the `bucket`. For example:
///
/// ```sh
/// $ pulumi import aws:s3/bucketV2:BucketV2 bucket bucket-name
/// ```
pub mod bucket_v_2 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketV2Args {
        /// Sets the accelerate configuration of an existing bucket. Can be `Enabled` or `Suspended`. Cannot be used in `cn-north-1` or `us-gov-west-1`. This provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketAccelerateConfigurationV2` instead.
        #[builder(into, default)]
        pub acceleration_status: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The [canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl) to apply. Valid values are `private`, `public-read`, `public-read-write`, `aws-exec-read`, `authenticated-read`, and `log-delivery-write`. Defaults to `private`.  Conflicts with `grant`. The provider will only perform drift detection if a configuration value is provided. Use the resource `aws.s3.BucketAclV2` instead.
        #[builder(into, default)]
        pub acl: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the bucket. If omitted, the provider will assign a random, unique name. Must be lowercase and less than or equal to 63 characters in length. A full list of bucket naming rules [may be found here](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html). The name must not be in the format `[bucket_name]--[azid]--x-s3`. Use the `aws.s3.DirectoryBucket` resource to manage S3 Express buckets.
        #[builder(into, default)]
        pub bucket: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Creates a unique bucket name beginning with the specified prefix. Conflicts with `bucket`. Must be lowercase and less than or equal to 37 characters in length. A full list of bucket naming rules [may be found here](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html).
        #[builder(into, default)]
        pub bucket_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Rule of [Cross-Origin Resource Sharing](https://docs.aws.amazon.com/AmazonS3/latest/dev/cors.html). See CORS rule below for details. This provider will only perform drift detection if a configuration value is provided. Use the resource `aws.s3.BucketCorsConfigurationV2` instead.
        #[builder(into, default)]
        pub cors_rules: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketV2CorsRule>>,
        >,
        /// Boolean that indicates all objects (including any [locked objects](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html)) should be deleted from the bucket *when the bucket is destroyed* so that the bucket can be destroyed without error. These objects are *not* recoverable. This only deletes objects when the bucket is destroyed, *not* when setting this parameter to `true`. Once this parameter is set to `true`, there must be a successful `pulumi up` run before a destroy is required to update this value in the resource state. Without a successful `pulumi up` after this parameter is set, this flag will have no effect. If setting this field in the same operation that would require replacing the bucket or destroying the bucket, this flag will not work. Additionally when importing a bucket, a successful `pulumi up` is required to set this value in state before it will take effect on a destroy operation.
        #[builder(into, default)]
        pub force_destroy: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// An [ACL policy grant](https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#sample-acl). See Grant below for details. Conflicts with `acl`. The provider will only perform drift detection if a configuration value is provided. Use the resource `aws.s3.BucketAclV2` instead.
        #[builder(into, default)]
        pub grants: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketV2Grant>>,
        >,
        /// Configuration of [object lifecycle management](http://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html). See Lifecycle Rule below for details. The provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketLifecycleConfigurationV2` instead.
        #[builder(into, default)]
        pub lifecycle_rules: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketV2LifecycleRule>>,
        >,
        /// Configuration of [S3 bucket logging](https://docs.aws.amazon.com/AmazonS3/latest/UG/ManagingBucketLogging.html) parameters. See Logging below for details. The provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketLoggingV2` instead.
        #[builder(into, default)]
        pub loggings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketV2Logging>>,
        >,
        /// Configuration of [S3 object locking](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html). See Object Lock Configuration below for details.
        /// The provider wil only perform drift detection if a configuration value is provided.
        /// Use the `object_lock_enabled` parameter and the resource `aws.s3.BucketObjectLockConfigurationV2` instead.
        #[builder(into, default)]
        pub object_lock_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::s3::BucketV2ObjectLockConfiguration>,
        >,
        /// Indicates whether this bucket has an Object Lock configuration enabled. Valid values are `true` or `false`. This argument is not supported in all regions or partitions.
        #[builder(into, default)]
        pub object_lock_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Valid [bucket policy](https://docs.aws.amazon.com/AmazonS3/latest/dev/example-bucket-policies.html) JSON document. Note that if the policy document is not specific enough (but still valid), this provider may view the policy as constantly changing. In this case, please make sure you use the verbose/specific version of the policy. For more information about building AWS IAM policy documents with this provider, see the AWS IAM Policy Document Guide.
        /// The provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketPolicy` instead.
        #[builder(into, default)]
        pub policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration of [replication configuration](http://docs.aws.amazon.com/AmazonS3/latest/dev/crr.html). See Replication Configuration below for details. The provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketReplicationConfig` instead.
        #[builder(into, default)]
        pub replication_configurations: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketV2ReplicationConfiguration>>,
        >,
        /// Specifies who should bear the cost of Amazon S3 data transfer.
        /// Can be either `BucketOwner` or `Requester`. By default, the owner of the S3 bucket would incur the costs of any data transfer.
        /// See [Requester Pays Buckets](http://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html) developer guide for more information.
        /// The provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketRequestPaymentConfigurationV2` instead.
        #[builder(into, default)]
        pub request_payer: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration of [server-side encryption configuration](http://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html). See Server Side Encryption Configuration below for details.
        /// The provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketServerSideEncryptionConfigurationV2` instead.
        #[builder(into, default)]
        pub server_side_encryption_configurations: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::types::s3::BucketV2ServerSideEncryptionConfiguration>,
            >,
        >,
        /// Map of tags to assign to the bucket. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// The following arguments are deprecated, and will be removed in a future major version:
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration of the [S3 bucket versioning state](https://docs.aws.amazon.com/AmazonS3/latest/dev/Versioning.html). See Versioning below for details. The provider will only perform drift detection if a configuration value is provided. Use the resource `aws.s3.BucketVersioningV2` instead.
        #[builder(into, default)]
        pub versionings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketV2Versioning>>,
        >,
        /// Configuration of the [S3 bucket website](https://docs.aws.amazon.com/AmazonS3/latest/userguide/WebsiteHosting.html). See Website below for details. The provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketWebsiteConfigurationV2` instead.
        #[builder(into, default)]
        pub websites: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketV2Website>>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketV2Result {
        /// Sets the accelerate configuration of an existing bucket. Can be `Enabled` or `Suspended`. Cannot be used in `cn-north-1` or `us-gov-west-1`. This provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketAccelerateConfigurationV2` instead.
        pub acceleration_status: pulumi_wasm_rust::Output<String>,
        /// The [canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl) to apply. Valid values are `private`, `public-read`, `public-read-write`, `aws-exec-read`, `authenticated-read`, and `log-delivery-write`. Defaults to `private`.  Conflicts with `grant`. The provider will only perform drift detection if a configuration value is provided. Use the resource `aws.s3.BucketAclV2` instead.
        pub acl: pulumi_wasm_rust::Output<String>,
        /// ARN of the bucket. Will be of format `arn:aws:s3:::bucketname`.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of the bucket. If omitted, the provider will assign a random, unique name. Must be lowercase and less than or equal to 63 characters in length. A full list of bucket naming rules [may be found here](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html). The name must not be in the format `[bucket_name]--[azid]--x-s3`. Use the `aws.s3.DirectoryBucket` resource to manage S3 Express buckets.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Bucket domain name. Will be of format `bucketname.s3.amazonaws.com`.
        pub bucket_domain_name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique bucket name beginning with the specified prefix. Conflicts with `bucket`. Must be lowercase and less than or equal to 37 characters in length. A full list of bucket naming rules [may be found here](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html).
        pub bucket_prefix: pulumi_wasm_rust::Output<String>,
        /// The bucket region-specific domain name. The bucket domain name including the region name. Please refer to the [S3 endpoints reference](https://docs.aws.amazon.com/general/latest/gr/s3.html#s3_region) for format. Note: AWS CloudFront allows specifying an S3 region-specific endpoint when creating an S3 origin. This will prevent redirect issues from CloudFront to the S3 Origin URL. For more information, see the [Virtual Hosted-Style Requests for Other Regions](https://docs.aws.amazon.com/AmazonS3/latest/userguide/VirtualHosting.html#deprecated-global-endpoint) section in the AWS S3 User Guide.
        pub bucket_regional_domain_name: pulumi_wasm_rust::Output<String>,
        /// Rule of [Cross-Origin Resource Sharing](https://docs.aws.amazon.com/AmazonS3/latest/dev/cors.html). See CORS rule below for details. This provider will only perform drift detection if a configuration value is provided. Use the resource `aws.s3.BucketCorsConfigurationV2` instead.
        pub cors_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketV2CorsRule>,
        >,
        /// Boolean that indicates all objects (including any [locked objects](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html)) should be deleted from the bucket *when the bucket is destroyed* so that the bucket can be destroyed without error. These objects are *not* recoverable. This only deletes objects when the bucket is destroyed, *not* when setting this parameter to `true`. Once this parameter is set to `true`, there must be a successful `pulumi up` run before a destroy is required to update this value in the resource state. Without a successful `pulumi up` after this parameter is set, this flag will have no effect. If setting this field in the same operation that would require replacing the bucket or destroying the bucket, this flag will not work. Additionally when importing a bucket, a successful `pulumi up` is required to set this value in state before it will take effect on a destroy operation.
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// An [ACL policy grant](https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#sample-acl). See Grant below for details. Conflicts with `acl`. The provider will only perform drift detection if a configuration value is provided. Use the resource `aws.s3.BucketAclV2` instead.
        pub grants: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketV2Grant>,
        >,
        /// [Route 53 Hosted Zone ID](https://docs.aws.amazon.com/general/latest/gr/rande.html#s3_website_region_endpoints) for this bucket's region.
        pub hosted_zone_id: pulumi_wasm_rust::Output<String>,
        /// Configuration of [object lifecycle management](http://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html). See Lifecycle Rule below for details. The provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketLifecycleConfigurationV2` instead.
        pub lifecycle_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketV2LifecycleRule>,
        >,
        /// Configuration of [S3 bucket logging](https://docs.aws.amazon.com/AmazonS3/latest/UG/ManagingBucketLogging.html) parameters. See Logging below for details. The provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketLoggingV2` instead.
        pub loggings: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketV2Logging>,
        >,
        /// Configuration of [S3 object locking](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html). See Object Lock Configuration below for details.
        /// The provider wil only perform drift detection if a configuration value is provided.
        /// Use the `object_lock_enabled` parameter and the resource `aws.s3.BucketObjectLockConfigurationV2` instead.
        pub object_lock_configuration: pulumi_wasm_rust::Output<
            super::super::types::s3::BucketV2ObjectLockConfiguration,
        >,
        /// Indicates whether this bucket has an Object Lock configuration enabled. Valid values are `true` or `false`. This argument is not supported in all regions or partitions.
        pub object_lock_enabled: pulumi_wasm_rust::Output<bool>,
        /// Valid [bucket policy](https://docs.aws.amazon.com/AmazonS3/latest/dev/example-bucket-policies.html) JSON document. Note that if the policy document is not specific enough (but still valid), this provider may view the policy as constantly changing. In this case, please make sure you use the verbose/specific version of the policy. For more information about building AWS IAM policy documents with this provider, see the AWS IAM Policy Document Guide.
        /// The provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketPolicy` instead.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// AWS region this bucket resides in.
        pub region: pulumi_wasm_rust::Output<String>,
        /// Configuration of [replication configuration](http://docs.aws.amazon.com/AmazonS3/latest/dev/crr.html). See Replication Configuration below for details. The provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketReplicationConfig` instead.
        pub replication_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketV2ReplicationConfiguration>,
        >,
        /// Specifies who should bear the cost of Amazon S3 data transfer.
        /// Can be either `BucketOwner` or `Requester`. By default, the owner of the S3 bucket would incur the costs of any data transfer.
        /// See [Requester Pays Buckets](http://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html) developer guide for more information.
        /// The provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketRequestPaymentConfigurationV2` instead.
        pub request_payer: pulumi_wasm_rust::Output<String>,
        /// Configuration of [server-side encryption configuration](http://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html). See Server Side Encryption Configuration below for details.
        /// The provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketServerSideEncryptionConfigurationV2` instead.
        pub server_side_encryption_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketV2ServerSideEncryptionConfiguration>,
        >,
        /// Map of tags to assign to the bucket. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// The following arguments are deprecated, and will be removed in a future major version:
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration of the [S3 bucket versioning state](https://docs.aws.amazon.com/AmazonS3/latest/dev/Versioning.html). See Versioning below for details. The provider will only perform drift detection if a configuration value is provided. Use the resource `aws.s3.BucketVersioningV2` instead.
        pub versionings: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketV2Versioning>,
        >,
        /// (**Deprecated**) Domain of the website endpoint, if the bucket is configured with a website. If not, this will be an empty string. This is used to create Route 53 alias records. Use the resource `aws.s3.BucketWebsiteConfigurationV2` instead.
        pub website_domain: pulumi_wasm_rust::Output<String>,
        /// (**Deprecated**) Website endpoint, if the bucket is configured with a website. If not, this will be an empty string. Use the resource `aws.s3.BucketWebsiteConfigurationV2` instead.
        pub website_endpoint: pulumi_wasm_rust::Output<String>,
        /// Configuration of the [S3 bucket website](https://docs.aws.amazon.com/AmazonS3/latest/userguide/WebsiteHosting.html). See Website below for details. The provider will only perform drift detection if a configuration value is provided.
        /// Use the resource `aws.s3.BucketWebsiteConfigurationV2` instead.
        pub websites: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketV2Website>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BucketV2Args,
    ) -> BucketV2Result {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let acceleration_status_binding = args
            .acceleration_status
            .get_output(context)
            .get_inner();
        let acl_binding = args.acl.get_output(context).get_inner();
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let bucket_prefix_binding = args.bucket_prefix.get_output(context).get_inner();
        let cors_rules_binding = args.cors_rules.get_output(context).get_inner();
        let force_destroy_binding = args.force_destroy.get_output(context).get_inner();
        let grants_binding = args.grants.get_output(context).get_inner();
        let lifecycle_rules_binding = args
            .lifecycle_rules
            .get_output(context)
            .get_inner();
        let loggings_binding = args.loggings.get_output(context).get_inner();
        let object_lock_configuration_binding = args
            .object_lock_configuration
            .get_output(context)
            .get_inner();
        let object_lock_enabled_binding = args
            .object_lock_enabled
            .get_output(context)
            .get_inner();
        let policy_binding = args.policy.get_output(context).get_inner();
        let replication_configurations_binding = args
            .replication_configurations
            .get_output(context)
            .get_inner();
        let request_payer_binding = args.request_payer.get_output(context).get_inner();
        let server_side_encryption_configurations_binding = args
            .server_side_encryption_configurations
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let versionings_binding = args.versionings.get_output(context).get_inner();
        let websites_binding = args.websites.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketV2:BucketV2".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accelerationStatus".into(),
                    value: &acceleration_status_binding,
                },
                register_interface::ObjectField {
                    name: "acl".into(),
                    value: &acl_binding,
                },
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "bucketPrefix".into(),
                    value: &bucket_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "corsRules".into(),
                    value: &cors_rules_binding,
                },
                register_interface::ObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "grants".into(),
                    value: &grants_binding,
                },
                register_interface::ObjectField {
                    name: "lifecycleRules".into(),
                    value: &lifecycle_rules_binding,
                },
                register_interface::ObjectField {
                    name: "loggings".into(),
                    value: &loggings_binding,
                },
                register_interface::ObjectField {
                    name: "objectLockConfiguration".into(),
                    value: &object_lock_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "objectLockEnabled".into(),
                    value: &object_lock_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "replicationConfigurations".into(),
                    value: &replication_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "requestPayer".into(),
                    value: &request_payer_binding,
                },
                register_interface::ObjectField {
                    name: "serverSideEncryptionConfigurations".into(),
                    value: &server_side_encryption_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "versionings".into(),
                    value: &versionings_binding,
                },
                register_interface::ObjectField {
                    name: "websites".into(),
                    value: &websites_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BucketV2Result {
            acceleration_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accelerationStatus"),
            ),
            acl: pulumi_wasm_rust::__private::into_domain(o.extract_field("acl")),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            bucket: pulumi_wasm_rust::__private::into_domain(o.extract_field("bucket")),
            bucket_domain_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bucketDomainName"),
            ),
            bucket_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bucketPrefix"),
            ),
            bucket_regional_domain_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bucketRegionalDomainName"),
            ),
            cors_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("corsRules"),
            ),
            force_destroy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forceDestroy"),
            ),
            grants: pulumi_wasm_rust::__private::into_domain(o.extract_field("grants")),
            hosted_zone_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostedZoneId"),
            ),
            lifecycle_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lifecycleRules"),
            ),
            loggings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loggings"),
            ),
            object_lock_configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("objectLockConfiguration"),
            ),
            object_lock_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("objectLockEnabled"),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(o.extract_field("policy")),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            replication_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("replicationConfigurations"),
            ),
            request_payer: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requestPayer"),
            ),
            server_side_encryption_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serverSideEncryptionConfigurations"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            versionings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versionings"),
            ),
            website_domain: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("websiteDomain"),
            ),
            website_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("websiteEndpoint"),
            ),
            websites: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("websites"),
            ),
        }
    }
}
