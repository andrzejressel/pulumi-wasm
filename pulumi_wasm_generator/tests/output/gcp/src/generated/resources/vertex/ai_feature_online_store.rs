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
pub mod ai_feature_online_store {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiFeatureOnlineStoreArgs {
        /// Settings for Cloud Bigtable instance that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore.
        /// Structure is documented below.
        #[builder(into, default)]
        pub bigtable: pulumi_wasm_rust::Output<
            Option<super::super::types::vertex::AiFeatureOnlineStoreBigtable>,
        >,
        /// The dedicated serving endpoint for this FeatureOnlineStore, which is different from common vertex service endpoint. Only need to be set when you choose Optimized storage type or enable EmbeddingManagement. Will use public endpoint by default.
        /// Structure is documented below.
        #[builder(into, default)]
        pub dedicated_serving_endpoint: pulumi_wasm_rust::Output<
            Option<
                super::super::types::vertex::AiFeatureOnlineStoreDedicatedServingEndpoint,
            >,
        >,
        /// The settings for embedding management in FeatureOnlineStore. Embedding management can only be set for BigTable. It is enabled by default for optimized storagetype.
        /// Structure is documented below.
        ///
        /// > **Warning:** `embedding_management` is deprecated. This field is no longer needed anymore and embedding management is automatically enabled when specifying Optimized storage type
        #[builder(into, default)]
        pub embedding_management: pulumi_wasm_rust::Output<
            Option<super::super::types::vertex::AiFeatureOnlineStoreEmbeddingManagement>,
        >,
        /// If set to true, any FeatureViews and Features for this FeatureOnlineStore will also be deleted.
        #[builder(into, default)]
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// The labels with user-defined metadata to organize your feature online stores.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name of the Feature Online Store. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Settings for the Optimized store that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore
        #[builder(into, default)]
        pub optimized: pulumi_wasm_rust::Output<
            Option<super::super::types::vertex::AiFeatureOnlineStoreOptimized>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region of feature online store. eg us-central1
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AiFeatureOnlineStoreResult {
        /// Settings for Cloud Bigtable instance that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore.
        /// Structure is documented below.
        pub bigtable: pulumi_wasm_rust::Output<
            Option<super::super::types::vertex::AiFeatureOnlineStoreBigtable>,
        >,
        /// The timestamp of when the feature online store was created in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The dedicated serving endpoint for this FeatureOnlineStore, which is different from common vertex service endpoint. Only need to be set when you choose Optimized storage type or enable EmbeddingManagement. Will use public endpoint by default.
        /// Structure is documented below.
        pub dedicated_serving_endpoint: pulumi_wasm_rust::Output<
            super::super::types::vertex::AiFeatureOnlineStoreDedicatedServingEndpoint,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The settings for embedding management in FeatureOnlineStore. Embedding management can only be set for BigTable. It is enabled by default for optimized storagetype.
        /// Structure is documented below.
        ///
        /// > **Warning:** `embedding_management` is deprecated. This field is no longer needed anymore and embedding management is automatically enabled when specifying Optimized storage type
        pub embedding_management: pulumi_wasm_rust::Output<
            super::super::types::vertex::AiFeatureOnlineStoreEmbeddingManagement,
        >,
        /// Used to perform consistent read-modify-write updates.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// If set to true, any FeatureViews and Features for this FeatureOnlineStore will also be deleted.
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// The labels with user-defined metadata to organize your feature online stores.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name of the Feature Online Store. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// Settings for the Optimized store that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore
        pub optimized: pulumi_wasm_rust::Output<
            Option<super::super::types::vertex::AiFeatureOnlineStoreOptimized>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of feature online store. eg us-central1
        pub region: pulumi_wasm_rust::Output<String>,
        /// The state of the Feature Online Store. See the possible states in [this link](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.featureOnlineStores#state).
        pub state: pulumi_wasm_rust::Output<String>,
        /// The timestamp of when the feature online store was last updated in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AiFeatureOnlineStoreArgs,
    ) -> AiFeatureOnlineStoreResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bigtable_binding = args.bigtable.get_inner();
        let dedicated_serving_endpoint_binding = args
            .dedicated_serving_endpoint
            .get_inner();
        let embedding_management_binding = args.embedding_management.get_inner();
        let force_destroy_binding = args.force_destroy.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let optimized_binding = args.optimized.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vertex/aiFeatureOnlineStore:AiFeatureOnlineStore".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "bigtable".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dedicatedServingEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "embeddingManagement".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "forceDestroy".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "optimized".into(),
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
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AiFeatureOnlineStoreResult {
            bigtable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bigtable").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            dedicated_serving_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dedicatedServingEndpoint").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            embedding_management: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("embeddingManagement").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            force_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceDestroy").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            optimized: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("optimized").unwrap(),
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
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
