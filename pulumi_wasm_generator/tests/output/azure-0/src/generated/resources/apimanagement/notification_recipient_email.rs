/// Manages a API Management Notification Recipient Email.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleNotificationRecipientEmail = notification_recipient_email::create(
///         "exampleNotificationRecipientEmail",
///         NotificationRecipientEmailArgs::builder()
///             .api_management_id("${exampleService.id}")
///             .email("foo@bar.com")
///             .notification_type("AccountClosedPublisher")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-apim")
///             .publisher_email("company@terraform.io")
///             .publisher_name("My Company")
///             .resource_group_name("${example.name}")
///             .sku_name("Developer_1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// API Management Notification Recipient Emails can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/notificationRecipientEmail:NotificationRecipientEmail example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/notifications/notificationName1/recipientEmails/email1
/// ```
///
pub mod notification_recipient_email {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotificationRecipientEmailArgs {
        /// The ID of the API Management Service from which to create this Notification Recipient Email. Changing this forces a new API Management Notification Recipient Email to be created.
        #[builder(into)]
        pub api_management_id: pulumi_wasm_rust::Output<String>,
        /// The recipient email address. Changing this forces a new API Management Notification Recipient Email to be created.
        #[builder(into)]
        pub email: pulumi_wasm_rust::Output<String>,
        /// The Notification Name to be received. Changing this forces a new API Management Notification Recipient Email to be created. Possible values are `AccountClosedPublisher`, `BCC`, `NewApplicationNotificationMessage`, `NewIssuePublisherNotificationMessage`, `PurchasePublisherNotificationMessage`, `QuotaLimitApproachingPublisherNotificationMessage`, and `RequestPublisherNotificationMessage`.
        #[builder(into)]
        pub notification_type: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NotificationRecipientEmailResult {
        /// The ID of the API Management Service from which to create this Notification Recipient Email. Changing this forces a new API Management Notification Recipient Email to be created.
        pub api_management_id: pulumi_wasm_rust::Output<String>,
        /// The recipient email address. Changing this forces a new API Management Notification Recipient Email to be created.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The Notification Name to be received. Changing this forces a new API Management Notification Recipient Email to be created. Possible values are `AccountClosedPublisher`, `BCC`, `NewApplicationNotificationMessage`, `NewIssuePublisherNotificationMessage`, `PurchasePublisherNotificationMessage`, `QuotaLimitApproachingPublisherNotificationMessage`, and `RequestPublisherNotificationMessage`.
        pub notification_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NotificationRecipientEmailArgs,
    ) -> NotificationRecipientEmailResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_id_binding = args.api_management_id.get_inner();
        let email_binding = args.email.get_inner();
        let notification_type_binding = args.notification_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/notificationRecipientEmail:NotificationRecipientEmail"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementId".into(),
                    value: &api_management_id_binding,
                },
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
                register_interface::ObjectField {
                    name: "notificationType".into(),
                    value: &notification_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiManagementId".into(),
                },
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "notificationType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NotificationRecipientEmailResult {
            api_management_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementId").unwrap(),
            ),
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            notification_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationType").unwrap(),
            ),
        }
    }
}
