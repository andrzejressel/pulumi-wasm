/// Represents an entity type. Entity types serve as a tool for extracting parameter values from natural language queries.
///
///
/// To get more information about EntityType, see:
///
/// * [API documentation](https://cloud.google.com/dialogflow/docs/reference/rest/v2/projects.agent.entityTypes)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dialogflow/docs/)
///
/// ## Example Usage
///
/// ### Dialogflow Entity Type Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicAgent = agent::create(
///         "basicAgent",
///         AgentArgs::builder()
///             .default_language_code("en")
///             .display_name("example_agent")
///             .time_zone("America/New_York")
///             .build_struct(),
///     );
///     let basicEntityType = entity_type::create(
///         "basicEntityType",
///         EntityTypeArgs::builder()
///             .display_name("")
///             .entities(
///                 vec![
///                     EntityTypeEntity::builder().synonyms(vec!["synonym1", "synonym2",])
///                     .value("value1").build_struct(), EntityTypeEntity::builder()
///                     .synonyms(vec!["synonym3", "synonym4",]).value("value2")
///                     .build_struct(),
///                 ],
///             )
///             .kind("KIND_MAP")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// EntityType can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, EntityType can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:diagflow/entityType:EntityType default {{name}}
/// ```
///
pub mod entity_type {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EntityTypeArgs {
        /// The name of this entity type to be displayed on the console.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Enables fuzzy entity extraction during classification.
        #[builder(into, default)]
        pub enable_fuzzy_extraction: pulumi_wasm_rust::Output<Option<bool>>,
        /// The collection of entity entries associated with the entity type.
        /// Structure is documented below.
        #[builder(into, default)]
        pub entities: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::diagflow::EntityTypeEntity>>,
        >,
        /// Indicates the kind of entity type.
        /// * KIND_MAP: Map entity types allow mapping of a group of synonyms to a reference value.
        /// * KIND_LIST: List entity types contain a set of entries that do not map to reference values. However, list entity
        /// types can contain references to other entity types (with or without aliases).
        /// * KIND_REGEXP: Regexp entity types allow to specify regular expressions in entries values.
        /// Possible values are: `KIND_MAP`, `KIND_LIST`, `KIND_REGEXP`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EntityTypeResult {
        /// The name of this entity type to be displayed on the console.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Enables fuzzy entity extraction during classification.
        pub enable_fuzzy_extraction: pulumi_wasm_rust::Output<Option<bool>>,
        /// The collection of entity entries associated with the entity type.
        /// Structure is documented below.
        pub entities: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::diagflow::EntityTypeEntity>>,
        >,
        /// Indicates the kind of entity type.
        /// * KIND_MAP: Map entity types allow mapping of a group of synonyms to a reference value.
        /// * KIND_LIST: List entity types contain a set of entries that do not map to reference values. However, list entity
        /// types can contain references to other entity types (with or without aliases).
        /// * KIND_REGEXP: Regexp entity types allow to specify regular expressions in entries values.
        /// Possible values are: `KIND_MAP`, `KIND_LIST`, `KIND_REGEXP`.
        ///
        ///
        /// - - -
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The unique identifier of the entity type.
        /// Format: projects/<Project ID>/agent/entityTypes/<Entity type ID>.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EntityTypeArgs) -> EntityTypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_inner();
        let enable_fuzzy_extraction_binding = args.enable_fuzzy_extraction.get_inner();
        let entities_binding = args.entities.get_inner();
        let kind_binding = args.kind.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:diagflow/entityType:EntityType".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "enableFuzzyExtraction".into(),
                    value: &enable_fuzzy_extraction_binding,
                },
                register_interface::ObjectField {
                    name: "entities".into(),
                    value: &entities_binding,
                },
                register_interface::ObjectField {
                    name: "kind".into(),
                    value: &kind_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "enableFuzzyExtraction".into(),
                },
                register_interface::ResultField {
                    name: "entities".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EntityTypeResult {
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            enable_fuzzy_extraction: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableFuzzyExtraction").unwrap(),
            ),
            entities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entities").unwrap(),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
        }
    }
}
