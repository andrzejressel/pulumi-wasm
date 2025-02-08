/// Vertex AI Search and Conversation can be used to create a search engine or a chat application by connecting it with a datastore
///
///
/// To get more information about SearchEngine, see:
///
/// * [API documentation](https://cloud.google.com/generative-ai-app-builder/docs/reference/rest/v1/projects.locations.collections.engines)
/// * How-to Guides
///     * [Create a Search Engine](https://cloud.google.com/generative-ai-app-builder/docs/create-engine-es)
///
/// ## Example Usage
///
/// ### Discoveryengine Searchengine Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = data_store::create(
///         "basic",
///         DataStoreArgs::builder()
///             .content_config("NO_CONTENT")
///             .create_advanced_site_search(false)
///             .data_store_id("example-datastore-id")
///             .display_name("tf-test-structured-datastore")
///             .industry_vertical("GENERIC")
///             .location("global")
///             .solution_types(vec!["SOLUTION_TYPE_SEARCH",])
///             .build_struct(),
///     );
///     let basicSearchEngine = search_engine::create(
///         "basicSearchEngine",
///         SearchEngineArgs::builder()
///             .collection_id("default_collection")
///             .data_store_ids(vec!["${basic.dataStoreId}",])
///             .display_name("Example Display Name")
///             .engine_id("example-engine-id")
///             .location("${basic.location}")
///             .search_engine_config(
///                 SearchEngineSearchEngineConfig::builder().build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// SearchEngine can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/collections/{{collection_id}}/engines/{{engine_id}}`
///
/// * `{{project}}/{{location}}/{{collection_id}}/{{engine_id}}`
///
/// * `{{location}}/{{collection_id}}/{{engine_id}}`
///
/// When using the `pulumi import` command, SearchEngine can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/searchEngine:SearchEngine default projects/{{project}}/locations/{{location}}/collections/{{collection_id}}/engines/{{engine_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/searchEngine:SearchEngine default {{project}}/{{location}}/{{collection_id}}/{{engine_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/searchEngine:SearchEngine default {{location}}/{{collection_id}}/{{engine_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod search_engine {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SearchEngineArgs {
        /// The collection ID.
        #[builder(into)]
        pub collection_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Common config spec that specifies the metadata of the engine.
        #[builder(into, default)]
        pub common_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::discoveryengine::SearchEngineCommonConfig>,
        >,
        /// The data stores associated with this engine. For SOLUTION_TYPE_SEARCH type of engines, they can only associate with at most one data store.
        #[builder(into)]
        pub data_store_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Unique ID to use for Search Engine App.
        #[builder(into)]
        pub engine_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore:
        /// If unspecified, default to GENERIC. Vertical on Engine has to match vertical of the DataStore liniked to the engine.
        /// Default value: "GENERIC" Possible values: ["GENERIC", "MEDIA", "HEALTHCARE_FHIR"]
        #[builder(into, default)]
        pub industry_vertical: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Location.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configurations for a Search Engine.
        /// Structure is documented below.
        #[builder(into)]
        pub search_engine_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::discoveryengine::SearchEngineSearchEngineConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct SearchEngineResult {
        /// The collection ID.
        pub collection_id: pulumi_gestalt_rust::Output<String>,
        /// Common config spec that specifies the metadata of the engine.
        pub common_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::discoveryengine::SearchEngineCommonConfig>,
        >,
        /// Timestamp the Engine was created at.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The data stores associated with this engine. For SOLUTION_TYPE_SEARCH type of engines, they can only associate with at most one data store.
        pub data_store_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Unique ID to use for Search Engine App.
        pub engine_id: pulumi_gestalt_rust::Output<String>,
        /// The industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore:
        /// If unspecified, default to GENERIC. Vertical on Engine has to match vertical of the DataStore liniked to the engine.
        /// Default value: "GENERIC" Possible values: ["GENERIC", "MEDIA", "HEALTHCARE_FHIR"]
        pub industry_vertical: pulumi_gestalt_rust::Output<Option<String>>,
        /// Location.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The unique full resource name of the search engine. Values are of the format
        /// `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}`.
        /// This field must be a UTF-8 encoded string with a length limit of 1024
        /// characters.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Configurations for a Search Engine.
        /// Structure is documented below.
        pub search_engine_config: pulumi_gestalt_rust::Output<
            super::super::types::discoveryengine::SearchEngineSearchEngineConfig,
        >,
        /// Timestamp the Engine was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SearchEngineArgs,
    ) -> SearchEngineResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let collection_id_binding = args.collection_id.get_output(context).get_inner();
        let common_config_binding = args.common_config.get_output(context).get_inner();
        let data_store_ids_binding = args.data_store_ids.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let engine_id_binding = args.engine_id.get_output(context).get_inner();
        let industry_vertical_binding = args
            .industry_vertical
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let search_engine_config_binding = args
            .search_engine_config
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:discoveryengine/searchEngine:SearchEngine".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "collectionId".into(),
                    value: &collection_id_binding,
                },
                register_interface::ObjectField {
                    name: "commonConfig".into(),
                    value: &common_config_binding,
                },
                register_interface::ObjectField {
                    name: "dataStoreIds".into(),
                    value: &data_store_ids_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "engineId".into(),
                    value: &engine_id_binding,
                },
                register_interface::ObjectField {
                    name: "industryVertical".into(),
                    value: &industry_vertical_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "searchEngineConfig".into(),
                    value: &search_engine_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SearchEngineResult {
            collection_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("collectionId"),
            ),
            common_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("commonConfig"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            data_store_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataStoreIds"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            engine_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineId"),
            ),
            industry_vertical: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("industryVertical"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            search_engine_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("searchEngineConfig"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
