/// A Dialogflow CX conversation (session) can be described and visualized as a state machine. The states of a CX session are represented by pages.
///
///
/// To get more information about Page, see:
///
/// * [API documentation](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows.pages)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dialogflow/cx/docs)
///
/// ## Example Usage
///
/// ### Dialogflowcx Page Full
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
///   basicPage:
///     type: gcp:diagflow:CxPage
///     name: basic_page
///     properties:
///       parent: ${agent.startFlow}
///       displayName: MyPage
///       entryFulfillment:
///         messages:
///           - channel: some-channel
///             text:
///               texts:
///                 - Welcome to page
///           - payload: |2
///                       {"some-key": "some-value", "other-key": ["other-value"]}
///           - conversationSuccess:
///               metadata: |2
///                           {"some-metadata-key": "some-value", "other-metadata-key": 1234}
///           - outputAudioText:
///               text: some output text
///           - outputAudioText:
///               ssml: |2
///                           <speak>Some example <say-as interpret-as="characters">SSML XML</say-as></speak>
///           - liveAgentHandoff:
///               metadata: |2
///                           {"some-metadata-key": "some-value", "other-metadata-key": 1234}
///           - playAudio:
///               audioUri: http://example.com/some-audio-file.mp3
///           - telephonyTransferCall:
///               phoneNumber: 1-234-567-8901
///         setParameterActions:
///           - parameter: some-param
///             value: '123.45'
///           - parameter: another-param
///             value:
///               fn::toJSON: abc
///           - parameter: other-param
///             value:
///               fn::toJSON:
///                 - foo
///         conditionalCases:
///           - cases:
///               fn::toJSON:
///                 - condition: $sys.func.RAND() < 0.5
///                   caseContent:
///                     - message:
///                         text:
///                           text:
///                             - First case
///                     - additionalCases:
///                         cases:
///                           - condition: $sys.func.RAND() < 0.2
///                             caseContent:
///                               - message:
///                                   text:
///                                     text:
///                                       - Nested case
///                 - caseContent:
///                     - message:
///                         text:
///                           text:
///                             - Final case
///       eventHandlers:
///         - event: some-event
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
///       form:
///         parameters:
///           - displayName: param1
///             entityType: projects/-/locations/-/agents/-/entityTypes/sys.date
///             defaultValue:
///               fn::toJSON: 2000-01-01
///             fillBehavior:
///               initialPromptFulfillment:
///                 messages:
///                   - channel: some-channel
///                     text:
///                       texts:
///                         - Please provide param1
///                   - payload: |2
///                                     {"some-key": "some-value", "other-key": ["other-value"]}
///                   - conversationSuccess:
///                       metadata: |2
///                                         {"some-metadata-key": "some-value", "other-metadata-key": 1234}
///                   - outputAudioText:
///                       text: some output text
///                   - outputAudioText:
///                       ssml: |2
///                                         <speak>Some example <say-as interpret-as="characters">SSML XML</say-as></speak>
///                   - liveAgentHandoff:
///                       metadata: |2
///                                         {"some-metadata-key": "some-value", "other-metadata-key": 1234}
///                   - playAudio:
///                       audioUri: http://example.com/some-audio-file.mp3
///                   - telephonyTransferCall:
///                       phoneNumber: 1-234-567-8901
///                 setParameterActions:
///                   - parameter: some-param
///                     value: '123.45'
///                   - parameter: another-param
///                     value:
///                       fn::toJSON: abc
///                   - parameter: other-param
///                     value:
///                       fn::toJSON:
///                         - foo
///                 conditionalCases:
///                   - cases:
///                       fn::toJSON:
///                         - condition: $sys.func.RAND() < 0.5
///                           caseContent:
///                             - message:
///                                 text:
///                                   text:
///                                     - First case
///                             - additionalCases:
///                                 cases:
///                                   - condition: $sys.func.RAND() < 0.2
///                                     caseContent:
///                                       - message:
///                                           text:
///                                             text:
///                                               - Nested case
///                         - caseContent:
///                             - message:
///                                 text:
///                                   text:
///                                     - Final case
///               repromptEventHandlers:
///                 - event: sys.no-match-1
///                   triggerFulfillment:
///                     returnPartialResponses: true
///                     webhook: ${myWebhook.id}
///                     tag: some-tag
///                     messages:
///                       - channel: some-channel
///                         text:
///                           texts:
///                             - Please provide param1
///                       - payload: |2
///                                           {"some-key": "some-value", "other-key": ["other-value"]}
///                       - conversationSuccess:
///                           metadata: |2
///                                               {"some-metadata-key": "some-value", "other-metadata-key": 1234}
///                       - outputAudioText:
///                           text: some output text
///                       - outputAudioText:
///                           ssml: |2
///                                               <speak>Some example <say-as interpret-as="characters">SSML XML</say-as></speak>
///                       - liveAgentHandoff:
///                           metadata: |2
///                                               {"some-metadata-key": "some-value", "other-metadata-key": 1234}
///                       - playAudio:
///                           audioUri: http://example.com/some-audio-file.mp3
///                       - telephonyTransferCall:
///                           phoneNumber: 1-234-567-8901
///                     setParameterActions:
///                       - parameter: some-param
///                         value: '123.45'
///                       - parameter: another-param
///                         value:
///                           fn::toJSON: abc
///                       - parameter: other-param
///                         value:
///                           fn::toJSON:
///                             - foo
///                     conditionalCases:
///                       - cases:
///                           fn::toJSON:
///                             - condition: $sys.func.RAND() < 0.5
///                               caseContent:
///                                 - message:
///                                     text:
///                                       text:
///                                         - First case
///                                 - additionalCases:
///                                     cases:
///                                       - condition: $sys.func.RAND() < 0.2
///                                         caseContent:
///                                           - message:
///                                               text:
///                                                 text:
///                                                   - Nested case
///                             - caseContent:
///                                 - message:
///                                     text:
///                                       text:
///                                         - Final case
///                 - event: sys.no-match-2
///                   targetFlow: ${agent.startFlow}
///                 - event: sys.no-match-3
///                   targetPage: ${myPage2.id}
///             required: 'true'
///             redact: 'true'
///             advancedSettings:
///               dtmfSettings:
///                 enabled: true
///                 maxDigits: 1
///                 finishDigit: '#'
///       transitionRoutes:
///         - condition: $page.params.status = 'FINAL'
///           triggerFulfillment:
///             messages:
///               - channel: some-channel
///                 text:
///                   texts:
///                     - information completed, navigating to page 2
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
///           targetPage: ${myPage2.id}
///       advancedSettings:
///         dtmfSettings:
///           enabled: true
///           maxDigits: 1
///           finishDigit: '#'
///   myPage2:
///     type: gcp:diagflow:CxPage
///     name: my_page2
///     properties:
///       parent: ${agent.startFlow}
///       displayName: MyPage2
///   myWebhook:
///     type: gcp:diagflow:CxWebhook
///     name: my_webhook
///     properties:
///       parent: ${agent.id}
///       displayName: MyWebhook
///       genericWebService:
///         uri: https://example.com
/// ```
///
/// ## Import
///
/// Page can be imported using any of these accepted formats:
///
/// * `{{parent}}/pages/{{name}}`
///
/// * `{{parent}}/{{name}}`
///
/// When using the `pulumi import` command, Page can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxPage:CxPage default {{parent}}/pages/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxPage:CxPage default {{parent}}/{{name}}
/// ```
///
pub mod cx_page {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CxPageArgs {
        /// Hierarchical advanced settings for this page. The settings exposed at the lower level overrides the settings exposed at the higher level.
        /// Hierarchy: Agent->Flow->Page->Fulfillment/Parameter.
        /// Structure is documented below.
        #[builder(into, default)]
        pub advanced_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::diagflow::CxPageAdvancedSettings>,
        >,
        /// The human-readable name of the page, unique within the agent.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The fulfillment to call when the session is entering the page.
        /// Structure is documented below.
        #[builder(into, default)]
        pub entry_fulfillment: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::diagflow::CxPageEntryFulfillment>,
        >,
        /// Handlers associated with the page to handle events such as webhook errors, no match or no input.
        /// Structure is documented below.
        #[builder(into, default)]
        pub event_handlers: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::diagflow::CxPageEventHandler>>,
        >,
        /// The form associated with the page, used for collecting parameters relevant to the page.
        /// Structure is documented below.
        #[builder(into, default)]
        pub form: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::diagflow::CxPageForm>,
        >,
        /// The language of the following fields in page:
        /// Page.entry_fulfillment.messages
        /// Page.entry_fulfillment.conditional_cases
        /// Page.event_handlers.trigger_fulfillment.messages
        /// Page.event_handlers.trigger_fulfillment.conditional_cases
        /// Page.form.parameters.fill_behavior.initial_prompt_fulfillment.messages
        /// Page.form.parameters.fill_behavior.initial_prompt_fulfillment.conditional_cases
        /// Page.form.parameters.fill_behavior.reprompt_event_handlers.messages
        /// Page.form.parameters.fill_behavior.reprompt_event_handlers.conditional_cases
        /// Page.transition_routes.trigger_fulfillment.messages
        /// Page.transition_routes.trigger_fulfillment.conditional_cases
        /// If not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used.
        #[builder(into, default)]
        pub language_code: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The flow to create a page for.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>.
        #[builder(into, default)]
        pub parent: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Ordered list of TransitionRouteGroups associated with the page. Transition route groups must be unique within a page.
        /// If multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route > page's transition route group > flow's transition routes.
        /// If multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence.
        /// Format:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>.
        #[builder(into, default)]
        pub transition_route_groups: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A list of transitions for the transition rules of this page. They route the conversation to another page in the same flow, or another flow.
        /// When we are in a certain page, the TransitionRoutes are evalauted in the following order:
        /// TransitionRoutes defined in the page with intent specified.
        /// TransitionRoutes defined in the transition route groups with intent specified.
        /// TransitionRoutes defined in flow with intent specified.
        /// TransitionRoutes defined in the transition route groups with intent specified.
        /// TransitionRoutes defined in the page with only condition specified.
        /// TransitionRoutes defined in the transition route groups with only condition specified.
        /// Structure is documented below.
        #[builder(into, default)]
        pub transition_routes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::diagflow::CxPageTransitionRoute>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CxPageResult {
        /// Hierarchical advanced settings for this page. The settings exposed at the lower level overrides the settings exposed at the higher level.
        /// Hierarchy: Agent->Flow->Page->Fulfillment/Parameter.
        /// Structure is documented below.
        pub advanced_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxPageAdvancedSettings>,
        >,
        /// The human-readable name of the page, unique within the agent.
        ///
        ///
        /// - - -
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The fulfillment to call when the session is entering the page.
        /// Structure is documented below.
        pub entry_fulfillment: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxPageEntryFulfillment>,
        >,
        /// Handlers associated with the page to handle events such as webhook errors, no match or no input.
        /// Structure is documented below.
        pub event_handlers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::diagflow::CxPageEventHandler>>,
        >,
        /// The form associated with the page, used for collecting parameters relevant to the page.
        /// Structure is documented below.
        pub form: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxPageForm>,
        >,
        /// The language of the following fields in page:
        /// Page.entry_fulfillment.messages
        /// Page.entry_fulfillment.conditional_cases
        /// Page.event_handlers.trigger_fulfillment.messages
        /// Page.event_handlers.trigger_fulfillment.conditional_cases
        /// Page.form.parameters.fill_behavior.initial_prompt_fulfillment.messages
        /// Page.form.parameters.fill_behavior.initial_prompt_fulfillment.conditional_cases
        /// Page.form.parameters.fill_behavior.reprompt_event_handlers.messages
        /// Page.form.parameters.fill_behavior.reprompt_event_handlers.conditional_cases
        /// Page.transition_routes.trigger_fulfillment.messages
        /// Page.transition_routes.trigger_fulfillment.conditional_cases
        /// If not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used.
        pub language_code: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier of the page.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The flow to create a page for.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>.
        pub parent: pulumi_wasm_rust::Output<Option<String>>,
        /// Ordered list of TransitionRouteGroups associated with the page. Transition route groups must be unique within a page.
        /// If multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route > page's transition route group > flow's transition routes.
        /// If multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence.
        /// Format:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>.
        pub transition_route_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of transitions for the transition rules of this page. They route the conversation to another page in the same flow, or another flow.
        /// When we are in a certain page, the TransitionRoutes are evalauted in the following order:
        /// TransitionRoutes defined in the page with intent specified.
        /// TransitionRoutes defined in the transition route groups with intent specified.
        /// TransitionRoutes defined in flow with intent specified.
        /// TransitionRoutes defined in the transition route groups with intent specified.
        /// TransitionRoutes defined in the page with only condition specified.
        /// TransitionRoutes defined in the transition route groups with only condition specified.
        /// Structure is documented below.
        pub transition_routes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::diagflow::CxPageTransitionRoute>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CxPageArgs,
    ) -> CxPageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let advanced_settings_binding = args
            .advanced_settings
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let entry_fulfillment_binding = args
            .entry_fulfillment
            .get_output(context)
            .get_inner();
        let event_handlers_binding = args.event_handlers.get_output(context).get_inner();
        let form_binding = args.form.get_output(context).get_inner();
        let language_code_binding = args.language_code.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let transition_route_groups_binding = args
            .transition_route_groups
            .get_output(context)
            .get_inner();
        let transition_routes_binding = args
            .transition_routes
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:diagflow/cxPage:CxPage".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "advancedSettings".into(),
                    value: &advanced_settings_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "entryFulfillment".into(),
                    value: &entry_fulfillment_binding,
                },
                register_interface::ObjectField {
                    name: "eventHandlers".into(),
                    value: &event_handlers_binding,
                },
                register_interface::ObjectField {
                    name: "form".into(),
                    value: &form_binding,
                },
                register_interface::ObjectField {
                    name: "languageCode".into(),
                    value: &language_code_binding,
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
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "entryFulfillment".into(),
                },
                register_interface::ResultField {
                    name: "eventHandlers".into(),
                },
                register_interface::ResultField {
                    name: "form".into(),
                },
                register_interface::ResultField {
                    name: "languageCode".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CxPageResult {
            advanced_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("advancedSettings").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            entry_fulfillment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entryFulfillment").unwrap(),
            ),
            event_handlers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventHandlers").unwrap(),
            ),
            form: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("form").unwrap(),
            ),
            language_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("languageCode").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
