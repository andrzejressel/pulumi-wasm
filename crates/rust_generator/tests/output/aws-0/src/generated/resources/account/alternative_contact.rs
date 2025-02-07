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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AlternativeContactArgs,
    ) -> AlternativeContactResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let alternate_contact_type_binding = args
            .alternate_contact_type
            .get_output(context)
            .get_inner();
        let email_address_binding = args.email_address.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let phone_number_binding = args.phone_number.get_output(context).get_inner();
        let title_binding = args.title.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:account/alternativeContact:AlternativeContact".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "alternateContactType".into(),
                    value: &alternate_contact_type_binding,
                },
                register_interface::ObjectField {
                    name: "emailAddress".into(),
                    value: &email_address_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "phoneNumber".into(),
                    value: &phone_number_binding,
                },
                register_interface::ObjectField {
                    name: "title".into(),
                    value: &title_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AlternativeContactResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            alternate_contact_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("alternateContactType"),
            ),
            email_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("emailAddress"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            phone_number: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("phoneNumber"),
            ),
            title: pulumi_gestalt_rust::__private::into_domain(o.extract_field("title")),
        }
    }
}
