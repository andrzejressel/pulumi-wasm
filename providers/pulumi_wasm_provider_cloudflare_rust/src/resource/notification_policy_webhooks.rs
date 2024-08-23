pub struct NotificationPolicyWebhooksArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub secret: pulumi_wasm_rust::Output<Option<String>>,
    pub url: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct NotificationPolicyWebhooksResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub created_at: pulumi_wasm_rust::Output<String>,
    pub last_failure: pulumi_wasm_rust::Output<String>,
    pub last_success: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub secret: pulumi_wasm_rust::Output<Option<String>>,
    pub type_: pulumi_wasm_rust::Output<String>,
    pub url: pulumi_wasm_rust::Output<Option<String>>,
}

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
