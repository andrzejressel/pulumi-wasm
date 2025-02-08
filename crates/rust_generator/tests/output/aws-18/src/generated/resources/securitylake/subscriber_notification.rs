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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SubscriberNotificationArgs,
    ) -> SubscriberNotificationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let configuration_binding = args.configuration.get_output(context).get_inner();
        let subscriber_id_binding = args.subscriber_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securitylake/subscriberNotification:SubscriberNotification"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "subscriberId".into(),
                    value: &subscriber_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SubscriberNotificationResult {
            configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configuration"),
            ),
            endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointId"),
            ),
            subscriber_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subscriberEndpoint"),
            ),
            subscriber_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subscriberId"),
            ),
        }
    }
}
