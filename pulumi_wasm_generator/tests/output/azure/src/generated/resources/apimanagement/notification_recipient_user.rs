/// Manages a API Management Notification Recipient User.
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
///     let exampleNotificationRecipientUser = notification_recipient_user::create(
///         "exampleNotificationRecipientUser",
///         NotificationRecipientUserArgs::builder()
///             .api_management_id("${exampleService.id}")
///             .notification_type("AccountClosedPublisher")
///             .user_id("${exampleUser.userId}")
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
///     let exampleUser = user::create(
///         "exampleUser",
///         UserArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .email("foo@bar.com")
///             .first_name("Example")
///             .last_name("User")
///             .resource_group_name("${example.name}")
///             .state("active")
///             .user_id("123")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// API Management Notification Recipient Users can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/notificationRecipientUser:NotificationRecipientUser example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/notifications/notificationName1/recipientUsers/userid1
/// ```
///
pub mod notification_recipient_user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotificationRecipientUserArgs {
        /// The ID of the API Management Service from which to create this Notification Recipient User. Changing this forces a new API Management Notification Recipient User to be created.
        #[builder(into)]
        pub api_management_id: pulumi_wasm_rust::Output<String>,
        /// The Notification Name to be received. Changing this forces a new API Management Notification Recipient User to be created. Possible values are `AccountClosedPublisher`, `BCC`, `NewApplicationNotificationMessage`, `NewIssuePublisherNotificationMessage`, `PurchasePublisherNotificationMessage`, `QuotaLimitApproachingPublisherNotificationMessage`, and `RequestPublisherNotificationMessage`.
        #[builder(into)]
        pub notification_type: pulumi_wasm_rust::Output<String>,
        /// The recipient user ID. Changing this forces a new API Management Notification Recipient User to be created.
        #[builder(into)]
        pub user_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NotificationRecipientUserResult {
        /// The ID of the API Management Service from which to create this Notification Recipient User. Changing this forces a new API Management Notification Recipient User to be created.
        pub api_management_id: pulumi_wasm_rust::Output<String>,
        /// The Notification Name to be received. Changing this forces a new API Management Notification Recipient User to be created. Possible values are `AccountClosedPublisher`, `BCC`, `NewApplicationNotificationMessage`, `NewIssuePublisherNotificationMessage`, `PurchasePublisherNotificationMessage`, `QuotaLimitApproachingPublisherNotificationMessage`, and `RequestPublisherNotificationMessage`.
        pub notification_type: pulumi_wasm_rust::Output<String>,
        /// The recipient user ID. Changing this forces a new API Management Notification Recipient User to be created.
        pub user_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NotificationRecipientUserArgs,
    ) -> NotificationRecipientUserResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_id_binding = args.api_management_id.get_inner();
        let notification_type_binding = args.notification_type.get_inner();
        let user_id_binding = args.user_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/notificationRecipientUser:NotificationRecipientUser"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementId".into(),
                    value: &api_management_id_binding,
                },
                register_interface::ObjectField {
                    name: "notificationType".into(),
                    value: &notification_type_binding,
                },
                register_interface::ObjectField {
                    name: "userId".into(),
                    value: &user_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiManagementId".into(),
                },
                register_interface::ResultField {
                    name: "notificationType".into(),
                },
                register_interface::ResultField {
                    name: "userId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NotificationRecipientUserResult {
            api_management_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementId").unwrap(),
            ),
            notification_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationType").unwrap(),
            ),
            user_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userId").unwrap(),
            ),
        }
    }
}