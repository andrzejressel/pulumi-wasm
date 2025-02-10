/// Provides an EventBridge event bus resource.
///
/// > **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let messenger = event_bus::create(
///         "messenger",
///         EventBusArgs::builder().name("chat-messages").build_struct(),
///     );
/// }
/// ```
///
/// ```yaml
/// resources:
///   examplepartnerEventBus:
///     type: aws:cloudwatch:EventBus
///     name: examplepartner
///     properties:
///       name: ${examplepartner.name}
///       description: Event bus for example partner events
///       eventSourceName: ${examplepartner.name}
/// variables:
///   examplepartner:
///     fn::invoke:
///       function: aws:cloudwatch:getEventSource
///       arguments:
///         namePrefix: aws.partner/examplepartner.com
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EventBridge event buses using the name of the event bus (which can also be a partner event source name). For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/eventBus:EventBus messenger chat-messages
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_bus {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventBusArgs {
        /// Event bus description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Partner event source that the new event bus will be matched with. Must match `name`.
        #[builder(into, default)]
        pub event_source_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the AWS KMS customer managed key for EventBridge to use, if you choose to use a customer managed key to encrypt events on this event bus. The identifier can be the key Amazon Resource Name (ARN), KeyId, key alias, or key alias ARN.
        #[builder(into, default)]
        pub kms_key_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the new event bus. The names of custom event buses can't contain the / character. To create a partner event bus, ensure that the `name` matches the `event_source_name`.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventBusResult {
        /// ARN of the event bus.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Event bus description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Partner event source that the new event bus will be matched with. Must match `name`.
        pub event_source_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of the AWS KMS customer managed key for EventBridge to use, if you choose to use a customer managed key to encrypt events on this event bus. The identifier can be the key Amazon Resource Name (ARN), KeyId, key alias, or key alias ARN.
        pub kms_key_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the new event bus. The names of custom event buses can't contain the / character. To create a partner event bus, ensure that the `name` matches the `event_source_name`.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: EventBusArgs,
    ) -> EventBusResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let event_source_name_binding = args.event_source_name.get_output(context);
        let kms_key_identifier_binding = args.kms_key_identifier.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/eventBus:EventBus".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventSourceName".into(),
                    value: event_source_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyIdentifier".into(),
                    value: kms_key_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventBusResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            event_source_name: o.get_field("eventSourceName"),
            kms_key_identifier: o.get_field("kmsKeyIdentifier"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
