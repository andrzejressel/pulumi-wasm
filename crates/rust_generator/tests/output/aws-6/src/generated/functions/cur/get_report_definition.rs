#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_report_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReportDefinitionArgs {
        /// Name of the report definition to match.
        #[builder(into)]
        pub report_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of key-value pairs assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetReportDefinitionResult {
        /// A list of additional artifacts.
        pub additional_artifacts: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of schema elements.
        pub additional_schema_elements: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Preferred format for report.
        pub compression: pulumi_gestalt_rust::Output<String>,
        /// Preferred compression format for report.
        pub format: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// If true reports are updated after they have been finalized.
        pub refresh_closed_reports: pulumi_gestalt_rust::Output<bool>,
        pub report_name: pulumi_gestalt_rust::Output<String>,
        /// Overwrite the previous version of each report or to deliver the report in addition to the previous versions.
        pub report_versioning: pulumi_gestalt_rust::Output<String>,
        /// Name of customer S3 bucket.
        pub s3_bucket: pulumi_gestalt_rust::Output<String>,
        /// Preferred report path prefix.
        pub s3_prefix: pulumi_gestalt_rust::Output<String>,
        /// Region of customer S3 bucket.
        pub s3_region: pulumi_gestalt_rust::Output<String>,
        /// Map of key-value pairs assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Frequency on which report data are measured and displayed.
        pub time_unit: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetReportDefinitionArgs,
    ) -> GetReportDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let report_name_binding = args.report_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cur/getReportDefinition:getReportDefinition".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reportName".into(),
                    value: report_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetReportDefinitionResult {
            additional_artifacts: o.get_field("additionalArtifacts"),
            additional_schema_elements: o.get_field("additionalSchemaElements"),
            compression: o.get_field("compression"),
            format: o.get_field("format"),
            id: o.get_field("id"),
            refresh_closed_reports: o.get_field("refreshClosedReports"),
            report_name: o.get_field("reportName"),
            report_versioning: o.get_field("reportVersioning"),
            s3_bucket: o.get_field("s3Bucket"),
            s3_prefix: o.get_field("s3Prefix"),
            s3_region: o.get_field("s3Region"),
            tags: o.get_field("tags"),
            time_unit: o.get_field("timeUnit"),
        }
    }
}
