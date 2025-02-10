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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ai_feature_group_feature {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiFeatureGroupFeatureArgs {
        /// The description of the FeatureGroup.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Feature Group.
        #[builder(into)]
        pub feature_group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The labels with user-defined metadata to organize your FeatureGroup.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name of the Feature Group Feature.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region for the resource. It should be the same as the feature group's region.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub region: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the BigQuery Table/View column hosting data for this version. If no value is provided, will use featureId.
        #[builder(into, default)]
        pub version_column_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AiFeatureGroupFeatureResult {
        /// The timestamp of when the FeatureGroup was created in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The description of the FeatureGroup.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the Feature Group.
        pub feature_group: pulumi_gestalt_rust::Output<String>,
        /// The labels with user-defined metadata to organize your FeatureGroup.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name of the Feature Group Feature.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region for the resource. It should be the same as the feature group's region.
        ///
        ///
        /// - - -
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The timestamp of when the FeatureGroup was last updated in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// The name of the BigQuery Table/View column hosting data for this version. If no value is provided, will use featureId.
        pub version_column_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AiFeatureGroupFeatureArgs,
    ) -> AiFeatureGroupFeatureResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let feature_group_binding = args.feature_group.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let version_column_name_binding = args.version_column_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:vertex/aiFeatureGroupFeature:AiFeatureGroupFeature".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "featureGroup".into(),
                    value: feature_group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionColumnName".into(),
                    value: version_column_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AiFeatureGroupFeatureResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            feature_group: o.get_field("featureGroup"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            update_time: o.get_field("updateTime"),
            version_column_name: o.get_field("versionColumnName"),
        }
    }
}
