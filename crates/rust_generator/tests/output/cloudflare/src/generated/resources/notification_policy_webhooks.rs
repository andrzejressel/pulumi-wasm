/// Provides a resource, that manages a webhook destination. These destinations can be tied to the notification policies created for Cloudflare's products.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = notification_policy_webhooks::create(
///         "example",
///         NotificationPolicyWebhooksArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .name("Webhooks destination")
///             .secret("my-secret")
///             .url("https://example.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/notificationPolicyWebhooks:NotificationPolicyWebhooks example <account_id>/<notification_webhook_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod notification_policy_webhooks {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotificationPolicyWebhooksArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the webhook destination.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An optional secret can be provided that will be passed in the `cf-webhook-auth` header when dispatching a webhook notification. Secrets are not returned in any API response body. Refer to the [documentation](https://api.cloudflare.com/#notification-webhooks-create-webhook) for more details.
        #[builder(into, default)]
        pub secret: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URL of the webhook destinations. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NotificationPolicyWebhooksResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Timestamp of when the notification webhook was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Timestamp of when the notification webhook last failed.
        pub last_failure: pulumi_gestalt_rust::Output<String>,
        /// Timestamp of when the notification webhook was last successful.
        pub last_success: pulumi_gestalt_rust::Output<String>,
        /// The name of the webhook destination.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An optional secret can be provided that will be passed in the `cf-webhook-auth` header when dispatching a webhook notification. Secrets are not returned in any API response body. Refer to the [documentation](https://api.cloudflare.com/#notification-webhooks-create-webhook) for more details.
        pub secret: pulumi_gestalt_rust::Output<Option<String>>,
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The URL of the webhook destinations. **Modifying this attribute will force creation of a new resource.**
        pub url: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NotificationPolicyWebhooksArgs,
    ) -> NotificationPolicyWebhooksResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let secret_binding = args.secret.get_output(context);
        let url_binding = args.url.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/notificationPolicyWebhooks:NotificationPolicyWebhooks"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secret".into(),
                    value: secret_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "url".into(),
                    value: url_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NotificationPolicyWebhooksResult {
            account_id: o.get_field("accountId"),
            created_at: o.get_field("createdAt"),
            last_failure: o.get_field("lastFailure"),
            last_success: o.get_field("lastSuccess"),
            name: o.get_field("name"),
            secret: o.get_field("secret"),
            type_: o.get_field("type"),
            url: o.get_field("url"),
        }
    }
}
