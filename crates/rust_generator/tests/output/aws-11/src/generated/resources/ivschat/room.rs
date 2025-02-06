/// Resource for managing an AWS IVS (Interactive Video) Chat Room.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = room::create(
///         "example",
///         RoomArgs::builder().name("tf-room").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IVS (Interactive Video) Chat Room using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:ivschat/room:Room example arn:aws:ivschat:us-west-2:326937407773:room/GoXEXyB4VwHb
/// ```
pub mod room {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RoomArgs {
        /// List of Logging Configuration
        /// ARNs to attach to the room.
        #[builder(into, default)]
        pub logging_configuration_identifiers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Maximum number of characters in a single
        /// message. Messages are expected to be UTF-8 encoded and this limit applies
        /// specifically to rune/code-point count, not number of bytes.
        #[builder(into, default)]
        pub maximum_message_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Maximum number of messages per
        /// second that can be sent to the room (by all clients).
        #[builder(into, default)]
        pub maximum_message_rate_per_second: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Configuration information for optional
        /// review of messages.
        #[builder(into, default)]
        pub message_review_handler: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ivschat::RoomMessageReviewHandler>,
        >,
        /// Room name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RoomResult {
        /// ARN of the Room.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// List of Logging Configuration
        /// ARNs to attach to the room.
        pub logging_configuration_identifiers: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// Maximum number of characters in a single
        /// message. Messages are expected to be UTF-8 encoded and this limit applies
        /// specifically to rune/code-point count, not number of bytes.
        pub maximum_message_length: pulumi_gestalt_rust::Output<i32>,
        /// Maximum number of messages per
        /// second that can be sent to the room (by all clients).
        pub maximum_message_rate_per_second: pulumi_gestalt_rust::Output<i32>,
        /// Configuration information for optional
        /// review of messages.
        pub message_review_handler: pulumi_gestalt_rust::Output<
            Option<super::super::types::ivschat::RoomMessageReviewHandler>,
        >,
        /// Room name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RoomArgs,
    ) -> RoomResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let logging_configuration_identifiers_binding = args
            .logging_configuration_identifiers
            .get_output(context)
            .get_inner();
        let maximum_message_length_binding = args
            .maximum_message_length
            .get_output(context)
            .get_inner();
        let maximum_message_rate_per_second_binding = args
            .maximum_message_rate_per_second
            .get_output(context)
            .get_inner();
        let message_review_handler_binding = args
            .message_review_handler
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ivschat/room:Room".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "loggingConfigurationIdentifiers".into(),
                    value: &logging_configuration_identifiers_binding,
                },
                register_interface::ObjectField {
                    name: "maximumMessageLength".into(),
                    value: &maximum_message_length_binding,
                },
                register_interface::ObjectField {
                    name: "maximumMessageRatePerSecond".into(),
                    value: &maximum_message_rate_per_second_binding,
                },
                register_interface::ObjectField {
                    name: "messageReviewHandler".into(),
                    value: &message_review_handler_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        RoomResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            logging_configuration_identifiers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loggingConfigurationIdentifiers"),
            ),
            maximum_message_length: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maximumMessageLength"),
            ),
            maximum_message_rate_per_second: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maximumMessageRatePerSecond"),
            ),
            message_review_handler: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("messageReviewHandler"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
