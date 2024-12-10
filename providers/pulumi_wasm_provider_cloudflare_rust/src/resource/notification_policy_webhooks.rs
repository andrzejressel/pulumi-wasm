//! Provides a resource, that manages a webhook destination. These destinations can be tied to the notification policies created for Cloudflare's products.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = notification_policy_webhooks::create(
//!         "example",
//!         NotificationPolicyWebhooksArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .name("Webhooks destination")
//!             .secret("my-secret")
//!             .url("https://example.com")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/notificationPolicyWebhooks:NotificationPolicyWebhooks example <account_id>/<notification_webhook_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct NotificationPolicyWebhooksArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the webhook destination.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// An optional secret can be provided that will be passed in the `cf-webhook-auth` header when dispatching a webhook notification. Secrets are not returned in any API response body. Refer to the [documentation](https://api.cloudflare.com/#notification-webhooks-create-webhook) for more details.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub secret: pulumi_wasm_rust::Output<Option<String>>,
    /// The URL of the webhook destinations. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub url: pulumi_wasm_rust::Output<Option<String>>,
}

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
pub fn create(name: &str, args: NotificationPolicyWebhooksArgs) -> NotificationPolicyWebhooksResult {

    let result = crate::bindings::pulumi::cloudflare::notification_policy_webhooks::invoke(name, &crate::bindings::pulumi::cloudflare::notification_policy_webhooks::Args {
        account_id: &args.account_id.get_inner(),
        name: &args.name.get_inner(),
        secret: &args.secret.get_inner(),
        url: &args.url.get_inner(),
    });

    NotificationPolicyWebhooksResult {
        account_id: crate::into_domain(result.account_id),
        created_at: crate::into_domain(result.created_at),
        last_failure: crate::into_domain(result.last_failure),
        last_success: crate::into_domain(result.last_success),
        name: crate::into_domain(result.name),
        secret: crate::into_domain(result.secret),
        type_: crate::into_domain(result.type_),
        url: crate::into_domain(result.url),
    }
}

