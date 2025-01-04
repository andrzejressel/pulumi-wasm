/// Provides a GameLift Alias resource.
///
/// ## Import
///
/// GameLift Matchmaking Configurations can be imported using the ID, e.g.,
///
/// ```sh
/// $ pulumi import aws:gamelift/matchmakingConfiguration:MatchmakingConfiguration example <matchmakingconfiguration-id>
/// ```
pub mod matchmaking_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MatchmakingConfigurationArgs {
        /// Specifies if the match that was created with this configuration must be accepted by matched players.
        #[builder(into, default)]
        pub acceptance_required: pulumi_wasm_rust::Output<Option<bool>>,
        /// The length of time (in seconds) to wait for players to accept a proposed match, if acceptance is required.
        #[builder(into, default)]
        pub acceptance_timeout_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The number of player slots in a match to keep open for future players.
        #[builder(into, default)]
        pub additional_player_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The method used to backfill game sessions that are created with this matchmaking configuration.
        #[builder(into, default)]
        pub backfill_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Information to be added to all events related to this matchmaking configuration.
        #[builder(into, default)]
        pub custom_event_data: pulumi_wasm_rust::Output<Option<String>>,
        /// A human-readable description of the matchmaking configuration.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether this matchmaking configuration is being used with GameLift hosting or as a standalone matchmaking solution.
        #[builder(into, default)]
        pub flex_match_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more custom game properties. See below.
        #[builder(into, default)]
        pub game_properties: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::gamelift::MatchmakingConfigurationGameProperty>,
            >,
        >,
        /// A set of custom game session properties.
        #[builder(into, default)]
        pub game_session_data: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARNs of the GameLift game session queue resources.
        #[builder(into, default)]
        pub game_session_queue_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name of the matchmaking configuration
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// An SNS topic ARN that is set up to receive matchmaking notifications.
        #[builder(into, default)]
        pub notification_target: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum duration, in seconds, that a matchmaking ticket can remain in process before timing out.
        #[builder(into)]
        pub request_timeout_seconds: pulumi_wasm_rust::Output<i32>,
        /// A rule set names for the matchmaking rule set to use with this configuration.
        #[builder(into)]
        pub rule_set_name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MatchmakingConfigurationResult {
        /// Specifies if the match that was created with this configuration must be accepted by matched players.
        pub acceptance_required: pulumi_wasm_rust::Output<Option<bool>>,
        /// The length of time (in seconds) to wait for players to accept a proposed match, if acceptance is required.
        pub acceptance_timeout_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The number of player slots in a match to keep open for future players.
        pub additional_player_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Matchmaking Configuration ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The method used to backfill game sessions that are created with this matchmaking configuration.
        pub backfill_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The time when the Matchmaking Configuration was created.
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// Information to be added to all events related to this matchmaking configuration.
        pub custom_event_data: pulumi_wasm_rust::Output<Option<String>>,
        /// A human-readable description of the matchmaking configuration.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether this matchmaking configuration is being used with GameLift hosting or as a standalone matchmaking solution.
        pub flex_match_mode: pulumi_wasm_rust::Output<String>,
        /// One or more custom game properties. See below.
        pub game_properties: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::gamelift::MatchmakingConfigurationGameProperty>,
            >,
        >,
        /// A set of custom game session properties.
        pub game_session_data: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARNs of the GameLift game session queue resources.
        pub game_session_queue_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name of the matchmaking configuration
        pub name: pulumi_wasm_rust::Output<String>,
        /// An SNS topic ARN that is set up to receive matchmaking notifications.
        pub notification_target: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum duration, in seconds, that a matchmaking ticket can remain in process before timing out.
        pub request_timeout_seconds: pulumi_wasm_rust::Output<i32>,
        pub rule_set_arn: pulumi_wasm_rust::Output<String>,
        /// A rule set names for the matchmaking rule set to use with this configuration.
        pub rule_set_name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: MatchmakingConfigurationArgs,
    ) -> MatchmakingConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let acceptance_required_binding = args.acceptance_required.get_inner();
        let acceptance_timeout_seconds_binding = args
            .acceptance_timeout_seconds
            .get_inner();
        let additional_player_count_binding = args.additional_player_count.get_inner();
        let backfill_mode_binding = args.backfill_mode.get_inner();
        let custom_event_data_binding = args.custom_event_data.get_inner();
        let description_binding = args.description.get_inner();
        let flex_match_mode_binding = args.flex_match_mode.get_inner();
        let game_properties_binding = args.game_properties.get_inner();
        let game_session_data_binding = args.game_session_data.get_inner();
        let game_session_queue_arns_binding = args.game_session_queue_arns.get_inner();
        let name_binding = args.name.get_inner();
        let notification_target_binding = args.notification_target.get_inner();
        let request_timeout_seconds_binding = args.request_timeout_seconds.get_inner();
        let rule_set_name_binding = args.rule_set_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:gamelift/matchmakingConfiguration:MatchmakingConfiguration"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceptanceRequired".into(),
                },
                register_interface::ResultField {
                    name: "acceptanceTimeoutSeconds".into(),
                },
                register_interface::ResultField {
                    name: "additionalPlayerCount".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "backfillMode".into(),
                },
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "customEventData".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "flexMatchMode".into(),
                },
                register_interface::ResultField {
                    name: "gameProperties".into(),
                },
                register_interface::ResultField {
                    name: "gameSessionData".into(),
                },
                register_interface::ResultField {
                    name: "gameSessionQueueArns".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notificationTarget".into(),
                },
                register_interface::ResultField {
                    name: "requestTimeoutSeconds".into(),
                },
                register_interface::ResultField {
                    name: "ruleSetArn".into(),
                },
                register_interface::ResultField {
                    name: "ruleSetName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MatchmakingConfigurationResult {
            acceptance_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceptanceRequired").unwrap(),
            ),
            acceptance_timeout_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceptanceTimeoutSeconds").unwrap(),
            ),
            additional_player_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalPlayerCount").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            backfill_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backfillMode").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            custom_event_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customEventData").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            flex_match_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("flexMatchMode").unwrap(),
            ),
            game_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gameProperties").unwrap(),
            ),
            game_session_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gameSessionData").unwrap(),
            ),
            game_session_queue_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gameSessionQueueArns").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notification_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationTarget").unwrap(),
            ),
            request_timeout_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestTimeoutSeconds").unwrap(),
            ),
            rule_set_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleSetArn").unwrap(),
            ),
            rule_set_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleSetName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
