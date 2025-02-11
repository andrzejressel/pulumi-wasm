/// Represents a user-visible job which provides the insights for the related data source.
///
///
/// To get more information about Datascan, see:
///
/// * [API documentation](https://cloud.google.com/dataplex/docs/reference/rest)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dataplex/docs)
///
/// ## Example Usage
///
/// ### Dataplex Datascan Basic Profile
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicProfile = datascan::create(
///         "basicProfile",
///         DatascanArgs::builder()
///             .data(
///                 DatascanData::builder()
///                     .resource(
///                         "//bigquery.googleapis.com/projects/bigquery-public-data/datasets/samples/tables/shakespeare",
///                     )
///                     .build_struct(),
///             )
///             .data_profile_spec(DatascanDataProfileSpec::builder().build_struct())
///             .data_scan_id("dataprofile-basic")
///             .execution_spec(
///                 DatascanExecutionSpec::builder()
///                     .trigger(
///                         DatascanExecutionSpecTrigger::builder()
///                             .onDemand(
///                                 DatascanExecutionSpecTriggerOnDemand::builder()
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .project("my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dataplex Datascan Full Profile
///
///
/// ```yaml
/// resources:
///   fullProfile:
///     type: gcp:dataplex:Datascan
///     name: full_profile
///     properties:
///       location: us-central1
///       displayName: Full Datascan Profile
///       dataScanId: dataprofile-full
///       description: Example resource - Full Datascan Profile
///       labels:
///         author: billing
///       data:
///         resource: //bigquery.googleapis.com/projects/bigquery-public-data/datasets/samples/tables/shakespeare
///       executionSpec:
///         trigger:
///           schedule:
///             cron: TZ=America/New_York 1 1 * * *
///       dataProfileSpec:
///         samplingPercent: 80
///         rowFilter: word_count > 10
///         includeFields:
///           fieldNames:
///             - word_count
///         excludeFields:
///           fieldNames:
///             - property_type
///         postScanActions:
///           bigqueryExport:
///             resultsTable: //bigquery.googleapis.com/projects/my-project-name/datasets/dataplex_dataset/tables/profile_export
///       project: my-project-name
///     options:
///       dependsOn:
///         - ${source}
///   source:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: dataplex_dataset
///       friendlyName: test
///       description: This is a test description
///       location: US
///       deleteContentsOnDestroy: true
/// ```
/// ### Dataplex Datascan Basic Quality
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicQuality = datascan::create(
///         "basicQuality",
///         DatascanArgs::builder()
///             .data(
///                 DatascanData::builder()
///                     .resource(
///                         "//bigquery.googleapis.com/projects/bigquery-public-data/datasets/samples/tables/shakespeare",
///                     )
///                     .build_struct(),
///             )
///             .data_quality_spec(
///                 DatascanDataQualitySpec::builder()
///                     .rules(
///                         vec![
///                             DatascanDataQualitySpecRule::builder()
///                             .description("rule 1 for validity dimension")
///                             .dimension("VALIDITY").name("rule1")
///                             .tableConditionExpectation(DatascanDataQualitySpecRuleTableConditionExpectation::builder()
///                             .sqlExpression("COUNT(*) > 0").build_struct())
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .data_scan_id("dataquality-basic")
///             .execution_spec(
///                 DatascanExecutionSpec::builder()
///                     .trigger(
///                         DatascanExecutionSpecTrigger::builder()
///                             .onDemand(
///                                 DatascanExecutionSpecTriggerOnDemand::builder()
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .project("my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dataplex Datascan Full Quality
///
///
/// ```yaml
/// resources:
///   fullQuality:
///     type: gcp:dataplex:Datascan
///     name: full_quality
///     properties:
///       location: us-central1
///       displayName: Full Datascan Quality
///       dataScanId: dataquality-full
///       description: Example resource - Full Datascan Quality
///       labels:
///         author: billing
///       data:
///         resource: //bigquery.googleapis.com/projects/bigquery-public-data/datasets/austin_bikeshare/tables/bikeshare_stations
///       executionSpec:
///         trigger:
///           schedule:
///             cron: TZ=America/New_York 1 1 * * *
///         field: modified_date
///       dataQualitySpec:
///         samplingPercent: 5
///         rowFilter: station_id > 1000
///         rules:
///           - column: address
///             dimension: VALIDITY
///             threshold: 0.99
///             nonNullExpectation: {}
///           - column: council_district
///             dimension: VALIDITY
///             ignoreNull: true
///             threshold: 0.9
///             rangeExpectation:
///               minValue: 1
///               maxValue: 10
///               strictMinEnabled: true
///               strictMaxEnabled: false
///           - column: power_type
///             dimension: VALIDITY
///             ignoreNull: false
///             regexExpectation:
///               regex: .*solar.*
///           - column: property_type
///             dimension: VALIDITY
///             ignoreNull: false
///             setExpectation:
///               values:
///                 - sidewalk
///                 - parkland
///           - column: address
///             dimension: UNIQUENESS
///             uniquenessExpectation: {}
///           - column: number_of_docks
///             dimension: VALIDITY
///             statisticRangeExpectation:
///               statistic: MEAN
///               minValue: 5
///               maxValue: 15
///               strictMinEnabled: true
///               strictMaxEnabled: true
///           - column: footprint_length
///             dimension: VALIDITY
///             rowConditionExpectation:
///               sqlExpression: footprint_length > 0 AND footprint_length <= 10
///           - dimension: VALIDITY
///             tableConditionExpectation:
///               sqlExpression: COUNT(*) > 0
///           - dimension: VALIDITY
///             sqlAssertion:
///               sqlStatement: select * from bigquery-public-data.austin_bikeshare.bikeshare_stations where station_id is null
///       project: my-project-name
/// ```
///
/// ## Import
///
/// Datascan can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/dataScans/{{data_scan_id}}`
///
/// * `{{project}}/{{location}}/{{data_scan_id}}`
///
/// * `{{location}}/{{data_scan_id}}`
///
/// * `{{data_scan_id}}`
///
/// When using the `pulumi import` command, Datascan can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataplex/datascan:Datascan default projects/{{project}}/locations/{{location}}/dataScans/{{data_scan_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/datascan:Datascan default {{project}}/{{location}}/{{data_scan_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/datascan:Datascan default {{location}}/{{data_scan_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/datascan:Datascan default {{data_scan_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod datascan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatascanArgs {
        /// The data source for DataScan.
        /// Structure is documented below.
        #[builder(into)]
        pub data: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::dataplex::DatascanData,
        >,
        /// DataProfileScan related setting.
        #[builder(into, default)]
        pub data_profile_spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataplex::DatascanDataProfileSpec>,
        >,
        /// DataQualityScan related setting.
        #[builder(into, default)]
        pub data_quality_spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataplex::DatascanDataQualitySpec>,
        >,
        /// DataScan identifier. Must contain only lowercase letters, numbers and hyphens. Must start with a letter. Must end with a number or a letter.
        #[builder(into)]
        pub data_scan_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the scan.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User friendly display name.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// DataScan execution settings.
        /// Structure is documented below.
        #[builder(into)]
        pub execution_spec: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::dataplex::DatascanExecutionSpec,
        >,
        /// User-defined labels for the scan. A list of key->value pairs. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the data scan should reside.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DatascanResult {
        /// The time when the scan was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The data source for DataScan.
        /// Structure is documented below.
        pub data: pulumi_gestalt_rust::Output<
            super::super::types::dataplex::DatascanData,
        >,
        /// DataProfileScan related setting.
        pub data_profile_spec: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataplex::DatascanDataProfileSpec>,
        >,
        /// DataQualityScan related setting.
        pub data_quality_spec: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataplex::DatascanDataQualitySpec>,
        >,
        /// DataScan identifier. Must contain only lowercase letters, numbers and hyphens. Must start with a letter. Must end with a number or a letter.
        pub data_scan_id: pulumi_gestalt_rust::Output<String>,
        /// Description of the scan.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// User friendly display name.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// DataScan execution settings.
        /// Structure is documented below.
        pub execution_spec: pulumi_gestalt_rust::Output<
            super::super::types::dataplex::DatascanExecutionSpec,
        >,
        /// Status of the data scan execution.
        /// Structure is documented below.
        pub execution_statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::dataplex::DatascanExecutionStatus>,
        >,
        /// User-defined labels for the scan. A list of key->value pairs. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the data scan should reside.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The relative resource name of the scan, of the form: projects/{project}/locations/{locationId}/dataScans/{datascan_id}, where project refers to a project_id or project_number and locationId refers to a GCP region.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Current state of the DataScan.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The type of DataScan.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// System generated globally unique ID for the scan. This ID will be different if the scan is deleted and re-created with the same name.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The time when the scan was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DatascanArgs,
    ) -> DatascanResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_binding = args.data.get_output(context);
        let data_profile_spec_binding = args.data_profile_spec.get_output(context);
        let data_quality_spec_binding = args.data_quality_spec.get_output(context);
        let data_scan_id_binding = args.data_scan_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let execution_spec_binding = args.execution_spec.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataplex/datascan:Datascan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "data".into(),
                    value: &data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataProfileSpec".into(),
                    value: &data_profile_spec_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataQualitySpec".into(),
                    value: &data_quality_spec_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataScanId".into(),
                    value: &data_scan_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "executionSpec".into(),
                    value: &execution_spec_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatascanResult {
            create_time: o.get_field("createTime"),
            data: o.get_field("data"),
            data_profile_spec: o.get_field("dataProfileSpec"),
            data_quality_spec: o.get_field("dataQualitySpec"),
            data_scan_id: o.get_field("dataScanId"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            execution_spec: o.get_field("executionSpec"),
            execution_statuses: o.get_field("executionStatuses"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            state: o.get_field("state"),
            type_: o.get_field("type"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
