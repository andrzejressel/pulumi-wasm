/// Provides an EventBridge event bus resource.
///
/// > **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod event_bus {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventBusArgs {
        /// Event bus description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Partner event source that the new event bus will be matched with. Must match `name`.
        #[builder(into, default)]
        pub event_source_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Identifier of the AWS KMS customer managed key for EventBridge to use, if you choose to use a customer managed key to encrypt events on this event bus. The identifier can be the key Amazon Resource Name (ARN), KeyId, key alias, or key alias ARN.
        #[builder(into, default)]
        pub kms_key_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the new event bus. The names of custom event buses can't contain the / character. To create a partner event bus, ensure that the `name` matches the `event_source_name`.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventBusResult {
        /// ARN of the event bus.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Event bus description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Partner event source that the new event bus will be matched with. Must match `name`.
        pub event_source_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of the AWS KMS customer managed key for EventBridge to use, if you choose to use a customer managed key to encrypt events on this event bus. The identifier can be the key Amazon Resource Name (ARN), KeyId, key alias, or key alias ARN.
        pub kms_key_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the new event bus. The names of custom event buses can't contain the / character. To create a partner event bus, ensure that the `name` matches the `event_source_name`.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EventBusArgs,
    ) -> EventBusResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let event_source_name_binding = args
            .event_source_name
            .get_output(context)
            .get_inner();
        let kms_key_identifier_binding = args
            .kms_key_identifier
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/eventBus:EventBus".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "eventSourceName".into(),
                    value: &event_source_name_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyIdentifier".into(),
                    value: &kms_key_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "eventSourceName".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EventBusResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            event_source_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventSourceName").unwrap(),
            ),
            kms_key_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyIdentifier").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
