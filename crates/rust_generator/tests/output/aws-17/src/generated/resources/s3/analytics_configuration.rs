/// Provides a S3 bucket [analytics configuration](https://docs.aws.amazon.com/AmazonS3/latest/dev/analytics-storage-class.html) resource.
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ### Add analytics configuration for entire S3 bucket and export results to a second S3 bucket
///
/// ```yaml
/// resources:
///   example-entire-bucket:
///     type: aws:s3:AnalyticsConfiguration
///     properties:
///       bucket: ${example.id}
///       name: EntireBucket
///       storageClassAnalysis:
///         dataExport:
///           destination:
///             s3BucketDestination:
///               bucketArn: ${analytics.arn}
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example
///   analytics:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: analytics-destination
/// ```
///
/// ### Add analytics configuration with S3 object filter
///
/// ```yaml
/// resources:
///   example-filtered:
///     type: aws:s3:AnalyticsConfiguration
///     properties:
///       bucket: ${example.id}
///       name: ImportantBlueDocuments
///       filter:
///         prefix: documents/
///         tags:
///           priority: high
///           class: blue
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 bucket analytics configurations using `bucket:analytics`. For example:
///
/// ```sh
/// $ pulumi import aws:s3/analyticsConfiguration:AnalyticsConfiguration my-bucket-entire-bucket my-bucket:EntireBucket
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod analytics_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AnalyticsConfigurationArgs {
        /// Name of the bucket this analytics configuration is associated with.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Object filtering that accepts a prefix, tags, or a logical AND of prefix and tags (documented below).
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::AnalyticsConfigurationFilter>,
        >,
        /// Unique identifier of the analytics configuration for the bucket.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for the analytics data export (documented below).
        #[builder(into, default)]
        pub storage_class_analysis: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::AnalyticsConfigurationStorageClassAnalysis>,
        >,
    }
    #[allow(dead_code)]
    pub struct AnalyticsConfigurationResult {
        /// Name of the bucket this analytics configuration is associated with.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Object filtering that accepts a prefix, tags, or a logical AND of prefix and tags (documented below).
        pub filter: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3::AnalyticsConfigurationFilter>,
        >,
        /// Unique identifier of the analytics configuration for the bucket.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration for the analytics data export (documented below).
        pub storage_class_analysis: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3::AnalyticsConfigurationStorageClassAnalysis>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AnalyticsConfigurationArgs,
    ) -> AnalyticsConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let name_binding = args.name.get_output(context);
        let storage_class_analysis_binding = args
            .storage_class_analysis
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/analyticsConfiguration:AnalyticsConfiguration".into(),
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
                    name: "storageClassAnalysis".into(),
                    value: &storage_class_analysis_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AnalyticsConfigurationResult {
            bucket: o.get_field("bucket"),
            filter: o.get_field("filter"),
            name: o.get_field("name"),
            storage_class_analysis: o.get_field("storageClassAnalysis"),
        }
    }
}
