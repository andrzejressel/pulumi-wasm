pub mod get_report_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReportDefinitionArgs {
        /// Name of the report definition to match.
        #[builder(into)]
        pub report_name: pulumi_wasm_rust::Output<String>,
        /// Map of key-value pairs assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
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
    pub fn invoke(args: GetReportDefinitionArgs) -> GetReportDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let report_name_binding = args.report_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cur/getReportDefinition:getReportDefinition".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalArtifacts".into(),
                },
                register_interface::ResultField {
                    name: "additionalSchemaElements".into(),
                },
                register_interface::ResultField {
                    name: "compression".into(),
                },
                register_interface::ResultField {
                    name: "format".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
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
                    name: "timeUnit".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetReportDefinitionResult {
            additional_artifacts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalArtifacts").unwrap(),
            ),
            additional_schema_elements: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalSchemaElements").unwrap(),
            ),
            compression: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("compression").unwrap(),
            ),
            format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("format").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
            time_unit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeUnit").unwrap(),
            ),
        }
    }
}
