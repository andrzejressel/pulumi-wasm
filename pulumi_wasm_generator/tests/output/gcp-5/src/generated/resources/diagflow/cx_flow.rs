/// Flows represents the conversation flows when you build your chatbot agent.
///
///
/// To get more information about Flow, see:
///
/// * [API documentation](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dialogflow/cx/docs)
///
/// ## Example Usage
///
/// ### Dialogflowcx Flow Basic
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
///     let basicFlow = cx_flow::create(
///         "basicFlow",
///         CxFlowArgs::builder()
///             .description("Test Flow")
///             .display_name("MyFlow")
///             .event_handlers(
///                 vec![
///                     CxFlowEventHandler::builder().event("custom-event")
///                     .triggerFulfillment(CxFlowEventHandlerTriggerFulfillment::builder()
///                     .messages(vec![CxFlowEventHandlerTriggerFulfillmentMessage::builder()
///                     .text(CxFlowEventHandlerTriggerFulfillmentMessageText::builder()
///                     .texts(vec!["I didn't get that. Can you say it again?",])
///                     .build_struct()).build_struct(),]).returnPartialResponses(false)
///                     .build_struct()).build_struct(), CxFlowEventHandler::builder()
///                     .event("sys.no-match-default")
///                     .triggerFulfillment(CxFlowEventHandlerTriggerFulfillment::builder()
///                     .messages(vec![CxFlowEventHandlerTriggerFulfillmentMessage::builder()
///                     .text(CxFlowEventHandlerTriggerFulfillmentMessageText::builder()
///                     .texts(vec!["Sorry, could you say that again?",]).build_struct())
///                     .build_struct(),]).returnPartialResponses(false).build_struct())
///                     .build_struct(), CxFlowEventHandler::builder()
///                     .event("sys.no-input-default")
///                     .triggerFulfillment(CxFlowEventHandlerTriggerFulfillment::builder()
///                     .messages(vec![CxFlowEventHandlerTriggerFulfillmentMessage::builder()
///                     .text(CxFlowEventHandlerTriggerFulfillmentMessageText::builder()
///                     .texts(vec!["One more time?",]).build_struct()).build_struct(),])
///                     .returnPartialResponses(false).build_struct()).build_struct(),
///                 ],
///             )
///             .nlu_settings(
///                 CxFlowNluSettings::builder()
///                     .classificationThreshold(0.3)
///                     .modelType("MODEL_TYPE_STANDARD")
///                     .build_struct(),
///             )
///             .parent("${agent.id}")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dialogflowcx Flow Full
///
///
/// ```yaml
/// resources:
///   agent:
///     type: gcp:diagflow:CxAgent
///     properties:
///       displayName: dialogflowcx-agent
///       location: global
///       defaultLanguageCode: en
///       supportedLanguageCodes:
///         - fr
///         - de
///         - es
///       timeZone: America/New_York
///       description: Example description.
///       avatarUri: https://cloud.google.com/_static/images/cloud/icons/favicons/onecloud/super_cloud.png
///       enableStackdriverLogging: true
///       enableSpellCorrection: true
///       speechToTextSettings:
///         enableSpeechAdaptation: true
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: dialogflowcx-bucket
///       location: US
///       uniformBucketLevelAccess: true
///   basicFlow:
///     type: gcp:diagflow:CxFlow
///     name: basic_flow
///     properties:
///       parent: ${agent.id}
///       displayName: MyFlow
///       description: Test Flow
///       nluSettings:
///         classificationThreshold: 0.3
///         modelType: MODEL_TYPE_STANDARD
///       eventHandlers:
///         - event: custom-event
///           triggerFulfillment:
///             returnPartialResponses: false
///             messages:
///               - text:
///                   texts:
///                     - I didn't get that. Can you say it again?
///         - event: sys.no-match-default
///           triggerFulfillment:
///             returnPartialResponses: false
///             messages:
///               - text:
///                   texts:
///                     - Sorry, could you say that again?
///         - event: sys.no-input-default
///           triggerFulfillment:
///             returnPartialResponses: false
///             messages:
///               - text:
///                   texts:
///                     - One more time?
///         - event: another-event
///           triggerFulfillment:
///             returnPartialResponses: true
///             messages:
///               - channel: some-channel
///                 text:
///                   texts:
///                     - Some text
///               - payload: |2
///                             {"some-key": "some-value", "other-key": ["other-value"]}
///               - conversationSuccess:
///                   metadata: |2
///                                 {"some-metadata-key": "some-value", "other-metadata-key": 1234}
///               - outputAudioText:
///                   text: some output text
///               - outputAudioText:
///                   ssml: |2
///                                 <speak>Some example <say-as interpret-as="characters">SSML XML</say-as></speak>
///               - liveAgentHandoff:
///                   metadata: |2
///                                 {"some-metadata-key": "some-value", "other-metadata-key": 1234}
///               - playAudio:
///                   audioUri: http://example.com/some-audio-file.mp3
///               - telephonyTransferCall:
///                   phoneNumber: 1-234-567-8901
///             setParameterActions:
///               - parameter: some-param
///                 value: '123.45'
///               - parameter: another-param
///                 value:
///                   fn::toJSON: abc
///               - parameter: other-param
///                 value:
///                   fn::toJSON:
///                     - foo
///             conditionalCases:
///               - cases:
///                   fn::toJSON:
///                     - condition: $sys.func.RAND() < 0.5
///                       caseContent:
///                         - message:
///                             text:
///                               text:
///                                 - First case
///                         - additionalCases:
///                             cases:
///                               - condition: $sys.func.RAND() < 0.2
///                                 caseContent:
///                                   - message:
///                                       text:
///                                         text:
///                                           - Nested case
///                     - caseContent:
///                         - message:
///                             text:
///                               text:
///                                 - Final case
///       transitionRoutes:
///         - condition: 'true'
///           triggerFulfillment:
///             returnPartialResponses: true
///             messages:
///               - channel: some-channel
///                 text:
///                   texts:
///                     - Some text
///               - payload: |2
///                             {"some-key": "some-value", "other-key": ["other-value"]}
///               - conversationSuccess:
///                   metadata: |2
///                                 {"some-metadata-key": "some-value", "other-metadata-key": 1234}
///               - outputAudioText:
///                   text: some output text
///               - outputAudioText:
///                   ssml: |2
///                                 <speak>Some example <say-as interpret-as="characters">SSML XML</say-as></speak>
///               - liveAgentHandoff:
///                   metadata: |2
///                                 {"some-metadata-key": "some-value", "other-metadata-key": 1234}
///               - playAudio:
///                   audioUri: http://example.com/some-audio-file.mp3
///               - telephonyTransferCall:
///                   phoneNumber: 1-234-567-8901
///             setParameterActions:
///               - parameter: some-param
///                 value: '123.45'
///               - parameter: another-param
///                 value:
///                   fn::toJSON: abc
///               - parameter: other-param
///                 value:
///                   fn::toJSON:
///                     - foo
///             conditionalCases:
///               - cases:
///                   fn::toJSON:
///                     - condition: $sys.func.RAND() < 0.5
///                       caseContent:
///                         - message:
///                             text:
///                               text:
///                                 - First case
///                         - additionalCases:
///                             cases:
///                               - condition: $sys.func.RAND() < 0.2
///                                 caseContent:
///                                   - message:
///                                       text:
///                                         text:
///                                           - Nested case
///                     - caseContent:
///                         - message:
///                             text:
///                               text:
///                                 - Final case
///           targetFlow: ${agent.startFlow}
///       advancedSettings:
///         audioExportGcsDestination:
///           uri: ${bucket.url}/prefix-
///         speechSettings:
///           endpointerSensitivity: 30
///           noSpeechTimeout: 3.500s
///           useTimeoutBasedEndpointing: true
///           models:
///             name: wrench
///             mass: 1.3kg
///             count: '3'
///         dtmfSettings:
///           enabled: true
///           maxDigits: 1
///           finishDigit: '#'
///         loggingSettings:
///           enableStackdriverLogging: true
///           enableInteractionLogging: true
///           enableConsentBasedRedaction: true
/// ```
///
/// ## Import
///
/// Flow can be imported using any of these accepted formats:
///
/// * `{{parent}}/flows/{{name}}`
///
/// * `{{parent}}/{{name}}`
///
/// When using the `pulumi import` command, Flow can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxFlow:CxFlow default {{parent}}/flows/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxFlow:CxFlow default {{parent}}/{{name}}
/// ```
///
pub mod cx_flow {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CxFlowArgs {
        /// Hierarchical advanced settings for this flow. The settings exposed at the lower level overrides the settings exposed at the higher level.
        /// Hierarchy: Agent->Flow->Page->Fulfillment/Parameter.
        /// Structure is documented below.
        #[builder(into, default)]
        pub advanced_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxFlowAdvancedSettings>,
        >,
        /// The description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The human-readable name of the flow.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// A flow's event handlers serve two purposes:
        /// They are responsible for handling events (e.g. no match, webhook errors) in the flow.
        /// They are inherited by every page's [event handlers][Page.event_handlers], which can be used to handle common events regardless of the current page. Event handlers defined in the page have higher priority than those defined in the flow.
        /// Unlike transitionRoutes, these handlers are evaluated on a first-match basis. The first one that matches the event get executed, with the rest being ignored.
        /// Structure is documented below.
        #[builder(into, default)]
        pub event_handlers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::diagflow::CxFlowEventHandler>>,
        >,
        /// Marks this as the [Default Start Flow](https://cloud.google.com/dialogflow/cx/docs/concept/flow#start) for an agent. When you create an agent, the Default Start Flow is created automatically.
        /// The Default Start Flow cannot be deleted; deleting the `gcp.diagflow.CxFlow` resource does nothing to the underlying GCP resources.
        ///
        /// > Avoid having multiple `gcp.diagflow.CxFlow` resources linked to the same agent with `is_default_start_flow = true` because they will compete to control a single Default Start Flow resource in GCP.
        #[builder(into, default)]
        pub is_default_start_flow: pulumi_wasm_rust::Output<Option<bool>>,
        /// The language of the following fields in flow:
        /// Flow.event_handlers.trigger_fulfillment.messages
        /// Flow.event_handlers.trigger_fulfillment.conditional_cases
        /// Flow.transition_routes.trigger_fulfillment.messages
        /// Flow.transition_routes.trigger_fulfillment.conditional_cases
        /// If not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used.
        #[builder(into, default)]
        pub language_code: pulumi_wasm_rust::Output<Option<String>>,
        /// NLU related settings of the flow.
        /// Structure is documented below.
        #[builder(into, default)]
        pub nlu_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxFlowNluSettings>,
        >,
        /// The agent to create a flow for.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>.
        #[builder(into, default)]
        pub parent: pulumi_wasm_rust::Output<Option<String>>,
        /// A flow's transition route group serve two purposes:
        /// They are responsible for matching the user's first utterances in the flow.
        /// They are inherited by every page's [transition route groups][Page.transition_route_groups]. Transition route groups defined in the page have higher priority than those defined in the flow.
        /// Format:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>.
        #[builder(into, default)]
        pub transition_route_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A flow's transition routes serve two purposes:
        /// They are responsible for matching the user's first utterances in the flow.
        /// They are inherited by every page's [transition routes][Page.transition_routes] and can support use cases such as the user saying "help" or "can I talk to a human?", which can be handled in a common way regardless of the current page. Transition routes defined in the page have higher priority than those defined in the flow.
        /// TransitionRoutes are evalauted in the following order:
        /// TransitionRoutes with intent specified.
        /// TransitionRoutes with only condition specified.
        /// TransitionRoutes with intent specified are inherited by pages in the flow.
        /// Structure is documented below.
        #[builder(into, default)]
        pub transition_routes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::diagflow::CxFlowTransitionRoute>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CxFlowResult {
        /// Hierarchical advanced settings for this flow. The settings exposed at the lower level overrides the settings exposed at the higher level.
        /// Hierarchy: Agent->Flow->Page->Fulfillment/Parameter.
        /// Structure is documented below.
        pub advanced_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxFlowAdvancedSettings>,
        >,
        /// The description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The human-readable name of the flow.
        ///
        ///
        /// - - -
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// A flow's event handlers serve two purposes:
        /// They are responsible for handling events (e.g. no match, webhook errors) in the flow.
        /// They are inherited by every page's [event handlers][Page.event_handlers], which can be used to handle common events regardless of the current page. Event handlers defined in the page have higher priority than those defined in the flow.
        /// Unlike transitionRoutes, these handlers are evaluated on a first-match basis. The first one that matches the event get executed, with the rest being ignored.
        /// Structure is documented below.
        pub event_handlers: pulumi_wasm_rust::Output<
            Vec<super::super::types::diagflow::CxFlowEventHandler>,
        >,
        /// Marks this as the [Default Start Flow](https://cloud.google.com/dialogflow/cx/docs/concept/flow#start) for an agent. When you create an agent, the Default Start Flow is created automatically.
        /// The Default Start Flow cannot be deleted; deleting the `gcp.diagflow.CxFlow` resource does nothing to the underlying GCP resources.
        ///
        /// > Avoid having multiple `gcp.diagflow.CxFlow` resources linked to the same agent with `is_default_start_flow = true` because they will compete to control a single Default Start Flow resource in GCP.
        pub is_default_start_flow: pulumi_wasm_rust::Output<Option<bool>>,
        /// The language of the following fields in flow:
        /// Flow.event_handlers.trigger_fulfillment.messages
        /// Flow.event_handlers.trigger_fulfillment.conditional_cases
        /// Flow.transition_routes.trigger_fulfillment.messages
        /// Flow.transition_routes.trigger_fulfillment.conditional_cases
        /// If not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used.
        pub language_code: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier of the flow.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>.
        pub name: pulumi_wasm_rust::Output<String>,
        /// NLU related settings of the flow.
        /// Structure is documented below.
        pub nlu_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxFlowNluSettings>,
        >,
        /// The agent to create a flow for.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>.
        pub parent: pulumi_wasm_rust::Output<Option<String>>,
        /// A flow's transition route group serve two purposes:
        /// They are responsible for matching the user's first utterances in the flow.
        /// They are inherited by every page's [transition route groups][Page.transition_route_groups]. Transition route groups defined in the page have higher priority than those defined in the flow.
        /// Format:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>.
        pub transition_route_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A flow's transition routes serve two purposes:
        /// They are responsible for matching the user's first utterances in the flow.
        /// They are inherited by every page's [transition routes][Page.transition_routes] and can support use cases such as the user saying "help" or "can I talk to a human?", which can be handled in a common way regardless of the current page. Transition routes defined in the page have higher priority than those defined in the flow.
        /// TransitionRoutes are evalauted in the following order:
        /// TransitionRoutes with intent specified.
        /// TransitionRoutes with only condition specified.
        /// TransitionRoutes with intent specified are inherited by pages in the flow.
        /// Structure is documented below.
        pub transition_routes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::diagflow::CxFlowTransitionRoute>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CxFlowArgs) -> CxFlowResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let advanced_settings_binding = args.advanced_settings.get_inner();
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let event_handlers_binding = args.event_handlers.get_inner();
        let is_default_start_flow_binding = args.is_default_start_flow.get_inner();
        let language_code_binding = args.language_code.get_inner();
        let nlu_settings_binding = args.nlu_settings.get_inner();
        let parent_binding = args.parent.get_inner();
        let transition_route_groups_binding = args.transition_route_groups.get_inner();
        let transition_routes_binding = args.transition_routes.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:diagflow/cxFlow:CxFlow".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "advancedSettings".into(),
                    value: &advanced_settings_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "eventHandlers".into(),
                    value: &event_handlers_binding,
                },
                register_interface::ObjectField {
                    name: "isDefaultStartFlow".into(),
                    value: &is_default_start_flow_binding,
                },
                register_interface::ObjectField {
                    name: "languageCode".into(),
                    value: &language_code_binding,
                },
                register_interface::ObjectField {
                    name: "nluSettings".into(),
                    value: &nlu_settings_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "transitionRouteGroups".into(),
                    value: &transition_route_groups_binding,
                },
                register_interface::ObjectField {
                    name: "transitionRoutes".into(),
                    value: &transition_routes_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "advancedSettings".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "eventHandlers".into(),
                },
                register_interface::ResultField {
                    name: "isDefaultStartFlow".into(),
                },
                register_interface::ResultField {
                    name: "languageCode".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nluSettings".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "transitionRouteGroups".into(),
                },
                register_interface::ResultField {
                    name: "transitionRoutes".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CxFlowResult {
            advanced_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("advancedSettings").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            event_handlers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventHandlers").unwrap(),
            ),
            is_default_start_flow: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isDefaultStartFlow").unwrap(),
            ),
            language_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("languageCode").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            nlu_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nluSettings").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            transition_route_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitionRouteGroups").unwrap(),
            ),
            transition_routes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitionRoutes").unwrap(),
            ),
        }
    }
}
