///
///
/// ## Import
///
/// Using `pulumi import`, import Lex V2 Models Slot using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:lex/v2modelsSlot:V2modelsSlot example bot-1234,1,intent-5678,en-US,slot-9012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod v_2_models_slot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2modelsSlotArgs {
        /// Identifier of the bot associated with the slot.
        #[builder(into)]
        pub bot_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the bot associated with the slot.
        #[builder(into)]
        pub bot_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the slot.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the intent that contains the slot.
        #[builder(into)]
        pub intent_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of the language and locale that the slot will be used in.
        #[builder(into)]
        pub locale_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the slot returns multiple values in one response.
        /// See the `multiple_values_setting` argument reference below.
        #[builder(into, default)]
        pub multiple_values_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::lex::V2ModelsSlotMultipleValuesSetting>>,
        >,
        /// Name of the slot.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Determines how slot values are used in Amazon CloudWatch logs.
        /// See the `obfuscation_setting` argument reference below.
        #[builder(into, default)]
        pub obfuscation_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::lex::V2ModelsSlotObfuscationSetting>>,
        >,
        /// Unique identifier for the slot type associated with this slot.
        #[builder(into, default)]
        pub slot_type_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifications for the constituent sub slots and the expression for the composite slot.
        /// See the `sub_slot_setting` argument reference below.
        #[builder(into, default)]
        pub sub_slot_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSetting>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsSlotTimeouts>,
        >,
        #[builder(into, default)]
        pub value_elicitation_setting: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsSlotValueElicitationSetting>,
        >,
    }
    #[allow(dead_code)]
    pub struct V2modelsSlotResult {
        /// Identifier of the bot associated with the slot.
        pub bot_id: pulumi_gestalt_rust::Output<String>,
        /// Version of the bot associated with the slot.
        pub bot_version: pulumi_gestalt_rust::Output<String>,
        /// Description of the slot.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of the intent that contains the slot.
        pub intent_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the language and locale that the slot will be used in.
        pub locale_id: pulumi_gestalt_rust::Output<String>,
        /// Whether the slot returns multiple values in one response.
        /// See the `multiple_values_setting` argument reference below.
        pub multiple_values_settings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::lex::V2ModelsSlotMultipleValuesSetting>>,
        >,
        /// Name of the slot.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Determines how slot values are used in Amazon CloudWatch logs.
        /// See the `obfuscation_setting` argument reference below.
        pub obfuscation_settings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::lex::V2ModelsSlotObfuscationSetting>>,
        >,
        /// Unique identifier associated with the slot.
        pub slot_id: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier for the slot type associated with this slot.
        pub slot_type_id: pulumi_gestalt_rust::Output<String>,
        /// Specifications for the constituent sub slots and the expression for the composite slot.
        /// See the `sub_slot_setting` argument reference below.
        pub sub_slot_settings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSetting>>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsSlotTimeouts>,
        >,
        pub value_elicitation_setting: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsSlotValueElicitationSetting>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: V2modelsSlotArgs,
    ) -> V2modelsSlotResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bot_id_binding = args.bot_id.get_output(context);
        let bot_version_binding = args.bot_version.get_output(context);
        let description_binding = args.description.get_output(context);
        let intent_id_binding = args.intent_id.get_output(context);
        let locale_id_binding = args.locale_id.get_output(context);
        let multiple_values_settings_binding = args
            .multiple_values_settings
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let obfuscation_settings_binding = args.obfuscation_settings.get_output(context);
        let slot_type_id_binding = args.slot_type_id.get_output(context);
        let sub_slot_settings_binding = args.sub_slot_settings.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let value_elicitation_setting_binding = args
            .value_elicitation_setting
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lex/v2modelsSlot:V2modelsSlot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "botId".into(),
                    value: bot_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "botVersion".into(),
                    value: bot_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "intentId".into(),
                    value: intent_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localeId".into(),
                    value: locale_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multipleValuesSettings".into(),
                    value: multiple_values_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "obfuscationSettings".into(),
                    value: obfuscation_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "slotTypeId".into(),
                    value: slot_type_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subSlotSettings".into(),
                    value: sub_slot_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "valueElicitationSetting".into(),
                    value: value_elicitation_setting_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        V2modelsSlotResult {
            bot_id: o.get_field("botId"),
            bot_version: o.get_field("botVersion"),
            description: o.get_field("description"),
            intent_id: o.get_field("intentId"),
            locale_id: o.get_field("localeId"),
            multiple_values_settings: o.get_field("multipleValuesSettings"),
            name: o.get_field("name"),
            obfuscation_settings: o.get_field("obfuscationSettings"),
            slot_id: o.get_field("slotId"),
            slot_type_id: o.get_field("slotTypeId"),
            sub_slot_settings: o.get_field("subSlotSettings"),
            timeouts: o.get_field("timeouts"),
            value_elicitation_setting: o.get_field("valueElicitationSetting"),
        }
    }
}
