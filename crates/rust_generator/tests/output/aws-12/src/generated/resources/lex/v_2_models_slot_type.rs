/// Resource for managing an AWS Lex V2 Models Slot Type.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:lex:V2modelsBot
///     properties:
///       name: example
///       idleSessionTtlInSeconds: 60
///       roleArn: ${exampleAwsIamRole.arn}
///       dataPrivacies:
///         - childDirected: true
///   exampleV2modelsBotLocale:
///     type: aws:lex:V2modelsBotLocale
///     name: example
///     properties:
///       localeId: en_US
///       botId: ${example.id}
///       botVersion: DRAFT
///       nLuIntentConfidenceThreshold: 0.7
///   exampleV2modelsBotVersion:
///     type: aws:lex:V2modelsBotVersion
///     name: example
///     properties:
///       botId: ${example.id}
///       localeSpecification:
///         ${exampleV2modelsBotLocale.localeId}:
///           sourceBotVersion: DRAFT
///   exampleV2modelsSlotType:
///     type: aws:lex:V2modelsSlotType
///     name: example
///     properties:
///       botId: ${example.id}
///       botVersion: ${exampleV2modelsBotLocale.botVersion}
///       name: example
///       localeId: ${exampleV2modelsBotLocale.localeId}
/// ```
///
/// ### value_selection_setting Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = v_2_models_slot_type::create(
///         "example",
///         V2ModelsSlotTypeArgs::builder()
///             .bot_id("${exampleAwsLexv2modelsBot.id}")
///             .bot_version("${exampleAwsLexv2modelsBotLocale.botVersion}")
///             .locale_id("${exampleAwsLexv2modelsBotLocale.localeId}")
///             .name("example")
///             .slot_type_values(
///                 V2ModelsSlotTypeSlotTypeValues::builder()
///                     .sampleValues(
///                         vec![
///                             V2ModelsSlotTypeSlotTypeValuesSampleValue::builder()
///                             .value("exampleValue").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .value_selection_setting(
///                 V2ModelsSlotTypeValueSelectionSetting::builder()
///                     .advancedRecognitionSettings(
///                         vec![
///                             V2ModelsSlotTypeValueSelectionSettingAdvancedRecognitionSetting::builder()
///                             .audioRecognitionStrategy("UseSlotValuesAsCustomVocabulary")
///                             .build_struct(),
///                         ],
///                     )
///                     .resolutionStrategy("OriginalValue")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lex V2 Models Slot Type using using a comma-delimited string concatenating `bot_id`, `bot_version`, `locale_id`, and `slot_type_id`. For example:
///
/// ```sh
/// $ pulumi import aws:lex/v2modelsSlotType:V2modelsSlotType example bot-1234,DRAFT,en_US,slot_type-id-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod v_2_models_slot_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2modelsSlotTypeArgs {
        /// Identifier of the bot associated with this slot type.
        #[builder(into)]
        pub bot_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the bot associated with this slot type.
        #[builder(into)]
        pub bot_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifications for a composite slot type.
        /// See `composite_slot_type_setting` argument reference below.
        #[builder(into, default)]
        pub composite_slot_type_setting: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsSlotTypeCompositeSlotTypeSetting>,
        >,
        /// Description of the slot type.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of external information used to create the slot type.
        /// See `external_source_setting` argument reference below.
        #[builder(into, default)]
        pub external_source_setting: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsSlotTypeExternalSourceSetting>,
        >,
        /// Identifier of the language and locale where this slot type is used.
        /// All of the bots, slot types, and slots used by the intent must have the same locale.
        #[builder(into)]
        pub locale_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the slot type.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Built-in slot type used as a parent of this slot type.
        /// When you define a parent slot type, the new slot type has the configuration of the parent slot type.
        /// Only `AMAZON.AlphaNumeric` is supported.
        #[builder(into, default)]
        pub parent_slot_type_signature: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// List of SlotTypeValue objects that defines the values that the slot type can take.
        /// Each value can have a list of synonyms, additional values that help train the machine learning model about the values that it resolves for a slot.
        /// See `slot_type_values` argument reference below.
        #[builder(into, default)]
        pub slot_type_values: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsSlotTypeSlotTypeValues>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsSlotTypeTimeouts>,
        >,
        /// Determines the strategy that Amazon Lex uses to select a value from the list of possible values.
        /// See `value_selection_setting` argument reference below.
        #[builder(into, default)]
        pub value_selection_setting: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsSlotTypeValueSelectionSetting>,
        >,
    }
    #[allow(dead_code)]
    pub struct V2modelsSlotTypeResult {
        /// Identifier of the bot associated with this slot type.
        pub bot_id: pulumi_gestalt_rust::Output<String>,
        /// Version of the bot associated with this slot type.
        pub bot_version: pulumi_gestalt_rust::Output<String>,
        /// Specifications for a composite slot type.
        /// See `composite_slot_type_setting` argument reference below.
        pub composite_slot_type_setting: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsSlotTypeCompositeSlotTypeSetting>,
        >,
        /// Description of the slot type.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Type of external information used to create the slot type.
        /// See `external_source_setting` argument reference below.
        pub external_source_setting: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsSlotTypeExternalSourceSetting>,
        >,
        /// Identifier of the language and locale where this slot type is used.
        /// All of the bots, slot types, and slots used by the intent must have the same locale.
        pub locale_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the slot type.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Built-in slot type used as a parent of this slot type.
        /// When you define a parent slot type, the new slot type has the configuration of the parent slot type.
        /// Only `AMAZON.AlphaNumeric` is supported.
        pub parent_slot_type_signature: pulumi_gestalt_rust::Output<Option<String>>,
        /// Unique identifier for the slot type.
        pub slot_type_id: pulumi_gestalt_rust::Output<String>,
        /// List of SlotTypeValue objects that defines the values that the slot type can take.
        /// Each value can have a list of synonyms, additional values that help train the machine learning model about the values that it resolves for a slot.
        /// See `slot_type_values` argument reference below.
        pub slot_type_values: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsSlotTypeSlotTypeValues>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsSlotTypeTimeouts>,
        >,
        /// Determines the strategy that Amazon Lex uses to select a value from the list of possible values.
        /// See `value_selection_setting` argument reference below.
        pub value_selection_setting: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsSlotTypeValueSelectionSetting>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: V2modelsSlotTypeArgs,
    ) -> V2modelsSlotTypeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bot_id_binding = args.bot_id.get_output(context);
        let bot_version_binding = args.bot_version.get_output(context);
        let composite_slot_type_setting_binding = args
            .composite_slot_type_setting
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let external_source_setting_binding = args
            .external_source_setting
            .get_output(context);
        let locale_id_binding = args.locale_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_slot_type_signature_binding = args
            .parent_slot_type_signature
            .get_output(context);
        let slot_type_values_binding = args.slot_type_values.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let value_selection_setting_binding = args
            .value_selection_setting
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lex/v2modelsSlotType:V2modelsSlotType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "botId".into(),
                    value: &bot_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "botVersion".into(),
                    value: &bot_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "compositeSlotTypeSetting".into(),
                    value: &composite_slot_type_setting_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "externalSourceSetting".into(),
                    value: &external_source_setting_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localeId".into(),
                    value: &locale_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentSlotTypeSignature".into(),
                    value: &parent_slot_type_signature_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "slotTypeValues".into(),
                    value: &slot_type_values_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "valueSelectionSetting".into(),
                    value: &value_selection_setting_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        V2modelsSlotTypeResult {
            bot_id: o.get_field("botId"),
            bot_version: o.get_field("botVersion"),
            composite_slot_type_setting: o.get_field("compositeSlotTypeSetting"),
            description: o.get_field("description"),
            external_source_setting: o.get_field("externalSourceSetting"),
            locale_id: o.get_field("localeId"),
            name: o.get_field("name"),
            parent_slot_type_signature: o.get_field("parentSlotTypeSignature"),
            slot_type_id: o.get_field("slotTypeId"),
            slot_type_values: o.get_field("slotTypeValues"),
            timeouts: o.get_field("timeouts"),
            value_selection_setting: o.get_field("valueSelectionSetting"),
        }
    }
}
