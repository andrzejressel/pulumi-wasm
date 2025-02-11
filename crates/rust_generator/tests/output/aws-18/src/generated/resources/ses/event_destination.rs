/// Provides an SES event destination
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
///     let cloudwatch = event_destination::create(
///         "cloudwatch",
///         EventDestinationArgs::builder()
///             .cloudwatch_destinations(
///                 vec![
///                     EventDestinationCloudwatchDestination::builder()
///                     .defaultValue("default").dimensionName("dimension")
///                     .valueSource("emailHeader").build_struct(),
///                 ],
///             )
///             .configuration_set_name("${example.name}")
///             .enabled(true)
///             .matching_types(vec!["bounce", "send",])
///             .name("event-destination-cloudwatch")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Kinesis Destination
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let kinesis = event_destination::create(
///         "kinesis",
///         EventDestinationArgs::builder()
///             .configuration_set_name("${exampleAwsSesConfigurationSet.name}")
///             .enabled(true)
///             .kinesis_destination(
///                 EventDestinationKinesisDestination::builder()
///                     .roleArn("${example.arn}")
///                     .streamArn("${exampleAwsKinesisFirehoseDeliveryStream.arn}")
///                     .build_struct(),
///             )
///             .matching_types(vec!["bounce", "send",])
///             .name("event-destination-kinesis")
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
///     let sns = event_destination::create(
///         "sns",
///         EventDestinationArgs::builder()
///             .configuration_set_name("${exampleAwsSesConfigurationSet.name}")
///             .enabled(true)
///             .matching_types(vec!["bounce", "send",])
///             .name("event-destination-sns")
///             .sns_destination(
///                 EventDestinationSnsDestination::builder()
///                     .topicArn("${example.arn}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SES event destinations using `configuration_set_name` together with the event destination's `name`. For example:
///
/// ```sh
/// $ pulumi import aws:ses/eventDestination:EventDestination sns some-configuration-set-test/event-destination-sns
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_destination {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventDestinationArgs {
        /// CloudWatch destination for the events
        #[builder(into, default)]
        pub cloudwatch_destinations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ses::EventDestinationCloudwatchDestination>>,
        >,
        /// The name of the configuration set
        #[builder(into)]
        pub configuration_set_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If true, the event destination will be enabled
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Send the events to a kinesis firehose destination
        #[builder(into, default)]
        pub kinesis_destination: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ses::EventDestinationKinesisDestination>,
        >,
        /// A list of matching types. May be any of `"send"`, `"reject"`, `"bounce"`, `"complaint"`, `"delivery"`, `"open"`, `"click"`, or `"renderingFailure"`.
        #[builder(into)]
        pub matching_types: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The name of the event destination
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Send the events to an SNS Topic destination
        ///
        /// > **NOTE:** You can specify `"cloudwatch_destination"` or `"kinesis_destination"` but not both
        #[builder(into, default)]
        pub sns_destination: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ses::EventDestinationSnsDestination>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventDestinationResult {
        /// The SES event destination ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// CloudWatch destination for the events
        pub cloudwatch_destinations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ses::EventDestinationCloudwatchDestination>>,
        >,
        /// The name of the configuration set
        pub configuration_set_name: pulumi_gestalt_rust::Output<String>,
        /// If true, the event destination will be enabled
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Send the events to a kinesis firehose destination
        pub kinesis_destination: pulumi_gestalt_rust::Output<
            Option<super::super::types::ses::EventDestinationKinesisDestination>,
        >,
        /// A list of matching types. May be any of `"send"`, `"reject"`, `"bounce"`, `"complaint"`, `"delivery"`, `"open"`, `"click"`, or `"renderingFailure"`.
        pub matching_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the event destination
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Send the events to an SNS Topic destination
        ///
        /// > **NOTE:** You can specify `"cloudwatch_destination"` or `"kinesis_destination"` but not both
        pub sns_destination: pulumi_gestalt_rust::Output<
            Option<super::super::types::ses::EventDestinationSnsDestination>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventDestinationArgs,
    ) -> EventDestinationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cloudwatch_destinations_binding = args
            .cloudwatch_destinations
            .get_output(context);
        let configuration_set_name_binding = args
            .configuration_set_name
            .get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let kinesis_destination_binding = args.kinesis_destination.get_output(context);
        let matching_types_binding = args.matching_types.get_output(context);
        let name_binding = args.name.get_output(context);
        let sns_destination_binding = args.sns_destination.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ses/eventDestination:EventDestination".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudwatchDestinations".into(),
                    value: &cloudwatch_destinations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationSetName".into(),
                    value: &configuration_set_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kinesisDestination".into(),
                    value: &kinesis_destination_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "matchingTypes".into(),
                    value: &matching_types_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snsDestination".into(),
                    value: &sns_destination_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventDestinationResult {
            arn: o.get_field("arn"),
            cloudwatch_destinations: o.get_field("cloudwatchDestinations"),
            configuration_set_name: o.get_field("configurationSetName"),
            enabled: o.get_field("enabled"),
            kinesis_destination: o.get_field("kinesisDestination"),
            matching_types: o.get_field("matchingTypes"),
            name: o.get_field("name"),
            sns_destination: o.get_field("snsDestination"),
        }
    }
}
