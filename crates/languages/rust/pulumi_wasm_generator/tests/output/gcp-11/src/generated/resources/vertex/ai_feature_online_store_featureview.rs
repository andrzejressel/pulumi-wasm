/// FeatureView is representation of values that the FeatureOnlineStore will serve based on its syncConfig.
///
///
/// To get more information about FeatureOnlineStoreFeatureview, see:
///
/// * [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.featureOnlineStores.featureViews)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/vertex-ai/docs)
///
/// ## Example Usage
///
/// ### Vertex Ai Featureonlinestore Featureview
///
///
/// ```yaml
/// resources:
///   featureonlinestore:
///     type: gcp:vertex:AiFeatureOnlineStore
///     properties:
///       name: example_feature_view
///       labels:
///         foo: bar
///       region: us-central1
///       bigtable:
///         autoScaling:
///           minNodeCount: 1
///           maxNodeCount: 2
///           cpuUtilizationTarget: 80
///   tf-test-dataset:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: example_feature_view
///       friendlyName: test
///       description: This is a test description
///       location: US
///   tf-test-table:
///     type: gcp:bigquery:Table
///     properties:
///       deletionProtection: false
///       datasetId: ${["tf-test-dataset"].datasetId}
///       tableId: example_feature_view
///       schema: |2
///           [
///           {
///             "name": "entity_id",
///             "mode": "NULLABLE",
///             "type": "STRING",
///             "description": "Test default entity_id"
///           },
///             {
///             "name": "test_entity_column",
///             "mode": "NULLABLE",
///             "type": "STRING",
///             "description": "test secondary entity column"
///           },
///           {
///             "name": "feature_timestamp",
///             "mode": "NULLABLE",
///             "type": "TIMESTAMP",
///             "description": "Default timestamp value"
///           }
///         ]
///   featureview:
///     type: gcp:vertex:AiFeatureOnlineStoreFeatureview
///     properties:
///       name: example_feature_view
///       region: us-central1
///       featureOnlineStore: ${featureonlinestore.name}
///       syncConfig:
///         cron: 0 0 * * *
///       bigQuerySource:
///         uri: bq://${["tf-test-table"].project}.${["tf-test-table"].datasetId}.${["tf-test-table"].tableId}
///         entityIdColumns:
///           - test_entity_column
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Vertex Ai Featureonlinestore Featureview Feature Registry
///
///
/// ```yaml
/// resources:
///   featureonlinestore:
///     type: gcp:vertex:AiFeatureOnlineStore
///     properties:
///       name: example_feature_view_feature_registry
///       labels:
///         foo: bar
///       region: us-central1
///       bigtable:
///         autoScaling:
///           minNodeCount: 1
///           maxNodeCount: 2
///           cpuUtilizationTarget: 80
///   sampleDataset:
///     type: gcp:bigquery:Dataset
///     name: sample_dataset
///     properties:
///       datasetId: example_feature_view_feature_registry
///       friendlyName: test
///       description: This is a test description
///       location: US
///   sampleTable:
///     type: gcp:bigquery:Table
///     name: sample_table
///     properties:
///       deletionProtection: false
///       datasetId: ${sampleDataset.datasetId}
///       tableId: example_feature_view_feature_registry
///       schema: |
///         [
///             {
///                 "name": "feature_id",
///                 "type": "STRING",
///                 "mode": "NULLABLE"
///             },
///             {
///                 "name": "example_feature_view_feature_registry",
///                 "type": "STRING",
///                 "mode": "NULLABLE"
///             },
///             {
///                 "name": "feature_timestamp",
///                 "type": "TIMESTAMP",
///                 "mode": "NULLABLE"
///             }
///         ]
///   sampleFeatureGroup:
///     type: gcp:vertex:AiFeatureGroup
///     name: sample_feature_group
///     properties:
///       name: example_feature_view_feature_registry
///       description: A sample feature group
///       region: us-central1
///       labels:
///         label-one: value-one
///       bigQuery:
///         bigQuerySource:
///           inputUri: bq://${sampleTable.project}.${sampleTable.datasetId}.${sampleTable.tableId}
///         entityIdColumns:
///           - feature_id
///   sampleFeature:
///     type: gcp:vertex:AiFeatureGroupFeature
///     name: sample_feature
///     properties:
///       name: example_feature_view_feature_registry
///       region: us-central1
///       featureGroup: ${sampleFeatureGroup.name}
///       description: A sample feature
///       labels:
///         label-one: value-one
///   featureviewFeatureregistry:
///     type: gcp:vertex:AiFeatureOnlineStoreFeatureview
///     name: featureview_featureregistry
///     properties:
///       name: example_feature_view_feature_registry
///       region: us-central1
///       featureOnlineStore: ${featureonlinestore.name}
///       syncConfig:
///         cron: 0 0 * * *
///       featureRegistrySource:
///         featureGroups:
///           - featureGroupId: ${sampleFeatureGroup.name}
///             featureIds:
///               - ${sampleFeature.name}
/// ```
/// ### Vertex Ai Featureonlinestore Featureview Cross Project
///
///
/// ```yaml
/// resources:
///   project:
///     type: gcp:organizations:Project
///     properties:
///       projectId: tf-test_89313
///       name: tf-test_60646
///       orgId: '123456789'
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
///   wait60Seconds:
///     type: time:sleep
///     name: wait_60_seconds
///     properties:
///       createDuration: 60s
///     options:
///       dependsOn:
///         - ${project}
///   wait30Seconds:
///     type: time:sleep
///     name: wait_30_seconds
///     properties:
///       createDuration: 30s
///     options:
///       dependsOn:
///         - ${viewer}
///   vertexai:
///     type: gcp:projects:Service
///     properties:
///       service: aiplatform.googleapis.com
///       project: ${project.projectId}
///       disableOnDestroy: false # Needed for CI tests for permissions to propagate, should not be needed for actual usage
///     options:
///       dependsOn:
///         - ${wait60Seconds}
///   viewer:
///     type: gcp:bigquery:DatasetIamMember
///     properties:
///       project: ${testProject.projectId}
///       datasetId: ${sampleDataset.datasetId}
///       role: roles/bigquery.dataViewer
///       member: serviceAccount:service-${project.number}@gcp-sa-aiplatform.iam.gserviceaccount.com
///     options:
///       dependsOn:
///         - ${featureonlinestore}
///   featureonlinestore:
///     type: gcp:vertex:AiFeatureOnlineStore
///     properties:
///       name: example_cross_project_featureview
///       project: ${project.projectId}
///       labels:
///         foo: bar
///       region: us-central1
///       bigtable:
///         autoScaling:
///           minNodeCount: 1
///           maxNodeCount: 2
///           cpuUtilizationTarget: 80
///     options:
///       dependsOn:
///         - ${vertexai}
///   sampleDataset:
///     type: gcp:bigquery:Dataset
///     name: sample_dataset
///     properties:
///       datasetId: example_cross_project_featureview
///       friendlyName: test
///       description: This is a test description
///       location: US
///   sampleTable:
///     type: gcp:bigquery:Table
///     name: sample_table
///     properties:
///       deletionProtection: false
///       datasetId: ${sampleDataset.datasetId}
///       tableId: example_cross_project_featureview
///       schema: |
///         [
///             {
///                 "name": "feature_id",
///                 "type": "STRING",
///                 "mode": "NULLABLE"
///             },
///             {
///                 "name": "example_cross_project_featureview",
///                 "type": "STRING",
///                 "mode": "NULLABLE"
///             },
///             {
///                 "name": "feature_timestamp",
///                 "type": "TIMESTAMP",
///                 "mode": "NULLABLE"
///             }
///         ]
///   sampleFeatureGroup:
///     type: gcp:vertex:AiFeatureGroup
///     name: sample_feature_group
///     properties:
///       name: example_cross_project_featureview
///       description: A sample feature group
///       region: us-central1
///       labels:
///         label-one: value-one
///       bigQuery:
///         bigQuerySource:
///           inputUri: bq://${sampleTable.project}.${sampleTable.datasetId}.${sampleTable.tableId}
///         entityIdColumns:
///           - feature_id
///   sampleFeature:
///     type: gcp:vertex:AiFeatureGroupFeature
///     name: sample_feature
///     properties:
///       name: example_cross_project_featureview
///       region: us-central1
///       featureGroup: ${sampleFeatureGroup.name}
///       description: A sample feature
///       labels:
///         label-one: value-one
///   crossProjectFeatureview:
///     type: gcp:vertex:AiFeatureOnlineStoreFeatureview
///     name: cross_project_featureview
///     properties:
///       name: example_cross_project_featureview
///       project: ${project.projectId}
///       region: us-central1
///       featureOnlineStore: ${featureonlinestore.name}
///       syncConfig:
///         cron: 0 0 * * *
///       featureRegistrySource:
///         featureGroups:
///           - featureGroupId: ${sampleFeatureGroup.name}
///             featureIds:
///               - ${sampleFeature.name}
///         projectNumber: ${testProject.number}
///     options:
///       dependsOn:
///         - ${vertexai}
///         - ${wait30Seconds}
/// variables:
///   testProject:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Vertex Ai Featureonlinestore Featureview With Vector Search
///
///
/// ```yaml
/// resources:
///   featureonlinestore:
///     type: gcp:vertex:AiFeatureOnlineStore
///     properties:
///       name: example_feature_view_vector_search
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
///   tf-test-dataset:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: example_feature_view_vector_search
///       friendlyName: test
///       description: This is a test description
///       location: US
///   tf-test-table:
///     type: gcp:bigquery:Table
///     properties:
///       deletionProtection: false
///       datasetId: ${["tf-test-dataset"].datasetId}
///       tableId: example_feature_view_vector_search
///       schema: |
///         [
///         {
///           "name": "test_primary_id",
///           "mode": "NULLABLE",
///           "type": "STRING",
///           "description": "primary test id"
///         },
///         {
///           "name": "embedding",
///           "mode": "REPEATED",
///           "type": "FLOAT",
///           "description": "embedding column for primary_id column"
///         },
///         {
///           "name": "country",
///           "mode": "NULLABLE",
///           "type": "STRING",
///           "description": "country"
///         },
///         {
///           "name": "test_crowding_column",
///           "mode": "NULLABLE",
///           "type": "INTEGER",
///           "description": "test crowding column"
///         },
///         {
///           "name": "entity_id",
///           "mode": "NULLABLE",
///           "type": "STRING",
///           "description": "Test default entity_id"
///         },
///         {
///           "name": "test_entity_column",
///           "mode": "NULLABLE",
///           "type": "STRING",
///           "description": "test secondary entity column"
///         },
///         {
///           "name": "feature_timestamp",
///           "mode": "NULLABLE",
///           "type": "TIMESTAMP",
///           "description": "Default timestamp value"
///         }
///         ]
///   featureviewVectorSearch:
///     type: gcp:vertex:AiFeatureOnlineStoreFeatureview
///     name: featureview_vector_search
///     properties:
///       name: example_feature_view_vector_search
///       region: us-central1
///       featureOnlineStore: ${featureonlinestore.name}
///       syncConfig:
///         cron: 0 0 * * *
///       bigQuerySource:
///         uri: bq://${["tf-test-table"].project}.${["tf-test-table"].datasetId}.${["tf-test-table"].tableId}
///         entityIdColumns:
///           - test_entity_column
///       vectorSearchConfig:
///         embeddingColumn: embedding
///         filterColumns:
///           - country
///         crowdingColumn: test_crowding_column
///         distanceMeasureType: DOT_PRODUCT_DISTANCE
///         treeAhConfig:
///           leafNodeEmbeddingCount: '1000'
///         embeddingDimension: '2'
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// FeatureOnlineStoreFeatureview can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/featureOnlineStores/{{feature_online_store}}/featureViews/{{name}}`
///
/// * `{{project}}/{{region}}/{{feature_online_store}}/{{name}}`
///
/// * `{{region}}/{{feature_online_store}}/{{name}}`
///
/// * `{{feature_online_store}}/{{name}}`
///
/// When using the `pulumi import` command, FeatureOnlineStoreFeatureview can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureOnlineStoreFeatureview:AiFeatureOnlineStoreFeatureview default projects/{{project}}/locations/{{region}}/featureOnlineStores/{{feature_online_store}}/featureViews/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureOnlineStoreFeatureview:AiFeatureOnlineStoreFeatureview default {{project}}/{{region}}/{{feature_online_store}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureOnlineStoreFeatureview:AiFeatureOnlineStoreFeatureview default {{region}}/{{feature_online_store}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureOnlineStoreFeatureview:AiFeatureOnlineStoreFeatureview default {{feature_online_store}}/{{name}}
/// ```
///
pub mod ai_feature_online_store_featureview {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiFeatureOnlineStoreFeatureviewArgs {
        /// Configures how data is supposed to be extracted from a BigQuery source to be loaded onto the FeatureOnlineStore.
        /// Structure is documented below.
        #[builder(into, default)]
        pub big_query_source: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::vertex::AiFeatureOnlineStoreFeatureviewBigQuerySource,
            >,
        >,
        /// The name of the FeatureOnlineStore to use for the featureview.
        #[builder(into)]
        pub feature_online_store: pulumi_wasm_rust::InputOrOutput<String>,
        /// Configures the features from a Feature Registry source that need to be loaded onto the FeatureOnlineStore.
        /// Structure is documented below.
        #[builder(into, default)]
        pub feature_registry_source: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::vertex::AiFeatureOnlineStoreFeatureviewFeatureRegistrySource,
            >,
        >,
        /// A set of key/value label pairs to assign to this FeatureView.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the FeatureView. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The region for the resource. It should be the same as the featureonlinestore region.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub region: pulumi_wasm_rust::InputOrOutput<String>,
        /// Configures when data is to be synced/updated for this FeatureView. At the end of the sync the latest featureValues for each entityId of this FeatureView are made ready for online serving.
        /// Structure is documented below.
        #[builder(into, default)]
        pub sync_config: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::vertex::AiFeatureOnlineStoreFeatureviewSyncConfig,
            >,
        >,
        /// Configuration for vector search. It contains the required configurations to create an index from source data, so that approximate nearest neighbor (a.k.a ANN) algorithms search can be performed during online serving.
        /// Structure is documented below.
        #[builder(into, default)]
        pub vector_search_config: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::vertex::AiFeatureOnlineStoreFeatureviewVectorSearchConfig,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct AiFeatureOnlineStoreFeatureviewResult {
        /// Configures how data is supposed to be extracted from a BigQuery source to be loaded onto the FeatureOnlineStore.
        /// Structure is documented below.
        pub big_query_source: pulumi_wasm_rust::Output<
            Option<
                super::super::types::vertex::AiFeatureOnlineStoreFeatureviewBigQuerySource,
            >,
        >,
        /// The timestamp of when the featureOnlinestore was created in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the FeatureOnlineStore to use for the featureview.
        pub feature_online_store: pulumi_wasm_rust::Output<String>,
        /// Configures the features from a Feature Registry source that need to be loaded onto the FeatureOnlineStore.
        /// Structure is documented below.
        pub feature_registry_source: pulumi_wasm_rust::Output<
            Option<
                super::super::types::vertex::AiFeatureOnlineStoreFeatureviewFeatureRegistrySource,
            >,
        >,
        /// A set of key/value label pairs to assign to this FeatureView.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the FeatureView. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region for the resource. It should be the same as the featureonlinestore region.
        ///
        ///
        /// - - -
        pub region: pulumi_wasm_rust::Output<String>,
        /// Configures when data is to be synced/updated for this FeatureView. At the end of the sync the latest featureValues for each entityId of this FeatureView are made ready for online serving.
        /// Structure is documented below.
        pub sync_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::vertex::AiFeatureOnlineStoreFeatureviewSyncConfig,
            >,
        >,
        /// The timestamp of when the featureOnlinestore was last updated in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// Configuration for vector search. It contains the required configurations to create an index from source data, so that approximate nearest neighbor (a.k.a ANN) algorithms search can be performed during online serving.
        /// Structure is documented below.
        pub vector_search_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::vertex::AiFeatureOnlineStoreFeatureviewVectorSearchConfig,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AiFeatureOnlineStoreFeatureviewArgs,
    ) -> AiFeatureOnlineStoreFeatureviewResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let big_query_source_binding = args
            .big_query_source
            .get_output(context)
            .get_inner();
        let feature_online_store_binding = args
            .feature_online_store
            .get_output(context)
            .get_inner();
        let feature_registry_source_binding = args
            .feature_registry_source
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let sync_config_binding = args.sync_config.get_output(context).get_inner();
        let vector_search_config_binding = args
            .vector_search_config
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vertex/aiFeatureOnlineStoreFeatureview:AiFeatureOnlineStoreFeatureview"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bigQuerySource".into(),
                    value: &big_query_source_binding,
                },
                register_interface::ObjectField {
                    name: "featureOnlineStore".into(),
                    value: &feature_online_store_binding,
                },
                register_interface::ObjectField {
                    name: "featureRegistrySource".into(),
                    value: &feature_registry_source_binding,
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
                    name: "syncConfig".into(),
                    value: &sync_config_binding,
                },
                register_interface::ObjectField {
                    name: "vectorSearchConfig".into(),
                    value: &vector_search_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AiFeatureOnlineStoreFeatureviewResult {
            big_query_source: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bigQuerySource"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            feature_online_store: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("featureOnlineStore"),
            ),
            feature_registry_source: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("featureRegistrySource"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            sync_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("syncConfig"),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            vector_search_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vectorSearchConfig"),
            ),
        }
    }
}
