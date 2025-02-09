/// Entities are extracted from user input and represent parameters that are meaningful to your application.
/// For example, a date range, a proper name such as a geographic location or landmark, and so on. Entities represent actionable data for your application.
///
///
/// To get more information about EntityType, see:
///
/// * [API documentation](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.entityTypes)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dialogflow/cx/docs)
///
/// ## Example Usage
///
/// ### Dialogflowcx Entity Type Full
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
///             .avatar_uri(
///                 "https://cloud.google.com/_static/images/cloud/icons/favicons/onecloud/super_cloud.png",
///             )
///             .default_language_code("en")
///             .description("Example description.")
///             .display_name("dialogflowcx-agent")
///             .enable_spell_correction(true)
///             .enable_stackdriver_logging(true)
///             .location("global")
///             .speech_to_text_settings(
///                 CxAgentSpeechToTextSettings::builder()
///                     .enableSpeechAdaptation(true)
///                     .build_struct(),
///             )
///             .supported_language_codes(vec!["fr", "de", "es",])
///             .time_zone("America/New_York")
///             .build_struct(),
///     );
///     let basicEntityType = cx_entity_type::create(
///         "basicEntityType",
///         CxEntityTypeArgs::builder()
///             .display_name("MyEntity")
///             .enable_fuzzy_extraction(false)
///             .entities(
///                 vec![
///                     CxEntityTypeEntity::builder().synonyms(vec!["synonym1", "synonym2",])
///                     .value("value1").build_struct(), CxEntityTypeEntity::builder()
///                     .synonyms(vec!["synonym3", "synonym4",]).value("value2")
///                     .build_struct(),
///                 ],
///             )
///             .kind("KIND_MAP")
///             .parent("${agent.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// EntityType can be imported using any of these accepted formats:
///
/// * `{{parent}}/entityTypes/{{name}}`
///
/// * `{{parent}}/{{name}}`
///
/// When using the `pulumi import` command, EntityType can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxEntityType:CxEntityType default {{parent}}/entityTypes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxEntityType:CxEntityType default {{parent}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cx_entity_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CxEntityTypeArgs {
        /// Represents kinds of entities. * AUTO_EXPANSION_MODE_UNSPECIFIED: Auto expansion disabled for the entity. *
        /// AUTO_EXPANSION_MODE_DEFAULT: Allows an agent to recognize values that have not been explicitly listed in the entity.
        /// Possible values: ["AUTO_EXPANSION_MODE_DEFAULT", "AUTO_EXPANSION_MODE_UNSPECIFIED"]
        #[builder(into, default)]
        pub auto_expansion_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The human-readable name of the entity type, unique within the agent.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Enables fuzzy entity extraction during classification.
        #[builder(into, default)]
        pub enable_fuzzy_extraction: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The collection of entity entries associated with the entity type.
        /// Structure is documented below.
        #[builder(into)]
        pub entities: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::diagflow::CxEntityTypeEntity>,
        >,
        /// Collection of exceptional words and phrases that shouldn't be matched. For example, if you have a size entity type with
        /// entry giant(an adjective), you might consider adding giants(a noun) as an exclusion. If the kind of entity type is
        /// KIND_MAP, then the phrases specified by entities and excluded phrases should be mutually exclusive.
        #[builder(into, default)]
        pub excluded_phrases: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::diagflow::CxEntityTypeExcludedPhrase>>,
        >,
        /// Indicates whether the entity type can be automatically expanded.
        /// * KIND_MAP: Map entity types allow mapping of a group of synonyms to a canonical value.
        /// * KIND_LIST: List entity types contain a set of entries that do not map to canonical values. However, list entity types can contain references to other entity types (with or without aliases).
        /// * KIND_REGEXP: Regexp entity types allow to specify regular expressions in entries values.
        /// Possible values are: `KIND_MAP`, `KIND_LIST`, `KIND_REGEXP`.
        #[builder(into)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The language of the following fields in entityType: EntityType.entities.value EntityType.entities.synonyms
        /// EntityType.excluded_phrases.value If not specified, the agent's default language is used. Many languages are supported.
        /// Note: languages must be enabled in the agent before they can be used.
        #[builder(into, default)]
        pub language_code: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The agent to create a entity type for. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>.
        #[builder(into, default)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether parameters of the entity type should be redacted in log. If redaction is enabled, page parameters and
        /// intent parameters referring to the entity type will be replaced by parameter name when logging.
        #[builder(into, default)]
        pub redact: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct CxEntityTypeResult {
        /// Represents kinds of entities. * AUTO_EXPANSION_MODE_UNSPECIFIED: Auto expansion disabled for the entity. *
        /// AUTO_EXPANSION_MODE_DEFAULT: Allows an agent to recognize values that have not been explicitly listed in the entity.
        /// Possible values: ["AUTO_EXPANSION_MODE_DEFAULT", "AUTO_EXPANSION_MODE_UNSPECIFIED"]
        pub auto_expansion_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The human-readable name of the entity type, unique within the agent.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Enables fuzzy entity extraction during classification.
        pub enable_fuzzy_extraction: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The collection of entity entries associated with the entity type.
        /// Structure is documented below.
        pub entities: pulumi_gestalt_rust::Output<
            Vec<super::super::types::diagflow::CxEntityTypeEntity>,
        >,
        /// Collection of exceptional words and phrases that shouldn't be matched. For example, if you have a size entity type with
        /// entry giant(an adjective), you might consider adding giants(a noun) as an exclusion. If the kind of entity type is
        /// KIND_MAP, then the phrases specified by entities and excluded phrases should be mutually exclusive.
        pub excluded_phrases: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::diagflow::CxEntityTypeExcludedPhrase>>,
        >,
        /// Indicates whether the entity type can be automatically expanded.
        /// * KIND_MAP: Map entity types allow mapping of a group of synonyms to a canonical value.
        /// * KIND_LIST: List entity types contain a set of entries that do not map to canonical values. However, list entity types can contain references to other entity types (with or without aliases).
        /// * KIND_REGEXP: Regexp entity types allow to specify regular expressions in entries values.
        /// Possible values are: `KIND_MAP`, `KIND_LIST`, `KIND_REGEXP`.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The language of the following fields in entityType: EntityType.entities.value EntityType.entities.synonyms
        /// EntityType.excluded_phrases.value If not specified, the agent's default language is used. Many languages are supported.
        /// Note: languages must be enabled in the agent before they can be used.
        pub language_code: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique identifier of the entity type.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/entityTypes/<Entity Type ID>.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The agent to create a entity type for. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>.
        pub parent: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates whether parameters of the entity type should be redacted in log. If redaction is enabled, page parameters and
        /// intent parameters referring to the entity type will be replaced by parameter name when logging.
        pub redact: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CxEntityTypeArgs,
    ) -> CxEntityTypeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_expansion_mode_binding = args.auto_expansion_mode.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enable_fuzzy_extraction_binding = args
            .enable_fuzzy_extraction
            .get_output(context);
        let entities_binding = args.entities.get_output(context);
        let excluded_phrases_binding = args.excluded_phrases.get_output(context);
        let kind_binding = args.kind.get_output(context);
        let language_code_binding = args.language_code.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let redact_binding = args.redact.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:diagflow/cxEntityType:CxEntityType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoExpansionMode".into(),
                    value: auto_expansion_mode_binding.get_id(),
                },
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
                    name: "excludedPhrases".into(),
                    value: excluded_phrases_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kind".into(),
                    value: kind_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "languageCode".into(),
                    value: language_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redact".into(),
                    value: redact_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CxEntityTypeResult {
            auto_expansion_mode: o.get_field("autoExpansionMode"),
            display_name: o.get_field("displayName"),
            enable_fuzzy_extraction: o.get_field("enableFuzzyExtraction"),
            entities: o.get_field("entities"),
            excluded_phrases: o.get_field("excludedPhrases"),
            kind: o.get_field("kind"),
            language_code: o.get_field("languageCode"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            redact: o.get_field("redact"),
        }
    }
}
