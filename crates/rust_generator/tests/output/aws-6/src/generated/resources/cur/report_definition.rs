/// Manages Cost and Usage Report Definitions.
///
/// > *NOTE:* The AWS Cost and Usage Report service is only available in `us-east-1` currently.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let exampleCurReportDefinition = report_definition::create(
///         "exampleCurReportDefinition",
///         ReportDefinitionArgs::builder()
///             .additional_artifacts(vec!["REDSHIFT", "QUICKSIGHT",])
///             .additional_schema_elements(vec!["RESOURCES", "SPLIT_COST_ALLOCATION_DATA",])
///             .compression("GZIP")
///             .format("textORcsv")
///             .report_name("example-cur-report-definition")
///             .s_3_bucket("example-bucket-name")
///             .s_3_region("us-east-1")
///             .time_unit("HOURLY")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Report Definitions using the `report_name`. For example:
///
/// ```sh
/// $ pulumi import aws:cur/reportDefinition:ReportDefinition example_cur_report_definition example-cur-report-definition
/// ```
pub mod report_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReportDefinitionArgs {
        /// A list of additional artifacts. Valid values are: `REDSHIFT`, `QUICKSIGHT`, `ATHENA`. When ATHENA exists within additional_artifacts, no other artifact type can be declared and report_versioning must be `OVERWRITE_REPORT`.
        #[builder(into, default)]
        pub additional_artifacts: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of schema elements. Valid values are: `RESOURCES`, `SPLIT_COST_ALLOCATION_DATA`.
        #[builder(into)]
        pub additional_schema_elements: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Compression format for report. Valid values are: `GZIP`, `ZIP`, `Parquet`. If `Parquet` is used, then format must also be `Parquet`.
        #[builder(into)]
        pub compression: pulumi_wasm_rust::InputOrOutput<String>,
        /// Format for report. Valid values are: `textORcsv`, `Parquet`. If `Parquet` is used, then Compression must also be `Parquet`.
        #[builder(into)]
        pub format: pulumi_wasm_rust::InputOrOutput<String>,
        /// Set to true to update your reports after they have been finalized if AWS detects charges related to previous months.
        #[builder(into, default)]
        pub refresh_closed_reports: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Unique name for the report. Must start with a number/letter and is case sensitive. Limited to 256 characters.
        #[builder(into)]
        pub report_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Overwrite the previous version of each report or to deliver the report in addition to the previous versions. Valid values are: `CREATE_NEW_REPORT` and `OVERWRITE_REPORT`.
        #[builder(into, default)]
        pub report_versioning: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the existing S3 bucket to hold generated reports.
        #[builder(into)]
        pub s3_bucket: pulumi_wasm_rust::InputOrOutput<String>,
        /// Report path prefix. Limited to 256 characters.
        #[builder(into, default)]
        pub s3_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Region of the existing S3 bucket to hold generated reports.
        #[builder(into)]
        pub s3_region: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The frequency on which report data are measured and displayed.  Valid values are: `DAILY`, `HOURLY`, `MONTHLY`.
        #[builder(into)]
        pub time_unit: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ReportDefinitionResult {
        /// A list of additional artifacts. Valid values are: `REDSHIFT`, `QUICKSIGHT`, `ATHENA`. When ATHENA exists within additional_artifacts, no other artifact type can be declared and report_versioning must be `OVERWRITE_REPORT`.
        pub additional_artifacts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of schema elements. Valid values are: `RESOURCES`, `SPLIT_COST_ALLOCATION_DATA`.
        pub additional_schema_elements: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Amazon Resource Name (ARN) specifying the cur report.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Compression format for report. Valid values are: `GZIP`, `ZIP`, `Parquet`. If `Parquet` is used, then format must also be `Parquet`.
        pub compression: pulumi_wasm_rust::Output<String>,
        /// Format for report. Valid values are: `textORcsv`, `Parquet`. If `Parquet` is used, then Compression must also be `Parquet`.
        pub format: pulumi_wasm_rust::Output<String>,
        /// Set to true to update your reports after they have been finalized if AWS detects charges related to previous months.
        pub refresh_closed_reports: pulumi_wasm_rust::Output<Option<bool>>,
        /// Unique name for the report. Must start with a number/letter and is case sensitive. Limited to 256 characters.
        pub report_name: pulumi_wasm_rust::Output<String>,
        /// Overwrite the previous version of each report or to deliver the report in addition to the previous versions. Valid values are: `CREATE_NEW_REPORT` and `OVERWRITE_REPORT`.
        pub report_versioning: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the existing S3 bucket to hold generated reports.
        pub s3_bucket: pulumi_wasm_rust::Output<String>,
        /// Report path prefix. Limited to 256 characters.
        pub s3_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Region of the existing S3 bucket to hold generated reports.
        pub s3_region: pulumi_wasm_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The frequency on which report data are measured and displayed.  Valid values are: `DAILY`, `HOURLY`, `MONTHLY`.
        pub time_unit: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ReportDefinitionArgs,
    ) -> ReportDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_artifacts_binding = args
            .additional_artifacts
            .get_output(context)
            .get_inner();
        let additional_schema_elements_binding = args
            .additional_schema_elements
            .get_output(context)
            .get_inner();
        let compression_binding = args.compression.get_output(context).get_inner();
        let format_binding = args.format.get_output(context).get_inner();
        let refresh_closed_reports_binding = args
            .refresh_closed_reports
            .get_output(context)
            .get_inner();
        let report_name_binding = args.report_name.get_output(context).get_inner();
        let report_versioning_binding = args
            .report_versioning
            .get_output(context)
            .get_inner();
        let s3_bucket_binding = args.s3_bucket.get_output(context).get_inner();
        let s3_prefix_binding = args.s3_prefix.get_output(context).get_inner();
        let s3_region_binding = args.s3_region.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let time_unit_binding = args.time_unit.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cur/reportDefinition:ReportDefinition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalArtifacts".into(),
                    value: &additional_artifacts_binding,
                },
                register_interface::ObjectField {
                    name: "additionalSchemaElements".into(),
                    value: &additional_schema_elements_binding,
                },
                register_interface::ObjectField {
                    name: "compression".into(),
                    value: &compression_binding,
                },
                register_interface::ObjectField {
                    name: "format".into(),
                    value: &format_binding,
                },
                register_interface::ObjectField {
                    name: "refreshClosedReports".into(),
                    value: &refresh_closed_reports_binding,
                },
                register_interface::ObjectField {
                    name: "reportName".into(),
                    value: &report_name_binding,
                },
                register_interface::ObjectField {
                    name: "reportVersioning".into(),
                    value: &report_versioning_binding,
                },
                register_interface::ObjectField {
                    name: "s3Bucket".into(),
                    value: &s3_bucket_binding,
                },
                register_interface::ObjectField {
                    name: "s3Prefix".into(),
                    value: &s3_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "s3Region".into(),
                    value: &s3_region_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeUnit".into(),
                    value: &time_unit_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ReportDefinitionResult {
            additional_artifacts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("additionalArtifacts"),
            ),
            additional_schema_elements: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("additionalSchemaElements"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            compression: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("compression"),
            ),
            format: pulumi_wasm_rust::__private::into_domain(o.extract_field("format")),
            refresh_closed_reports: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("refreshClosedReports"),
            ),
            report_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reportName"),
            ),
            report_versioning: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reportVersioning"),
            ),
            s3_bucket: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("s3Bucket"),
            ),
            s3_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("s3Prefix"),
            ),
            s3_region: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("s3Region"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            time_unit: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeUnit"),
            ),
        }
    }
}
