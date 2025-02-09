/// Resource for managing an AWS SESv2 (Simple Email V2) Configuration Set Event Destination.
///
/// ## Example Usage
///
/// ### CloudWatch Destination
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// ```yaml
/// resources:
///   example:
///     type: aws:sesv2:ConfigurationSetEventDestination
///     properties:
///       configurationSetName: ${exampleAwsSesv2ConfigurationSet.configurationSetName}
///       eventDestinationName: example
///       eventDestination:
///         eventBridgeDestination:
///           eventBusArn: ${default.arn}
///         enabled: true
///         matchingEventTypes:
///           - SEND
/// variables:
///   default:
///     fn::invoke:
///       function: aws:cloudwatch:getEventBus
///       arguments:
///         name: default
/// ```
///
/// ### Kinesis Firehose Destination
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod configuration_set_event_destination {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationSetEventDestinationArgs {
        /// The name of the configuration set.
        #[builder(into)]
        pub configuration_set_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A name that identifies the event destination within the configuration set.
        #[builder(into)]
        pub event_destination: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::sesv2::ConfigurationSetEventDestinationEventDestination,
        >,
        /// An object that defines the event destination. See `event_destination` Block for details.
        #[builder(into)]
        pub event_destination_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConfigurationSetEventDestinationResult {
        /// The name of the configuration set.
        pub configuration_set_name: pulumi_gestalt_rust::Output<String>,
        /// A name that identifies the event destination within the configuration set.
        pub event_destination: pulumi_gestalt_rust::Output<
            super::super::types::sesv2::ConfigurationSetEventDestinationEventDestination,
        >,
        /// An object that defines the event destination. See `event_destination` Block for details.
        pub event_destination_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigurationSetEventDestinationArgs,
    ) -> ConfigurationSetEventDestinationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_set_name_binding = args
            .configuration_set_name
            .get_output(context);
        let event_destination_binding = args.event_destination.get_output(context);
        let event_destination_name_binding = args
            .event_destination_name
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sesv2/configurationSetEventDestination:ConfigurationSetEventDestination"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationSetName".into(),
                    value: configuration_set_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventDestination".into(),
                    value: event_destination_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventDestinationName".into(),
                    value: event_destination_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConfigurationSetEventDestinationResult {
            configuration_set_name: o.get_field("configurationSetName"),
            event_destination: o.get_field("eventDestination"),
            event_destination_name: o.get_field("eventDestinationName"),
        }
    }
}
