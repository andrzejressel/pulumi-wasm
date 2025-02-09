/// Provides an Elastic Transcoder pipeline resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bar = pipeline::create(
///         "bar",
///         PipelineArgs::builder()
///             .content_config(
///                 PipelineContentConfig::builder()
///                     .bucket("${contentBucket.id}")
///                     .storageClass("Standard")
///                     .build_struct(),
///             )
///             .input_bucket("${inputBucket.id}")
///             .name("aws_elastictranscoder_pipeline_my_test_")
///             .role("${testRole.arn}")
///             .thumbnail_config(
///                 PipelineThumbnailConfig::builder()
///                     .bucket("${thumbBucket.id}")
///                     .storageClass("Standard")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Elastic Transcoder pipelines using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:elastictranscoder/pipeline:Pipeline basic_pipeline 1407981661351-cttk8b
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod pipeline {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PipelineArgs {
        /// The AWS Key Management Service (AWS KMS) key that you want to use with this pipeline.
        #[builder(into, default)]
        pub aws_kms_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ContentConfig object specifies information about the Amazon S3 bucket in which you want Elastic Transcoder to save transcoded files and playlists. (documented below)
        #[builder(into, default)]
        pub content_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elastictranscoder::PipelineContentConfig>,
        >,
        /// The permissions for the `content_config` object. (documented below)
        #[builder(into, default)]
        pub content_config_permissions: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::elastictranscoder::PipelineContentConfigPermission,
                >,
            >,
        >,
        /// The Amazon S3 bucket in which you saved the media files that you want to transcode and the graphics that you want to use as watermarks.
        #[builder(into)]
        pub input_bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the pipeline. Maximum 40 characters
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Simple Notification Service (Amazon SNS) topic that you want to notify to report job status. (documented below)
        #[builder(into, default)]
        pub notifications: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elastictranscoder::PipelineNotifications>,
        >,
        /// The Amazon S3 bucket in which you want Elastic Transcoder to save the transcoded files.
        #[builder(into, default)]
        pub output_bucket: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IAM Amazon Resource Name (ARN) for the role that you want Elastic Transcoder to use to transcode jobs for this pipeline.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ThumbnailConfig object specifies information about the Amazon S3 bucket in which you want Elastic Transcoder to save thumbnail files. (documented below)
        #[builder(into, default)]
        pub thumbnail_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elastictranscoder::PipelineThumbnailConfig>,
        >,
        /// The permissions for the `thumbnail_config` object. (documented below)
        ///
        /// The `content_config` object specifies information about the Amazon S3 bucket in
        /// which you want Elastic Transcoder to save transcoded files and playlists: which
        /// bucket to use, and the storage class that you want to assign to the files. If
        /// you specify values for `content_config`, you must also specify values for
        /// `thumbnail_config`. If you specify values for `content_config` and
        /// `thumbnail_config`, omit the `output_bucket` object.
        #[builder(into, default)]
        pub thumbnail_config_permissions: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::elastictranscoder::PipelineThumbnailConfigPermission,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct PipelineResult {
        /// The ARN of the Elastictranscoder pipeline.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The AWS Key Management Service (AWS KMS) key that you want to use with this pipeline.
        pub aws_kms_key_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ContentConfig object specifies information about the Amazon S3 bucket in which you want Elastic Transcoder to save transcoded files and playlists. (documented below)
        pub content_config: pulumi_gestalt_rust::Output<
            super::super::types::elastictranscoder::PipelineContentConfig,
        >,
        /// The permissions for the `content_config` object. (documented below)
        pub content_config_permissions: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::elastictranscoder::PipelineContentConfigPermission,
                >,
            >,
        >,
        /// The Amazon S3 bucket in which you saved the media files that you want to transcode and the graphics that you want to use as watermarks.
        pub input_bucket: pulumi_gestalt_rust::Output<String>,
        /// The name of the pipeline. Maximum 40 characters
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Simple Notification Service (Amazon SNS) topic that you want to notify to report job status. (documented below)
        pub notifications: pulumi_gestalt_rust::Output<
            Option<super::super::types::elastictranscoder::PipelineNotifications>,
        >,
        /// The Amazon S3 bucket in which you want Elastic Transcoder to save the transcoded files.
        pub output_bucket: pulumi_gestalt_rust::Output<String>,
        /// The IAM Amazon Resource Name (ARN) for the role that you want Elastic Transcoder to use to transcode jobs for this pipeline.
        pub role: pulumi_gestalt_rust::Output<String>,
        /// The ThumbnailConfig object specifies information about the Amazon S3 bucket in which you want Elastic Transcoder to save thumbnail files. (documented below)
        pub thumbnail_config: pulumi_gestalt_rust::Output<
            super::super::types::elastictranscoder::PipelineThumbnailConfig,
        >,
        /// The permissions for the `thumbnail_config` object. (documented below)
        ///
        /// The `content_config` object specifies information about the Amazon S3 bucket in
        /// which you want Elastic Transcoder to save transcoded files and playlists: which
        /// bucket to use, and the storage class that you want to assign to the files. If
        /// you specify values for `content_config`, you must also specify values for
        /// `thumbnail_config`. If you specify values for `content_config` and
        /// `thumbnail_config`, omit the `output_bucket` object.
        pub thumbnail_config_permissions: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::elastictranscoder::PipelineThumbnailConfigPermission,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PipelineArgs,
    ) -> PipelineResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let aws_kms_key_arn_binding = args.aws_kms_key_arn.get_output(context);
        let content_config_binding = args.content_config.get_output(context);
        let content_config_permissions_binding = args
            .content_config_permissions
            .get_output(context);
        let input_bucket_binding = args.input_bucket.get_output(context);
        let name_binding = args.name.get_output(context);
        let notifications_binding = args.notifications.get_output(context);
        let output_bucket_binding = args.output_bucket.get_output(context);
        let role_binding = args.role.get_output(context);
        let thumbnail_config_binding = args.thumbnail_config.get_output(context);
        let thumbnail_config_permissions_binding = args
            .thumbnail_config_permissions
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:elastictranscoder/pipeline:Pipeline".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsKmsKeyArn".into(),
                    value: aws_kms_key_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentConfig".into(),
                    value: content_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentConfigPermissions".into(),
                    value: content_config_permissions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputBucket".into(),
                    value: input_bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notifications".into(),
                    value: notifications_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outputBucket".into(),
                    value: output_bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "thumbnailConfig".into(),
                    value: thumbnail_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "thumbnailConfigPermissions".into(),
                    value: thumbnail_config_permissions_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PipelineResult {
            arn: o.get_field("arn"),
            aws_kms_key_arn: o.get_field("awsKmsKeyArn"),
            content_config: o.get_field("contentConfig"),
            content_config_permissions: o.get_field("contentConfigPermissions"),
            input_bucket: o.get_field("inputBucket"),
            name: o.get_field("name"),
            notifications: o.get_field("notifications"),
            output_bucket: o.get_field("outputBucket"),
            role: o.get_field("role"),
            thumbnail_config: o.get_field("thumbnailConfig"),
            thumbnail_config_permissions: o.get_field("thumbnailConfigPermissions"),
        }
    }
}
