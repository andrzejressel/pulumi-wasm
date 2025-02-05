pub mod get_report_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReportDefinitionArgs {
        /// Name of the report definition to match.
        #[builder(into)]
        pub report_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Map of key-value pairs assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetReportDefinitionResult {
        /// A list of additional artifacts.
        pub additional_artifacts: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of schema elements.
        pub additional_schema_elements: pulumi_wasm_rust::Output<Vec<String>>,
        /// Preferred format for report.
        pub compression: pulumi_wasm_rust::Output<String>,
        /// Preferred compression format for report.
        pub format: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// If true reports are updated after they have been finalized.
        pub refresh_closed_reports: pulumi_wasm_rust::Output<bool>,
        pub report_name: pulumi_wasm_rust::Output<String>,
        /// Overwrite the previous version of each report or to deliver the report in addition to the previous versions.
        pub report_versioning: pulumi_wasm_rust::Output<String>,
        /// Name of customer S3 bucket.
        pub s3_bucket: pulumi_wasm_rust::Output<String>,
        /// Preferred report path prefix.
        pub s3_prefix: pulumi_wasm_rust::Output<String>,
        /// Region of customer S3 bucket.
        pub s3_region: pulumi_wasm_rust::Output<String>,
        /// Map of key-value pairs assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Frequency on which report data are measured and displayed.
        pub time_unit: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetReportDefinitionArgs,
    ) -> GetReportDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let report_name_binding = args.report_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cur/getReportDefinition:getReportDefinition".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "reportName".into(),
                    value: &report_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetReportDefinitionResult {
            additional_artifacts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("additionalArtifacts"),
            ),
            additional_schema_elements: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("additionalSchemaElements"),
            ),
            compression: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("compression"),
            ),
            format: pulumi_wasm_rust::__private::into_domain(o.extract_field("format")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
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
            time_unit: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeUnit"),
            ),
        }
    }
}
