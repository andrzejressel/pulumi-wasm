/// Resource for managing an AWS SESv2 (Simple Email V2) Configuration Set Event Destination.
///
/// ## Example Usage
///
/// ### CloudWatch Destination
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = configuration_set::create(
///         "example",
///         ConfigurationSetArgs::builder().configuration_set_name("example").build_struct(),
///     );
///     let exampleConfigurationSetEventDestination = configuration_set_event_destination::create(
///         "exampleConfigurationSetEventDestination",
///         ConfigurationSetEventDestinationArgs::builder()
///             .configuration_set_name("${example.configurationSetName}")
///             .event_destination(
///                 ConfigurationSetEventDestinationEventDestination::builder()
///                     .cloudWatchDestination(
///                         ConfigurationSetEventDestinationEventDestinationCloudWatchDestination::builder()
///                             .dimensionConfigurations(
///                                 vec![
///                                     ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfiguration::builder()
///                                     .defaultDimensionValue("example").dimensionName("example")
///                                     .dimensionValueSource("MESSAGE_TAG").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .enabled(true)
///                     .matchingEventTypes(vec!["SEND",])
///                     .build_struct(),
///             )
///             .event_destination_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### EventBridge Destination
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = get_event_bus::invoke(
///         GetEventBusArgs::builder().name("default").build_struct(),
///     );
///     let example = configuration_set_event_destination::create(
///         "example",
///         ConfigurationSetEventDestinationArgs::builder()
///             .configuration_set_name(
///                 "${exampleAwsSesv2ConfigurationSet.configurationSetName}",
///             )
///             .event_destination(
///                 ConfigurationSetEventDestinationEventDestination::builder()
///                     .enabled(true)
///                     .eventBridgeDestination(
///                         ConfigurationSetEventDestinationEventDestinationEventBridgeDestination::builder()
///                             .eventBusArn("${default.arn}")
///                             .build_struct(),
///                     )
///                     .matchingEventTypes(vec!["SEND",])
///                     .build_struct(),
///             )
///             .event_destination_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Kinesis Firehose Destination
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = configuration_set::create(
///         "example",
///         ConfigurationSetArgs::builder().configuration_set_name("example").build_struct(),
///     );
///     let exampleConfigurationSetEventDestination = configuration_set_event_destination::create(
///         "exampleConfigurationSetEventDestination",
///         ConfigurationSetEventDestinationArgs::builder()
///             .configuration_set_name("${example.configurationSetName}")
///             .event_destination(
///                 ConfigurationSetEventDestinationEventDestination::builder()
///                     .enabled(true)
///                     .kinesisFirehoseDestination(
///                         ConfigurationSetEventDestinationEventDestinationKinesisFirehoseDestination::builder()
///                             .deliveryStreamArn(
///                                 "${exampleAwsKinesisFirehoseDeliveryStream.arn}",
///                             )
///                             .iamRoleArn("${exampleAwsIamRole.arn}")
///                             .build_struct(),
///                     )
///                     .matchingEventTypes(vec!["SEND",])
///                     .build_struct(),
///             )
///             .event_destination_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Pinpoint Destination
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = configuration_set::create(
///         "example",
///         ConfigurationSetArgs::builder().configuration_set_name("example").build_struct(),
///     );
///     let exampleConfigurationSetEventDestination = configuration_set_event_destination::create(
///         "exampleConfigurationSetEventDestination",
///         ConfigurationSetEventDestinationArgs::builder()
///             .configuration_set_name("${example.configurationSetName}")
///             .event_destination(
///                 ConfigurationSetEventDestinationEventDestination::builder()
///                     .enabled(true)
///                     .matchingEventTypes(vec!["SEND",])
///                     .pinpointDestination(
///                         ConfigurationSetEventDestinationEventDestinationPinpointDestination::builder()
///                             .applicationArn("${exampleAwsPinpointApp.arn}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .event_destination_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### SNS Destination
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = configuration_set::create(
///         "example",
///         ConfigurationSetArgs::builder().configuration_set_name("example").build_struct(),
///     );
///     let exampleConfigurationSetEventDestination = configuration_set_event_destination::create(
///         "exampleConfigurationSetEventDestination",
///         ConfigurationSetEventDestinationArgs::builder()
///             .configuration_set_name("${example.configurationSetName}")
///             .event_destination(
///                 ConfigurationSetEventDestinationEventDestination::builder()
///                     .enabled(true)
///                     .matchingEventTypes(vec!["SEND",])
///                     .snsDestination(
///                         ConfigurationSetEventDestinationEventDestinationSnsDestination::builder()
///                             .topicArn("${exampleAwsSnsTopic.arn}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .event_destination_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SESv2 (Simple Email V2) Configuration Set Event Destination using the `id` (`configuration_set_name|event_destination_name`). For example:
///
/// ```sh
/// $ pulumi import aws:sesv2/configurationSetEventDestination:ConfigurationSetEventDestination example example_configuration_set|example_event_destination
/// ```
pub mod configuration_set_event_destination {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationSetEventDestinationArgs {
        /// The name of the configuration set.
        #[builder(into)]
        pub configuration_set_name: pulumi_wasm_rust::Output<String>,
        /// A name that identifies the event destination within the configuration set.
        #[builder(into)]
        pub event_destination: pulumi_wasm_rust::Output<
            super::super::types::sesv2::ConfigurationSetEventDestinationEventDestination,
        >,
        /// An object that defines the event destination. See `event_destination` Block for details.
        #[builder(into)]
        pub event_destination_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ConfigurationSetEventDestinationResult {
        /// The name of the configuration set.
        pub configuration_set_name: pulumi_wasm_rust::Output<String>,
        /// A name that identifies the event destination within the configuration set.
        pub event_destination: pulumi_wasm_rust::Output<
            super::super::types::sesv2::ConfigurationSetEventDestinationEventDestination,
        >,
        /// An object that defines the event destination. See `event_destination` Block for details.
        pub event_destination_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ConfigurationSetEventDestinationArgs,
    ) -> ConfigurationSetEventDestinationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_set_name_binding = args.configuration_set_name.get_inner();
        let event_destination_binding = args.event_destination.get_inner();
        let event_destination_name_binding = args.event_destination_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sesv2/configurationSetEventDestination:ConfigurationSetEventDestination"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurationSetName".into(),
                    value: &configuration_set_name_binding,
                },
                register_interface::ObjectField {
                    name: "eventDestination".into(),
                    value: &event_destination_binding,
                },
                register_interface::ObjectField {
                    name: "eventDestinationName".into(),
                    value: &event_destination_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configurationSetName".into(),
                },
                register_interface::ResultField {
                    name: "eventDestination".into(),
                },
                register_interface::ResultField {
                    name: "eventDestinationName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConfigurationSetEventDestinationResult {
            configuration_set_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationSetName").unwrap(),
            ),
            event_destination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventDestination").unwrap(),
            ),
            event_destination_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventDestinationName").unwrap(),
            ),
        }
    }
}