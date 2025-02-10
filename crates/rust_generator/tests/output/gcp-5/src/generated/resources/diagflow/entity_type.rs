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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod entity_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EntityTypeArgs {
        /// The name of this entity type to be displayed on the console.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Enables fuzzy entity extraction during classification.
        #[builder(into, default)]
        pub enable_fuzzy_extraction: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The collection of entity entries associated with the entity type.
        /// Structure is documented below.
        #[builder(into, default)]
        pub entities: pulumi_gestalt_rust::InputOrOutput<
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
        pub kind: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EntityTypeResult {
        /// The name of this entity type to be displayed on the console.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Enables fuzzy entity extraction during classification.
        pub enable_fuzzy_extraction: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The collection of entity entries associated with the entity type.
        /// Structure is documented below.
        pub entities: pulumi_gestalt_rust::Output<
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
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier of the entity type.
        /// Format: projects/<Project ID>/agent/entityTypes/<Entity type ID>.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EntityTypeArgs,
    ) -> EntityTypeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let enable_fuzzy_extraction_binding = args
            .enable_fuzzy_extraction
            .get_output(context);
        let entities_binding = args.entities.get_output(context);
        let kind_binding = args.kind.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:diagflow/entityType:EntityType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableFuzzyExtraction".into(),
                    value: enable_fuzzy_extraction_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "entities".into(),
                    value: entities_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kind".into(),
                    value: kind_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EntityTypeResult {
            display_name: o.get_field("displayName"),
            enable_fuzzy_extraction: o.get_field("enableFuzzyExtraction"),
            entities: o.get_field("entities"),
            kind: o.get_field("kind"),
            name: o.get_field("name"),
            project: o.get_field("project"),
        }
    }
}
