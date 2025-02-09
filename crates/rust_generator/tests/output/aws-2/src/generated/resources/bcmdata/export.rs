/// Resource for managing an AWS BCM Data Exports Export.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:bcmdata:Export
///     properties:
///       export:
///         name: testexample
///         dataQueries:
///           - queryStatement: SELECT identity_line_item_id, identity_time_interval, line_item_product_code,line_item_unblended_cost FROM COST_AND_USAGE_REPORT
///             tableConfigurations:
///               COST_AND_USAGE_REPORT:
///                 TIME_GRANULARITY: HOURLY
///                 INCLUDE_RESOURCES: FALSE
///                 INCLUDE_MANUAL_DISCOUNT_COMPATIBILITY: FALSE
///                 INCLUDE_SPLIT_COST_ALLOCATION_DATA: FALSE
///         destinationConfigurations:
///           - s3Destinations:
///               - s3Bucket: ${testAwsS3Bucket.bucket}
///                 s3Prefix: ${testAwsS3Bucket.bucketPrefix}
///                 s3Region: ${testAwsS3Bucket.region}
///                 s3OutputConfigurations:
///                   - overwrite: OVERWRITE_REPORT
///                     format: TEXT_OR_CSV
///                     compression: GZIP
///                     outputType: CUSTOM
///         refreshCadences:
///           - frequency: SYNCHRONOUS
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import BCM Data Exports Export using the export ARN. For example:
///
/// ```sh
/// $ pulumi import aws:bcmdata/export:Export example arn:aws:bcm-data-exports:us-east-1:123456789012:export/CostUsageReport-9f1c75f3-f982-4d9a-b936-1e7ecab814b7
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod export {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExportArgs {
        /// The details of the export, including data query, name, description, and destination configuration.  See the `export` argument reference below.
        #[builder(into, default)]
        pub export: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bcmdata::ExportExport>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bcmdata::ExportTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ExportResult {
        /// The details of the export, including data query, name, description, and destination configuration.  See the `export` argument reference below.
        pub export: pulumi_gestalt_rust::Output<
            Option<super::super::types::bcmdata::ExportExport>,
        >,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::bcmdata::ExportTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExportArgs,
    ) -> ExportResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let export_binding = args.export.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:bcmdata/export:Export".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "export".into(),
                    value: export_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExportResult {
            export: o.get_field("export"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
