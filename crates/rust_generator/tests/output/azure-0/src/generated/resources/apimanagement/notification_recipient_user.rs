/// Manages a API Management Notification Recipient User.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod notification_recipient_user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotificationRecipientUserArgs {
        /// The ID of the API Management Service from which to create this Notification Recipient User. Changing this forces a new API Management Notification Recipient User to be created.
        #[builder(into)]
        pub api_management_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Notification Name to be received. Changing this forces a new API Management Notification Recipient User to be created. Possible values are `AccountClosedPublisher`, `BCC`, `NewApplicationNotificationMessage`, `NewIssuePublisherNotificationMessage`, `PurchasePublisherNotificationMessage`, `QuotaLimitApproachingPublisherNotificationMessage`, and `RequestPublisherNotificationMessage`.
        #[builder(into)]
        pub notification_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The recipient user ID. Changing this forces a new API Management Notification Recipient User to be created.
        #[builder(into)]
        pub user_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NotificationRecipientUserResult {
        /// The ID of the API Management Service from which to create this Notification Recipient User. Changing this forces a new API Management Notification Recipient User to be created.
        pub api_management_id: pulumi_gestalt_rust::Output<String>,
        /// The Notification Name to be received. Changing this forces a new API Management Notification Recipient User to be created. Possible values are `AccountClosedPublisher`, `BCC`, `NewApplicationNotificationMessage`, `NewIssuePublisherNotificationMessage`, `PurchasePublisherNotificationMessage`, `QuotaLimitApproachingPublisherNotificationMessage`, and `RequestPublisherNotificationMessage`.
        pub notification_type: pulumi_gestalt_rust::Output<String>,
        /// The recipient user ID. Changing this forces a new API Management Notification Recipient User to be created.
        pub user_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NotificationRecipientUserArgs,
    ) -> NotificationRecipientUserResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_id_binding = args.api_management_id.get_output(context);
        let notification_type_binding = args.notification_type.get_output(context);
        let user_id_binding = args.user_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/notificationRecipientUser:NotificationRecipientUser"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementId".into(),
                    value: api_management_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationType".into(),
                    value: notification_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userId".into(),
                    value: user_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NotificationRecipientUserResult {
            api_management_id: o.get_field("apiManagementId"),
            notification_type: o.get_field("notificationType"),
            user_id: o.get_field("userId"),
        }
    }
}
