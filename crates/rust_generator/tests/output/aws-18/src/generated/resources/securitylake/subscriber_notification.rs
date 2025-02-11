/// Resource for managing an AWS Security Lake Subscriber Notification.
///
/// ## Example Usage
///
/// ### SQS Notification
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = subscriber_notification::create(
///         "example",
///         SubscriberNotificationArgs::builder()
///             .configuration(
///                 SubscriberNotificationConfiguration::builder()
///                     .sqsNotificationConfiguration(
///                         SubscriberNotificationConfigurationSqsNotificationConfiguration::builder()
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .subscriber_id("${exampleAwsSecuritylakeSubscriber.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### HTTPS Notification
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = subscriber_notification::create(
///         "example",
///         SubscriberNotificationArgs::builder()
///             .configuration(
///                 SubscriberNotificationConfiguration::builder()
///                     .httpsNotificationConfiguration(
///                         SubscriberNotificationConfigurationHttpsNotificationConfiguration::builder()
///                             .endpoint("${test.apiEndpoint}")
///                             .targetRoleArn("${eventBridge.arn}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .subscriber_id("${exampleAwsSecuritylakeSubscriber.id}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod subscriber_notification {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriberNotificationArgs {
        /// Specify the configuration using which you want to create the subscriber notification..
        #[builder(into, default)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::securitylake::SubscriberNotificationConfiguration,
            >,
        >,
        /// The subscriber ID for the notification subscription.
        #[builder(into)]
        pub subscriber_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubscriberNotificationResult {
        /// Specify the configuration using which you want to create the subscriber notification..
        pub configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::securitylake::SubscriberNotificationConfiguration,
            >,
        >,
        /// (**Deprecated**) The subscriber endpoint to which exception messages are posted.
        pub endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// The subscriber endpoint to which exception messages are posted.
        pub subscriber_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The subscriber ID for the notification subscription.
        pub subscriber_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SubscriberNotificationArgs,
    ) -> SubscriberNotificationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_binding = args.configuration.get_output(context);
        let subscriber_id_binding = args.subscriber_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:securitylake/subscriberNotification:SubscriberNotification"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriberId".into(),
                    value: &subscriber_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SubscriberNotificationResult {
            configuration: o.get_field("configuration"),
            endpoint_id: o.get_field("endpointId"),
            subscriber_endpoint: o.get_field("subscriberEndpoint"),
            subscriber_id: o.get_field("subscriberId"),
        }
    }
}
