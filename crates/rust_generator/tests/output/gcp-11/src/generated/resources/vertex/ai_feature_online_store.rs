/// Vertex AI Feature Online Store provides a centralized repository for serving ML features and embedding indexes at low latency. The Feature Online Store is a top-level container.
///
///
/// To get more information about FeatureOnlineStore, see:
///
/// * [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.featureOnlineStores)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/vertex-ai/docs)
///
/// ## Example Usage
///
/// ### Vertex Ai Feature Online Store
///
///
/// ```yaml
/// resources:
///   featureOnlineStore:
///     type: gcp:vertex:AiFeatureOnlineStore
///     name: feature_online_store
///     properties:
///       name: example_feature_online_store
///       labels:
///         foo: bar
///       region: us-central1
///       bigtable:
///         autoScaling:
///           minNodeCount: 1
///           maxNodeCount: 3
///           cpuUtilizationTarget: 50
/// ```
/// ### Vertex Ai Featureonlinestore With Optimized
///
///
/// ```yaml
/// resources:
///   featureonlinestore:
///     type: gcp:vertex:AiFeatureOnlineStore
///     properties:
///       name: example_feature_online_store_optimized
///       labels:
///         foo: bar
///       region: us-central1
///       optimized: {}
///       dedicatedServingEndpoint:
///         privateServiceConnectConfig:
///           enablePrivateServiceConnect: true
///           projectAllowlists:
///             - ${project.number}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Vertex Ai Featureonlinestore With Beta Fields Bigtable
///
///
/// ```yaml
/// resources:
///   featureonlinestore:
///     type: gcp:vertex:AiFeatureOnlineStore
///     properties:
///       name: example_feature_online_store_beta_bigtable
///       labels:
///         foo: bar
///       region: us-central1
///       bigtable:
///         autoScaling:
///           minNodeCount: 1
///           maxNodeCount: 2
///           cpuUtilizationTarget: 80
///       embeddingManagement:
///         enabled: true
///       forceDestroy: true
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// FeatureOnlineStore can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/featureOnlineStores/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, FeatureOnlineStore can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureOnlineStore:AiFeatureOnlineStore default projects/{{project}}/locations/{{region}}/featureOnlineStores/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureOnlineStore:AiFeatureOnlineStore default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureOnlineStore:AiFeatureOnlineStore default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureOnlineStore:AiFeatureOnlineStore default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod ai_feature_online_store {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiFeatureOnlineStoreArgs {
        /// Settings for Cloud Bigtable instance that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore.
        /// Structure is documented below.
        #[builder(into, default)]
        pub bigtable: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::vertex::AiFeatureOnlineStoreBigtable>,
        >,
        /// The dedicated serving endpoint for this FeatureOnlineStore, which is different from common vertex service endpoint. Only need to be set when you choose Optimized storage type or enable EmbeddingManagement. Will use public endpoint by default.
        /// Structure is documented below.
        #[builder(into, default)]
        pub dedicated_serving_endpoint: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::vertex::AiFeatureOnlineStoreDedicatedServingEndpoint,
            >,
        >,
        /// The settings for embedding management in FeatureOnlineStore. Embedding management can only be set for BigTable. It is enabled by default for optimized storagetype.
        /// Structure is documented below.
        ///
        /// > **Warning:** `embedding_management` is deprecated. This field is no longer needed anymore and embedding management is automatically enabled when specifying Optimized storage type
        #[builder(into, default)]
        pub embedding_management: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::vertex::AiFeatureOnlineStoreEmbeddingManagement>,
        >,
        /// If set to true, any FeatureViews and Features for this FeatureOnlineStore will also be deleted.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The labels with user-defined metadata to organize your feature online stores.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name of the Feature Online Store. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Settings for the Optimized store that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore
        #[builder(into, default)]
        pub optimized: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::vertex::AiFeatureOnlineStoreOptimized>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of feature online store. eg us-central1
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AiFeatureOnlineStoreResult {
        /// Settings for Cloud Bigtable instance that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore.
        /// Structure is documented below.
        pub bigtable: pulumi_gestalt_rust::Output<
            Option<super::super::types::vertex::AiFeatureOnlineStoreBigtable>,
        >,
        /// The timestamp of when the feature online store was created in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The dedicated serving endpoint for this FeatureOnlineStore, which is different from common vertex service endpoint. Only need to be set when you choose Optimized storage type or enable EmbeddingManagement. Will use public endpoint by default.
        /// Structure is documented below.
        pub dedicated_serving_endpoint: pulumi_gestalt_rust::Output<
            super::super::types::vertex::AiFeatureOnlineStoreDedicatedServingEndpoint,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The settings for embedding management in FeatureOnlineStore. Embedding management can only be set for BigTable. It is enabled by default for optimized storagetype.
        /// Structure is documented below.
        ///
        /// > **Warning:** `embedding_management` is deprecated. This field is no longer needed anymore and embedding management is automatically enabled when specifying Optimized storage type
        pub embedding_management: pulumi_gestalt_rust::Output<
            super::super::types::vertex::AiFeatureOnlineStoreEmbeddingManagement,
        >,
        /// Used to perform consistent read-modify-write updates.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// If set to true, any FeatureViews and Features for this FeatureOnlineStore will also be deleted.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The labels with user-defined metadata to organize your feature online stores.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name of the Feature Online Store. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Settings for the Optimized store that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore
        pub optimized: pulumi_gestalt_rust::Output<
            Option<super::super::types::vertex::AiFeatureOnlineStoreOptimized>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of feature online store. eg us-central1
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The state of the Feature Online Store. See the possible states in [this link](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.featureOnlineStores#state).
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The timestamp of when the feature online store was last updated in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AiFeatureOnlineStoreArgs,
    ) -> AiFeatureOnlineStoreResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bigtable_binding = args.bigtable.get_output(context).get_inner();
        let dedicated_serving_endpoint_binding = args
            .dedicated_serving_endpoint
            .get_output(context)
            .get_inner();
        let embedding_management_binding = args
            .embedding_management
            .get_output(context)
            .get_inner();
        let force_destroy_binding = args.force_destroy.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let optimized_binding = args.optimized.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vertex/aiFeatureOnlineStore:AiFeatureOnlineStore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bigtable".into(),
                    value: &bigtable_binding,
                },
                register_interface::ObjectField {
                    name: "dedicatedServingEndpoint".into(),
                    value: &dedicated_serving_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "embeddingManagement".into(),
                    value: &embedding_management_binding,
                },
                register_interface::ObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding,
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
                    name: "optimized".into(),
                    value: &optimized_binding,
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
        AiFeatureOnlineStoreResult {
            bigtable: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bigtable"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            dedicated_serving_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dedicatedServingEndpoint"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            embedding_management: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("embeddingManagement"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            force_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceDestroy"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            optimized: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("optimized"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
