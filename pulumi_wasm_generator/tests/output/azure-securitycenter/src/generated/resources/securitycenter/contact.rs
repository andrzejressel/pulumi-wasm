/// Manages the subscription's Security Center Contact.
///
/// > **NOTE:** Owner access permission is required.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = contact::create(
///         "example",
///         ContactArgs::builder()
///             .alert_notifications(true)
///             .alerts_to_admins(true)
///             .email("contact@example.com")
///             .name("contact")
///             .phone("+1-555-555-5555")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// The contact can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:securitycenter/contact:Contact example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Security/securityContacts/default1
/// ```
///
pub mod contact {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContactArgs {
        /// Whether to send security alerts notifications to the security contact.
        #[builder(into)]
        pub alert_notifications: pulumi_wasm_rust::Output<bool>,
        /// Whether to send security alerts notifications to subscription admins.
        #[builder(into)]
        pub alerts_to_admins: pulumi_wasm_rust::Output<bool>,
        /// The email of the Security Center Contact.
        #[builder(into)]
        pub email: pulumi_wasm_rust::Output<String>,
        /// The name of the Security Center Contact. Defaults to `default1`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The phone number of the Security Center Contact.
        #[builder(into, default)]
        pub phone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ContactResult {
        /// Whether to send security alerts notifications to the security contact.
        pub alert_notifications: pulumi_wasm_rust::Output<bool>,
        /// Whether to send security alerts notifications to subscription admins.
        pub alerts_to_admins: pulumi_wasm_rust::Output<bool>,
        /// The email of the Security Center Contact.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The name of the Security Center Contact. Defaults to `default1`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The phone number of the Security Center Contact.
        pub phone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ContactArgs) -> ContactResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alert_notifications_binding = args.alert_notifications.get_inner();
        let alerts_to_admins_binding = args.alerts_to_admins.get_inner();
        let email_binding = args.email.get_inner();
        let name_binding = args.name.get_inner();
        let phone_binding = args.phone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:securitycenter/contact:Contact".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alertNotifications".into(),
                    value: &alert_notifications_binding,
                },
                register_interface::ObjectField {
                    name: "alertsToAdmins".into(),
                    value: &alerts_to_admins_binding,
                },
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "phone".into(),
                    value: &phone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alertNotifications".into(),
                },
                register_interface::ResultField {
                    name: "alertsToAdmins".into(),
                },
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "phone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ContactResult {
            alert_notifications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alertNotifications").unwrap(),
            ),
            alerts_to_admins: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alertsToAdmins").unwrap(),
            ),
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            phone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("phone").unwrap(),
            ),
        }
    }
}