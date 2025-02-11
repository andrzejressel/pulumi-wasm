/// Provides a S3 bucket resource.
///
/// > **NOTE:** Please use [aws.s3.BucketV2](https://www.pulumi.com/registry/packages/aws/api-docs/s3/bucketv2) instead.
/// This resource is maintained for backwards compatibility only. Please see [BucketV2 Migration
/// Guide](https://www.pulumi.com/registry/packages/aws/how-to-guides/bucketv2-migration/) for instructions on migrating
/// existing Bucket resources to BucketV2.
///
/// ## Example Usage
///
/// ### Private Bucket w/ Tags
///
/// ```yaml
/// resources:
///   b:
///     type: aws:s3:Bucket
///     properties:
///       bucket: my-tf-test-bucket
///       acl: private
///       tags:
///         Name: My bucket
///         Environment: Dev
/// ```
///
/// ### Static Website Hosting
///
/// ```yaml
/// resources:
///   b:
///     type: aws:s3:Bucket
///     properties:
///       bucket: s3-website-test.mydomain.com
///       acl: public-read
///       policy:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: policy.json
///           return: result
///       website:
///         indexDocument: index.html
///         errorDocument: error.html
///         routingRules: |
///           [{
///               "Condition": {
///                   "KeyPrefixEquals": "docs/"
///               },
///               "Redirect": {
///                   "ReplaceKeyPrefixWith": "documents/"
///               }
///           }]
/// ```
///
/// ### Using CORS
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let b = bucket::create(
///         "b",
///         BucketArgs::builder()
///             .acl("public-read")
///             .bucket("s3-website-test.mydomain.com")
///             .cors_rules(
///                 vec![
///                     BucketCorsRule::builder().allowedHeaders(vec!["*",])
///                     .allowedMethods(vec!["PUT", "POST",])
///                     .allowedOrigins(vec!["https://s3-website-test.mydomain.com",])
///                     .exposeHeaders(vec!["ETag",]).maxAgeSeconds(3000).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Using versioning
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let b = bucket::create(
///         "b",
///         BucketArgs::builder()
///             .acl("private")
///             .bucket("my-tf-test-bucket")
///             .versioning(BucketVersioning::builder().enabled(true).build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Enable Logging
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let b = bucket::create(
///         "b",
///         BucketArgs::builder()
///             .acl("private")
///             .bucket("my-tf-test-bucket")
///             .loggings(
///                 vec![
///                     BucketLogging::builder().targetBucket("${logBucket.id}")
///                     .targetPrefix("log/").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let logBucket = bucket::create(
///         "logBucket",
///         BucketArgs::builder()
///             .acl("log-delivery-write")
///             .bucket("my-tf-log-bucket")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Using object lifecycle
///
/// ```yaml
/// resources:
///   bucket:
///     type: aws:s3:Bucket
///     properties:
///       bucket: my-bucket
///       acl: private
///       lifecycleRules:
///         - id: log
///           enabled: true
///           prefix: log/
///           tags:
///             rule: log
///             autoclean: 'true'
///           transitions:
///             - days: 30
///               storageClass: STANDARD_IA
///             - days: 60
///               storageClass: GLACIER
///           expiration:
///             days: 90
///         - id: tmp
///           prefix: tmp/
///           enabled: true
///           expiration:
///             date: 2016-01-12
///   versioningBucket:
///     type: aws:s3:Bucket
///     name: versioning_bucket
///     properties:
///       bucket: my-versioning-bucket
///       acl: private
///       versioning:
///         enabled: true
///       lifecycleRules:
///         - prefix: config/
///           enabled: true
///           noncurrentVersionTransitions:
///             - days: 30
///               storageClass: STANDARD_IA
///             - days: 60
///               storageClass: GLACIER
///           noncurrentVersionExpiration:
///             days: 90
/// ```
///
/// ### Using replication configuration
///
/// > **NOTE:** See the `aws.s3.BucketReplicationConfig` resource to support bi-directional replication configuration and additional features.
///
/// ```yaml
/// resources:
///   replication:
///     type: aws:iam:Role
///     properties:
///       name: tf-iam-role-replication-12345
///       assumeRolePolicy: |
///         {
///           "Version": "2012-10-17",
///           "Statement": [
///             {
///               "Action": "sts:AssumeRole",
///               "Principal": {
///                 "Service": "s3.amazonaws.com"
///               },
///               "Effect": "Allow",
///               "Sid": ""
///             }
///           ]
///         }
///   replicationPolicy:
///     type: aws:iam:Policy
///     name: replication
///     properties:
///       name: tf-iam-role-policy-replication-12345
///       policy: |
///         {
///           "Version": "2012-10-17",
///           "Statement": [
///             {
///               "Action": [
///                 "s3:GetReplicationConfiguration",
///                 "s3:ListBucket"
///               ],
///               "Effect": "Allow",
///               "Resource": [
///                 "${source.arn}"
///               ]
///             },
///             {
///               "Action": [
///                 "s3:GetObjectVersionForReplication",
///                 "s3:GetObjectVersionAcl",
///                  "s3:GetObjectVersionTagging"
///               ],
///               "Effect": "Allow",
///               "Resource": [
///                 "${source.arn}/*"
///               ]
///             },
///             {
///               "Action": [
///                 "s3:ReplicateObject",
///                 "s3:ReplicateDelete",
///                 "s3:ReplicateTags"
///               ],
///               "Effect": "Allow",
///               "Resource": "${destination.arn}/*"
///             }
///           ]
///         }
///   replicationRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: replication
///     properties:
///       role: ${replication.name}
///       policyArn: ${replicationPolicy.arn}
///   destination:
///     type: aws:s3:Bucket
///     properties:
///       bucket: tf-test-bucket-destination-12345
///       versioning:
///         enabled: true
///   source:
///     type: aws:s3:Bucket
///     properties:
///       bucket: tf-test-bucket-source-12345
///       acl: private
///       versioning:
///         enabled: true
///       replicationConfiguration:
///         role: ${replication.arn}
///         rules:
///           - id: foobar
///             status: Enabled
///             filter:
///               tags: {}
///             destination:
///               bucket: ${destination.arn}
///               storageClass: STANDARD
///               replicationTime:
///                 status: Enabled
///                 minutes: 15
///               metrics:
///                 status: Enabled
///                 minutes: 15
/// ```
///
/// ### Enable Default Server Side Encryption
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let mybucket = bucket::create(
///         "mybucket",
///         BucketArgs::builder()
///             .bucket("mybucket")
///             .server_side_encryption_configuration(
///                 BucketServerSideEncryptionConfiguration::builder()
///                     .rule(
///                         BucketServerSideEncryptionConfigurationRule::builder()
///                             .applyServerSideEncryptionByDefault(
///                                 BucketServerSideEncryptionConfigurationRuleApplyServerSideEncryptionByDefault::builder()
///                                     .kmsMasterKeyId("${mykey.arn}")
///                                     .sseAlgorithm("aws:kms")
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
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
/// ### Using ACL policy grants
///
/// ```yaml
/// resources:
///   bucket:
///     type: aws:s3:Bucket
///     properties:
///       bucket: mybucket
///       grants:
///         - id: ${currentUser.id}
///           type: CanonicalUser
///           permissions:
///             - FULL_CONTROL
///         - type: Group
///           permissions:
///             - READ_ACP
///             - WRITE
///           uri: http://acs.amazonaws.com/groups/s3/LogDelivery
/// variables:
///   currentUser:
///     fn::invoke:
///       function: aws:s3:getCanonicalUserId
///       arguments: {}
/// ```
///
/// ## Import
///
/// S3 bucket can be imported using the `bucket`, e.g.,
///
/// ```sh
/// $ pulumi import aws:s3/bucket:Bucket bucket bucket-name
/// ```
/// The `policy` argument is not imported and will be deprecated in a future version of the provider. Use the `aws_s3_bucket_policy` resource to manage the S3 Bucket Policy instead.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketArgs {
        /// Sets the accelerate configuration of an existing bucket. Can be `Enabled` or `Suspended`.
        #[builder(into, default)]
        pub acceleration_status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The [canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl) to apply. Valid values are `private`, `public-read`, `public-read-write`, `aws-exec-read`, `authenticated-read`, and `log-delivery-write`. Defaults to `private`.  Conflicts with `grant`.
        #[builder(into, default)]
        pub acl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the bucket. Will be of format `arn:aws:s3:::bucketname`.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the bucket. If omitted, this provider will assign a random, unique name. Must be lowercase and less than or equal to 63 characters in length. A full list of bucket naming rules [may be found here](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html).
        #[builder(into, default)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique bucket name beginning with the specified prefix. Conflicts with `bucket`. Must be lowercase and less than or equal to 37 characters in length. A full list of bucket naming rules [may be found here](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html).
        #[builder(into, default)]
        pub bucket_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A rule of [Cross-Origin Resource Sharing](https://docs.aws.amazon.com/AmazonS3/latest/dev/cors.html) (documented below).
        #[builder(into, default)]
        pub cors_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketCorsRule>>,
        >,
        /// A boolean that indicates all objects (including any [locked objects](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html)) should be deleted from the bucket so that the bucket can be destroyed without error. These objects are *not* recoverable.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An [ACL policy grant](https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#sample-acl) (documented below). Conflicts with `acl`.
        #[builder(into, default)]
        pub grants: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketGrant>>,
        >,
        /// The [Route 53 Hosted Zone ID](https://docs.aws.amazon.com/general/latest/gr/rande.html#s3_website_region_endpoints) for this bucket's region.
        #[builder(into, default)]
        pub hosted_zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A configuration of [object lifecycle management](http://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html) (documented below).
        #[builder(into, default)]
        pub lifecycle_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketLifecycleRule>>,
        >,
        /// A settings of [bucket logging](https://docs.aws.amazon.com/AmazonS3/latest/UG/ManagingBucketLogging.html) (documented below).
        #[builder(into, default)]
        pub loggings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketLogging>>,
        >,
        /// A configuration of [S3 object locking](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html) (documented below)
        ///
        /// > **NOTE:** You cannot use `acceleration_status` in `cn-north-1` or `us-gov-west-1`
        #[builder(into, default)]
        pub object_lock_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::BucketObjectLockConfiguration>,
        >,
        /// A valid [bucket policy](https://docs.aws.amazon.com/AmazonS3/latest/dev/example-bucket-policies.html) JSON document. Note that if the policy document is not specific enough (but still valid), this provider may view the policy as constantly changing in a `pulumi preview`. In this case, please make sure you use the verbose/specific version of the policy.
        #[builder(into, default)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A configuration of [replication configuration](http://docs.aws.amazon.com/AmazonS3/latest/dev/crr.html) (documented below).
        #[builder(into, default)]
        pub replication_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::BucketReplicationConfiguration>,
        >,
        /// Specifies who should bear the cost of Amazon S3 data transfer.
        /// Can be either `BucketOwner` or `Requester`. By default, the owner of the S3 bucket would incur
        /// the costs of any data transfer. See [Requester Pays Buckets](http://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html)
        /// developer guide for more information.
        #[builder(into, default)]
        pub request_payer: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A configuration of [server-side encryption configuration](http://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html) (documented below)
        #[builder(into, default)]
        pub server_side_encryption_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::BucketServerSideEncryptionConfiguration>,
        >,
        /// A map of tags to assign to the bucket. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A state of [versioning](https://docs.aws.amazon.com/AmazonS3/latest/dev/Versioning.html) (documented below)
        #[builder(into, default)]
        pub versioning: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::BucketVersioning>,
        >,
        /// A website object (documented below).
        #[builder(into, default)]
        pub website: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::BucketWebsite>,
        >,
        /// The domain of the website endpoint, if the bucket is configured with a website. If not, this will be an empty string. This is used to create Route 53 alias records.
        #[builder(into, default)]
        pub website_domain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The website endpoint, if the bucket is configured with a website. If not, this will be an empty string.
        #[builder(into, default)]
        pub website_endpoint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BucketResult {
        /// Sets the accelerate configuration of an existing bucket. Can be `Enabled` or `Suspended`.
        pub acceleration_status: pulumi_gestalt_rust::Output<String>,
        /// The [canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl) to apply. Valid values are `private`, `public-read`, `public-read-write`, `aws-exec-read`, `authenticated-read`, and `log-delivery-write`. Defaults to `private`.  Conflicts with `grant`.
        pub acl: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN of the bucket. Will be of format `arn:aws:s3:::bucketname`.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the bucket. If omitted, this provider will assign a random, unique name. Must be lowercase and less than or equal to 63 characters in length. A full list of bucket naming rules [may be found here](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html).
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// The bucket domain name. Will be of format `bucketname.s3.amazonaws.com`.
        pub bucket_domain_name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique bucket name beginning with the specified prefix. Conflicts with `bucket`. Must be lowercase and less than or equal to 37 characters in length. A full list of bucket naming rules [may be found here](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html).
        pub bucket_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// The bucket region-specific domain name. The bucket domain name including the region name, please refer [here](https://docs.aws.amazon.com/general/latest/gr/rande.html#s3_region) for format. Note: The AWS CloudFront allows specifying S3 region-specific endpoint when creating S3 origin, it will prevent [redirect issues](https://forums.aws.amazon.com/thread.jspa?threadID=216814) from CloudFront to S3 Origin URL.
        pub bucket_regional_domain_name: pulumi_gestalt_rust::Output<String>,
        /// A rule of [Cross-Origin Resource Sharing](https://docs.aws.amazon.com/AmazonS3/latest/dev/cors.html) (documented below).
        pub cors_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::s3::BucketCorsRule>>,
        >,
        /// A boolean that indicates all objects (including any [locked objects](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html)) should be deleted from the bucket so that the bucket can be destroyed without error. These objects are *not* recoverable.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An [ACL policy grant](https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#sample-acl) (documented below). Conflicts with `acl`.
        pub grants: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::s3::BucketGrant>>,
        >,
        /// The [Route 53 Hosted Zone ID](https://docs.aws.amazon.com/general/latest/gr/rande.html#s3_website_region_endpoints) for this bucket's region.
        pub hosted_zone_id: pulumi_gestalt_rust::Output<String>,
        /// A configuration of [object lifecycle management](http://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html) (documented below).
        pub lifecycle_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::s3::BucketLifecycleRule>>,
        >,
        /// A settings of [bucket logging](https://docs.aws.amazon.com/AmazonS3/latest/UG/ManagingBucketLogging.html) (documented below).
        pub loggings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::s3::BucketLogging>>,
        >,
        /// A configuration of [S3 object locking](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html) (documented below)
        ///
        /// > **NOTE:** You cannot use `acceleration_status` in `cn-north-1` or `us-gov-west-1`
        pub object_lock_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3::BucketObjectLockConfiguration>,
        >,
        /// A valid [bucket policy](https://docs.aws.amazon.com/AmazonS3/latest/dev/example-bucket-policies.html) JSON document. Note that if the policy document is not specific enough (but still valid), this provider may view the policy as constantly changing in a `pulumi preview`. In this case, please make sure you use the verbose/specific version of the policy.
        pub policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The AWS region this bucket resides in.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// A configuration of [replication configuration](http://docs.aws.amazon.com/AmazonS3/latest/dev/crr.html) (documented below).
        pub replication_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3::BucketReplicationConfiguration>,
        >,
        /// Specifies who should bear the cost of Amazon S3 data transfer.
        /// Can be either `BucketOwner` or `Requester`. By default, the owner of the S3 bucket would incur
        /// the costs of any data transfer. See [Requester Pays Buckets](http://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html)
        /// developer guide for more information.
        pub request_payer: pulumi_gestalt_rust::Output<String>,
        /// A configuration of [server-side encryption configuration](http://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html) (documented below)
        pub server_side_encryption_configuration: pulumi_gestalt_rust::Output<
            super::super::types::s3::BucketServerSideEncryptionConfiguration,
        >,
        /// A map of tags to assign to the bucket. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A state of [versioning](https://docs.aws.amazon.com/AmazonS3/latest/dev/Versioning.html) (documented below)
        pub versioning: pulumi_gestalt_rust::Output<
            super::super::types::s3::BucketVersioning,
        >,
        /// A website object (documented below).
        pub website: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3::BucketWebsite>,
        >,
        /// The domain of the website endpoint, if the bucket is configured with a website. If not, this will be an empty string. This is used to create Route 53 alias records.
        pub website_domain: pulumi_gestalt_rust::Output<String>,
        /// The website endpoint, if the bucket is configured with a website. If not, this will be an empty string.
        pub website_endpoint: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketArgs,
    ) -> BucketResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let acceleration_status_binding = args.acceleration_status.get_output(context);
        let acl_binding = args.acl.get_output(context);
        let arn_binding = args.arn.get_output(context);
        let bucket_binding = args.bucket.get_output(context);
        let bucket_prefix_binding = args.bucket_prefix.get_output(context);
        let cors_rules_binding = args.cors_rules.get_output(context);
        let force_destroy_binding = args.force_destroy.get_output(context);
        let grants_binding = args.grants.get_output(context);
        let hosted_zone_id_binding = args.hosted_zone_id.get_output(context);
        let lifecycle_rules_binding = args.lifecycle_rules.get_output(context);
        let loggings_binding = args.loggings.get_output(context);
        let object_lock_configuration_binding = args
            .object_lock_configuration
            .get_output(context);
        let policy_binding = args.policy.get_output(context);
        let replication_configuration_binding = args
            .replication_configuration
            .get_output(context);
        let request_payer_binding = args.request_payer.get_output(context);
        let server_side_encryption_configuration_binding = args
            .server_side_encryption_configuration
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let versioning_binding = args.versioning.get_output(context);
        let website_binding = args.website.get_output(context);
        let website_domain_binding = args.website_domain.get_output(context);
        let website_endpoint_binding = args.website_endpoint.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/bucket:Bucket".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accelerationStatus".into(),
                    value: &acceleration_status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acl".into(),
                    value: &acl_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucketPrefix".into(),
                    value: &bucket_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "corsRules".into(),
                    value: &cors_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "grants".into(),
                    value: &grants_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostedZoneId".into(),
                    value: &hosted_zone_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lifecycleRules".into(),
                    value: &lifecycle_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loggings".into(),
                    value: &loggings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "objectLockConfiguration".into(),
                    value: &object_lock_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationConfiguration".into(),
                    value: &replication_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestPayer".into(),
                    value: &request_payer_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverSideEncryptionConfiguration".into(),
                    value: &server_side_encryption_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versioning".into(),
                    value: &versioning_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "website".into(),
                    value: &website_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "websiteDomain".into(),
                    value: &website_domain_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "websiteEndpoint".into(),
                    value: &website_endpoint_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketResult {
            acceleration_status: o.get_field("accelerationStatus"),
            acl: o.get_field("acl"),
            arn: o.get_field("arn"),
            bucket: o.get_field("bucket"),
            bucket_domain_name: o.get_field("bucketDomainName"),
            bucket_prefix: o.get_field("bucketPrefix"),
            bucket_regional_domain_name: o.get_field("bucketRegionalDomainName"),
            cors_rules: o.get_field("corsRules"),
            force_destroy: o.get_field("forceDestroy"),
            grants: o.get_field("grants"),
            hosted_zone_id: o.get_field("hostedZoneId"),
            lifecycle_rules: o.get_field("lifecycleRules"),
            loggings: o.get_field("loggings"),
            object_lock_configuration: o.get_field("objectLockConfiguration"),
            policy: o.get_field("policy"),
            region: o.get_field("region"),
            replication_configuration: o.get_field("replicationConfiguration"),
            request_payer: o.get_field("requestPayer"),
            server_side_encryption_configuration: o
                .get_field("serverSideEncryptionConfiguration"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            versioning: o.get_field("versioning"),
            website: o.get_field("website"),
            website_domain: o.get_field("websiteDomain"),
            website_endpoint: o.get_field("websiteEndpoint"),
        }
    }
}
