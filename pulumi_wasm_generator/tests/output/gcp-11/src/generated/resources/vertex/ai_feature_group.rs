/// Vertex AI Feature Group.
///
///
/// To get more information about FeatureGroup, see:
///
/// * [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.featureGroups)
/// * How-to Guides
///     * [Creating a Feature Group](https://cloud.google.com/vertex-ai/docs/featurestore/latest/create-featuregroup)
///
/// ## Example Usage
///
/// ### Vertex Ai Feature Group
///
///
/// ```yaml
/// resources:
///   featureGroup:
///     type: gcp:vertex:AiFeatureGroup
///     name: feature_group
///     properties:
///       name: example_feature_group
///       description: A sample feature group
///       region: us-central1
///       labels:
///         label-one: value-one
///       bigQuery:
///         bigQuerySource:
///           inputUri: bq://${sampleTable.project}.${sampleTable.datasetId}.${sampleTable.tableId}
///         entityIdColumns:
///           - feature_id
///   sampleDataset:
///     type: gcp:bigquery:Dataset
///     name: sample_dataset
///     properties:
///       datasetId: job_load_dataset
///       friendlyName: test
///       description: This is a test description
///       location: US
///   sampleTable:
///     type: gcp:bigquery:Table
///     name: sample_table
///     properties:
///       deletionProtection: false
///       datasetId: ${sampleDataset.datasetId}
///       tableId: job_load_table
///       schema: |
///         [
///             {
///                 "name": "feature_id",
///                 "type": "STRING",
///                 "mode": "NULLABLE"
///             },
///             {
///                 "name": "feature_timestamp",
///                 "type": "TIMESTAMP",
///                 "mode": "NULLABLE"
///             }
///         ]
/// ```
///
/// ## Import
///
/// FeatureGroup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/featureGroups/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, FeatureGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureGroup:AiFeatureGroup default projects/{{project}}/locations/{{region}}/featureGroups/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureGroup:AiFeatureGroup default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureGroup:AiFeatureGroup default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureGroup:AiFeatureGroup default {{name}}
/// ```
///
pub mod ai_feature_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiFeatureGroupArgs {
        /// Indicates that features for this group come from BigQuery Table/View. By default treats the source as a sparse time series source, which is required to have an entityId and a feature_timestamp column in the source.
        /// Structure is documented below.
        #[builder(into, default)]
        pub big_query: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::vertex::AiFeatureGroupBigQuery>,
        >,
        /// The description of the FeatureGroup.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The labels with user-defined metadata to organize your FeatureGroup.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name of the Feature Group.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The region of feature group. eg us-central1
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AiFeatureGroupResult {
        /// Indicates that features for this group come from BigQuery Table/View. By default treats the source as a sparse time series source, which is required to have an entityId and a feature_timestamp column in the source.
        /// Structure is documented below.
        pub big_query: pulumi_wasm_rust::Output<
            Option<super::super::types::vertex::AiFeatureGroupBigQuery>,
        >,
        /// The timestamp of when the FeatureGroup was created in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The description of the FeatureGroup.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Used to perform consistent read-modify-write updates.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The labels with user-defined metadata to organize your FeatureGroup.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name of the Feature Group.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of feature group. eg us-central1
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The timestamp of when the FeatureGroup was last updated in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AiFeatureGroupArgs,
    ) -> AiFeatureGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let big_query_binding = args.big_query.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vertex/aiFeatureGroup:AiFeatureGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bigQuery".into(),
                    value: &big_query_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AiFeatureGroupResult {
            big_query: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bigQuery"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
