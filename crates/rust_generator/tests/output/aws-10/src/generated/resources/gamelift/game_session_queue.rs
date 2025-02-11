/// Provides an GameLift Game Session Queue resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = game_session_queue::create(
///         "test",
///         GameSessionQueueArgs::builder()
///             .destinations(vec!["${usWest2Fleet.arn}", "${euCentral1Fleet.arn}",])
///             .name("example-session-queue")
///             .notification_target("${gameSessionQueueNotifications.arn}")
///             .player_latency_policies(
///                 vec![
///                     GameSessionQueuePlayerLatencyPolicy::builder()
///                     .maximumIndividualPlayerLatencyMilliseconds(100)
///                     .policyDurationSeconds(5).build_struct(),
///                     GameSessionQueuePlayerLatencyPolicy::builder()
///                     .maximumIndividualPlayerLatencyMilliseconds(200).build_struct(),
///                 ],
///             )
///             .timeout_in_seconds(60)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import GameLift Game Session Queues using their `name`. For example:
///
/// ```sh
/// $ pulumi import aws:gamelift/gameSessionQueue:GameSessionQueue example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod game_session_queue {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GameSessionQueueArgs {
        /// Information to be added to all events that are related to this game session queue.
        #[builder(into, default)]
        pub custom_event_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of fleet/alias ARNs used by session queue for placing game sessions.
        #[builder(into, default)]
        pub destinations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the session queue.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An SNS topic ARN that is set up to receive game session placement notifications.
        #[builder(into, default)]
        pub notification_target: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more policies used to choose fleet based on player latency. See below.
        #[builder(into, default)]
        pub player_latency_policies: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::gamelift::GameSessionQueuePlayerLatencyPolicy>,
            >,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Maximum time a game session request can remain in the queue.
        #[builder(into, default)]
        pub timeout_in_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct GameSessionQueueResult {
        /// Game Session Queue ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Information to be added to all events that are related to this game session queue.
        pub custom_event_data: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of fleet/alias ARNs used by session queue for placing game sessions.
        pub destinations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Name of the session queue.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An SNS topic ARN that is set up to receive game session placement notifications.
        pub notification_target: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more policies used to choose fleet based on player latency. See below.
        pub player_latency_policies: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::gamelift::GameSessionQueuePlayerLatencyPolicy>,
            >,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Maximum time a game session request can remain in the queue.
        pub timeout_in_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GameSessionQueueArgs,
    ) -> GameSessionQueueResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_event_data_binding = args.custom_event_data.get_output(context);
        let destinations_binding = args.destinations.get_output(context);
        let name_binding = args.name.get_output(context);
        let notification_target_binding = args.notification_target.get_output(context);
        let player_latency_policies_binding = args
            .player_latency_policies
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeout_in_seconds_binding = args.timeout_in_seconds.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:gamelift/gameSessionQueue:GameSessionQueue".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customEventData".into(),
                    value: &custom_event_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinations".into(),
                    value: &destinations_binding.drop_type(),
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
                    name: "playerLatencyPolicies".into(),
                    value: &player_latency_policies_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeoutInSeconds".into(),
                    value: &timeout_in_seconds_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GameSessionQueueResult {
            arn: o.get_field("arn"),
            custom_event_data: o.get_field("customEventData"),
            destinations: o.get_field("destinations"),
            name: o.get_field("name"),
            notification_target: o.get_field("notificationTarget"),
            player_latency_policies: o.get_field("playerLatencyPolicies"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeout_in_seconds: o.get_field("timeoutInSeconds"),
        }
    }
}
