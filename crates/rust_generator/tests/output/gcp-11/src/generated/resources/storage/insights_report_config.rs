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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod insights_report_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InsightsReportConfigArgs {
        /// Options for configuring the format of the inventory report CSV file.
        /// Structure is documented below.
        #[builder(into)]
        pub csv_options: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::storage::InsightsReportConfigCsvOptions,
        >,
        /// The editable display name of the inventory report configuration. Has a limit of 256 characters. Can be empty.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Options for configuring how inventory reports are generated.
        #[builder(into, default)]
        pub frequency_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::InsightsReportConfigFrequencyOptions>,
        >,
        /// The location of the ReportConfig. The source and destination buckets specified in the ReportConfig
        /// must be in the same location.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Options for including metadata in an inventory report.
        #[builder(into, default)]
        pub object_metadata_report_options: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::storage::InsightsReportConfigObjectMetadataReportOptions,
            >,
        >,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InsightsReportConfigResult {
        /// Options for configuring the format of the inventory report CSV file.
        /// Structure is documented below.
        pub csv_options: pulumi_gestalt_rust::Output<
            super::super::types::storage::InsightsReportConfigCsvOptions,
        >,
        /// The editable display name of the inventory report configuration. Has a limit of 256 characters. Can be empty.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Options for configuring how inventory reports are generated.
        pub frequency_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::storage::InsightsReportConfigFrequencyOptions>,
        >,
        /// The location of the ReportConfig. The source and destination buckets specified in the ReportConfig
        /// must be in the same location.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The UUID of the inventory report configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Options for including metadata in an inventory report.
        pub object_metadata_report_options: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::storage::InsightsReportConfigObjectMetadataReportOptions,
            >,
        >,
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InsightsReportConfigArgs,
    ) -> InsightsReportConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let csv_options_binding = args.csv_options.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let frequency_options_binding = args
            .frequency_options
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let object_metadata_report_options_binding = args
            .object_metadata_report_options
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        InsightsReportConfigResult {
            csv_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("csvOptions"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            frequency_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frequencyOptions"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            object_metadata_report_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("objectMetadataReportOptions"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
        }
    }
}
