/// Webhooks host the developer's business logic. During a session, webhooks allow the developer to use the data extracted by Dialogflow's natural language processing to generate dynamic responses, validate collected data, or trigger actions on the backend.
///
///
/// To get more information about Webhook, see:
///
/// * [API documentation](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.webhooks)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dialogflow/cx/docs)
///
/// ## Example Usage
///
/// ### Dialogflowcx Webhook Full
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
///             .supported_language_codes(vec!["it", "de", "es",])
///             .time_zone("America/New_York")
///             .build_struct(),
///     );
///     let basicWebhook = cx_webhook::create(
///         "basicWebhook",
///         CxWebhookArgs::builder()
///             .display_name("MyFlow")
///             .generic_web_service(
///                 CxWebhookGenericWebService::builder()
///                     .uri("https://example.com")
///                     .build_struct(),
///             )
///             .parent("${agent.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Webhook can be imported using any of these accepted formats:
///
/// * `{{parent}}/webhooks/{{name}}`
///
/// * `{{parent}}/{{name}}`
///
/// When using the `pulumi import` command, Webhook can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxWebhook:CxWebhook default {{parent}}/webhooks/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxWebhook:CxWebhook default {{parent}}/{{name}}
/// ```
///
pub mod cx_webhook {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CxWebhookArgs {
        /// Indicates whether the webhook is disabled.
        #[builder(into, default)]
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The human-readable name of the webhook, unique within the agent.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Indicates if automatic spell correction is enabled in detect intent requests.
        #[builder(into, default)]
        pub enable_spell_correction: pulumi_wasm_rust::Output<Option<bool>>,
        /// Determines whether this agent should log conversation queries.
        #[builder(into, default)]
        pub enable_stackdriver_logging: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration for a generic web service.
        /// Structure is documented below.
        #[builder(into, default)]
        pub generic_web_service: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxWebhookGenericWebService>,
        >,
        /// The agent to create a webhook for.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>.
        #[builder(into, default)]
        pub parent: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>.
        #[builder(into, default)]
        pub security_settings: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration for a Service Directory service.
        /// Structure is documented below.
        #[builder(into, default)]
        pub service_directory: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxWebhookServiceDirectory>,
        >,
        /// Webhook execution timeout.
        #[builder(into, default)]
        pub timeout: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CxWebhookResult {
        /// Indicates whether the webhook is disabled.
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The human-readable name of the webhook, unique within the agent.
        ///
        ///
        /// - - -
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Indicates if automatic spell correction is enabled in detect intent requests.
        pub enable_spell_correction: pulumi_wasm_rust::Output<Option<bool>>,
        /// Determines whether this agent should log conversation queries.
        pub enable_stackdriver_logging: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration for a generic web service.
        /// Structure is documented below.
        pub generic_web_service: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxWebhookGenericWebService>,
        >,
        /// The unique identifier of the webhook.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The agent to create a webhook for.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>.
        pub parent: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>.
        pub security_settings: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration for a Service Directory service.
        /// Structure is documented below.
        pub service_directory: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxWebhookServiceDirectory>,
        >,
        /// Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>.
        pub start_flow: pulumi_wasm_rust::Output<String>,
        /// Webhook execution timeout.
        pub timeout: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CxWebhookArgs) -> CxWebhookResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let disabled_binding = args.disabled.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let enable_spell_correction_binding = args.enable_spell_correction.get_inner();
        let enable_stackdriver_logging_binding = args
            .enable_stackdriver_logging
            .get_inner();
        let generic_web_service_binding = args.generic_web_service.get_inner();
        let parent_binding = args.parent.get_inner();
        let security_settings_binding = args.security_settings.get_inner();
        let service_directory_binding = args.service_directory.get_inner();
        let timeout_binding = args.timeout.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:diagflow/cxWebhook:CxWebhook".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
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
                    name: "genericWebService".into(),
                    value: &generic_web_service_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "securitySettings".into(),
                    value: &security_settings_binding,
                },
                register_interface::ObjectField {
                    name: "serviceDirectory".into(),
                    value: &service_directory_binding,
                },
                register_interface::ObjectField {
                    name: "timeout".into(),
                    value: &timeout_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "disabled".into(),
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
                    name: "genericWebService".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "securitySettings".into(),
                },
                register_interface::ResultField {
                    name: "serviceDirectory".into(),
                },
                register_interface::ResultField {
                    name: "startFlow".into(),
                },
                register_interface::ResultField {
                    name: "timeout".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CxWebhookResult {
            disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disabled").unwrap(),
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
            generic_web_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("genericWebService").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            security_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securitySettings").unwrap(),
            ),
            service_directory: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceDirectory").unwrap(),
            ),
            start_flow: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startFlow").unwrap(),
            ),
            timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeout").unwrap(),
            ),
        }
    }
}
