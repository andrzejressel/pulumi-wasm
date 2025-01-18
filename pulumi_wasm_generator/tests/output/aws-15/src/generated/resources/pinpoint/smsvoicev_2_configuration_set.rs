/// Manages an AWS End User Messaging SMS Configuration Set.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod smsvoicev_2_configuration_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Smsvoicev2ConfigurationSetArgs {
        /// The default message type. Must either be "TRANSACTIONAL" or "PROMOTIONAL"
        #[builder(into, default)]
        pub default_message_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The default sender ID to use for this configuration set.
        #[builder(into, default)]
        pub default_sender_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the configuration set.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct Smsvoicev2ConfigurationSetResult {
        /// ARN of the configuration set.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The default message type. Must either be "TRANSACTIONAL" or "PROMOTIONAL"
        pub default_message_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The default sender ID to use for this configuration set.
        pub default_sender_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the configuration set.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: Smsvoicev2ConfigurationSetArgs,
    ) -> Smsvoicev2ConfigurationSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_message_type_binding = args.default_message_type.get_inner();
        let default_sender_id_binding = args.default_sender_id.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:pinpoint/smsvoicev2ConfigurationSet:Smsvoicev2ConfigurationSet"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultMessageType".into(),
                    value: &default_message_type_binding,
                },
                register_interface::ObjectField {
                    name: "defaultSenderId".into(),
                    value: &default_sender_id_binding,
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
                    name: "defaultMessageType".into(),
                },
                register_interface::ResultField {
                    name: "defaultSenderId".into(),
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        Smsvoicev2ConfigurationSetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            default_message_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultMessageType").unwrap(),
            ),
            default_sender_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSenderId").unwrap(),
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
