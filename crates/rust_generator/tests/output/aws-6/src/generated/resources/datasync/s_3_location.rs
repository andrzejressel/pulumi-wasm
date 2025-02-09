/// Manages an S3 Location within AWS DataSync.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = s_3_location::create(
///         "example",
///         S3LocationArgs::builder()
///             .s_3_bucket_arn("${exampleAwsS3Bucket.arn}")
///             .s_3_config(
///                 S3LocationS3Config::builder()
///                     .bucketAccessRoleArn("${exampleAwsIamRole.arn}")
///                     .build_struct(),
///             )
///             .subdirectory("/example/prefix")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datasync_location_s3` using the DataSync Task Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:datasync/s3Location:S3Location example arn:aws:datasync:us-east-1:123456789012:location/loc-12345678901234567
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod s_3_location {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct S3LocationArgs {
        /// A list of DataSync Agent ARNs with which this location will be associated.
        #[builder(into, default)]
        pub agent_arns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Amazon Resource Name (ARN) of the S3 Bucket.
        #[builder(into)]
        pub s3_bucket_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block containing information for connecting to S3.
        #[builder(into)]
        pub s3_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::datasync::S3LocationS3Config,
        >,
        /// The Amazon S3 storage class that you want to store your files in when this location is used as a task destination. [Valid values](https://docs.aws.amazon.com/datasync/latest/userguide/create-s3-location.html#using-storage-classes)
        #[builder(into, default)]
        pub s3_storage_class: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Prefix to perform actions as source or destination.
        #[builder(into)]
        pub subdirectory: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct S3LocationResult {
        /// A list of DataSync Agent ARNs with which this location will be associated.
        pub agent_arns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Amazon Resource Name (ARN) of the DataSync Location.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the S3 Bucket.
        pub s3_bucket_arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block containing information for connecting to S3.
        pub s3_config: pulumi_gestalt_rust::Output<
            super::super::types::datasync::S3LocationS3Config,
        >,
        /// The Amazon S3 storage class that you want to store your files in when this location is used as a task destination. [Valid values](https://docs.aws.amazon.com/datasync/latest/userguide/create-s3-location.html#using-storage-classes)
        pub s3_storage_class: pulumi_gestalt_rust::Output<String>,
        /// Prefix to perform actions as source or destination.
        pub subdirectory: pulumi_gestalt_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub uri: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: S3LocationArgs,
    ) -> S3LocationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let agent_arns_binding = args.agent_arns.get_output(context);
        let s3_bucket_arn_binding = args.s3_bucket_arn.get_output(context);
        let s3_config_binding = args.s3_config.get_output(context);
        let s3_storage_class_binding = args.s3_storage_class.get_output(context);
        let subdirectory_binding = args.subdirectory.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:datasync/s3Location:S3Location".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentArns".into(),
                    value: agent_arns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3BucketArn".into(),
                    value: s3_bucket_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Config".into(),
                    value: s3_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3StorageClass".into(),
                    value: s3_storage_class_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subdirectory".into(),
                    value: subdirectory_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        S3LocationResult {
            agent_arns: o.get_field("agentArns"),
            arn: o.get_field("arn"),
            s3_bucket_arn: o.get_field("s3BucketArn"),
            s3_config: o.get_field("s3Config"),
            s3_storage_class: o.get_field("s3StorageClass"),
            subdirectory: o.get_field("subdirectory"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            uri: o.get_field("uri"),
        }
    }
}
