/// Manages an AWS End User Messaging SMS Configuration Set.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = smsvoicev_2_configuration_set::create(
///         "example",
///         Smsvoicev2ConfigurationSetArgs::builder()
///             .default_message_type("TRANSACTIONAL")
///             .default_sender_id("example")
///             .name("example-configuration-set")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import configuration sets using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:pinpoint/smsvoicev2ConfigurationSet:Smsvoicev2ConfigurationSet example example-configuration-set
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod smsvoicev_2_configuration_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Smsvoicev2ConfigurationSetArgs {
        /// The default message type. Must either be "TRANSACTIONAL" or "PROMOTIONAL"
        #[builder(into, default)]
        pub default_message_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The default sender ID to use for this configuration set.
        #[builder(into, default)]
        pub default_sender_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the configuration set.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct Smsvoicev2ConfigurationSetResult {
        /// ARN of the configuration set.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The default message type. Must either be "TRANSACTIONAL" or "PROMOTIONAL"
        pub default_message_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The default sender ID to use for this configuration set.
        pub default_sender_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the configuration set.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: Smsvoicev2ConfigurationSetArgs,
    ) -> Smsvoicev2ConfigurationSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let default_message_type_binding = args.default_message_type.get_output(context);
        let default_sender_id_binding = args.default_sender_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:pinpoint/smsvoicev2ConfigurationSet:Smsvoicev2ConfigurationSet"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultMessageType".into(),
                    value: &default_message_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultSenderId".into(),
                    value: &default_sender_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        Smsvoicev2ConfigurationSetResult {
            arn: o.get_field("arn"),
            default_message_type: o.get_field("defaultMessageType"),
            default_sender_id: o.get_field("defaultSenderId"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
