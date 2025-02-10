/// A collection of DataItems and Annotations on them.
///
///
/// To get more information about Featurestore, see:
///
/// * [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.featurestores)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/vertex-ai/docs)
///
/// ## Example Usage
///
/// ### Vertex Ai Featurestore
///
///
/// ```yaml
/// resources:
///   featurestore:
///     type: gcp:vertex:AiFeatureStore
///     properties:
///       name: terraform
///       labels:
///         foo: bar
///       region: us-central1
///       onlineServingConfig:
///         fixedNodeCount: 2
///       encryptionSpec:
///         kmsKeyName: kms-name
///       forceDestroy: true
/// ```
/// ### Vertex Ai Featurestore With Beta Fields
///
///
/// ```yaml
/// resources:
///   featurestore:
///     type: gcp:vertex:AiFeatureStore
///     properties:
///       name: terraform2
///       labels:
///         foo: bar
///       region: us-central1
///       onlineServingConfig:
///         fixedNodeCount: 2
///       encryptionSpec:
///         kmsKeyName: kms-name
///       onlineStorageTtlDays: 30
///       forceDestroy: true
/// ```
/// ### Vertex Ai Featurestore Scaling
///
///
/// ```yaml
/// resources:
///   featurestore:
///     type: gcp:vertex:AiFeatureStore
///     properties:
///       name: terraform3
///       labels:
///         foo: bar
///       region: us-central1
///       onlineServingConfig:
///         scaling:
///           minNodeCount: 2
///           maxNodeCount: 10
///       encryptionSpec:
///         kmsKeyName: kms-name
///       forceDestroy: true
/// ```
///
/// ## Import
///
/// Featurestore can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/featurestores/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Featurestore can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureStore:AiFeatureStore default projects/{{project}}/locations/{{region}}/featurestores/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureStore:AiFeatureStore default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureStore:AiFeatureStore default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureStore:AiFeatureStore default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ai_feature_store {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiFeatureStoreArgs {
        /// If set, both of the online and offline data storage will be secured by this key.
        /// Structure is documented below.
        #[builder(into, default)]
        pub encryption_spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::vertex::AiFeatureStoreEncryptionSpec>,
        >,
        /// If set to true, any EntityTypes and Features for this Featurestore will also be deleted
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A set of key/value label pairs to assign to this Featurestore.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Featurestore. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Config for online serving resources.
        /// Structure is documented below.
        #[builder(into, default)]
        pub online_serving_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::vertex::AiFeatureStoreOnlineServingConfig>,
        >,
        /// TTL in days for feature values that will be stored in online serving storage. The Feature Store online storage periodically removes obsolete feature values older than onlineStorageTtlDays since the feature generation time. Note that onlineStorageTtlDays should be less than or equal to offlineStorageTtlDays for each EntityType under a featurestore. If not set, default to 4000 days
        #[builder(into, default)]
        pub online_storage_ttl_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the dataset. eg us-central1
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AiFeatureStoreResult {
        /// The timestamp of when the featurestore was created in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If set, both of the online and offline data storage will be secured by this key.
        /// Structure is documented below.
        pub encryption_spec: pulumi_gestalt_rust::Output<
            Option<super::super::types::vertex::AiFeatureStoreEncryptionSpec>,
        >,
        /// Used to perform consistent read-modify-write updates.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// If set to true, any EntityTypes and Features for this Featurestore will also be deleted
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A set of key/value label pairs to assign to this Featurestore.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Featurestore. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Config for online serving resources.
        /// Structure is documented below.
        pub online_serving_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::vertex::AiFeatureStoreOnlineServingConfig>,
        >,
        /// TTL in days for feature values that will be stored in online serving storage. The Feature Store online storage periodically removes obsolete feature values older than onlineStorageTtlDays since the feature generation time. Note that onlineStorageTtlDays should be less than or equal to offlineStorageTtlDays for each EntityType under a featurestore. If not set, default to 4000 days
        pub online_storage_ttl_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of the dataset. eg us-central1
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The timestamp of when the featurestore was last updated in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AiFeatureStoreArgs,
    ) -> AiFeatureStoreResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let encryption_spec_binding = args.encryption_spec.get_output(context);
        let force_destroy_binding = args.force_destroy.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let online_serving_config_binding = args
            .online_serving_config
            .get_output(context);
        let online_storage_ttl_days_binding = args
            .online_storage_ttl_days
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:vertex/aiFeatureStore:AiFeatureStore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionSpec".into(),
                    value: encryption_spec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: force_destroy_binding.get_id(),
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
                    name: "onlineServingConfig".into(),
                    value: online_serving_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "onlineStorageTtlDays".into(),
                    value: online_storage_ttl_days_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AiFeatureStoreResult {
            create_time: o.get_field("createTime"),
            effective_labels: o.get_field("effectiveLabels"),
            encryption_spec: o.get_field("encryptionSpec"),
            etag: o.get_field("etag"),
            force_destroy: o.get_field("forceDestroy"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            online_serving_config: o.get_field("onlineServingConfig"),
            online_storage_ttl_days: o.get_field("onlineStorageTtlDays"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            update_time: o.get_field("updateTime"),
        }
    }
}
