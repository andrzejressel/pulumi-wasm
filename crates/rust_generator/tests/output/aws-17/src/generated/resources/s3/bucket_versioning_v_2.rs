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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_versioning_v_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketVersioningV2Args {
        /// Name of the S3 bucket.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Account ID of the expected bucket owner.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.
        #[builder(into, default)]
        pub mfa: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for the versioning parameters. See below.
        #[builder(into)]
        pub versioning_configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::s3::BucketVersioningV2VersioningConfiguration,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketVersioningV2Result {
        /// Name of the S3 bucket.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Account ID of the expected bucket owner.
        pub expected_bucket_owner: pulumi_gestalt_rust::Output<Option<String>>,
        /// Concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.
        pub mfa: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block for the versioning parameters. See below.
        pub versioning_configuration: pulumi_gestalt_rust::Output<
            super::super::types::s3::BucketVersioningV2VersioningConfiguration,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BucketVersioningV2Args,
    ) -> BucketVersioningV2Result {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bucket_binding_1 = args.bucket.get_output(context);
        let bucket_binding = bucket_binding_1.get_inner();
        let expected_bucket_owner_binding_1 = args
            .expected_bucket_owner
            .get_output(context);
        let expected_bucket_owner_binding = expected_bucket_owner_binding_1.get_inner();
        let mfa_binding_1 = args.mfa.get_output(context);
        let mfa_binding = mfa_binding_1.get_inner();
        let versioning_configuration_binding_1 = args
            .versioning_configuration
            .get_output(context);
        let versioning_configuration_binding = versioning_configuration_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketVersioningV2:BucketVersioningV2".into(),
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
                    name: "mfa".into(),
                    value: &mfa_binding,
                },
                register_interface::ObjectField {
                    name: "versioningConfiguration".into(),
                    value: &versioning_configuration_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BucketVersioningV2Result {
            bucket: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucket"),
            ),
            expected_bucket_owner: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expectedBucketOwner"),
            ),
            mfa: pulumi_gestalt_rust::__private::into_domain(o.extract_field("mfa")),
            versioning_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versioningConfiguration"),
            ),
        }
    }
}
