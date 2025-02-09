/// Provides a GameLift Alias resource.
///
/// ## Import
///
/// GameLift Matchmaking Configurations can be imported using the ID, e.g.,
///
/// ```sh
/// $ pulumi import aws:gamelift/matchmakingConfiguration:MatchmakingConfiguration example <matchmakingconfiguration-id>
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod matchmaking_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MatchmakingConfigurationArgs {
        /// Specifies if the match that was created with this configuration must be accepted by matched players.
        #[builder(into, default)]
        pub acceptance_required: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The length of time (in seconds) to wait for players to accept a proposed match, if acceptance is required.
        #[builder(into, default)]
        pub acceptance_timeout_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The number of player slots in a match to keep open for future players.
        #[builder(into, default)]
        pub additional_player_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The method used to backfill game sessions that are created with this matchmaking configuration.
        #[builder(into, default)]
        pub backfill_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Information to be added to all events related to this matchmaking configuration.
        #[builder(into, default)]
        pub custom_event_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A human-readable description of the matchmaking configuration.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether this matchmaking configuration is being used with GameLift hosting or as a standalone matchmaking solution.
        #[builder(into, default)]
        pub flex_match_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more custom game properties. See below.
        #[builder(into, default)]
        pub game_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::gamelift::MatchmakingConfigurationGameProperty>,
            >,
        >,
        /// A set of custom game session properties.
        #[builder(into, default)]
        pub game_session_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARNs of the GameLift game session queue resources.
        #[builder(into, default)]
        pub game_session_queue_arns: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Name of the matchmaking configuration
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An SNS topic ARN that is set up to receive matchmaking notifications.
        #[builder(into, default)]
        pub notification_target: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum duration, in seconds, that a matchmaking ticket can remain in process before timing out.
        #[builder(into)]
        pub request_timeout_seconds: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A rule set names for the matchmaking rule set to use with this configuration.
        #[builder(into)]
        pub rule_set_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MatchmakingConfigurationResult {
        /// Specifies if the match that was created with this configuration must be accepted by matched players.
        pub acceptance_required: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The length of time (in seconds) to wait for players to accept a proposed match, if acceptance is required.
        pub acceptance_timeout_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The number of player slots in a match to keep open for future players.
        pub additional_player_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Matchmaking Configuration ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The method used to backfill game sessions that are created with this matchmaking configuration.
        pub backfill_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The time when the Matchmaking Configuration was created.
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// Information to be added to all events related to this matchmaking configuration.
        pub custom_event_data: pulumi_gestalt_rust::Output<Option<String>>,
        /// A human-readable description of the matchmaking configuration.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates whether this matchmaking configuration is being used with GameLift hosting or as a standalone matchmaking solution.
        pub flex_match_mode: pulumi_gestalt_rust::Output<String>,
        /// One or more custom game properties. See below.
        pub game_properties: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::gamelift::MatchmakingConfigurationGameProperty>,
            >,
        >,
        /// A set of custom game session properties.
        pub game_session_data: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARNs of the GameLift game session queue resources.
        pub game_session_queue_arns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Name of the matchmaking configuration
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An SNS topic ARN that is set up to receive matchmaking notifications.
        pub notification_target: pulumi_gestalt_rust::Output<Option<String>>,
        /// The maximum duration, in seconds, that a matchmaking ticket can remain in process before timing out.
        pub request_timeout_seconds: pulumi_gestalt_rust::Output<i32>,
        pub rule_set_arn: pulumi_gestalt_rust::Output<String>,
        /// A rule set names for the matchmaking rule set to use with this configuration.
        pub rule_set_name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MatchmakingConfigurationArgs,
    ) -> MatchmakingConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let acceptance_required_binding_1 = args.acceptance_required.get_output(context);
        let acceptance_required_binding = acceptance_required_binding_1.get_inner();
        let acceptance_timeout_seconds_binding_1 = args
            .acceptance_timeout_seconds
            .get_output(context);
        let acceptance_timeout_seconds_binding = acceptance_timeout_seconds_binding_1
            .get_inner();
        let additional_player_count_binding_1 = args
            .additional_player_count
            .get_output(context);
        let additional_player_count_binding = additional_player_count_binding_1
            .get_inner();
        let backfill_mode_binding_1 = args.backfill_mode.get_output(context);
        let backfill_mode_binding = backfill_mode_binding_1.get_inner();
        let custom_event_data_binding_1 = args.custom_event_data.get_output(context);
        let custom_event_data_binding = custom_event_data_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let flex_match_mode_binding_1 = args.flex_match_mode.get_output(context);
        let flex_match_mode_binding = flex_match_mode_binding_1.get_inner();
        let game_properties_binding_1 = args.game_properties.get_output(context);
        let game_properties_binding = game_properties_binding_1.get_inner();
        let game_session_data_binding_1 = args.game_session_data.get_output(context);
        let game_session_data_binding = game_session_data_binding_1.get_inner();
        let game_session_queue_arns_binding_1 = args
            .game_session_queue_arns
            .get_output(context);
        let game_session_queue_arns_binding = game_session_queue_arns_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let notification_target_binding_1 = args.notification_target.get_output(context);
        let notification_target_binding = notification_target_binding_1.get_inner();
        let request_timeout_seconds_binding_1 = args
            .request_timeout_seconds
            .get_output(context);
        let request_timeout_seconds_binding = request_timeout_seconds_binding_1
            .get_inner();
        let rule_set_name_binding_1 = args.rule_set_name.get_output(context);
        let rule_set_name_binding = rule_set_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:gamelift/matchmakingConfiguration:MatchmakingConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceptanceRequired".into(),
                    value: &acceptance_required_binding,
                },
                register_interface::ObjectField {
                    name: "acceptanceTimeoutSeconds".into(),
                    value: &acceptance_timeout_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "additionalPlayerCount".into(),
                    value: &additional_player_count_binding,
                },
                register_interface::ObjectField {
                    name: "backfillMode".into(),
                    value: &backfill_mode_binding,
                },
                register_interface::ObjectField {
                    name: "customEventData".into(),
                    value: &custom_event_data_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "flexMatchMode".into(),
                    value: &flex_match_mode_binding,
                },
                register_interface::ObjectField {
                    name: "gameProperties".into(),
                    value: &game_properties_binding,
                },
                register_interface::ObjectField {
                    name: "gameSessionData".into(),
                    value: &game_session_data_binding,
                },
                register_interface::ObjectField {
                    name: "gameSessionQueueArns".into(),
                    value: &game_session_queue_arns_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notificationTarget".into(),
                    value: &notification_target_binding,
                },
                register_interface::ObjectField {
                    name: "requestTimeoutSeconds".into(),
                    value: &request_timeout_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "ruleSetName".into(),
                    value: &rule_set_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MatchmakingConfigurationResult {
            acceptance_required: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceptanceRequired"),
            ),
            acceptance_timeout_seconds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceptanceTimeoutSeconds"),
            ),
            additional_player_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("additionalPlayerCount"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            backfill_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backfillMode"),
            ),
            creation_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTime"),
            ),
            custom_event_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customEventData"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            flex_match_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("flexMatchMode"),
            ),
            game_properties: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gameProperties"),
            ),
            game_session_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gameSessionData"),
            ),
            game_session_queue_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gameSessionQueueArns"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            notification_target: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notificationTarget"),
            ),
            request_timeout_seconds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requestTimeoutSeconds"),
            ),
            rule_set_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ruleSetArn"),
            ),
            rule_set_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ruleSetName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
