/// Represents an inventory report configuration.
///
///
/// To get more information about ReportConfig, see:
///
/// * [API documentation](https://cloud.google.com/storage/docs/json_api/v1/reportConfig)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/storage/docs/insights/using-storage-insights)
///
/// ## Example Usage
///
/// ### Storage Insights Report Config
///
///
/// ```yaml
/// resources:
///   config:
///     type: gcp:storage:InsightsReportConfig
///     properties:
///       displayName: Test Report Config
///       location: us-central1
///       frequencyOptions:
///         frequency: WEEKLY
///         startDate:
///           day: 15
///           month: 3
///           year: 2050
///         endDate:
///           day: 15
///           month: 4
///           year: 2050
///       csvOptions:
///         recordSeparator: |2+
///         delimiter: ','
///         headerRequired: false
///       objectMetadataReportOptions:
///         metadataFields:
///           - bucket
///           - name
///           - project
///         storageFilters:
///           bucket: ${reportBucket.name}
///         storageDestinationOptions:
///           bucket: ${reportBucket.name}
///           destinationPath: test-insights-reports
///     options:
///       dependsOn:
///         - ${admin}
///   reportBucket:
///     type: gcp:storage:Bucket
///     name: report_bucket
///     properties:
///       name: my-bucket
///       location: us-central1
///       forceDestroy: true
///       uniformBucketLevelAccess: true
///   admin:
///     type: gcp:storage:BucketIAMMember
///     properties:
///       bucket: ${reportBucket.name}
///       role: roles/storage.admin
///       member: serviceAccount:service-${project.number}@gcp-sa-storageinsights.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// ReportConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/reportConfigs/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, ReportConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:storage/insightsReportConfig:InsightsReportConfig default projects/{{project}}/locations/{{location}}/reportConfigs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:storage/insightsReportConfig:InsightsReportConfig default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:storage/insightsReportConfig:InsightsReportConfig default {{location}}/{{name}}
/// ```
///
pub mod insights_report_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InsightsReportConfigArgs {
        /// Options for configuring the format of the inventory report CSV file.
        /// Structure is documented below.
        #[builder(into)]
        pub csv_options: pulumi_wasm_rust::Output<
            super::super::types::storage::InsightsReportConfigCsvOptions,
        >,
        /// The editable display name of the inventory report configuration. Has a limit of 256 characters. Can be empty.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Options for configuring how inventory reports are generated.
        #[builder(into, default)]
        pub frequency_options: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::InsightsReportConfigFrequencyOptions>,
        >,
        /// The location of the ReportConfig. The source and destination buckets specified in the ReportConfig
        /// must be in the same location.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Options for including metadata in an inventory report.
        #[builder(into, default)]
        pub object_metadata_report_options: pulumi_wasm_rust::Output<
            Option<
                super::super::types::storage::InsightsReportConfigObjectMetadataReportOptions,
            >,
        >,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InsightsReportConfigResult {
        /// Options for configuring the format of the inventory report CSV file.
        /// Structure is documented below.
        pub csv_options: pulumi_wasm_rust::Output<
            super::super::types::storage::InsightsReportConfigCsvOptions,
        >,
        /// The editable display name of the inventory report configuration. Has a limit of 256 characters. Can be empty.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Options for configuring how inventory reports are generated.
        pub frequency_options: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::InsightsReportConfigFrequencyOptions>,
        >,
        /// The location of the ReportConfig. The source and destination buckets specified in the ReportConfig
        /// must be in the same location.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The UUID of the inventory report configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Options for including metadata in an inventory report.
        pub object_metadata_report_options: pulumi_wasm_rust::Output<
            Option<
                super::super::types::storage::InsightsReportConfigObjectMetadataReportOptions,
            >,
        >,
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: InsightsReportConfigArgs,
    ) -> InsightsReportConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let csv_options_binding = args.csv_options.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let frequency_options_binding = args.frequency_options.get_inner();
        let location_binding = args.location.get_inner();
        let object_metadata_report_options_binding = args
            .object_metadata_report_options
            .get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:storage/insightsReportConfig:InsightsReportConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "csvOptions".into(),
                    value: &csv_options_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "frequencyOptions".into(),
                    value: &frequency_options_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "objectMetadataReportOptions".into(),
                    value: &object_metadata_report_options_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "csvOptions".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "frequencyOptions".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "objectMetadataReportOptions".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InsightsReportConfigResult {
            csv_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("csvOptions").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            frequency_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frequencyOptions").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            object_metadata_report_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("objectMetadataReportOptions").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
        }
    }
}
