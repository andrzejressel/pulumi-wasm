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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReportDefinitionArgs {
        /// A list of additional artifacts. Valid values are: `REDSHIFT`, `QUICKSIGHT`, `ATHENA`. When ATHENA exists within additional_artifacts, no other artifact type can be declared and report_versioning must be `OVERWRITE_REPORT`.
        #[builder(into, default)]
        pub additional_artifacts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of schema elements. Valid values are: `RESOURCES`, `SPLIT_COST_ALLOCATION_DATA`.
        #[builder(into)]
        pub additional_schema_elements: pulumi_wasm_rust::Output<Vec<String>>,
        /// Compression format for report. Valid values are: `GZIP`, `ZIP`, `Parquet`. If `Parquet` is used, then format must also be `Parquet`.
        #[builder(into)]
        pub compression: pulumi_wasm_rust::Output<String>,
        /// Format for report. Valid values are: `textORcsv`, `Parquet`. If `Parquet` is used, then Compression must also be `Parquet`.
        #[builder(into)]
        pub format: pulumi_wasm_rust::Output<String>,
        /// Set to true to update your reports after they have been finalized if AWS detects charges related to previous months.
        #[builder(into, default)]
        pub refresh_closed_reports: pulumi_wasm_rust::Output<Option<bool>>,
        /// Unique name for the report. Must start with a number/letter and is case sensitive. Limited to 256 characters.
        #[builder(into)]
        pub report_name: pulumi_wasm_rust::Output<String>,
        /// Overwrite the previous version of each report or to deliver the report in addition to the previous versions. Valid values are: `CREATE_NEW_REPORT` and `OVERWRITE_REPORT`.
        #[builder(into, default)]
        pub report_versioning: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the existing S3 bucket to hold generated reports.
        #[builder(into)]
        pub s3_bucket: pulumi_wasm_rust::Output<String>,
        /// Report path prefix. Limited to 256 characters.
        #[builder(into, default)]
        pub s3_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Region of the existing S3 bucket to hold generated reports.
        #[builder(into)]
        pub s3_region: pulumi_wasm_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The frequency on which report data are measured and displayed.  Valid values are: `DAILY`, `HOURLY`, `MONTHLY`.
        #[builder(into)]
        pub time_unit: pulumi_wasm_rust::Output<String>,
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
    pub fn create(name: &str, args: ReportDefinitionArgs) -> ReportDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_artifacts_binding = args.additional_artifacts.get_inner();
        let additional_schema_elements_binding = args
            .additional_schema_elements
            .get_inner();
        let compression_binding = args.compression.get_inner();
        let format_binding = args.format.get_inner();
        let refresh_closed_reports_binding = args.refresh_closed_reports.get_inner();
        let report_name_binding = args.report_name.get_inner();
        let report_versioning_binding = args.report_versioning.get_inner();
        let s3_bucket_binding = args.s3_bucket.get_inner();
        let s3_prefix_binding = args.s3_prefix.get_inner();
        let s3_region_binding = args.s3_region.get_inner();
        let tags_binding = args.tags.get_inner();
        let time_unit_binding = args.time_unit.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cur/reportDefinition:ReportDefinition".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalArtifacts".into(),
                },
                register_interface::ResultField {
                    name: "additionalSchemaElements".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "compression".into(),
                },
                register_interface::ResultField {
                    name: "format".into(),
                },
                register_interface::ResultField {
                    name: "refreshClosedReports".into(),
                },
                register_interface::ResultField {
                    name: "reportName".into(),
                },
                register_interface::ResultField {
                    name: "reportVersioning".into(),
                },
                register_interface::ResultField {
                    name: "s3Bucket".into(),
                },
                register_interface::ResultField {
                    name: "s3Prefix".into(),
                },
                register_interface::ResultField {
                    name: "s3Region".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeUnit".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReportDefinitionResult {
            additional_artifacts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalArtifacts").unwrap(),
            ),
            additional_schema_elements: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalSchemaElements").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            compression: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("compression").unwrap(),
            ),
            format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("format").unwrap(),
            ),
            refresh_closed_reports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("refreshClosedReports").unwrap(),
            ),
            report_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reportName").unwrap(),
            ),
            report_versioning: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reportVersioning").unwrap(),
            ),
            s3_bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3Bucket").unwrap(),
            ),
            s3_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3Prefix").unwrap(),
            ),
            s3_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3Region").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            time_unit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeUnit").unwrap(),
            ),
        }
    }
}