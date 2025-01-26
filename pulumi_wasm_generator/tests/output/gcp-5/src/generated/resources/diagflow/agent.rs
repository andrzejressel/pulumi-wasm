/// A Dialogflow agent is a virtual agent that handles conversations with your end-users. It is a natural language
/// understanding module that understands the nuances of human language. Dialogflow translates end-user text or audio
/// during a conversation to structured data that your apps and services can understand. You design and build a Dialogflow
/// agent to handle the types of conversations required for your system.
///
///
/// To get more information about Agent, see:
///
/// * [API documentation](https://cloud.google.com/dialogflow/docs/reference/rest/v2/projects/agent)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dialogflow/docs/)
///
/// ## Example Usage
///
/// ### Dialogflow Agent Full
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let fullAgent = agent::create(
///         "fullAgent",
///         AgentArgs::builder()
///             .api_version("API_VERSION_V2_BETA_1")
///             .avatar_uri(
///                 "https://cloud.google.com/_static/images/cloud/icons/favicons/onecloud/super_cloud.png",
///             )
///             .classification_threshold(0.3)
///             .default_language_code("en")
///             .description("Example description.")
///             .display_name("dialogflow-agent")
///             .enable_logging(true)
///             .match_mode("MATCH_MODE_ML_ONLY")
///             .supported_language_codes(vec!["fr", "de", "es",])
///             .tier("TIER_STANDARD")
///             .time_zone("America/New_York")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Agent can be imported using any of these accepted formats:
///
/// * `{{project}}`
///
/// When using the `pulumi import` command, Agent can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:diagflow/agent:Agent default {{project}}
/// ```
///
pub mod agent {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgentArgs {
        /// API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query
        /// different service endpoints for different API versions. However, bots connectors and webhook calls will follow
        /// the specified API version.
        /// * API_VERSION_V1: Legacy V1 API.
        /// * API_VERSION_V2: V2 API.
        /// * API_VERSION_V2_BETA_1: V2beta1 API.
        /// Possible values are: `API_VERSION_V1`, `API_VERSION_V2`, `API_VERSION_V2_BETA_1`.
        #[builder(into, default)]
        pub api_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The URI of the agent's avatar, which are used throughout the Dialogflow console. When an image URL is entered
        /// into this field, the Dialogflow will save the image in the backend. The address of the backend image returned
        /// from the API will be shown in the [avatarUriBackend] field.
        #[builder(into, default)]
        pub avatar_uri: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// To filter out false positive results and still get variety in matched natural language inputs for your agent,
        /// you can tune the machine learning classification threshold. If the returned score value is less than the threshold
        /// value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be
        /// triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the
        /// default of 0.3 is used.
        #[builder(into, default)]
        pub classification_threshold: pulumi_wasm_rust::InputOrOutput<Option<f64>>,
        /// The default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/docs/reference/language)
        /// for a list of the currently supported language codes. This field cannot be updated after creation.
        #[builder(into)]
        pub default_language_code: pulumi_wasm_rust::InputOrOutput<String>,
        /// The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of this agent.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Determines whether this agent should log conversation queries.
        #[builder(into, default)]
        pub enable_logging: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Determines how intents are detected from user queries.
        /// * MATCH_MODE_HYBRID: Best for agents with a small number of examples in intents and/or wide use of templates
        /// syntax and composite entities.
        /// * MATCH_MODE_ML_ONLY: Can be used for agents with a large number of examples in intents, especially the ones
        /// using @sys.any or very large developer entities.
        /// Possible values are: `MATCH_MODE_HYBRID`, `MATCH_MODE_ML_ONLY`.
        #[builder(into, default)]
        pub match_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The list of all languages supported by this agent (except for the defaultLanguageCode).
        #[builder(into, default)]
        pub supported_language_codes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The agent tier. If not specified, TIER_STANDARD is assumed.
        /// * TIER_STANDARD: Standard tier.
        /// * TIER_ENTERPRISE: Enterprise tier (Essentials).
        /// * TIER_ENTERPRISE_PLUS: Enterprise tier (Plus).
        /// NOTE: Due to consistency issues, the provider will not read this field from the API. Drift is possible between
        /// the the provider state and Dialogflow if the agent tier is changed outside of the provider.
        #[builder(into, default)]
        pub tier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,
        /// Europe/Paris.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub time_zone: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AgentResult {
        /// API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query
        /// different service endpoints for different API versions. However, bots connectors and webhook calls will follow
        /// the specified API version.
        /// * API_VERSION_V1: Legacy V1 API.
        /// * API_VERSION_V2: V2 API.
        /// * API_VERSION_V2_BETA_1: V2beta1 API.
        /// Possible values are: `API_VERSION_V1`, `API_VERSION_V2`, `API_VERSION_V2_BETA_1`.
        pub api_version: pulumi_wasm_rust::Output<String>,
        /// The URI of the agent's avatar, which are used throughout the Dialogflow console. When an image URL is entered
        /// into this field, the Dialogflow will save the image in the backend. The address of the backend image returned
        /// from the API will be shown in the [avatarUriBackend] field.
        pub avatar_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// The URI of the agent's avatar as returned from the API. Output only. To provide an image URL for the agent avatar,
        /// the [avatarUri] field can be used.
        pub avatar_uri_backend: pulumi_wasm_rust::Output<String>,
        /// To filter out false positive results and still get variety in matched natural language inputs for your agent,
        /// you can tune the machine learning classification threshold. If the returned score value is less than the threshold
        /// value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be
        /// triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the
        /// default of 0.3 is used.
        pub classification_threshold: pulumi_wasm_rust::Output<Option<f64>>,
        /// The default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/docs/reference/language)
        /// for a list of the currently supported language codes. This field cannot be updated after creation.
        pub default_language_code: pulumi_wasm_rust::Output<String>,
        /// The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of this agent.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Determines whether this agent should log conversation queries.
        pub enable_logging: pulumi_wasm_rust::Output<Option<bool>>,
        /// Determines how intents are detected from user queries.
        /// * MATCH_MODE_HYBRID: Best for agents with a small number of examples in intents and/or wide use of templates
        /// syntax and composite entities.
        /// * MATCH_MODE_ML_ONLY: Can be used for agents with a large number of examples in intents, especially the ones
        /// using @sys.any or very large developer entities.
        /// Possible values are: `MATCH_MODE_HYBRID`, `MATCH_MODE_ML_ONLY`.
        pub match_mode: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The list of all languages supported by this agent (except for the defaultLanguageCode).
        pub supported_language_codes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The agent tier. If not specified, TIER_STANDARD is assumed.
        /// * TIER_STANDARD: Standard tier.
        /// * TIER_ENTERPRISE: Enterprise tier (Essentials).
        /// * TIER_ENTERPRISE_PLUS: Enterprise tier (Plus).
        /// NOTE: Due to consistency issues, the provider will not read this field from the API. Drift is possible between
        /// the the provider state and Dialogflow if the agent tier is changed outside of the provider.
        pub tier: pulumi_wasm_rust::Output<Option<String>>,
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
        args: AgentArgs,
    ) -> AgentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_version_binding = args.api_version.get_output(context).get_inner();
        let avatar_uri_binding = args.avatar_uri.get_output(context).get_inner();
        let classification_threshold_binding = args
            .classification_threshold
            .get_output(context)
            .get_inner();
        let default_language_code_binding = args
            .default_language_code
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let enable_logging_binding = args.enable_logging.get_output(context).get_inner();
        let match_mode_binding = args.match_mode.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let supported_language_codes_binding = args
            .supported_language_codes
            .get_output(context)
            .get_inner();
        let tier_binding = args.tier.get_output(context).get_inner();
        let time_zone_binding = args.time_zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:diagflow/agent:Agent".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiVersion".into(),
                    value: &api_version_binding,
                },
                register_interface::ObjectField {
                    name: "avatarUri".into(),
                    value: &avatar_uri_binding,
                },
                register_interface::ObjectField {
                    name: "classificationThreshold".into(),
                    value: &classification_threshold_binding,
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
                    name: "enableLogging".into(),
                    value: &enable_logging_binding,
                },
                register_interface::ObjectField {
                    name: "matchMode".into(),
                    value: &match_mode_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "supportedLanguageCodes".into(),
                    value: &supported_language_codes_binding,
                },
                register_interface::ObjectField {
                    name: "tier".into(),
                    value: &tier_binding,
                },
                register_interface::ObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AgentResult {
            api_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("apiVersion"),
            ),
            avatar_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("avatarUri"),
            ),
            avatar_uri_backend: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("avatarUriBackend"),
            ),
            classification_threshold: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("classificationThreshold"),
            ),
            default_language_code: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultLanguageCode"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            enable_logging: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableLogging"),
            ),
            match_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("matchMode"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            supported_language_codes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("supportedLanguageCodes"),
            ),
            tier: pulumi_wasm_rust::__private::into_domain(o.extract_field("tier")),
            time_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeZone"),
            ),
        }
    }
}
