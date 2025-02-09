/// Manages the specified alternate contact attached to an AWS Account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let operations = alternative_contact::create(
///         "operations",
///         AlternativeContactArgs::builder()
///             .alternate_contact_type("OPERATIONS")
///             .email_address("test@example.com")
///             .name("Example")
///             .phone_number("+1234567890")
///             .title("Example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Import the Alternate Contact for another account using the `account_id` and `alternate_contact_type` separated by a forward slash (`/`):
///
/// __Using `pulumi import` to import__ the Alternate Contact for the current or another account using the `alternate_contact_type`. For example:
///
/// Import the Alternate Contact for the current account:
///
/// ```sh
/// $ pulumi import aws:account/alternativeContact:AlternativeContact operations OPERATIONS
/// ```
/// Import the Alternate Contact for another account using the `account_id` and `alternate_contact_type` separated by a forward slash (`/`):
///
/// ```sh
/// $ pulumi import aws:account/alternativeContact:AlternativeContact operations 1234567890/OPERATIONS
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod alternative_contact {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlternativeContactArgs {
        /// ID of the target account when managing member accounts. Will manage current user's account by default if omitted.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of the alternate contact. Allowed values are: `BILLING`, `OPERATIONS`, `SECURITY`.
        #[builder(into)]
        pub alternate_contact_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An email address for the alternate contact.
        #[builder(into)]
        pub email_address: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the alternate contact.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Phone number for the alternate contact.
        #[builder(into)]
        pub phone_number: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Title for the alternate contact.
        #[builder(into)]
        pub title: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AlternativeContactResult {
        /// ID of the target account when managing member accounts. Will manage current user's account by default if omitted.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Type of the alternate contact. Allowed values are: `BILLING`, `OPERATIONS`, `SECURITY`.
        pub alternate_contact_type: pulumi_gestalt_rust::Output<String>,
        /// An email address for the alternate contact.
        pub email_address: pulumi_gestalt_rust::Output<String>,
        /// Name of the alternate contact.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Phone number for the alternate contact.
        pub phone_number: pulumi_gestalt_rust::Output<String>,
        /// Title for the alternate contact.
        pub title: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AlternativeContactArgs,
    ) -> AlternativeContactResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let alternate_contact_type_binding = args
            .alternate_contact_type
            .get_output(context);
        let email_address_binding = args.email_address.get_output(context);
        let name_binding = args.name.get_output(context);
        let phone_number_binding = args.phone_number.get_output(context);
        let title_binding = args.title.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:account/alternativeContact:AlternativeContact".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alternateContactType".into(),
                    value: alternate_contact_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailAddress".into(),
                    value: email_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "phoneNumber".into(),
                    value: phone_number_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "title".into(),
                    value: title_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AlternativeContactResult {
            account_id: o.get_field("accountId"),
            alternate_contact_type: o.get_field("alternateContactType"),
            email_address: o.get_field("emailAddress"),
            name: o.get_field("name"),
            phone_number: o.get_field("phoneNumber"),
            title: o.get_field("title"),
        }
    }
}
