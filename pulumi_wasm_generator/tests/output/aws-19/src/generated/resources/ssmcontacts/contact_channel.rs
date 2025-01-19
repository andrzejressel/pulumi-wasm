/// Resource for managing an AWS SSM Contacts Contact Channel.
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
///     let example = contact_channel::create(
///         "example",
///         ContactChannelArgs::builder()
///             .contact_id(
///                 "arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias",
///             )
///             .delivery_address(
///                 ContactChannelDeliveryAddress::builder()
///                     .simpleAddress("email@example.com")
///                     .build_struct(),
///             )
///             .name("Example contact channel")
///             .type_("EMAIL")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Usage with SSM Contact
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = contact_channel::create(
///         "example",
///         ContactChannelArgs::builder()
///             .contact_id("${exampleContact.arn}")
///             .delivery_address(
///                 ContactChannelDeliveryAddress::builder()
///                     .simpleAddress("email@example.com")
///                     .build_struct(),
///             )
///             .name("Example contact channel")
///             .type_("EMAIL")
///             .build_struct(),
///     );
///     let exampleContact = contact::create(
///         "exampleContact",
///         ContactArgs::builder().alias("example_contact").type_("PERSONAL").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSM Contact Channel using the `ARN`. For example:
///
/// ```sh
/// $ pulumi import aws:ssmcontacts/contactChannel:ContactChannel example arn:aws:ssm-contacts:us-west-2:123456789012:contact-channel/example
/// ```
pub mod contact_channel {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContactChannelArgs {
        /// Amazon Resource Name (ARN) of the AWS SSM Contact that the contact channel belongs to.
        #[builder(into)]
        pub contact_id: pulumi_wasm_rust::Output<String>,
        /// Block that contains contact engagement details. See details below.
        #[builder(into)]
        pub delivery_address: pulumi_wasm_rust::Output<
            super::super::types::ssmcontacts::ContactChannelDeliveryAddress,
        >,
        /// Name of the contact channel. Must be between 1 and 255 characters, and may contain alphanumerics, underscores (`_`), hyphens (`-`), periods (`.`), and spaces.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of the contact channel. One of `SMS`, `VOICE` or `EMAIL`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ContactChannelResult {
        /// Whether the contact channel is activated. The contact channel must be activated to use it to engage the contact. One of `ACTIVATED` or `NOT_ACTIVATED`.
        pub activation_status: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the contact channel.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the AWS SSM Contact that the contact channel belongs to.
        pub contact_id: pulumi_wasm_rust::Output<String>,
        /// Block that contains contact engagement details. See details below.
        pub delivery_address: pulumi_wasm_rust::Output<
            super::super::types::ssmcontacts::ContactChannelDeliveryAddress,
        >,
        /// Name of the contact channel. Must be between 1 and 255 characters, and may contain alphanumerics, underscores (`_`), hyphens (`-`), periods (`.`), and spaces.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Type of the contact channel. One of `SMS`, `VOICE` or `EMAIL`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ContactChannelArgs) -> ContactChannelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let contact_id_binding = args.contact_id.get_inner();
        let delivery_address_binding = args.delivery_address.get_inner();
        let name_binding = args.name.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssmcontacts/contactChannel:ContactChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contactId".into(),
                    value: &contact_id_binding,
                },
                register_interface::ObjectField {
                    name: "deliveryAddress".into(),
                    value: &delivery_address_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "activationStatus".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "contactId".into(),
                },
                register_interface::ResultField {
                    name: "deliveryAddress".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ContactChannelResult {
            activation_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activationStatus").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            contact_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contactId").unwrap(),
            ),
            delivery_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deliveryAddress").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
