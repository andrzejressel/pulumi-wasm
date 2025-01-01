/// You can use the built-in test feature to uncover bugs and prevent regressions. A test execution verifies that agent responses have not changed for end-user inputs defined in the test case.
///
///
/// To get more information about TestCase, see:
///
/// * [API documentation](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.testCases)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dialogflow/cx/docs)
///
/// ## Example Usage
///
/// ### Dialogflowcx Test Case Full
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
///       avatarUri: https://storage.cloud.google.com/dialogflow-test-host-image/cloud-logo.png
///       enableStackdriverLogging: true
///       enableSpellCorrection: true
///       speechToTextSettings:
///         enableSpeechAdaptation: true
///   page:
///     type: gcp:diagflow:CxPage
///     properties:
///       parent: ${agent.startFlow}
///       displayName: MyPage
///       transitionRoutes:
///         - intent: ${intent.id}
///           triggerFulfillment:
///             messages:
///               - text:
///                   texts:
///                     - Training phrase response
///       eventHandlers:
///         - event: some-event
///           triggerFulfillment:
///             messages:
///               - text:
///                   texts:
///                     - Handling some event
///   intent:
///     type: gcp:diagflow:CxIntent
///     properties:
///       parent: ${agent.id}
///       displayName: MyIntent
///       priority: 1
///       trainingPhrases:
///         - parts:
///             - text: training phrase
///           repeatCount: 1
///   basicTestCase:
///     type: gcp:diagflow:CxTestCase
///     name: basic_test_case
///     properties:
///       parent: ${agent.id}
///       displayName: MyTestCase
///       tags:
///         - '#tag1'
///       notes: demonstrates a simple training phrase response
///       testConfig:
///         trackingParameters:
///           - some_param
///         page: ${page.id}
///       testCaseConversationTurns:
///         - userInput:
///             input:
///               languageCode: en
///               text:
///                 text: training phrase
///             injectedParameters:
///               fn::toJSON:
///                 some_param: '1'
///             isWebhookEnabled: true
///             enableSentimentAnalysis: true
///           virtualAgentOutput:
///             sessionParameters:
///               fn::toJSON:
///                 some_param: '1'
///             triggeredIntent:
///               name: ${intent.id}
///             currentPage:
///               name: ${page.id}
///             textResponses:
///               - texts:
///                   - Training phrase response
///         - userInput:
///             input:
///               event:
///                 event: some-event
///           virtualAgentOutput:
///             currentPage:
///               name: ${page.id}
///             textResponses:
///               - texts:
///                   - Handling some event
///         - userInput:
///             input:
///               dtmf:
///                 digits: '12'
///                 finishDigit: '3'
///           virtualAgentOutput:
///             textResponses:
///               - texts:
///                   - I didn't get that. Can you say it again?
/// ```
///
/// ## Import
///
/// TestCase can be imported using any of these accepted formats:
///
/// * `{{parent}}/testCases/{{name}}`
///
/// When using the `pulumi import` command, TestCase can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxTestCase:CxTestCase default {{parent}}/testCases/{{name}}
/// ```
///
pub mod cx_test_case {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CxTestCaseArgs {
        /// The human-readable name of the test case, unique within the agent. Limit of 200 characters.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Additional freeform notes about the test case. Limit of 400 characters.
        #[builder(into, default)]
        pub notes: pulumi_wasm_rust::Output<Option<String>>,
        /// The agent to create the test case for.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>.
        #[builder(into, default)]
        pub parent: pulumi_wasm_rust::Output<Option<String>>,
        /// Tags are short descriptions that users may apply to test cases for organizational and filtering purposes.
        /// Each tag should start with "#" and has a limit of 30 characters
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The conversation turns uttered when the test case was created, in chronological order. These include the canonical set of agent utterances that should occur when the agent is working properly.
        /// Structure is documented below.
        #[builder(into, default)]
        pub test_case_conversation_turns: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::diagflow::CxTestCaseTestCaseConversationTurn>,
            >,
        >,
        /// Config for the test case.
        /// Structure is documented below.
        #[builder(into, default)]
        pub test_config: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxTestCaseTestConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct CxTestCaseResult {
        /// When the test was created. A timestamp in RFC3339 text format.
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// The human-readable name of the test case, unique within the agent. Limit of 200 characters.
        ///
        ///
        /// - - -
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The latest test result.
        /// Structure is documented below.
        pub last_test_results: pulumi_wasm_rust::Output<
            Vec<super::super::types::diagflow::CxTestCaseLastTestResult>,
        >,
        /// The unique identifier of the page.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Additional freeform notes about the test case. Limit of 400 characters.
        pub notes: pulumi_wasm_rust::Output<Option<String>>,
        /// The agent to create the test case for.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>.
        pub parent: pulumi_wasm_rust::Output<Option<String>>,
        /// Tags are short descriptions that users may apply to test cases for organizational and filtering purposes.
        /// Each tag should start with "#" and has a limit of 30 characters
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The conversation turns uttered when the test case was created, in chronological order. These include the canonical set of agent utterances that should occur when the agent is working properly.
        /// Structure is documented below.
        pub test_case_conversation_turns: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::diagflow::CxTestCaseTestCaseConversationTurn>,
            >,
        >,
        /// Config for the test case.
        /// Structure is documented below.
        pub test_config: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxTestCaseTestConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CxTestCaseArgs) -> CxTestCaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_inner();
        let notes_binding = args.notes.get_inner();
        let parent_binding = args.parent.get_inner();
        let tags_binding = args.tags.get_inner();
        let test_case_conversation_turns_binding = args
            .test_case_conversation_turns
            .get_inner();
        let test_config_binding = args.test_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:diagflow/cxTestCase:CxTestCase".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "notes".into(),
                    value: &notes_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "testCaseConversationTurns".into(),
                    value: &test_case_conversation_turns_binding,
                },
                register_interface::ObjectField {
                    name: "testConfig".into(),
                    value: &test_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "lastTestResults".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notes".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "testCaseConversationTurns".into(),
                },
                register_interface::ResultField {
                    name: "testConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CxTestCaseResult {
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            last_test_results: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastTestResults").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notes").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            test_case_conversation_turns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("testCaseConversationTurns").unwrap(),
            ),
            test_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("testConfig").unwrap(),
            ),
        }
    }
}
