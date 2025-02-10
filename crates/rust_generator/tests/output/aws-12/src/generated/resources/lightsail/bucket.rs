/// Provides a lightsail bucket.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = bucket::create(
///         "test",
///         BucketArgs::builder().bundle_id("small_1_0").name("mytestbucket").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_bucket` using the `name` attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/bucket:Bucket test example-bucket
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketArgs {
        /// The ID of the bundle to use for the bucket. A bucket bundle specifies the monthly cost, storage space, and data transfer quota for a bucket. Use the [get-bucket-bundles](https://docs.aws.amazon.com/cli/latest/reference/lightsail/get-bucket-bundles.html) cli command to get a list of bundle IDs that you can specify.
        #[builder(into)]
        pub bundle_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Force Delete non-empty buckets using `pulumi destroy`. AWS by default will not delete an s3 bucket which is not empty, to prevent losing bucket data and affecting other resources in lightsail. If `force_delete` is set to `true` the bucket will be deleted even when not empty.
        #[builder(into, default)]
        pub force_delete: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name for the bucket.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketResult {
        /// The ARN of the lightsail bucket.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The resource Availability Zone. Follows the format us-east-2a (case-sensitive).
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// The ID of the bundle to use for the bucket. A bucket bundle specifies the monthly cost, storage space, and data transfer quota for a bucket. Use the [get-bucket-bundles](https://docs.aws.amazon.com/cli/latest/reference/lightsail/get-bucket-bundles.html) cli command to get a list of bundle IDs that you can specify.
        pub bundle_id: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the bucket was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Force Delete non-empty buckets using `pulumi destroy`. AWS by default will not delete an s3 bucket which is not empty, to prevent losing bucket data and affecting other resources in lightsail. If `force_delete` is set to `true` the bucket will be deleted even when not empty.
        pub force_delete: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name for the bucket.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Web Services Region name.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The support code for the resource. Include this code in your email to support when you have questions about a resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.
        pub support_code: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub url: pulumi_gestalt_rust::Output<String>,
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
        let bundle_id_binding = args.bundle_id.get_output(context);
        let force_delete_binding = args.force_delete.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/bucket:Bucket".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bundleId".into(),
                    value: bundle_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDelete".into(),
                    value: force_delete_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketResult {
            arn: o.get_field("arn"),
            availability_zone: o.get_field("availabilityZone"),
            bundle_id: o.get_field("bundleId"),
            created_at: o.get_field("createdAt"),
            force_delete: o.get_field("forceDelete"),
            name: o.get_field("name"),
            region: o.get_field("region"),
            support_code: o.get_field("supportCode"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            url: o.get_field("url"),
        }
    }
}
