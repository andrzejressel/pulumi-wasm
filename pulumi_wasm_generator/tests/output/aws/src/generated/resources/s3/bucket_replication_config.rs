/// Provides an independent configuration resource for S3 bucket [replication configuration](http://docs.aws.amazon.com/AmazonS3/latest/dev/crr.html).
///
/// > **NOTE:** S3 Buckets only support a single replication configuration. Declaring multiple `aws.s3.BucketReplicationConfig` resources to the same S3 Bucket will cause a perpetual difference in configuration.
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ### Using replication configuration
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let assumeRole = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sts:AssumeRole",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["s3.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let replication = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["s3:GetReplicationConfiguration", "s3:ListBucket",])
///                     .effect("Allow").resources(vec!["${source.arn}",]).build_struct(),
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["s3:GetObjectVersionForReplication",
///                     "s3:GetObjectVersionAcl", "s3:GetObjectVersionTagging",])
///                     .effect("Allow").resources(vec!["${source.arn}/*",]).build_struct(),
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["s3:ReplicateObject", "s3:ReplicateDelete",
///                     "s3:ReplicateTags",]).effect("Allow")
///                     .resources(vec!["${destination.arn}/*",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let destination = bucket_v_2::create(
///         "destination",
///         BucketV2Args::builder().bucket("tf-test-bucket-destination-12345").build_struct(),
///     );
///     let destinationBucketVersioningV2 = bucket_versioning_v_2::create(
///         "destinationBucketVersioningV2",
///         BucketVersioningV2Args::builder()
///             .bucket("${destination.id}")
///             .versioning_configuration(
///                 BucketVersioningV2VersioningConfiguration::builder()
///                     .status("Enabled")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let replicationBucketReplicationConfig = bucket_replication_config::create(
///         "replicationBucketReplicationConfig",
///         BucketReplicationConfigArgs::builder()
///             .bucket("${source.id}")
///             .role("${replicationRole.arn}")
///             .rules(
///                 vec![
///                     BucketReplicationConfigRule::builder()
///                     .destination(BucketReplicationConfigRuleDestination::builder()
///                     .bucket("${destination.arn}").storageClass("STANDARD")
///                     .build_struct()).filter(BucketReplicationConfigRuleFilter::builder()
///                     .prefix("foo").build_struct()).id("foobar").status("Enabled")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let replicationPolicy = policy::create(
///         "replicationPolicy",
///         PolicyArgs::builder()
///             .name("tf-iam-role-policy-replication-12345")
///             .policy("${replication.json}")
///             .build_struct(),
///     );
///     let replicationRole = role::create(
///         "replicationRole",
///         RoleArgs::builder()
///             .assume_role_policy("${assumeRole.json}")
///             .name("tf-iam-role-replication-12345")
///             .build_struct(),
///     );
///     let replicationRolePolicyAttachment = role_policy_attachment::create(
///         "replicationRolePolicyAttachment",
///         RolePolicyAttachmentArgs::builder()
///             .policy_arn("${replicationPolicy.arn}")
///             .role("${replicationRole.name}")
///             .build_struct(),
///     );
///     let source = bucket_v_2::create(
///         "source",
///         BucketV2Args::builder().bucket("tf-test-bucket-source-12345").build_struct(),
///     );
///     let sourceBucketAcl = bucket_acl_v_2::create(
///         "sourceBucketAcl",
///         BucketAclV2Args::builder().acl("private").bucket("${source.id}").build_struct(),
///     );
///     let sourceBucketVersioningV2 = bucket_versioning_v_2::create(
///         "sourceBucketVersioningV2",
///         BucketVersioningV2Args::builder()
///             .bucket("${source.id}")
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
/// ### Bi-Directional Replication
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let east = bucket_v_2::create(
///         "east",
///         BucketV2Args::builder().bucket("tf-test-bucket-east-12345").build_struct(),
///     );
///     let eastBucketVersioningV2 = bucket_versioning_v_2::create(
///         "eastBucketVersioningV2",
///         BucketVersioningV2Args::builder()
///             .bucket("${east.id}")
///             .versioning_configuration(
///                 BucketVersioningV2VersioningConfiguration::builder()
///                     .status("Enabled")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let eastToWest = bucket_replication_config::create(
///         "eastToWest",
///         BucketReplicationConfigArgs::builder()
///             .bucket("${east.id}")
///             .role("${eastReplication.arn}")
///             .rules(
///                 vec![
///                     BucketReplicationConfigRule::builder()
///                     .destination(BucketReplicationConfigRuleDestination::builder()
///                     .bucket("${west.arn}").storageClass("STANDARD").build_struct())
///                     .filter(BucketReplicationConfigRuleFilter::builder().prefix("foo")
///                     .build_struct()).id("foobar").status("Enabled").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let west = bucket_v_2::create(
///         "west",
///         BucketV2Args::builder().bucket("tf-test-bucket-west-12345").build_struct(),
///     );
///     let westBucketVersioningV2 = bucket_versioning_v_2::create(
///         "westBucketVersioningV2",
///         BucketVersioningV2Args::builder()
///             .bucket("${west.id}")
///             .versioning_configuration(
///                 BucketVersioningV2VersioningConfiguration::builder()
///                     .status("Enabled")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let westToEast = bucket_replication_config::create(
///         "westToEast",
///         BucketReplicationConfigArgs::builder()
///             .bucket("${west.id}")
///             .role("${westReplication.arn}")
///             .rules(
///                 vec![
///                     BucketReplicationConfigRule::builder()
///                     .destination(BucketReplicationConfigRuleDestination::builder()
///                     .bucket("${east.arn}").storageClass("STANDARD").build_struct())
///                     .filter(BucketReplicationConfigRuleFilter::builder().prefix("foo")
///                     .build_struct()).id("foobar").status("Enabled").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 bucket replication configuration using the `bucket`. For example:
///
/// ```sh
/// $ pulumi import aws:s3/bucketReplicationConfig:BucketReplicationConfig replication bucket-name
/// ```
pub mod bucket_replication_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketReplicationConfigArgs {
        /// Name of the source S3 bucket you want Amazon S3 to monitor.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role for Amazon S3 to assume when replicating the objects.
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
        /// List of configuration blocks describing the rules managing the replication. See below.
        #[builder(into)]
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketReplicationConfigRule>,
        >,
        /// Token to allow replication to be enabled on an Object Lock-enabled bucket. You must contact AWS support for the bucket's "Object Lock token".
        /// For more details, see [Using S3 Object Lock with replication](https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-lock-managing.html#object-lock-managing-replication).
        #[builder(into, default)]
        pub token: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BucketReplicationConfigResult {
        /// Name of the source S3 bucket you want Amazon S3 to monitor.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role for Amazon S3 to assume when replicating the objects.
        pub role: pulumi_wasm_rust::Output<String>,
        /// List of configuration blocks describing the rules managing the replication. See below.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketReplicationConfigRule>,
        >,
        /// Token to allow replication to be enabled on an Object Lock-enabled bucket. You must contact AWS support for the bucket's "Object Lock token".
        /// For more details, see [Using S3 Object Lock with replication](https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-lock-managing.html#object-lock-managing-replication).
        pub token: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BucketReplicationConfigArgs,
    ) -> BucketReplicationConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let role_binding = args.role.get_inner();
        let rules_binding = args.rules.get_inner();
        let token_binding = args.token.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketReplicationConfig:BucketReplicationConfig".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
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
                    name: "role".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
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
        BucketReplicationConfigResult {
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
            token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("token").unwrap(),
            ),
        }
    }
}