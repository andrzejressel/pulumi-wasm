//! Provides a resource, that manages a webhook destination. These destinations can be tied to the notification policies created for Cloudflare's products.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.NotificationPolicyWebhooks("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "Webhooks destination",
//!     secret: "my-secret",
//!     url: "https://example.com",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.NotificationPolicyWebhooks("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="Webhooks destination",
//!     secret="my-secret",
//!     url="https://example.com")
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//!
//! return await Deployment.RunAsync(() =>
//! {
//!     var example = new Cloudflare.NotificationPolicyWebhooks("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "Webhooks destination",
//!         Secret = "my-secret",
//!         Url = "https://example.com",
//!     });
//!
//! });
//! ```
//! ### Go
//! ```go
//! package main
//!
//! import (
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//!
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		_, err := cloudflare.NewNotificationPolicyWebhooks(ctx, "example", &cloudflare.NotificationPolicyWebhooksArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("Webhooks destination"),
//! 			Secret:    pulumi.String("my-secret"),
//! 			Url:       pulumi.String("https://example.com"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		return nil
//! 	})
//! }
//! ```
//! ### Java
//! ```java
//! package generated_program;
//!
//! import com.pulumi.Context;
//! import com.pulumi.Pulumi;
//! import com.pulumi.core.Output;
//! import com.pulumi.cloudflare.NotificationPolicyWebhooks;
//! import com.pulumi.cloudflare.NotificationPolicyWebhooksArgs;
//! import java.util.List;
//! import java.util.ArrayList;
//! import java.util.Map;
//! import java.io.File;
//! import java.nio.file.Files;
//! import java.nio.file.Paths;
//!
//! public class App {
//!     public static void main(String[] args) {
//!         Pulumi.run(App::stack);
//!     }
//!
//!     public static void stack(Context ctx) {
//!         var example = new NotificationPolicyWebhooks("example", NotificationPolicyWebhooksArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("Webhooks destination")
//!             .secret("my-secret")
//!             .url("https://example.com")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:NotificationPolicyWebhooks
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: Webhooks destination
//!       secret: my-secret
//!       url: https://example.com
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/notificationPolicyWebhooks:NotificationPolicyWebhooks example <account_id>/<notification_webhook_id>
//! ```
//!

pub struct NotificationPolicyWebhooksArgs {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the webhook destination.
    pub name: pulumi_wasm_rust::Output<String>,
    /// An optional secret can be provided that will be passed in the `cf-webhook-auth` header when dispatching a webhook notification. Secrets are not returned in any API response body. Refer to the [documentation](https://api.cloudflare.com/#notification-webhooks-create-webhook) for more details.
    pub secret: pulumi_wasm_rust::Output<Option<String>>,
    /// The URL of the webhook destinations. **Modifying this attribute will force creation of a new resource.**
    pub url: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct NotificationPolicyWebhooksResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Timestamp of when the notification webhook was created.
    pub created_at: pulumi_wasm_rust::Output<String>,
    /// Timestamp of when the notification webhook last faiuled.
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
pub fn create(
    name: &str,
    args: NotificationPolicyWebhooksArgs,
) -> NotificationPolicyWebhooksResult {
    let result = crate::bindings::pulumi::cloudflare::notification_policy_webhooks::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::notification_policy_webhooks::Args {
            account_id: args.account_id.get_inner(),
            name: args.name.get_inner(),
            secret: args.secret.get_inner(),
            url: args.url.get_inner(),
        },
    );

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
