pub struct NotificationPolicyArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub alert_type: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub email_integrations:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyEmailIntegration>>>,
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub filters: pulumi_wasm_rust::Output<Option<crate::types::NotificationPolicyFilters>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub pagerduty_integrations:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyPagerdutyIntegration>>>,
    pub webhooks_integrations:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyWebhooksIntegration>>>,
}

pub struct NotificationPolicyResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub alert_type: pulumi_wasm_rust::Output<String>,
    pub created: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub email_integrations:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyEmailIntegration>>>,
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub filters: pulumi_wasm_rust::Output<Option<crate::types::NotificationPolicyFilters>>,
    pub modified: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub pagerduty_integrations:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyPagerdutyIntegration>>>,
    pub webhooks_integrations:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyWebhooksIntegration>>>,
}

pub fn create(name: &str, args: NotificationPolicyArgs) -> NotificationPolicyResult {
    let result = crate::bindings::pulumi::cloudflare::notification_policy::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::notification_policy::Args {
            account_id: args.account_id.get_inner(),
            alert_type: args.alert_type.get_inner(),
            description: args.description.get_inner(),
            email_integrations: args.email_integrations.get_inner(),
            enabled: args.enabled.get_inner(),
            filters: args.filters.get_inner(),
            name: args.name.get_inner(),
            pagerduty_integrations: args.pagerduty_integrations.get_inner(),
            webhooks_integrations: args.webhooks_integrations.get_inner(),
        },
    );

    NotificationPolicyResult {
        account_id: crate::into_domain(result.account_id),
        alert_type: crate::into_domain(result.alert_type),
        created: crate::into_domain(result.created),
        description: crate::into_domain(result.description),
        email_integrations: crate::into_domain(result.email_integrations),
        enabled: crate::into_domain(result.enabled),
        filters: crate::into_domain(result.filters),
        modified: crate::into_domain(result.modified),
        name: crate::into_domain(result.name),
        pagerduty_integrations: crate::into_domain(result.pagerduty_integrations),
        webhooks_integrations: crate::into_domain(result.webhooks_integrations),
    }
}
