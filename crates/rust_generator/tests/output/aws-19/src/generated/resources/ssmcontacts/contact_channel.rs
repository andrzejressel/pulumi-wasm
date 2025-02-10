/// Resource for managing an AWS SSM Contacts Contact Channel.
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod contact_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContactChannelArgs {
        /// Amazon Resource Name (ARN) of the AWS SSM Contact that the contact channel belongs to.
        #[builder(into)]
        pub contact_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Block that contains contact engagement details. See details below.
        #[builder(into)]
        pub delivery_address: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::ssmcontacts::ContactChannelDeliveryAddress,
        >,
        /// Name of the contact channel. Must be between 1 and 255 characters, and may contain alphanumerics, underscores (`_`), hyphens (`-`), periods (`.`), and spaces.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of the contact channel. One of `SMS`, `VOICE` or `EMAIL`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ContactChannelResult {
        /// Whether the contact channel is activated. The contact channel must be activated to use it to engage the contact. One of `ACTIVATED` or `NOT_ACTIVATED`.
        pub activation_status: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the contact channel.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the AWS SSM Contact that the contact channel belongs to.
        pub contact_id: pulumi_gestalt_rust::Output<String>,
        /// Block that contains contact engagement details. See details below.
        pub delivery_address: pulumi_gestalt_rust::Output<
            super::super::types::ssmcontacts::ContactChannelDeliveryAddress,
        >,
        /// Name of the contact channel. Must be between 1 and 255 characters, and may contain alphanumerics, underscores (`_`), hyphens (`-`), periods (`.`), and spaces.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Type of the contact channel. One of `SMS`, `VOICE` or `EMAIL`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ContactChannelArgs,
    ) -> ContactChannelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let contact_id_binding = args.contact_id.get_output(context);
        let delivery_address_binding = args.delivery_address.get_output(context);
        let name_binding = args.name.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssmcontacts/contactChannel:ContactChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contactId".into(),
                    value: contact_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deliveryAddress".into(),
                    value: delivery_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ContactChannelResult {
            activation_status: o.get_field("activationStatus"),
            arn: o.get_field("arn"),
            contact_id: o.get_field("contactId"),
            delivery_address: o.get_field("deliveryAddress"),
            name: o.get_field("name"),
            type_: o.get_field("type"),
        }
    }
}
