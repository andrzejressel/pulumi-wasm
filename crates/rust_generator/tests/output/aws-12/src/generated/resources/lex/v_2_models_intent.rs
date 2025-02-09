///
///
/// ## Import
///
/// Using `pulumi import`, import Lex V2 Models Intent using the `intent_id:bot_id:bot_version:locale_id`. For example:
///
/// ```sh
/// $ pulumi import aws:lex/v2modelsIntent:V2modelsIntent example intent-42874:bot-11376:DRAFT:en_US
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod v_2_models_intent {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2modelsIntentArgs {
        /// Identifier of the bot associated with this intent.
        #[builder(into)]
        pub bot_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the bot associated with this intent.
        #[builder(into)]
        pub bot_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block for the response that Amazon Lex sends to the user when the intent is closed. See `closing_setting`.
        #[builder(into, default)]
        pub closing_setting: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsIntentClosingSetting>,
        >,
        #[builder(into, default)]
        pub confirmation_setting: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsIntentConfirmationSetting>,
        >,
        /// Description of the intent. Use the description to help identify the intent in lists.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for invoking the alias Lambda function for each user input. You can invoke this Lambda function to personalize user interaction. See `dialog_code_hook`.
        #[builder(into, default)]
        pub dialog_code_hook: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsIntentDialogCodeHook>,
        >,
        /// Configuration block for invoking the alias Lambda function when the intent is ready for fulfillment. You can invoke this function to complete the bot's transaction with the user. See `fulfillment_code_hook`.
        #[builder(into, default)]
        pub fulfillment_code_hook: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsIntentFulfillmentCodeHook>,
        >,
        /// Configuration block for the response that is sent to the user at the beginning of a conversation, before eliciting slot values. See `initial_response_setting`.
        #[builder(into, default)]
        pub initial_response_setting: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsIntentInitialResponseSetting>,
        >,
        /// Configuration blocks for contexts that must be active for this intent to be considered by Amazon Lex. When an intent has an input context list, Amazon Lex only considers using the intent in an interaction with the user when the specified contexts are included in the active context list for the session. If the contexts are not active, then Amazon Lex will not use the intent. A context can be automatically activated using the outputContexts property or it can be set at runtime. See `input_context`.
        #[builder(into, default)]
        pub input_contexts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::lex::V2ModelsIntentInputContext>>,
        >,
        /// Configuration block for information required to use the AMAZON.KendraSearchIntent intent to connect to an Amazon Kendra index. The AMAZON.KendraSearchIntent intent is called when Amazon Lex can't determine another intent to invoke. See `kendra_configuration`.
        #[builder(into, default)]
        pub kendra_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsIntentKendraConfiguration>,
        >,
        /// Identifier of the language and locale where this intent is used. All of the bots, slot types, and slots used by the intent must have the same locale.
        #[builder(into)]
        pub locale_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the intent. Intent names must be unique in the locale that contains the intent and cannot match the name of any built-in intent.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration blocks for contexts that the intent activates when it is fulfilled. You can use an output context to indicate the intents that Amazon Lex should consider for the next turn of the conversation with a customer. When you use the outputContextsList property, all of the contexts specified in the list are activated when the intent is fulfilled. You can set up to 10 output contexts. You can also set the number of conversation turns that the context should be active, or the length of time that the context should be active. See `output_context`.
        #[builder(into, default)]
        pub output_contexts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::lex::V2ModelsIntentOutputContext>>,
        >,
        /// Identifier for the built-in intent to base this intent on.
        #[builder(into, default)]
        pub parent_intent_signature: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for strings that a user might say to signal the intent. See `sample_utterance`.
        #[builder(into, default)]
        pub sample_utterances: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::lex::V2ModelsIntentSampleUtterance>>,
        >,
        /// Configuration block for a new list of slots and their priorities that are contained by the intent. This is ignored on create and only valid for updates. See `slot_priority`.
        #[builder(into, default)]
        pub slot_priorities: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::lex::V2ModelsIntentSlotPriority>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsIntentTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct V2modelsIntentResult {
        /// Identifier of the bot associated with this intent.
        pub bot_id: pulumi_gestalt_rust::Output<String>,
        /// Version of the bot associated with this intent.
        pub bot_version: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the response that Amazon Lex sends to the user when the intent is closed. See `closing_setting`.
        pub closing_setting: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsIntentClosingSetting>,
        >,
        pub confirmation_setting: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsIntentConfirmationSetting>,
        >,
        /// Timestamp of the date and time that the intent was created.
        pub creation_date_time: pulumi_gestalt_rust::Output<String>,
        /// Description of the intent. Use the description to help identify the intent in lists.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block for invoking the alias Lambda function for each user input. You can invoke this Lambda function to personalize user interaction. See `dialog_code_hook`.
        pub dialog_code_hook: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsIntentDialogCodeHook>,
        >,
        /// Configuration block for invoking the alias Lambda function when the intent is ready for fulfillment. You can invoke this function to complete the bot's transaction with the user. See `fulfillment_code_hook`.
        pub fulfillment_code_hook: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsIntentFulfillmentCodeHook>,
        >,
        /// Configuration block for the response that is sent to the user at the beginning of a conversation, before eliciting slot values. See `initial_response_setting`.
        pub initial_response_setting: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsIntentInitialResponseSetting>,
        >,
        /// Configuration blocks for contexts that must be active for this intent to be considered by Amazon Lex. When an intent has an input context list, Amazon Lex only considers using the intent in an interaction with the user when the specified contexts are included in the active context list for the session. If the contexts are not active, then Amazon Lex will not use the intent. A context can be automatically activated using the outputContexts property or it can be set at runtime. See `input_context`.
        pub input_contexts: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::lex::V2ModelsIntentInputContext>>,
        >,
        /// Unique identifier for the intent.
        pub intent_id: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for information required to use the AMAZON.KendraSearchIntent intent to connect to an Amazon Kendra index. The AMAZON.KendraSearchIntent intent is called when Amazon Lex can't determine another intent to invoke. See `kendra_configuration`.
        pub kendra_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsIntentKendraConfiguration>,
        >,
        /// Timestamp of the last time that the intent was modified.
        pub last_updated_date_time: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the language and locale where this intent is used. All of the bots, slot types, and slots used by the intent must have the same locale.
        pub locale_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the intent. Intent names must be unique in the locale that contains the intent and cannot match the name of any built-in intent.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration blocks for contexts that the intent activates when it is fulfilled. You can use an output context to indicate the intents that Amazon Lex should consider for the next turn of the conversation with a customer. When you use the outputContextsList property, all of the contexts specified in the list are activated when the intent is fulfilled. You can set up to 10 output contexts. You can also set the number of conversation turns that the context should be active, or the length of time that the context should be active. See `output_context`.
        pub output_contexts: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::lex::V2ModelsIntentOutputContext>>,
        >,
        /// Identifier for the built-in intent to base this intent on.
        pub parent_intent_signature: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block for strings that a user might say to signal the intent. See `sample_utterance`.
        pub sample_utterances: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::lex::V2ModelsIntentSampleUtterance>>,
        >,
        /// Configuration block for a new list of slots and their priorities that are contained by the intent. This is ignored on create and only valid for updates. See `slot_priority`.
        pub slot_priorities: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::lex::V2ModelsIntentSlotPriority>>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsIntentTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: V2modelsIntentArgs,
    ) -> V2modelsIntentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bot_id_binding_1 = args.bot_id.get_output(context);
        let bot_id_binding = bot_id_binding_1.get_inner();
        let bot_version_binding_1 = args.bot_version.get_output(context);
        let bot_version_binding = bot_version_binding_1.get_inner();
        let closing_setting_binding_1 = args.closing_setting.get_output(context);
        let closing_setting_binding = closing_setting_binding_1.get_inner();
        let confirmation_setting_binding_1 = args
            .confirmation_setting
            .get_output(context);
        let confirmation_setting_binding = confirmation_setting_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let dialog_code_hook_binding_1 = args.dialog_code_hook.get_output(context);
        let dialog_code_hook_binding = dialog_code_hook_binding_1.get_inner();
        let fulfillment_code_hook_binding_1 = args
            .fulfillment_code_hook
            .get_output(context);
        let fulfillment_code_hook_binding = fulfillment_code_hook_binding_1.get_inner();
        let initial_response_setting_binding_1 = args
            .initial_response_setting
            .get_output(context);
        let initial_response_setting_binding = initial_response_setting_binding_1
            .get_inner();
        let input_contexts_binding_1 = args.input_contexts.get_output(context);
        let input_contexts_binding = input_contexts_binding_1.get_inner();
        let kendra_configuration_binding_1 = args
            .kendra_configuration
            .get_output(context);
        let kendra_configuration_binding = kendra_configuration_binding_1.get_inner();
        let locale_id_binding_1 = args.locale_id.get_output(context);
        let locale_id_binding = locale_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let output_contexts_binding_1 = args.output_contexts.get_output(context);
        let output_contexts_binding = output_contexts_binding_1.get_inner();
        let parent_intent_signature_binding_1 = args
            .parent_intent_signature
            .get_output(context);
        let parent_intent_signature_binding = parent_intent_signature_binding_1
            .get_inner();
        let sample_utterances_binding_1 = args.sample_utterances.get_output(context);
        let sample_utterances_binding = sample_utterances_binding_1.get_inner();
        let slot_priorities_binding_1 = args.slot_priorities.get_output(context);
        let slot_priorities_binding = slot_priorities_binding_1.get_inner();
        let timeouts_binding_1 = args.timeouts.get_output(context);
        let timeouts_binding = timeouts_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lex/v2modelsIntent:V2modelsIntent".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "botId".into(),
                    value: &bot_id_binding,
                },
                register_interface::ObjectField {
                    name: "botVersion".into(),
                    value: &bot_version_binding,
                },
                register_interface::ObjectField {
                    name: "closingSetting".into(),
                    value: &closing_setting_binding,
                },
                register_interface::ObjectField {
                    name: "confirmationSetting".into(),
                    value: &confirmation_setting_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "dialogCodeHook".into(),
                    value: &dialog_code_hook_binding,
                },
                register_interface::ObjectField {
                    name: "fulfillmentCodeHook".into(),
                    value: &fulfillment_code_hook_binding,
                },
                register_interface::ObjectField {
                    name: "initialResponseSetting".into(),
                    value: &initial_response_setting_binding,
                },
                register_interface::ObjectField {
                    name: "inputContexts".into(),
                    value: &input_contexts_binding,
                },
                register_interface::ObjectField {
                    name: "kendraConfiguration".into(),
                    value: &kendra_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "localeId".into(),
                    value: &locale_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "outputContexts".into(),
                    value: &output_contexts_binding,
                },
                register_interface::ObjectField {
                    name: "parentIntentSignature".into(),
                    value: &parent_intent_signature_binding,
                },
                register_interface::ObjectField {
                    name: "sampleUtterances".into(),
                    value: &sample_utterances_binding,
                },
                register_interface::ObjectField {
                    name: "slotPriorities".into(),
                    value: &slot_priorities_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        V2modelsIntentResult {
            bot_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("botId"),
            ),
            bot_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("botVersion"),
            ),
            closing_setting: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("closingSetting"),
            ),
            confirmation_setting: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("confirmationSetting"),
            ),
            creation_date_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationDateTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            dialog_code_hook: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dialogCodeHook"),
            ),
            fulfillment_code_hook: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fulfillmentCodeHook"),
            ),
            initial_response_setting: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("initialResponseSetting"),
            ),
            input_contexts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputContexts"),
            ),
            intent_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("intentId"),
            ),
            kendra_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kendraConfiguration"),
            ),
            last_updated_date_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastUpdatedDateTime"),
            ),
            locale_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localeId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            output_contexts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outputContexts"),
            ),
            parent_intent_signature: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parentIntentSignature"),
            ),
            sample_utterances: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sampleUtterances"),
            ),
            slot_priorities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("slotPriorities"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
