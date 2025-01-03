/// Vertex chat and Conversation Engine Chat type
///
///
/// To get more information about ChatEngine, see:
///
/// * [API documentation](https://cloud.google.com/generative-ai-app-builder/docs/reference/rest/v1/projects.locations.collections.engines)
/// * How-to Guides
///     * [Vertex AI Conversation](https://cloud.google.com/dialogflow/cx/docs/concept/generative)
///
/// ## Example Usage
///
/// ### Discoveryengine Chat Engine Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let primary = chat_engine::create(
///         "primary",
///         ChatEngineArgs::builder()
///             .chat_engine_config(
///                 ChatEngineChatEngineConfig::builder()
///                     .agentCreationConfig(
///                         ChatEngineChatEngineConfigAgentCreationConfig::builder()
///                             .business("test business name")
///                             .defaultLanguageCode("en")
///                             .timeZone("America/Los_Angeles")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .collection_id("default_collection")
///             .common_config(
///                 ChatEngineCommonConfig::builder()
///                     .companyName("test-company")
///                     .build_struct(),
///             )
///             .data_store_ids(
///                 vec!["${testDataStore.dataStoreId}", "${testDataStore2.dataStoreId}",],
///             )
///             .display_name("Chat engine")
///             .engine_id("chat-engine-id")
///             .industry_vertical("GENERIC")
///             .location("${testDataStore.location}")
///             .build_struct(),
///     );
///     let testDataStore = data_store::create(
///         "testDataStore",
///         DataStoreArgs::builder()
///             .content_config("NO_CONTENT")
///             .data_store_id("data-store")
///             .display_name("Structured datastore")
///             .industry_vertical("GENERIC")
///             .location("global")
///             .solution_types(vec!["SOLUTION_TYPE_CHAT",])
///             .build_struct(),
///     );
///     let testDataStore2 = data_store::create(
///         "testDataStore2",
///         DataStoreArgs::builder()
///             .content_config("NO_CONTENT")
///             .data_store_id("data-store-2")
///             .display_name("Structured datastore 2")
///             .industry_vertical("GENERIC")
///             .location("${testDataStore.location}")
///             .solution_types(vec!["SOLUTION_TYPE_CHAT",])
///             .build_struct(),
///     );
/// }
/// ```
/// ### Discoveryengine Chat Engine Existing Dialogflow Agent
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let agent = cx_agent::create(
///         "agent",
///         CxAgentArgs::builder()
///             .default_language_code("en")
///             .display_name("dialogflowcx-agent")
///             .location("global")
///             .time_zone("America/Los_Angeles")
///             .build_struct(),
///     );
///     let primary = chat_engine::create(
///         "primary",
///         ChatEngineArgs::builder()
///             .chat_engine_config(
///                 ChatEngineChatEngineConfig::builder()
///                     .dialogflowAgentToLink("${agent.id}")
///                     .build_struct(),
///             )
///             .collection_id("default_collection")
///             .common_config(
///                 ChatEngineCommonConfig::builder()
///                     .companyName("test-company")
///                     .build_struct(),
///             )
///             .data_store_ids(vec!["${testDataStore.dataStoreId}",])
///             .display_name("Chat engine")
///             .engine_id("chat-engine-id")
///             .industry_vertical("GENERIC")
///             .location("${testDataStore.location}")
///             .build_struct(),
///     );
///     let testDataStore = data_store::create(
///         "testDataStore",
///         DataStoreArgs::builder()
///             .content_config("NO_CONTENT")
///             .data_store_id("data-store")
///             .display_name("Structured datastore")
///             .industry_vertical("GENERIC")
///             .location("global")
///             .solution_types(vec!["SOLUTION_TYPE_CHAT",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ChatEngine can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/collections/{{collection_id}}/engines/{{engine_id}}`
///
/// * `{{project}}/{{location}}/{{collection_id}}/{{engine_id}}`
///
/// * `{{location}}/{{collection_id}}/{{engine_id}}`
///
/// When using the `pulumi import` command, ChatEngine can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/chatEngine:ChatEngine default projects/{{project}}/locations/{{location}}/collections/{{collection_id}}/engines/{{engine_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/chatEngine:ChatEngine default {{project}}/{{location}}/{{collection_id}}/{{engine_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/chatEngine:ChatEngine default {{location}}/{{collection_id}}/{{engine_id}}
/// ```
///
pub mod chat_engine {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChatEngineArgs {
        /// Configurations for a chat Engine.
        /// Structure is documented below.
        #[builder(into)]
        pub chat_engine_config: pulumi_wasm_rust::Output<
            super::super::types::discoveryengine::ChatEngineChatEngineConfig,
        >,
        /// The collection ID.
        #[builder(into)]
        pub collection_id: pulumi_wasm_rust::Output<String>,
        /// Common config spec that specifies the metadata of the engine.
        #[builder(into, default)]
        pub common_config: pulumi_wasm_rust::Output<
            Option<super::super::types::discoveryengine::ChatEngineCommonConfig>,
        >,
        /// The data stores associated with this engine. Multiple DataStores in the same Collection can be associated here. All listed DataStores must be `SOLUTION_TYPE_CHAT`. Adding or removing data stores will force recreation.
        #[builder(into)]
        pub data_store_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The ID to use for chat engine.
        #[builder(into)]
        pub engine_id: pulumi_wasm_rust::Output<String>,
        /// The industry vertical that the chat engine registers. Vertical on Engine has to match vertical of the DataStore linked
        /// to the engine. Default value: "GENERIC" Possible values: ["GENERIC"]
        #[builder(into, default)]
        pub industry_vertical: pulumi_wasm_rust::Output<Option<String>>,
        /// Location.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ChatEngineResult {
        /// Configurations for a chat Engine.
        /// Structure is documented below.
        pub chat_engine_config: pulumi_wasm_rust::Output<
            super::super::types::discoveryengine::ChatEngineChatEngineConfig,
        >,
        /// Additional information of the Chat Engine.
        /// Structure is documented below.
        pub chat_engine_metadatas: pulumi_wasm_rust::Output<
            Vec<super::super::types::discoveryengine::ChatEngineChatEngineMetadata>,
        >,
        /// The collection ID.
        pub collection_id: pulumi_wasm_rust::Output<String>,
        /// Common config spec that specifies the metadata of the engine.
        pub common_config: pulumi_wasm_rust::Output<
            Option<super::super::types::discoveryengine::ChatEngineCommonConfig>,
        >,
        /// Timestamp the Engine was created at.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The data stores associated with this engine. Multiple DataStores in the same Collection can be associated here. All listed DataStores must be `SOLUTION_TYPE_CHAT`. Adding or removing data stores will force recreation.
        pub data_store_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The ID to use for chat engine.
        pub engine_id: pulumi_wasm_rust::Output<String>,
        /// The industry vertical that the chat engine registers. Vertical on Engine has to match vertical of the DataStore linked
        /// to the engine. Default value: "GENERIC" Possible values: ["GENERIC"]
        pub industry_vertical: pulumi_wasm_rust::Output<Option<String>>,
        /// Location.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The unique full resource name of the chat engine. Values are of the format
        /// `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}`.
        /// This field must be a UTF-8 encoded string with a length limit of 1024
        /// characters.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// Timestamp the Engine was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ChatEngineArgs) -> ChatEngineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let chat_engine_config_binding = args.chat_engine_config.get_inner();
        let collection_id_binding = args.collection_id.get_inner();
        let common_config_binding = args.common_config.get_inner();
        let data_store_ids_binding = args.data_store_ids.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let engine_id_binding = args.engine_id.get_inner();
        let industry_vertical_binding = args.industry_vertical.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:discoveryengine/chatEngine:ChatEngine".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "chatEngineConfig".into(),
                    value: &chat_engine_config_binding,
                },
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
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "chatEngineConfig".into(),
                },
                register_interface::ResultField {
                    name: "chatEngineMetadatas".into(),
                },
                register_interface::ResultField {
                    name: "collectionId".into(),
                },
                register_interface::ResultField {
                    name: "commonConfig".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dataStoreIds".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "engineId".into(),
                },
                register_interface::ResultField {
                    name: "industryVertical".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
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
        ChatEngineResult {
            chat_engine_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("chatEngineConfig").unwrap(),
            ),
            chat_engine_metadatas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("chatEngineMetadatas").unwrap(),
            ),
            collection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("collectionId").unwrap(),
            ),
            common_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("commonConfig").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            data_store_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataStoreIds").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            engine_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineId").unwrap(),
            ),
            industry_vertical: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("industryVertical").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
