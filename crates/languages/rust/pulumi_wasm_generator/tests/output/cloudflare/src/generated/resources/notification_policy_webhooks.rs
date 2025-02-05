/// Provides a resource, that manages a webhook destination. These destinations can be tied to the notification policies created for Cloudflare's products.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod notification_policy_webhooks {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotificationPolicyWebhooksArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the webhook destination.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// An optional secret can be provided that will be passed in the `cf-webhook-auth` header when dispatching a webhook notification. Secrets are not returned in any API response body. Refer to the [documentation](https://api.cloudflare.com/#notification-webhooks-create-webhook) for more details.
        #[builder(into, default)]
        pub secret: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The URL of the webhook destinations. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub url: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NotificationPolicyWebhooksResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Timestamp of when the notification webhook was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Timestamp of when the notification webhook last failed.
        pub last_failure: pulumi_wasm_rust::Output<String>,
        /// Timestamp of when the notification webhook was last successful.
        pub last_success: pulumi_wasm_rust::Output<String>,
        /// The name of the webhook destination.
        pub name: pulumi_wasm_rust::Output<String>,
        /// An optional secret can be provided that will be passed in the `cf-webhook-auth` header when dispatching a webhook notification. Secrets are not returned in any API response body. Refer to the [documentation](https://api.cloudflare.com/#notification-webhooks-create-webhook) for more details.
        pub secret: pulumi_wasm_rust::Output<Option<String>>,
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The URL of the webhook destinations. **Modifying this attribute will force creation of a new resource.**
        pub url: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NotificationPolicyWebhooksArgs,
    ) -> NotificationPolicyWebhooksResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let secret_binding = args.secret.get_output(context).get_inner();
        let url_binding = args.url.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/notificationPolicyWebhooks:NotificationPolicyWebhooks"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "secret".into(),
                    value: &secret_binding,
                },
                register_interface::ObjectField {
                    name: "url".into(),
                    value: &url_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NotificationPolicyWebhooksResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            last_failure: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastFailure"),
            ),
            last_success: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastSuccess"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            secret: pulumi_wasm_rust::__private::into_domain(o.extract_field("secret")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            url: pulumi_wasm_rust::__private::into_domain(o.extract_field("url")),
        }
    }
}
