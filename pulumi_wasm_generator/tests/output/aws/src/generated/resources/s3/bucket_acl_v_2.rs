/// Provides an S3 bucket ACL resource.
///
/// > **Note:** destroy does not delete the S3 Bucket ACL but does remove the resource from state.
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ### With `private` ACL
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("my-tf-example-bucket").build_struct(),
///     );
///     let exampleBucketAclV2 = bucket_acl_v_2::create(
///         "exampleBucketAclV2",
///         BucketAclV2Args::builder().acl("private").bucket("${example.id}").build_struct(),
///     );
///     let exampleBucketOwnershipControls = bucket_ownership_controls::create(
///         "exampleBucketOwnershipControls",
///         BucketOwnershipControlsArgs::builder()
///             .bucket("${example.id}")
///             .rule(
///                 BucketOwnershipControlsRule::builder()
///                     .objectOwnership("BucketOwnerPreferred")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With `public-read` ACL
///
/// > This example explicitly disables the default S3 bucket security settings. This
/// should be done with caution, as all bucket objects become publicly exposed.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("my-tf-example-bucket").build_struct(),
///     );
///     let exampleBucketAclV2 = bucket_acl_v_2::create(
///         "exampleBucketAclV2",
///         BucketAclV2Args::builder()
///             .acl("public-read")
///             .bucket("${example.id}")
///             .build_struct(),
///     );
///     let exampleBucketOwnershipControls = bucket_ownership_controls::create(
///         "exampleBucketOwnershipControls",
///         BucketOwnershipControlsArgs::builder()
///             .bucket("${example.id}")
///             .rule(
///                 BucketOwnershipControlsRule::builder()
///                     .objectOwnership("BucketOwnerPreferred")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleBucketPublicAccessBlock = bucket_public_access_block::create(
///         "exampleBucketPublicAccessBlock",
///         BucketPublicAccessBlockArgs::builder()
///             .block_public_acls(false)
///             .block_public_policy(false)
///             .bucket("${example.id}")
///             .ignore_public_acls(false)
///             .restrict_public_buckets(false)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Grants
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let current = get_canonical_user_id::invoke(
///         GetCanonicalUserIdArgs::builder().build_struct(),
///     );
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("my-tf-example-bucket").build_struct(),
///     );
///     let exampleBucketAclV2 = bucket_acl_v_2::create(
///         "exampleBucketAclV2",
///         BucketAclV2Args::builder()
///             .access_control_policy(
///                 BucketAclV2AccessControlPolicy::builder()
///                     .grants(
///                         vec![
///                             BucketAclV2AccessControlPolicyGrant::builder()
///                             .grantee(BucketAclV2AccessControlPolicyGrantGrantee::builder()
///                             .id("${current.id}"). type ("CanonicalUser").build_struct())
///                             .permission("READ").build_struct(),
///                             BucketAclV2AccessControlPolicyGrant::builder()
///                             .grantee(BucketAclV2AccessControlPolicyGrantGrantee::builder()
///                             . type ("Group")
///                             .uri("http://acs.amazonaws.com/groups/s3/LogDelivery")
///                             .build_struct()).permission("READ_ACP").build_struct(),
///                         ],
///                     )
///                     .owner(
///                         BucketAclV2AccessControlPolicyOwner::builder()
///                             .id("${current.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .bucket("${example.id}")
///             .build_struct(),
///     );
///     let exampleBucketOwnershipControls = bucket_ownership_controls::create(
///         "exampleBucketOwnershipControls",
///         BucketOwnershipControlsArgs::builder()
///             .bucket("${example.id}")
///             .rule(
///                 BucketOwnershipControlsRule::builder()
///                     .objectOwnership("BucketOwnerPreferred")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// If the owner (account ID) of the source bucket is the _same_ account used to configure the AWS Provider, and the source bucket is __configured__ with a
/// [canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/userguide/acl-overview.html#canned-acl) (i.e. predefined grant), import using the `bucket` and `acl` separated by a comma (`,`):
///
/// If the owner (account ID) of the source bucket _differs_ from the account used to configure the AWS Provider, and the source bucket is __not configured__ with a [canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/userguide/acl-overview.html#canned-acl) (i.e. predefined grant), imported using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// If the owner (account ID) of the source bucket _differs_ from the account used to configure the AWS Provider, and the source bucket is __configured__ with a
/// [canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/userguide/acl-overview.html#canned-acl) (i.e. predefined grant), imported using the `bucket`, `expected_bucket_owner`, and `acl` separated by commas (`,`):
///
/// __Using `pulumi import` to import__ using `bucket`, `expected_bucket_owner`, and/or `acl`, depending on your situation. For example:
///
/// If the owner (account ID) of the source bucket is the _same_ account used to configure the AWS Provider, and the source bucket is __not configured__ with a
/// [canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/userguide/acl-overview.html#canned-acl) (i.e. predefined grant), import using the `bucket`:
///
/// ```sh
/// $ pulumi import aws:s3/bucketAclV2:BucketAclV2 example bucket-name
/// ```
/// If the owner (account ID) of the source bucket is the _same_ account used to configure the AWS Provider, and the source bucket is __configured__ with a [canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/userguide/acl-overview.html#canned-acl) (i.e. predefined grant), import using the `bucket` and `acl` separated by a comma (`,`):
///
/// ```sh
/// $ pulumi import aws:s3/bucketAclV2:BucketAclV2 example bucket-name,private
/// ```
/// If the owner (account ID) of the source bucket _differs_ from the account used to configure the AWS Provider, and the source bucket is __not configured__ with a [canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/userguide/acl-overview.html#canned-acl) (i.e. predefined grant), imported using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// ```sh
/// $ pulumi import aws:s3/bucketAclV2:BucketAclV2 example bucket-name,123456789012
/// ```
/// If the owner (account ID) of the source bucket _differs_ from the account used to configure the AWS Provider, and the source bucket is __configured__ with a [canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/userguide/acl-overview.html#canned-acl) (i.e. predefined grant), imported using the `bucket`, `expected_bucket_owner`, and `acl` separated by commas (`,`):
///
/// ```sh
/// $ pulumi import aws:s3/bucketAclV2:BucketAclV2 example bucket-name,123456789012,private
/// ```
pub mod bucket_acl_v_2 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketAclV2Args {
        /// Configuration block that sets the ACL permissions for an object per grantee. See below.
        #[builder(into, default)]
        pub access_control_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::s3::BucketAclV2AccessControlPolicy>,
        >,
        /// Specifies the Canned ACL to apply to the bucket. Valid values: `private`, `public-read`, `public-read-write`, `aws-exec-read`, `authenticated-read`, `bucket-owner-read`, `bucket-owner-full-control`, `log-delivery-write`. Full details are available on the [AWS documentation](https://docs.aws.amazon.com/AmazonS3/latest/userguide/acl-overview.html#canned-acl).
        #[builder(into, default)]
        pub acl: pulumi_wasm_rust::Output<Option<String>>,
        /// Bucket to which to apply the ACL.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Account ID of the expected bucket owner.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BucketAclV2Result {
        /// Configuration block that sets the ACL permissions for an object per grantee. See below.
        pub access_control_policy: pulumi_wasm_rust::Output<
            super::super::types::s3::BucketAclV2AccessControlPolicy,
        >,
        /// Specifies the Canned ACL to apply to the bucket. Valid values: `private`, `public-read`, `public-read-write`, `aws-exec-read`, `authenticated-read`, `bucket-owner-read`, `bucket-owner-full-control`, `log-delivery-write`. Full details are available on the [AWS documentation](https://docs.aws.amazon.com/AmazonS3/latest/userguide/acl-overview.html#canned-acl).
        pub acl: pulumi_wasm_rust::Output<Option<String>>,
        /// Bucket to which to apply the ACL.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Account ID of the expected bucket owner.
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BucketAclV2Args) -> BucketAclV2Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_control_policy_binding = args.access_control_policy.get_inner();
        let acl_binding = args.acl.get_inner();
        let bucket_binding = args.bucket.get_inner();
        let expected_bucket_owner_binding = args.expected_bucket_owner.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketAclV2:BucketAclV2".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessControlPolicy".into(),
                    value: &access_control_policy_binding,
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
                    name: "expectedBucketOwner".into(),
                    value: &expected_bucket_owner_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessControlPolicy".into(),
                },
                register_interface::ResultField {
                    name: "acl".into(),
                },
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "expectedBucketOwner".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketAclV2Result {
            access_control_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessControlPolicy").unwrap(),
            ),
            acl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acl").unwrap(),
            ),
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            expected_bucket_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expectedBucketOwner").unwrap(),
            ),
        }
    }
}