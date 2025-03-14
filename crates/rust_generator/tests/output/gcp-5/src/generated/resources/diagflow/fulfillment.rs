/// By default, your agent responds to a matched intent with a static response. If you're using one of the integration options, you can provide a more dynamic response by using fulfillment. When you enable fulfillment for an intent, Dialogflow responds to that intent by calling a service that you define. For example, if an end-user wants to schedule a haircut on Friday, your service can check your database and respond to the end-user with availability information for Friday.
///
///
/// To get more information about Fulfillment, see:
///
/// * [API documentation](https://cloud.google.com/dialogflow/es/docs/reference/rest/v2/projects.agent/getFulfillment)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dialogflow/es/docs/fulfillment-overview)
///
/// ## Example Usage
///
/// ### Dialogflow Fulfillment Basic
///
///
/// ```yaml
/// resources:
///   basicAgent:
///     type: gcp:diagflow:Agent
///     name: basic_agent
///     properties:
///       displayName: example_agent
///       defaultLanguageCode: en
///       timeZone: America/New_York
///   basicFulfillment:
///     type: gcp:diagflow:Fulfillment
///     name: basic_fulfillment
///     properties:
///       displayName: basic-fulfillment
///       enabled: true
///       genericWebService:
///         uri: https://google.com
///         username: admin
///         password: password
///         requestHeaders:
///           name: wrench
///     options:
///       dependsOn:
///         - ${basicAgent}
/// ```
///
/// ## Import
///
/// Fulfillment can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Fulfillment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:diagflow/fulfillment:Fulfillment default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod fulfillment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FulfillmentArgs {
        /// The human-readable name of the fulfillment, unique within the agent.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether fulfillment is enabled.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The field defines whether the fulfillment is enabled for certain features.
        /// Structure is documented below.
        #[builder(into, default)]
        pub features: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::diagflow::FulfillmentFeature>>,
        >,
        /// Represents configuration for a generic web service. Dialogflow supports two mechanisms for authentications: - Basic authentication with username and password. - Authentication with additional authentication headers.
        /// Structure is documented below.
        #[builder(into, default)]
        pub generic_web_service: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::diagflow::FulfillmentGenericWebService>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FulfillmentResult {
        /// The human-readable name of the fulfillment, unique within the agent.
        ///
        ///
        /// - - -
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Whether fulfillment is enabled.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The field defines whether the fulfillment is enabled for certain features.
        /// Structure is documented below.
        pub features: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::diagflow::FulfillmentFeature>>,
        >,
        /// Represents configuration for a generic web service. Dialogflow supports two mechanisms for authentications: - Basic authentication with username and password. - Authentication with additional authentication headers.
        /// Structure is documented below.
        pub generic_web_service: pulumi_gestalt_rust::Output<
            Option<super::super::types::diagflow::FulfillmentGenericWebService>,
        >,
        /// The unique identifier of the fulfillment.
        /// Format: projects/<Project ID>/agent/fulfillment - projects/<Project ID>/locations/<Location ID>/agent/fulfillment
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FulfillmentArgs,
    ) -> FulfillmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let features_binding = args.features.get_output(context);
        let generic_web_service_binding = args.generic_web_service.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:diagflow/fulfillment:Fulfillment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "features".into(),
                    value: &features_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "genericWebService".into(),
                    value: &generic_web_service_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FulfillmentResult {
            display_name: o.get_field("displayName"),
            enabled: o.get_field("enabled"),
            features: o.get_field("features"),
            generic_web_service: o.get_field("genericWebService"),
            name: o.get_field("name"),
            project: o.get_field("project"),
        }
    }
}
