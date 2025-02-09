/// Provides a resource to manage S3 Bucket Ownership Controls. For more information, see the [S3 Developer Guide](https://docs.aws.amazon.com/AmazonS3/latest/dev/about-object-ownership.html).
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("example").build_struct(),
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
/// Using `pulumi import`, import S3 Bucket Ownership Controls using S3 Bucket name. For example:
///
/// ```sh
/// $ pulumi import aws:s3/bucketOwnershipControls:BucketOwnershipControls example my-bucket
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_ownership_controls {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketOwnershipControlsArgs {
        /// Name of the bucket that you want to associate this access point with.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block(s) with Ownership Controls rules. Detailed below.
        #[builder(into)]
        pub rule: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::s3::BucketOwnershipControlsRule,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketOwnershipControlsResult {
        /// Name of the bucket that you want to associate this access point with.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Configuration block(s) with Ownership Controls rules. Detailed below.
        pub rule: pulumi_gestalt_rust::Output<
            super::super::types::s3::BucketOwnershipControlsRule,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketOwnershipControlsArgs,
    ) -> BucketOwnershipControlsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let rule_binding = args.rule.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/bucketOwnershipControls:BucketOwnershipControls".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rule".into(),
                    value: rule_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketOwnershipControlsResult {
            bucket: o.get_field("bucket"),
            rule: o.get_field("rule"),
        }
    }
}
