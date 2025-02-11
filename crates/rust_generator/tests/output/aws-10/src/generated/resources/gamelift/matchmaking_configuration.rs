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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MatchmakingConfigurationArgs,
    ) -> MatchmakingConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let acceptance_required_binding = args.acceptance_required.get_output(context);
        let acceptance_timeout_seconds_binding = args
            .acceptance_timeout_seconds
            .get_output(context);
        let additional_player_count_binding = args
            .additional_player_count
            .get_output(context);
        let backfill_mode_binding = args.backfill_mode.get_output(context);
        let custom_event_data_binding = args.custom_event_data.get_output(context);
        let description_binding = args.description.get_output(context);
        let flex_match_mode_binding = args.flex_match_mode.get_output(context);
        let game_properties_binding = args.game_properties.get_output(context);
        let game_session_data_binding = args.game_session_data.get_output(context);
        let game_session_queue_arns_binding = args
            .game_session_queue_arns
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let notification_target_binding = args.notification_target.get_output(context);
        let request_timeout_seconds_binding = args
            .request_timeout_seconds
            .get_output(context);
        let rule_set_name_binding = args.rule_set_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:gamelift/matchmakingConfiguration:MatchmakingConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acceptanceRequired".into(),
                    value: &acceptance_required_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acceptanceTimeoutSeconds".into(),
                    value: &acceptance_timeout_seconds_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalPlayerCount".into(),
                    value: &additional_player_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backfillMode".into(),
                    value: &backfill_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customEventData".into(),
                    value: &custom_event_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "flexMatchMode".into(),
                    value: &flex_match_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gameProperties".into(),
                    value: &game_properties_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gameSessionData".into(),
                    value: &game_session_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gameSessionQueueArns".into(),
                    value: &game_session_queue_arns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationTarget".into(),
                    value: &notification_target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestTimeoutSeconds".into(),
                    value: &request_timeout_seconds_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleSetName".into(),
                    value: &rule_set_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MatchmakingConfigurationResult {
            acceptance_required: o.get_field("acceptanceRequired"),
            acceptance_timeout_seconds: o.get_field("acceptanceTimeoutSeconds"),
            additional_player_count: o.get_field("additionalPlayerCount"),
            arn: o.get_field("arn"),
            backfill_mode: o.get_field("backfillMode"),
            creation_time: o.get_field("creationTime"),
            custom_event_data: o.get_field("customEventData"),
            description: o.get_field("description"),
            flex_match_mode: o.get_field("flexMatchMode"),
            game_properties: o.get_field("gameProperties"),
            game_session_data: o.get_field("gameSessionData"),
            game_session_queue_arns: o.get_field("gameSessionQueueArns"),
            name: o.get_field("name"),
            notification_target: o.get_field("notificationTarget"),
            request_timeout_seconds: o.get_field("requestTimeoutSeconds"),
            rule_set_arn: o.get_field("ruleSetArn"),
            rule_set_name: o.get_field("ruleSetName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
