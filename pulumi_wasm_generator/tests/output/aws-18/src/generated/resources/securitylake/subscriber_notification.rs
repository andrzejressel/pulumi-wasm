/// Resource for managing an AWS Security Lake Subscriber Notification.
///
/// ## Example Usage
///
/// ### SQS Notification
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod subscriber_notification {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriberNotificationArgs {
        /// Specify the configuration using which you want to create the subscriber notification..
        #[builder(into, default)]
        pub configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::securitylake::SubscriberNotificationConfiguration,
            >,
        >,
        /// The subscriber ID for the notification subscription.
        #[builder(into)]
        pub subscriber_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SubscriberNotificationResult {
        /// Specify the configuration using which you want to create the subscriber notification..
        pub configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::securitylake::SubscriberNotificationConfiguration,
            >,
        >,
        /// (**Deprecated**) The subscriber endpoint to which exception messages are posted.
        pub endpoint_id: pulumi_wasm_rust::Output<String>,
        /// The subscriber endpoint to which exception messages are posted.
        pub subscriber_endpoint: pulumi_wasm_rust::Output<String>,
        /// The subscriber ID for the notification subscription.
        pub subscriber_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SubscriberNotificationArgs,
    ) -> SubscriberNotificationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_binding = args.configuration.get_inner();
        let subscriber_id_binding = args.subscriber_id.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "configuration".into(),
                },
                register_interface::ResultField {
                    name: "endpointId".into(),
                },
                register_interface::ResultField {
                    name: "subscriberEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "subscriberId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SubscriberNotificationResult {
            configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configuration").unwrap(),
            ),
            endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointId").unwrap(),
            ),
            subscriber_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriberEndpoint").unwrap(),
            ),
            subscriber_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriberId").unwrap(),
            ),
        }
    }
}
