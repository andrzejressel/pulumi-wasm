/// Provides a resource for controlling versioning on an S3 bucket.
/// Deleting this resource will either suspend versioning on the associated S3 bucket or
/// simply remove the resource from state if the associated S3 bucket is unversioned.
///
/// For more information, see [How S3 versioning works](https://docs.aws.amazon.com/AmazonS3/latest/userguide/manage-versioning-examples.html).
///
/// > **NOTE:** If you are enabling versioning on the bucket for the first time, AWS recommends that you wait for 15 minutes after enabling versioning before issuing write operations (PUT or DELETE) on objects in the bucket.
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ### With Versioning Enabled
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("example-bucket").build_struct(),
///     );
///     let exampleBucketAclV2 = bucket_acl_v_2::create(
///         "exampleBucketAclV2",
///         BucketAclV2Args::builder().acl("private").bucket("${example.id}").build_struct(),
///     );
///     let versioningExample = bucket_versioning_v_2::create(
///         "versioningExample",
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
/// ### With Versioning Disabled
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("example-bucket").build_struct(),
///     );
///     let exampleBucketAclV2 = bucket_acl_v_2::create(
///         "exampleBucketAclV2",
///         BucketAclV2Args::builder().acl("private").bucket("${example.id}").build_struct(),
///     );
///     let versioningExample = bucket_versioning_v_2::create(
///         "versioningExample",
///         BucketVersioningV2Args::builder()
///             .bucket("${example.id}")
///             .versioning_configuration(
///                 BucketVersioningV2VersioningConfiguration::builder()
///                     .status("Disabled")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Object Dependency On Versioning
///
/// When you create an object whose `version_id` you need and an `aws.s3.BucketVersioningV2` resource in the same configuration, you are more likely to have success by ensuring the `s3_object` depends either implicitly (see below) or explicitly (i.e., using `depends_on = [aws_s3_bucket_versioning.example]`) on the `aws.s3.BucketVersioningV2` resource.
///
/// > **NOTE:** For critical and/or production S3 objects, do not create a bucket, enable versioning, and create an object in the bucket within the same configuration. Doing so will not allow the AWS-recommended 15 minutes between enabling versioning and writing to the bucket.
///
/// This example shows the `aws_s3_object.example` depending implicitly on the versioning resource through the reference to `aws_s3_bucket_versioning.example.bucket` to define `bucket`:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: yotto
///   exampleBucketVersioningV2:
///     type: aws:s3:BucketVersioningV2
///     name: example
///     properties:
///       bucket: ${example.id}
///       versioningConfiguration:
///         status: Enabled
///   exampleBucketObjectv2:
///     type: aws:s3:BucketObjectv2
///     name: example
///     properties:
///       bucket: ${exampleBucketVersioningV2.id}
///       key: droeloe
///       source:
///         fn::FileAsset: example.txt
/// ```
///
/// ## Import
///
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// __Using `pulumi import` to import__ S3 bucket versioning using the `bucket` or using the `bucket` and `expected_bucket_owner` separated by a comma (`,`). For example:
///
/// If the owner (account ID) of the source bucket is the same account used to configure the AWS Provider, import using the `bucket`:
///
/// ```sh
/// $ pulumi import aws:s3/bucketVersioningV2:BucketVersioningV2 example bucket-name
/// ```
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// ```sh
/// $ pulumi import aws:s3/bucketVersioningV2:BucketVersioningV2 example bucket-name,123456789012
/// ```
pub mod bucket_versioning_v_2 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketVersioningV2Args {
        /// Name of the S3 bucket.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Account ID of the expected bucket owner.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.
        #[builder(into, default)]
        pub mfa: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for the versioning parameters. See below.
        #[builder(into)]
        pub versioning_configuration: pulumi_wasm_rust::Output<
            super::super::types::s3::BucketVersioningV2VersioningConfiguration,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketVersioningV2Result {
        /// Name of the S3 bucket.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Account ID of the expected bucket owner.
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.
        pub mfa: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for the versioning parameters. See below.
        pub versioning_configuration: pulumi_wasm_rust::Output<
            super::super::types::s3::BucketVersioningV2VersioningConfiguration,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BucketVersioningV2Args) -> BucketVersioningV2Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let expected_bucket_owner_binding = args.expected_bucket_owner.get_inner();
        let mfa_binding = args.mfa.get_inner();
        let versioning_configuration_binding = args.versioning_configuration.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketVersioningV2:BucketVersioningV2".into(),
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
                    name: "mfa".into(),
                    value: &mfa_binding,
                },
                register_interface::ObjectField {
                    name: "versioningConfiguration".into(),
                    value: &versioning_configuration_binding,
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
                    name: "mfa".into(),
                },
                register_interface::ResultField {
                    name: "versioningConfiguration".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketVersioningV2Result {
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            expected_bucket_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expectedBucketOwner").unwrap(),
            ),
            mfa: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mfa").unwrap(),
            ),
            versioning_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versioningConfiguration").unwrap(),
            ),
        }
    }
}