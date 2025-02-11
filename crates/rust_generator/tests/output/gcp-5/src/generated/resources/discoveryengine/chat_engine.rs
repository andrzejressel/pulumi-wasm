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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod chat_engine {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChatEngineArgs {
        /// Configurations for a chat Engine.
        /// Structure is documented below.
        #[builder(into)]
        pub chat_engine_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::discoveryengine::ChatEngineChatEngineConfig,
        >,
        /// The collection ID.
        #[builder(into)]
        pub collection_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Common config spec that specifies the metadata of the engine.
        #[builder(into, default)]
        pub common_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::discoveryengine::ChatEngineCommonConfig>,
        >,
        /// The data stores associated with this engine. Multiple DataStores in the same Collection can be associated here. All listed DataStores must be `SOLUTION_TYPE_CHAT`. Adding or removing data stores will force recreation.
        #[builder(into)]
        pub data_store_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID to use for chat engine.
        #[builder(into)]
        pub engine_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The industry vertical that the chat engine registers. Vertical on Engine has to match vertical of the DataStore linked
        /// to the engine. Default value: "GENERIC" Possible values: ["GENERIC"]
        #[builder(into, default)]
        pub industry_vertical: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Location.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ChatEngineResult {
        /// Configurations for a chat Engine.
        /// Structure is documented below.
        pub chat_engine_config: pulumi_gestalt_rust::Output<
            super::super::types::discoveryengine::ChatEngineChatEngineConfig,
        >,
        /// Additional information of the Chat Engine.
        /// Structure is documented below.
        pub chat_engine_metadatas: pulumi_gestalt_rust::Output<
            Vec<super::super::types::discoveryengine::ChatEngineChatEngineMetadata>,
        >,
        /// The collection ID.
        pub collection_id: pulumi_gestalt_rust::Output<String>,
        /// Common config spec that specifies the metadata of the engine.
        pub common_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::discoveryengine::ChatEngineCommonConfig>,
        >,
        /// Timestamp the Engine was created at.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The data stores associated with this engine. Multiple DataStores in the same Collection can be associated here. All listed DataStores must be `SOLUTION_TYPE_CHAT`. Adding or removing data stores will force recreation.
        pub data_store_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The ID to use for chat engine.
        pub engine_id: pulumi_gestalt_rust::Output<String>,
        /// The industry vertical that the chat engine registers. Vertical on Engine has to match vertical of the DataStore linked
        /// to the engine. Default value: "GENERIC" Possible values: ["GENERIC"]
        pub industry_vertical: pulumi_gestalt_rust::Output<Option<String>>,
        /// Location.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The unique full resource name of the chat engine. Values are of the format
        /// `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}`.
        /// This field must be a UTF-8 encoded string with a length limit of 1024
        /// characters.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Timestamp the Engine was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ChatEngineArgs,
    ) -> ChatEngineResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let chat_engine_config_binding = args.chat_engine_config.get_output(context);
        let collection_id_binding = args.collection_id.get_output(context);
        let common_config_binding = args.common_config.get_output(context);
        let data_store_ids_binding = args.data_store_ids.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let engine_id_binding = args.engine_id.get_output(context);
        let industry_vertical_binding = args.industry_vertical.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:discoveryengine/chatEngine:ChatEngine".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "chatEngineConfig".into(),
                    value: &chat_engine_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "collectionId".into(),
                    value: &collection_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "commonConfig".into(),
                    value: &common_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataStoreIds".into(),
                    value: &data_store_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineId".into(),
                    value: &engine_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "industryVertical".into(),
                    value: &industry_vertical_binding.drop_type(),
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
        ChatEngineResult {
            chat_engine_config: o.get_field("chatEngineConfig"),
            chat_engine_metadatas: o.get_field("chatEngineMetadatas"),
            collection_id: o.get_field("collectionId"),
            common_config: o.get_field("commonConfig"),
            create_time: o.get_field("createTime"),
            data_store_ids: o.get_field("dataStoreIds"),
            display_name: o.get_field("displayName"),
            engine_id: o.get_field("engineId"),
            industry_vertical: o.get_field("industryVertical"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            update_time: o.get_field("updateTime"),
        }
    }
}
