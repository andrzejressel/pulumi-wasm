/// Manages the specified alternate contact attached to an AWS Account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlternativeContactArgs {
        /// ID of the target account when managing member accounts. Will manage current user's account by default if omitted.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of the alternate contact. Allowed values are: `BILLING`, `OPERATIONS`, `SECURITY`.
        #[builder(into)]
        pub alternate_contact_type: pulumi_wasm_rust::Output<String>,
        /// An email address for the alternate contact.
        #[builder(into)]
        pub email_address: pulumi_wasm_rust::Output<String>,
        /// Name of the alternate contact.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Phone number for the alternate contact.
        #[builder(into)]
        pub phone_number: pulumi_wasm_rust::Output<String>,
        /// Title for the alternate contact.
        #[builder(into)]
        pub title: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AlternativeContactResult {
        /// ID of the target account when managing member accounts. Will manage current user's account by default if omitted.
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of the alternate contact. Allowed values are: `BILLING`, `OPERATIONS`, `SECURITY`.
        pub alternate_contact_type: pulumi_wasm_rust::Output<String>,
        /// An email address for the alternate contact.
        pub email_address: pulumi_wasm_rust::Output<String>,
        /// Name of the alternate contact.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Phone number for the alternate contact.
        pub phone_number: pulumi_wasm_rust::Output<String>,
        /// Title for the alternate contact.
        pub title: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AlternativeContactArgs) -> AlternativeContactResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let alternate_contact_type_binding = args.alternate_contact_type.get_inner();
        let email_address_binding = args.email_address.get_inner();
        let name_binding = args.name.get_inner();
        let phone_number_binding = args.phone_number.get_inner();
        let title_binding = args.title.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:account/alternativeContact:AlternativeContact".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "alternateContactType".into(),
                },
                register_interface::ResultField {
                    name: "emailAddress".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "phoneNumber".into(),
                },
                register_interface::ResultField {
                    name: "title".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AlternativeContactResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            alternate_contact_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alternateContactType").unwrap(),
            ),
            email_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailAddress").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            phone_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("phoneNumber").unwrap(),
            ),
            title: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("title").unwrap(),
            ),
        }
    }
}
