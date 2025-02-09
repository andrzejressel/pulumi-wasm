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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cx_webhook {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CxWebhookArgs {
        /// Indicates whether the webhook is disabled.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The human-readable name of the webhook, unique within the agent.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicates if automatic spell correction is enabled in detect intent requests.
        #[builder(into, default)]
        pub enable_spell_correction: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Determines whether this agent should log conversation queries.
        #[builder(into, default)]
        pub enable_stackdriver_logging: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration for a generic web service.
        /// Structure is documented below.
        #[builder(into, default)]
        pub generic_web_service: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::diagflow::CxWebhookGenericWebService>,
        >,
        /// The agent to create a webhook for.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>.
        #[builder(into, default)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>.
        #[builder(into, default)]
        pub security_settings: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for a Service Directory service.
        /// Structure is documented below.
        #[builder(into, default)]
        pub service_directory: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::diagflow::CxWebhookServiceDirectory>,
        >,
        /// Webhook execution timeout.
        #[builder(into, default)]
        pub timeout: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CxWebhookResult {
        /// Indicates whether the webhook is disabled.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The human-readable name of the webhook, unique within the agent.
        ///
        ///
        /// - - -
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Indicates if automatic spell correction is enabled in detect intent requests.
        pub enable_spell_correction: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Determines whether this agent should log conversation queries.
        pub enable_stackdriver_logging: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration for a generic web service.
        /// Structure is documented below.
        pub generic_web_service: pulumi_gestalt_rust::Output<
            Option<super::super::types::diagflow::CxWebhookGenericWebService>,
        >,
        /// The unique identifier of the webhook.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The agent to create a webhook for.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>.
        pub parent: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>.
        pub security_settings: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration for a Service Directory service.
        /// Structure is documented below.
        pub service_directory: pulumi_gestalt_rust::Output<
            Option<super::super::types::diagflow::CxWebhookServiceDirectory>,
        >,
        /// Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>.
        pub start_flow: pulumi_gestalt_rust::Output<String>,
        /// Webhook execution timeout.
        pub timeout: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CxWebhookArgs,
    ) -> CxWebhookResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let disabled_binding = args.disabled.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enable_spell_correction_binding = args
            .enable_spell_correction
            .get_output(context);
        let enable_stackdriver_logging_binding = args
            .enable_stackdriver_logging
            .get_output(context);
        let generic_web_service_binding = args.generic_web_service.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let security_settings_binding = args.security_settings.get_output(context);
        let service_directory_binding = args.service_directory.get_output(context);
        let timeout_binding = args.timeout.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:diagflow/cxWebhook:CxWebhook".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: disabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableSpellCorrection".into(),
                    value: enable_spell_correction_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableStackdriverLogging".into(),
                    value: enable_stackdriver_logging_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "genericWebService".into(),
                    value: generic_web_service_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securitySettings".into(),
                    value: security_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceDirectory".into(),
                    value: service_directory_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeout".into(),
                    value: timeout_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CxWebhookResult {
            disabled: o.get_field("disabled"),
            display_name: o.get_field("displayName"),
            enable_spell_correction: o.get_field("enableSpellCorrection"),
            enable_stackdriver_logging: o.get_field("enableStackdriverLogging"),
            generic_web_service: o.get_field("genericWebService"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            security_settings: o.get_field("securitySettings"),
            service_directory: o.get_field("serviceDirectory"),
            start_flow: o.get_field("startFlow"),
            timeout: o.get_field("timeout"),
        }
    }
}
