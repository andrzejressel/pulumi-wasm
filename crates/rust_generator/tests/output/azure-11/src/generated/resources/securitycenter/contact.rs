/// Manages the subscription's Security Center Contact.
///
/// > **NOTE:** Owner access permission is required.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod contact {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContactArgs {
        /// Whether to send security alerts notifications to the security contact.
        #[builder(into)]
        pub alert_notifications: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Whether to send security alerts notifications to subscription admins.
        #[builder(into)]
        pub alerts_to_admins: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The email of the Security Center Contact.
        #[builder(into)]
        pub email: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Security Center Contact. Defaults to `default1`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The phone number of the Security Center Contact.
        #[builder(into, default)]
        pub phone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ContactResult {
        /// Whether to send security alerts notifications to the security contact.
        pub alert_notifications: pulumi_gestalt_rust::Output<bool>,
        /// Whether to send security alerts notifications to subscription admins.
        pub alerts_to_admins: pulumi_gestalt_rust::Output<bool>,
        /// The email of the Security Center Contact.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The name of the Security Center Contact. Defaults to `default1`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The phone number of the Security Center Contact.
        pub phone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ContactArgs,
    ) -> ContactResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alert_notifications_binding = args.alert_notifications.get_output(context);
        let alerts_to_admins_binding = args.alerts_to_admins.get_output(context);
        let email_binding = args.email.get_output(context);
        let name_binding = args.name.get_output(context);
        let phone_binding = args.phone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:securitycenter/contact:Contact".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alertNotifications".into(),
                    value: alert_notifications_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alertsToAdmins".into(),
                    value: alerts_to_admins_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "email".into(),
                    value: email_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "phone".into(),
                    value: phone_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ContactResult {
            alert_notifications: o.get_field("alertNotifications"),
            alerts_to_admins: o.get_field("alertsToAdmins"),
            email: o.get_field("email"),
            name: o.get_field("name"),
            phone: o.get_field("phone"),
        }
    }
}
