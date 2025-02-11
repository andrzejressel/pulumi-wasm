/// Provides an [S3 Intelligent-Tiering](https://docs.aws.amazon.com/AmazonS3/latest/userguide/intelligent-tiering.html) configuration resource.
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ### Add intelligent tiering configuration for entire S3 bucket
///
/// ```yaml
/// resources:
///   example-entire-bucket:
///     type: aws:s3:BucketIntelligentTieringConfiguration
///     properties:
///       bucket: ${example.id}
///       name: EntireBucket
///       tierings:
///         - accessTier: DEEP_ARCHIVE_ACCESS
///           days: 180
///         - accessTier: ARCHIVE_ACCESS
///           days: 125
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example
/// ```
///
/// ### Add intelligent tiering configuration with S3 object filter
///
/// ```yaml
/// resources:
///   example-filtered:
///     type: aws:s3:BucketIntelligentTieringConfiguration
///     properties:
///       bucket: ${example.id}
///       name: ImportantBlueDocuments
///       status: Disabled
///       filter:
///         prefix: documents/
///         tags:
///           priority: high
///           class: blue
///       tierings:
///         - accessTier: ARCHIVE_ACCESS
///           days: 125
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 bucket intelligent tiering configurations using `bucket:name`. For example:
///
/// ```sh
/// $ pulumi import aws:s3/bucketIntelligentTieringConfiguration:BucketIntelligentTieringConfiguration my-bucket-entire-bucket my-bucket:EntireBucket
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_intelligent_tiering_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketIntelligentTieringConfigurationArgs {
        /// Name of the bucket this intelligent tiering configuration is associated with.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Bucket filter. The configuration only includes objects that meet the filter's criteria (documented below).
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::BucketIntelligentTieringConfigurationFilter>,
        >,
        /// Unique name used to identify the S3 Intelligent-Tiering configuration for the bucket.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the status of the configuration. Valid values: `Enabled`, `Disabled`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// S3 Intelligent-Tiering storage class tiers of the configuration (documented below).
        #[builder(into)]
        pub tierings: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::s3::BucketIntelligentTieringConfigurationTiering>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketIntelligentTieringConfigurationResult {
        /// Name of the bucket this intelligent tiering configuration is associated with.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Bucket filter. The configuration only includes objects that meet the filter's criteria (documented below).
        pub filter: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3::BucketIntelligentTieringConfigurationFilter>,
        >,
        /// Unique name used to identify the S3 Intelligent-Tiering configuration for the bucket.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the status of the configuration. Valid values: `Enabled`, `Disabled`.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// S3 Intelligent-Tiering storage class tiers of the configuration (documented below).
        pub tierings: pulumi_gestalt_rust::Output<
            Vec<super::super::types::s3::BucketIntelligentTieringConfigurationTiering>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketIntelligentTieringConfigurationArgs,
    ) -> BucketIntelligentTieringConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let name_binding = args.name.get_output(context);
        let status_binding = args.status.get_output(context);
        let tierings_binding = args.tierings.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/bucketIntelligentTieringConfiguration:BucketIntelligentTieringConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tierings".into(),
                    value: &tierings_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketIntelligentTieringConfigurationResult {
            bucket: o.get_field("bucket"),
            filter: o.get_field("filter"),
            name: o.get_field("name"),
            status: o.get_field("status"),
            tierings: o.get_field("tierings"),
        }
    }
}
