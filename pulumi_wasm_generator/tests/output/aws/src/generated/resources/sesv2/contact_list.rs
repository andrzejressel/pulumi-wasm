/// Resource for managing an AWS SESv2 (Simple Email V2) Contact List.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod contact_list {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContactListArgs {
        /// Name of the contact list.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub contact_list_name: pulumi_wasm_rust::Output<String>,
        /// Description of what the contact list is about.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags for the contact list. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block(s) with topic for the contact list. Detailed below.
        #[builder(into, default)]
        pub topics: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::sesv2::ContactListTopic>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ContactListResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of the contact list.
        ///
        /// The following arguments are optional:
        pub contact_list_name: pulumi_wasm_rust::Output<String>,
        /// Timestamp noting when the contact list was created in ISO 8601 format.
        pub created_timestamp: pulumi_wasm_rust::Output<String>,
        /// Description of what the contact list is about.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Timestamp noting the last time the contact list was updated in ISO 8601 format.
        pub last_updated_timestamp: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the contact list. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block(s) with topic for the contact list. Detailed below.
        pub topics: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::sesv2::ContactListTopic>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ContactListArgs) -> ContactListResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let contact_list_name_binding = args.contact_list_name.get_inner();
        let description_binding = args.description.get_inner();
        let tags_binding = args.tags.get_inner();
        let topics_binding = args.topics.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sesv2/contactList:ContactList".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contactListName".into(),
                    value: &contact_list_name_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "topics".into(),
                    value: &topics_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "contactListName".into(),
                },
                register_interface::ResultField {
                    name: "createdTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "topics".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ContactListResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            contact_list_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contactListName").unwrap(),
            ),
            created_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            last_updated_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedTimestamp").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            topics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("topics").unwrap(),
            ),
        }
    }
}