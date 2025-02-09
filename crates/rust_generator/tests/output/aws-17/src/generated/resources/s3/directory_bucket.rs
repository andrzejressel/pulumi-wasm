/// Provides an Amazon S3 Express directory bucket resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = directory_bucket::create(
///         "example",
///         DirectoryBucketArgs::builder()
///             .bucket("example--usw2-az1--x-s3")
///             .location(DirectoryBucketLocation::builder().name("usw2-az1").build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 bucket using `bucket`. For example:
///
/// ```sh
/// $ pulumi import aws:s3/directoryBucket:DirectoryBucket example example--usw2-az1--x-s3
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod directory_bucket {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DirectoryBucketArgs {
        /// Name of the bucket. The name must be in the format `[bucket_name]--[azid]--x-s3`. Use the `aws.s3.BucketV2` resource to manage general purpose buckets.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Data redundancy. Valid values: `SingleAvailabilityZone`.
        #[builder(into, default)]
        pub data_redundancy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean that indicates all objects should be deleted from the bucket *when the bucket is destroyed* so that the bucket can be destroyed without error. These objects are *not* recoverable. This only deletes objects when the bucket is destroyed, *not* when setting this parameter to `true`. Once this parameter is set to `true`, there must be a successful `pulumi up` run before a destroy is required to update this value in the resource state. Without a successful `pulumi up` after this parameter is set, this flag will have no effect. If setting this field in the same operation that would require replacing the bucket or destroying the bucket, this flag will not work. Additionally when importing a bucket, a successful `pulumi up` is required to set this value in state before it will take effect on a destroy operation.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Bucket location. See Location below for more details.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::DirectoryBucketLocation>,
        >,
        /// Bucket type. Valid values: `Directory`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DirectoryBucketResult {
        /// ARN of the bucket.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the bucket. The name must be in the format `[bucket_name]--[azid]--x-s3`. Use the `aws.s3.BucketV2` resource to manage general purpose buckets.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Data redundancy. Valid values: `SingleAvailabilityZone`.
        pub data_redundancy: pulumi_gestalt_rust::Output<String>,
        /// Boolean that indicates all objects should be deleted from the bucket *when the bucket is destroyed* so that the bucket can be destroyed without error. These objects are *not* recoverable. This only deletes objects when the bucket is destroyed, *not* when setting this parameter to `true`. Once this parameter is set to `true`, there must be a successful `pulumi up` run before a destroy is required to update this value in the resource state. Without a successful `pulumi up` after this parameter is set, this flag will have no effect. If setting this field in the same operation that would require replacing the bucket or destroying the bucket, this flag will not work. Additionally when importing a bucket, a successful `pulumi up` is required to set this value in state before it will take effect on a destroy operation.
        pub force_destroy: pulumi_gestalt_rust::Output<bool>,
        /// Bucket location. See Location below for more details.
        pub location: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3::DirectoryBucketLocation>,
        >,
        /// Bucket type. Valid values: `Directory`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DirectoryBucketArgs,
    ) -> DirectoryBucketResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let data_redundancy_binding = args.data_redundancy.get_output(context);
        let force_destroy_binding = args.force_destroy.get_output(context);
        let location_binding = args.location.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/directoryBucket:DirectoryBucket".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataRedundancy".into(),
                    value: data_redundancy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: force_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DirectoryBucketResult {
            arn: o.get_field("arn"),
            bucket: o.get_field("bucket"),
            data_redundancy: o.get_field("dataRedundancy"),
            force_destroy: o.get_field("forceDestroy"),
            location: o.get_field("location"),
            type_: o.get_field("type"),
        }
    }
}
