/// Vertex AI Feature Group Feature is feature metadata information.
///
///
/// To get more information about FeatureGroupFeature, see:
///
/// * [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.featureGroups.features)
/// * How-to Guides
///     * [Creating a Feature](https://cloud.google.com/vertex-ai/docs/featurestore/latest/create-feature)
///
/// ## Example Usage
///
/// ### Vertex Ai Feature Group Feature
///
///
/// ```yaml
/// resources:
///   featureGroupFeature:
///     type: gcp:vertex:AiFeatureGroupFeature
///     name: feature_group_feature
///     properties:
///       name: example_feature
///       region: us-central1
///       featureGroup: ${sampleFeatureGroup.name}
///       description: A sample feature
///       labels:
///         label-one: value-one
///   sampleFeatureGroup:
///     type: gcp:vertex:AiFeatureGroup
///     name: sample_feature_group
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
///                 "name": "example_feature",
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
/// FeatureGroupFeature can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/featureGroups/{{feature_group}}/features/{{name}}`
///
/// * `{{project}}/{{region}}/{{feature_group}}/{{name}}`
///
/// * `{{region}}/{{feature_group}}/{{name}}`
///
/// * `{{feature_group}}/{{name}}`
///
/// When using the `pulumi import` command, FeatureGroupFeature can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureGroupFeature:AiFeatureGroupFeature default projects/{{project}}/locations/{{region}}/featureGroups/{{feature_group}}/features/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureGroupFeature:AiFeatureGroupFeature default {{project}}/{{region}}/{{feature_group}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureGroupFeature:AiFeatureGroupFeature default {{region}}/{{feature_group}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureGroupFeature:AiFeatureGroupFeature default {{feature_group}}/{{name}}
/// ```
///
pub mod ai_feature_group_feature {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiFeatureGroupFeatureArgs {
        /// The description of the FeatureGroup.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Feature Group.
        #[builder(into)]
        pub feature_group: pulumi_wasm_rust::Output<String>,
        /// The labels with user-defined metadata to organize your FeatureGroup.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name of the Feature Group Feature.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region for the resource. It should be the same as the feature group's region.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub region: pulumi_wasm_rust::Output<String>,
        /// The name of the BigQuery Table/View column hosting data for this version. If no value is provided, will use featureId.
        #[builder(into, default)]
        pub version_column_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AiFeatureGroupFeatureResult {
        /// The timestamp of when the FeatureGroup was created in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The description of the FeatureGroup.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the Feature Group.
        pub feature_group: pulumi_wasm_rust::Output<String>,
        /// The labels with user-defined metadata to organize your FeatureGroup.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name of the Feature Group Feature.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region for the resource. It should be the same as the feature group's region.
        ///
        ///
        /// - - -
        pub region: pulumi_wasm_rust::Output<String>,
        /// The timestamp of when the FeatureGroup was last updated in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// The name of the BigQuery Table/View column hosting data for this version. If no value is provided, will use featureId.
        pub version_column_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AiFeatureGroupFeatureArgs,
    ) -> AiFeatureGroupFeatureResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let feature_group_binding = args.feature_group.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let version_column_name_binding = args.version_column_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vertex/aiFeatureGroupFeature:AiFeatureGroupFeature".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "featureGroup".into(),
                    value: &feature_group_binding,
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
                register_interface::ObjectField {
                    name: "versionColumnName".into(),
                    value: &version_column_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "featureGroup".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "versionColumnName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AiFeatureGroupFeatureResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            feature_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("featureGroup").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            version_column_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionColumnName").unwrap(),
            ),
        }
    }
}
