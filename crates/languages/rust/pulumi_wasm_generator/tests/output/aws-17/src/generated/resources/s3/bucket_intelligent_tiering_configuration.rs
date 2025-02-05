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
pub mod bucket_intelligent_tiering_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketIntelligentTieringConfigurationArgs {
        /// Name of the bucket this intelligent tiering configuration is associated with.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::InputOrOutput<String>,
        /// Bucket filter. The configuration only includes objects that meet the filter's criteria (documented below).
        #[builder(into, default)]
        pub filter: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::s3::BucketIntelligentTieringConfigurationFilter>,
        >,
        /// Unique name used to identify the S3 Intelligent-Tiering configuration for the bucket.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the status of the configuration. Valid values: `Enabled`, `Disabled`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// S3 Intelligent-Tiering storage class tiers of the configuration (documented below).
        #[builder(into)]
        pub tierings: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::s3::BucketIntelligentTieringConfigurationTiering>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketIntelligentTieringConfigurationResult {
        /// Name of the bucket this intelligent tiering configuration is associated with.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Bucket filter. The configuration only includes objects that meet the filter's criteria (documented below).
        pub filter: pulumi_wasm_rust::Output<
            Option<super::super::types::s3::BucketIntelligentTieringConfigurationFilter>,
        >,
        /// Unique name used to identify the S3 Intelligent-Tiering configuration for the bucket.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the status of the configuration. Valid values: `Enabled`, `Disabled`.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// S3 Intelligent-Tiering storage class tiers of the configuration (documented below).
        pub tierings: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketIntelligentTieringConfigurationTiering>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BucketIntelligentTieringConfigurationArgs,
    ) -> BucketIntelligentTieringConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let filter_binding = args.filter.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let status_binding = args.status.get_output(context).get_inner();
        let tierings_binding = args.tierings.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketIntelligentTieringConfiguration:BucketIntelligentTieringConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tierings".into(),
                    value: &tierings_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BucketIntelligentTieringConfigurationResult {
            bucket: pulumi_wasm_rust::__private::into_domain(o.extract_field("bucket")),
            filter: pulumi_wasm_rust::__private::into_domain(o.extract_field("filter")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tierings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tierings"),
            ),
        }
    }
}
