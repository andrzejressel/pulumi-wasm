/// Agents are best described as Natural Language Understanding (NLU) modules that transform user requests into actionable data. You can include agents in your app, product, or service to determine user intent and respond to the user in a natural way.
///
///
/// To get more information about Agent, see:
///
/// * [API documentation](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dialogflow/cx/docs)
///
///
///
/// ## Example Usage
///
/// ### Dialogflowcx Agent Full
///
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: dialogflowcx-bucket
///       location: US
///       uniformBucketLevelAccess: true
///   fullAgent:
///     type: gcp:diagflow:CxAgent
///     name: full_agent
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
///       gitIntegrationSettings:
///         githubSettings:
///           displayName: Github Repo
///           repositoryUri: https://api.github.com/repos/githubtraining/hellogitworld
///           trackingBranch: main
///           accessToken: secret-token
///           branches:
///             - main
///       textToSpeechSettings:
///         synthesizeSpeechConfigs:
///           fn::toJSON:
///             en:
///               voice:
///                 name: en-US-Neural2-A
///             fr:
///               voice:
///                 name: fr-CA-Neural2-A
/// ```
///
/// ## Import
///
/// Agent can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/agents/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Agent can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxAgent:CxAgent default projects/{{project}}/locations/{{location}}/agents/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxAgent:CxAgent default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxAgent:CxAgent default {{location}}/{{name}}
/// ```
///
pub mod cx_agent {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CxAgentArgs {
        /// Hierarchical advanced settings for this agent. The settings exposed at the lower level overrides the settings exposed at the higher level.
        /// Hierarchy: Agent->Flow->Page->Fulfillment/Parameter.
        /// Structure is documented below.
        #[builder(into, default)]
        pub advanced_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::diagflow::CxAgentAdvancedSettings>,
        >,
        /// The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted Web Demo integration.
        #[builder(into, default)]
        pub avatar_uri: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)
        /// for a list of the currently supported language codes. This field cannot be updated after creation.
        #[builder(into)]
        pub default_language_code: pulumi_wasm_rust::InputOrOutput<String>,
        /// The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The human-readable name of the agent, unique within the location.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Indicates if automatic spell correction is enabled in detect intent requests.
        #[builder(into, default)]
        pub enable_spell_correction: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// (Optional, Deprecated)
        /// Determines whether this agent should log conversation queries.
        ///
        /// > **Warning:** `enable_stackdriver_logging` is deprecated and will be removed in a future major release. Please use `advanced_settings.logging_settings.enable_stackdriver_logging`instead.
        #[builder(into, default)]
        pub enable_stackdriver_logging: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Git integration settings for this agent.
        /// Structure is documented below.
        #[builder(into, default)]
        pub git_integration_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::diagflow::CxAgentGitIntegrationSettings>,
        >,
        /// The name of the location this agent is located in.
        /// > **Note:** The first time you are deploying an Agent in your project you must configure location settings.
        /// This is a one time step but at the moment you can only [configure location settings](https://cloud.google.com/dialogflow/cx/docs/concept/region#location-settings) via the Dialogflow CX console.
        /// Another options is to use global location so you don't need to manually configure location settings.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>.
        #[builder(into, default)]
        pub security_settings: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Settings related to speech recognition.
        /// Structure is documented below.
        #[builder(into, default)]
        pub speech_to_text_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::diagflow::CxAgentSpeechToTextSettings>,
        >,
        /// The list of all languages supported by this agent (except for the default_language_code).
        #[builder(into, default)]
        pub supported_language_codes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Settings related to speech synthesizing.
        /// Structure is documented below.
        #[builder(into, default)]
        pub text_to_speech_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::diagflow::CxAgentTextToSpeechSettings>,
        >,
        /// The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,
        /// Europe/Paris.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub time_zone: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CxAgentResult {
        /// Hierarchical advanced settings for this agent. The settings exposed at the lower level overrides the settings exposed at the higher level.
        /// Hierarchy: Agent->Flow->Page->Fulfillment/Parameter.
        /// Structure is documented below.
        pub advanced_settings: pulumi_wasm_rust::Output<
            super::super::types::diagflow::CxAgentAdvancedSettings,
        >,
        /// The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted Web Demo integration.
        pub avatar_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// The default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)
        /// for a list of the currently supported language codes. This field cannot be updated after creation.
        pub default_language_code: pulumi_wasm_rust::Output<String>,
        /// The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The human-readable name of the agent, unique within the location.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Indicates if automatic spell correction is enabled in detect intent requests.
        pub enable_spell_correction: pulumi_wasm_rust::Output<Option<bool>>,
        /// (Optional, Deprecated)
        /// Determines whether this agent should log conversation queries.
        ///
        /// > **Warning:** `enable_stackdriver_logging` is deprecated and will be removed in a future major release. Please use `advanced_settings.logging_settings.enable_stackdriver_logging`instead.
        pub enable_stackdriver_logging: pulumi_wasm_rust::Output<Option<bool>>,
        /// Git integration settings for this agent.
        /// Structure is documented below.
        pub git_integration_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxAgentGitIntegrationSettings>,
        >,
        /// The name of the location this agent is located in.
        /// > **Note:** The first time you are deploying an Agent in your project you must configure location settings.
        /// This is a one time step but at the moment you can only [configure location settings](https://cloud.google.com/dialogflow/cx/docs/concept/region#location-settings) via the Dialogflow CX console.
        /// Another options is to use global location so you don't need to manually configure location settings.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The unique identifier of the agent.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Name of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>.
        pub security_settings: pulumi_wasm_rust::Output<Option<String>>,
        /// Settings related to speech recognition.
        /// Structure is documented below.
        pub speech_to_text_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxAgentSpeechToTextSettings>,
        >,
        /// Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>.
        pub start_flow: pulumi_wasm_rust::Output<String>,
        /// The list of all languages supported by this agent (except for the default_language_code).
        pub supported_language_codes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Settings related to speech synthesizing.
        /// Structure is documented below.
        pub text_to_speech_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxAgentTextToSpeechSettings>,
        >,
        /// The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,
        /// Europe/Paris.
        ///
        ///
        /// - - -
        pub time_zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CxAgentArgs,
    ) -> CxAgentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let advanced_settings_binding = args
            .advanced_settings
            .get_output(context)
            .get_inner();
        let avatar_uri_binding = args.avatar_uri.get_output(context).get_inner();
        let default_language_code_binding = args
            .default_language_code
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let enable_spell_correction_binding = args
            .enable_spell_correction
            .get_output(context)
            .get_inner();
        let enable_stackdriver_logging_binding = args
            .enable_stackdriver_logging
            .get_output(context)
            .get_inner();
        let git_integration_settings_binding = args
            .git_integration_settings
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let security_settings_binding = args
            .security_settings
            .get_output(context)
            .get_inner();
        let speech_to_text_settings_binding = args
            .speech_to_text_settings
            .get_output(context)
            .get_inner();
        let supported_language_codes_binding = args
            .supported_language_codes
            .get_output(context)
            .get_inner();
        let text_to_speech_settings_binding = args
            .text_to_speech_settings
            .get_output(context)
            .get_inner();
        let time_zone_binding = args.time_zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:diagflow/cxAgent:CxAgent".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "advancedSettings".into(),
                    value: &advanced_settings_binding,
                },
                register_interface::ObjectField {
                    name: "avatarUri".into(),
                    value: &avatar_uri_binding,
                },
                register_interface::ObjectField {
                    name: "defaultLanguageCode".into(),
                    value: &default_language_code_binding,
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
                    name: "enableSpellCorrection".into(),
                    value: &enable_spell_correction_binding,
                },
                register_interface::ObjectField {
                    name: "enableStackdriverLogging".into(),
                    value: &enable_stackdriver_logging_binding,
                },
                register_interface::ObjectField {
                    name: "gitIntegrationSettings".into(),
                    value: &git_integration_settings_binding,
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
                    name: "securitySettings".into(),
                    value: &security_settings_binding,
                },
                register_interface::ObjectField {
                    name: "speechToTextSettings".into(),
                    value: &speech_to_text_settings_binding,
                },
                register_interface::ObjectField {
                    name: "supportedLanguageCodes".into(),
                    value: &supported_language_codes_binding,
                },
                register_interface::ObjectField {
                    name: "textToSpeechSettings".into(),
                    value: &text_to_speech_settings_binding,
                },
                register_interface::ObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "advancedSettings".into(),
                },
                register_interface::ResultField {
                    name: "avatarUri".into(),
                },
                register_interface::ResultField {
                    name: "defaultLanguageCode".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "enableSpellCorrection".into(),
                },
                register_interface::ResultField {
                    name: "enableStackdriverLogging".into(),
                },
                register_interface::ResultField {
                    name: "gitIntegrationSettings".into(),
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
                    name: "securitySettings".into(),
                },
                register_interface::ResultField {
                    name: "speechToTextSettings".into(),
                },
                register_interface::ResultField {
                    name: "startFlow".into(),
                },
                register_interface::ResultField {
                    name: "supportedLanguageCodes".into(),
                },
                register_interface::ResultField {
                    name: "textToSpeechSettings".into(),
                },
                register_interface::ResultField {
                    name: "timeZone".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CxAgentResult {
            advanced_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("advancedSettings").unwrap(),
            ),
            avatar_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("avatarUri").unwrap(),
            ),
            default_language_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultLanguageCode").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            enable_spell_correction: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableSpellCorrection").unwrap(),
            ),
            enable_stackdriver_logging: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableStackdriverLogging").unwrap(),
            ),
            git_integration_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gitIntegrationSettings").unwrap(),
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
            security_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securitySettings").unwrap(),
            ),
            speech_to_text_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("speechToTextSettings").unwrap(),
            ),
            start_flow: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startFlow").unwrap(),
            ),
            supported_language_codes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedLanguageCodes").unwrap(),
            ),
            text_to_speech_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("textToSpeechSettings").unwrap(),
            ),
            time_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeZone").unwrap(),
            ),
        }
    }
}
