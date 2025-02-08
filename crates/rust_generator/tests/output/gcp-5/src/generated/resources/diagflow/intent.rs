/// Represents a Dialogflow intent. Intents convert a number of user expressions or patterns into an action. An action
/// is an extraction of a user command or sentence semantics.
///
///
/// To get more information about Intent, see:
///
/// * [API documentation](https://cloud.google.com/dialogflow/docs/reference/rest/v2/projects.agent.intents)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dialogflow/docs/)
///
/// ## Example Usage
///
/// ### Dialogflow Intent Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicAgent = agent::create(
///         "basicAgent",
///         AgentArgs::builder()
///             .default_language_code("en")
///             .display_name("example_agent")
///             .time_zone("America/New_York")
///             .build_struct(),
///     );
///     let basicIntent = intent::create(
///         "basicIntent",
///         IntentArgs::builder().display_name("basic-intent").build_struct(),
///     );
/// }
/// ```
/// ### Dialogflow Intent Full
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let agentCreate = iam_member::create(
///         "agentCreate",
///         IamMemberArgs::builder()
///             .member("serviceAccount:${dialogflowServiceAccount.email}")
///             .project("${agentProjectService.project}")
///             .role("roles/dialogflow.admin")
///             .build_struct(),
///     );
///     let agentProject = project::create(
///         "agentProject",
///         ProjectArgs::builder()
///             .deletion_policy("DELETE")
///             .name("my-project")
///             .org_id("123456789")
///             .project_id("my-project")
///             .build_struct(),
///     );
///     let agentProjectService = service::create(
///         "agentProjectService",
///         ServiceArgs::builder()
///             .disable_dependent_services(false)
///             .project("${agentProject.projectId}")
///             .service("dialogflow.googleapis.com")
///             .build_struct(),
///     );
///     let basicAgent = agent::create(
///         "basicAgent",
///         AgentArgs::builder()
///             .default_language_code("en")
///             .display_name("example_agent")
///             .project("${agentProject.projectId}")
///             .time_zone("America/New_York")
///             .build_struct(),
///     );
///     let dialogflowServiceAccount = account::create(
///         "dialogflowServiceAccount",
///         AccountArgs::builder().account_id("my-account").build_struct(),
///     );
///     let fullIntent = intent::create(
///         "fullIntent",
///         IntentArgs::builder()
///             .action("some_action")
///             .default_response_platforms(vec!["FACEBOOK", "SLACK",])
///             .display_name("full-intent")
///             .events(vec!["some_event",])
///             .input_context_names(
///                 vec![
///                     "projects/${agentProject.projectId}/agent/sessions/-/contexts/some_id",
///                 ],
///             )
///             .is_fallback(false)
///             .ml_disabled(true)
///             .priority(1)
///             .project("${agentProject.projectId}")
///             .reset_contexts(true)
///             .webhook_state("WEBHOOK_STATE_ENABLED")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Intent can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Intent can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:diagflow/intent:Intent default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod intent {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntentArgs {
        /// The name of the action associated with the intent.
        /// Note: The action name must not contain whitespaces.
        #[builder(into, default)]
        pub action: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED
        /// (i.e. default platform).
        /// Each value may be one of: `FACEBOOK`, `SLACK`, `TELEGRAM`, `KIK`, `SKYPE`, `LINE`, `VIBER`, `ACTIONS_ON_GOOGLE`, `GOOGLE_HANGOUTS`.
        #[builder(into, default)]
        pub default_response_platforms: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The name of this intent to be displayed on the console.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of
        /// the contexts must be present in the active user session for an event to trigger this intent. See the
        /// [events reference](https://cloud.google.com/dialogflow/docs/events-overview) for more details.
        #[builder(into, default)]
        pub events: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The list of context names required for this intent to be triggered.
        /// Format: projects/<Project ID>/agent/sessions/-/contexts/<Context ID>.
        #[builder(into, default)]
        pub input_context_names: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Indicates whether this is a fallback intent.
        #[builder(into, default)]
        pub is_fallback: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Indicates whether Machine Learning is disabled for the intent.
        /// Note: If mlDisabled setting is set to true, then this intent is not taken into account during inference in ML
        /// ONLY match mode. Also, auto-markup in the UI is turned off.
        #[builder(into, default)]
        pub ml_disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The unique identifier of the parent intent in the chain of followup intents.
        /// Format: projects/<Project ID>/agent/intents/<Intent ID>.
        #[builder(into, default)]
        pub parent_followup_intent_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The priority of this intent. Higher numbers represent higher priorities.
        /// - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds
        /// to the Normal priority in the console.
        /// - If the supplied value is negative, the intent is ignored in runtime detect intent requests.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether to delete all contexts in the current session when this intent is matched.
        #[builder(into, default)]
        pub reset_contexts: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Indicates whether webhooks are enabled for the intent.
        /// * WEBHOOK_STATE_ENABLED: Webhook is enabled in the agent and in the intent.
        /// * WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING: Webhook is enabled in the agent and in the intent. Also, each slot
        /// filling prompt is forwarded to the webhook.
        /// Possible values are: `WEBHOOK_STATE_ENABLED`, `WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING`.
        #[builder(into, default)]
        pub webhook_state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IntentResult {
        /// The name of the action associated with the intent.
        /// Note: The action name must not contain whitespaces.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED
        /// (i.e. default platform).
        /// Each value may be one of: `FACEBOOK`, `SLACK`, `TELEGRAM`, `KIK`, `SKYPE`, `LINE`, `VIBER`, `ACTIONS_ON_GOOGLE`, `GOOGLE_HANGOUTS`.
        pub default_response_platforms: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name of this intent to be displayed on the console.
        ///
        ///
        /// - - -
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of
        /// the contexts must be present in the active user session for an event to trigger this intent. See the
        /// [events reference](https://cloud.google.com/dialogflow/docs/events-overview) for more details.
        pub events: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Information about all followup intents that have this intent as a direct or indirect parent. We populate this field
        /// only in the output.
        /// Structure is documented below.
        pub followup_intent_infos: pulumi_gestalt_rust::Output<
            Vec<super::super::types::diagflow::IntentFollowupIntentInfo>,
        >,
        /// The list of context names required for this intent to be triggered.
        /// Format: projects/<Project ID>/agent/sessions/-/contexts/<Context ID>.
        pub input_context_names: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Indicates whether this is a fallback intent.
        pub is_fallback: pulumi_gestalt_rust::Output<bool>,
        /// Indicates whether Machine Learning is disabled for the intent.
        /// Note: If mlDisabled setting is set to true, then this intent is not taken into account during inference in ML
        /// ONLY match mode. Also, auto-markup in the UI is turned off.
        pub ml_disabled: pulumi_gestalt_rust::Output<bool>,
        /// The unique identifier of this intent.
        /// Format: projects/<Project ID>/agent/intents/<Intent ID>.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier of the parent intent in the chain of followup intents.
        /// Format: projects/<Project ID>/agent/intents/<Intent ID>.
        pub parent_followup_intent_name: pulumi_gestalt_rust::Output<String>,
        /// The priority of this intent. Higher numbers represent higher priorities.
        /// - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds
        /// to the Normal priority in the console.
        /// - If the supplied value is negative, the intent is ignored in runtime detect intent requests.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether to delete all contexts in the current session when this intent is matched.
        pub reset_contexts: pulumi_gestalt_rust::Output<bool>,
        /// The unique identifier of the root intent in the chain of followup intents. It identifies the correct followup
        /// intents chain for this intent.
        /// Format: projects/<Project ID>/agent/intents/<Intent ID>.
        pub root_followup_intent_name: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether webhooks are enabled for the intent.
        /// * WEBHOOK_STATE_ENABLED: Webhook is enabled in the agent and in the intent.
        /// * WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING: Webhook is enabled in the agent and in the intent. Also, each slot
        /// filling prompt is forwarded to the webhook.
        /// Possible values are: `WEBHOOK_STATE_ENABLED`, `WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING`.
        pub webhook_state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: IntentArgs,
    ) -> IntentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let default_response_platforms_binding = args
            .default_response_platforms
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let events_binding = args.events.get_output(context).get_inner();
        let input_context_names_binding = args
            .input_context_names
            .get_output(context)
            .get_inner();
        let is_fallback_binding = args.is_fallback.get_output(context).get_inner();
        let ml_disabled_binding = args.ml_disabled.get_output(context).get_inner();
        let parent_followup_intent_name_binding = args
            .parent_followup_intent_name
            .get_output(context)
            .get_inner();
        let priority_binding = args.priority.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let reset_contexts_binding = args.reset_contexts.get_output(context).get_inner();
        let webhook_state_binding = args.webhook_state.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:diagflow/intent:Intent".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "defaultResponsePlatforms".into(),
                    value: &default_response_platforms_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "events".into(),
                    value: &events_binding,
                },
                register_interface::ObjectField {
                    name: "inputContextNames".into(),
                    value: &input_context_names_binding,
                },
                register_interface::ObjectField {
                    name: "isFallback".into(),
                    value: &is_fallback_binding,
                },
                register_interface::ObjectField {
                    name: "mlDisabled".into(),
                    value: &ml_disabled_binding,
                },
                register_interface::ObjectField {
                    name: "parentFollowupIntentName".into(),
                    value: &parent_followup_intent_name_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "resetContexts".into(),
                    value: &reset_contexts_binding,
                },
                register_interface::ObjectField {
                    name: "webhookState".into(),
                    value: &webhook_state_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IntentResult {
            action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("action"),
            ),
            default_response_platforms: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultResponsePlatforms"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            events: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("events"),
            ),
            followup_intent_infos: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("followupIntentInfos"),
            ),
            input_context_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputContextNames"),
            ),
            is_fallback: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isFallback"),
            ),
            ml_disabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mlDisabled"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent_followup_intent_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parentFollowupIntentName"),
            ),
            priority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            reset_contexts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resetContexts"),
            ),
            root_followup_intent_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rootFollowupIntentName"),
            ),
            webhook_state: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("webhookState"),
            ),
        }
    }
}
