/// Resource for managing an AWS SESv2 (Simple Email V2) Contact List.
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
///     let example = contact_list::create(
///         "example",
///         ContactListArgs::builder().contact_list_name("example").build_struct(),
///     );
/// }
/// ```
///
/// ### Extended Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = contact_list::create(
///         "example",
///         ContactListArgs::builder()
///             .contact_list_name("example")
///             .description("description")
///             .topics(
///                 vec![
///                     ContactListTopic::builder().defaultSubscriptionStatus("OPT_IN")
///                     .description("topic description").displayName("Example Topic")
///                     .topicName("example-topic").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SESv2 (Simple Email V2) Contact List using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:sesv2/contactList:ContactList example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod contact_list {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContactListArgs {
        /// Name of the contact list.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub contact_list_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of what the contact list is about.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags for the contact list. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block(s) with topic for the contact list. Detailed below.
        #[builder(into, default)]
        pub topics: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::sesv2::ContactListTopic>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ContactListResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the contact list.
        ///
        /// The following arguments are optional:
        pub contact_list_name: pulumi_gestalt_rust::Output<String>,
        /// Timestamp noting when the contact list was created in ISO 8601 format.
        pub created_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Description of what the contact list is about.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Timestamp noting the last time the contact list was updated in ISO 8601 format.
        pub last_updated_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the contact list. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block(s) with topic for the contact list. Detailed below.
        pub topics: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::sesv2::ContactListTopic>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ContactListArgs,
    ) -> ContactListResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let contact_list_name_binding = args.contact_list_name.get_output(context);
        let description_binding = args.description.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let topics_binding = args.topics.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sesv2/contactList:ContactList".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contactListName".into(),
                    value: contact_list_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "topics".into(),
                    value: topics_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ContactListResult {
            arn: o.get_field("arn"),
            contact_list_name: o.get_field("contactListName"),
            created_timestamp: o.get_field("createdTimestamp"),
            description: o.get_field("description"),
            last_updated_timestamp: o.get_field("lastUpdatedTimestamp"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            topics: o.get_field("topics"),
        }
    }
}
