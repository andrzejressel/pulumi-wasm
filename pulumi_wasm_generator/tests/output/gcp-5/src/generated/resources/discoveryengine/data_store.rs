/// Data store is a collection of websites and documents used to find answers for
/// end-user's questions in Discovery Engine (a.k.a. Vertex AI Search and
/// Conversation).
///
///
/// To get more information about DataStore, see:
///
/// * [API documentation](https://cloud.google.com/generative-ai-app-builder/docs/reference/rest/v1/projects.locations.collections.dataStores)
/// * How-to Guides
///     * [Create a search data store](https://cloud.google.com/generative-ai-app-builder/docs/create-data-store-es)
///
/// ## Example Usage
///
/// ### Discoveryengine Datastore Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = data_store::create(
///         "basic",
///         DataStoreArgs::builder()
///             .content_config("NO_CONTENT")
///             .create_advanced_site_search(false)
///             .data_store_id("data-store-id")
///             .display_name("tf-test-structured-datastore")
///             .industry_vertical("GENERIC")
///             .location("global")
///             .skip_default_schema_creation(false)
///             .solution_types(vec!["SOLUTION_TYPE_SEARCH",])
///             .build_struct(),
///     );
/// }
/// ```
/// ### Discoveryengine Datastore Document Processing Config
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let documentProcessingConfig = data_store::create(
///         "documentProcessingConfig",
///         DataStoreArgs::builder()
///             .content_config("NO_CONTENT")
///             .create_advanced_site_search(false)
///             .data_store_id("data-store-id")
///             .display_name("tf-test-structured-datastore")
///             .document_processing_config(
///                 DataStoreDocumentProcessingConfig::builder()
///                     .defaultParsingConfig(
///                         DataStoreDocumentProcessingConfigDefaultParsingConfig::builder()
///                             .digitalParsingConfig(
///                                 DataStoreDocumentProcessingConfigDefaultParsingConfigDigitalParsingConfig::builder()
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .parsingConfigOverrides(
///                         vec![
///                             DataStoreDocumentProcessingConfigParsingConfigOverride::builder()
///                             .fileType("pdf")
///                             .ocrParsingConfig(DataStoreDocumentProcessingConfigParsingConfigOverrideOcrParsingConfig::builder()
///                             .useNativeText(true).build_struct()).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .industry_vertical("GENERIC")
///             .location("global")
///             .solution_types(vec!["SOLUTION_TYPE_SEARCH",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// DataStore can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/collections/default_collection/dataStores/{{data_store_id}}`
///
/// * `{{project}}/{{location}}/{{data_store_id}}`
///
/// * `{{location}}/{{data_store_id}}`
///
/// When using the `pulumi import` command, DataStore can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/dataStore:DataStore default projects/{{project}}/locations/{{location}}/collections/default_collection/dataStores/{{data_store_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/dataStore:DataStore default {{project}}/{{location}}/{{data_store_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/dataStore:DataStore default {{location}}/{{data_store_id}}
/// ```
///
pub mod data_store {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataStoreArgs {
        /// The content config of the data store.
        /// Possible values are: `NO_CONTENT`, `CONTENT_REQUIRED`, `PUBLIC_WEBSITE`.
        #[builder(into)]
        pub content_config: pulumi_wasm_rust::Output<String>,
        /// If true, an advanced data store for site search will be created. If the
        /// data store is not configured as site search (GENERIC vertical and
        /// PUBLIC_WEBSITE contentConfig), this flag will be ignored.
        #[builder(into, default)]
        pub create_advanced_site_search: pulumi_wasm_rust::Output<Option<bool>>,
        /// The unique id of the data store.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub data_store_id: pulumi_wasm_rust::Output<String>,
        /// The display name of the data store. This field must be a UTF-8 encoded
        /// string with a length limit of 128 characters.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Configuration for Document understanding and enrichment.
        /// Structure is documented below.
        #[builder(into, default)]
        pub document_processing_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::discoveryengine::DataStoreDocumentProcessingConfig,
            >,
        >,
        /// The industry vertical that the data store registers.
        /// Possible values are: `GENERIC`, `MEDIA`, `HEALTHCARE_FHIR`.
        #[builder(into)]
        pub industry_vertical: pulumi_wasm_rust::Output<String>,
        /// The geographic location where the data store should reside. The value can
        /// only be one of "global", "us" and "eu".
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A boolean flag indicating whether to skip the default schema creation for
        /// the data store. Only enable this flag if you are certain that the default
        /// schema is incompatible with your use case.
        /// If set to true, you must manually create a schema for the data store
        /// before any documents can be ingested.
        /// This flag cannot be specified if `data_store.starting_schema` is
        /// specified.
        #[builder(into, default)]
        pub skip_default_schema_creation: pulumi_wasm_rust::Output<Option<bool>>,
        /// The solutions that the data store enrolls.
        /// Each value may be one of: `SOLUTION_TYPE_RECOMMENDATION`, `SOLUTION_TYPE_SEARCH`, `SOLUTION_TYPE_CHAT`, `SOLUTION_TYPE_GENERATIVE_CHAT`.
        #[builder(into, default)]
        pub solution_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct DataStoreResult {
        /// The content config of the data store.
        /// Possible values are: `NO_CONTENT`, `CONTENT_REQUIRED`, `PUBLIC_WEBSITE`.
        pub content_config: pulumi_wasm_rust::Output<String>,
        /// If true, an advanced data store for site search will be created. If the
        /// data store is not configured as site search (GENERIC vertical and
        /// PUBLIC_WEBSITE contentConfig), this flag will be ignored.
        pub create_advanced_site_search: pulumi_wasm_rust::Output<Option<bool>>,
        /// Timestamp when the DataStore was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The unique id of the data store.
        ///
        ///
        /// - - -
        pub data_store_id: pulumi_wasm_rust::Output<String>,
        /// The id of the default Schema associated with this data store.
        pub default_schema_id: pulumi_wasm_rust::Output<String>,
        /// The display name of the data store. This field must be a UTF-8 encoded
        /// string with a length limit of 128 characters.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Configuration for Document understanding and enrichment.
        /// Structure is documented below.
        pub document_processing_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::discoveryengine::DataStoreDocumentProcessingConfig,
            >,
        >,
        /// The industry vertical that the data store registers.
        /// Possible values are: `GENERIC`, `MEDIA`, `HEALTHCARE_FHIR`.
        pub industry_vertical: pulumi_wasm_rust::Output<String>,
        /// The geographic location where the data store should reside. The value can
        /// only be one of "global", "us" and "eu".
        pub location: pulumi_wasm_rust::Output<String>,
        /// The unique full resource name of the data store. Values are of the format
        /// `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`.
        /// This field must be a UTF-8 encoded string with a length limit of 1024
        /// characters.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// A boolean flag indicating whether to skip the default schema creation for
        /// the data store. Only enable this flag if you are certain that the default
        /// schema is incompatible with your use case.
        /// If set to true, you must manually create a schema for the data store
        /// before any documents can be ingested.
        /// This flag cannot be specified if `data_store.starting_schema` is
        /// specified.
        pub skip_default_schema_creation: pulumi_wasm_rust::Output<Option<bool>>,
        /// The solutions that the data store enrolls.
        /// Each value may be one of: `SOLUTION_TYPE_RECOMMENDATION`, `SOLUTION_TYPE_SEARCH`, `SOLUTION_TYPE_CHAT`, `SOLUTION_TYPE_GENERATIVE_CHAT`.
        pub solution_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DataStoreArgs) -> DataStoreResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_config_binding = args.content_config.get_inner();
        let create_advanced_site_search_binding = args
            .create_advanced_site_search
            .get_inner();
        let data_store_id_binding = args.data_store_id.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let document_processing_config_binding = args
            .document_processing_config
            .get_inner();
        let industry_vertical_binding = args.industry_vertical.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let skip_default_schema_creation_binding = args
            .skip_default_schema_creation
            .get_inner();
        let solution_types_binding = args.solution_types.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:discoveryengine/dataStore:DataStore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contentConfig".into(),
                    value: &content_config_binding,
                },
                register_interface::ObjectField {
                    name: "createAdvancedSiteSearch".into(),
                    value: &create_advanced_site_search_binding,
                },
                register_interface::ObjectField {
                    name: "dataStoreId".into(),
                    value: &data_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "documentProcessingConfig".into(),
                    value: &document_processing_config_binding,
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
                    name: "skipDefaultSchemaCreation".into(),
                    value: &skip_default_schema_creation_binding,
                },
                register_interface::ObjectField {
                    name: "solutionTypes".into(),
                    value: &solution_types_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "contentConfig".into(),
                },
                register_interface::ResultField {
                    name: "createAdvancedSiteSearch".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dataStoreId".into(),
                },
                register_interface::ResultField {
                    name: "defaultSchemaId".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "documentProcessingConfig".into(),
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
                    name: "skipDefaultSchemaCreation".into(),
                },
                register_interface::ResultField {
                    name: "solutionTypes".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DataStoreResult {
            content_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentConfig").unwrap(),
            ),
            create_advanced_site_search: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createAdvancedSiteSearch").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            data_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataStoreId").unwrap(),
            ),
            default_schema_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSchemaId").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            document_processing_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("documentProcessingConfig").unwrap(),
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
            skip_default_schema_creation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipDefaultSchemaCreation").unwrap(),
            ),
            solution_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("solutionTypes").unwrap(),
            ),
        }
    }
}
