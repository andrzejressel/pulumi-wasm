#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct NotificationPolicyArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The event type that will trigger the dispatch of a notification. See the developer documentation for descriptions of [available alert types](https://developers.cloudflare.com/fundamentals/notifications/notification-available/). Available values: `advanced_http_alert_error`, `access_custom_certificate_expiration_type`, `advanced_ddos_attack_l4_alert`, `advanced_ddos_attack_l7_alert`, `bgp_hijack_notification`, `billing_usage_alert`, `block_notification_block_removed`, `block_notification_new_block`, `block_notification_review_rejected`, `brand_protection_alert`, `brand_protection_digest`, `clickhouse_alert_fw_anomaly`, `clickhouse_alert_fw_ent_anomaly`, `custom_ssl_certificate_event_type`, `dedicated_ssl_certificate_event_type`, `dos_attack_l4`, `dos_attack_l7`, `expiring_service_token_alert`, `failing_logpush_job_disabled_alert`, `fbm_auto_advertisement`, `fbm_dosd_attack`, `fbm_volumetric_attack`, `health_check_status_notification`, `hostname_aop_custom_certificate_expiration_type`, `http_alert_edge_error`, `http_alert_origin_error`, `image_notification`, `incident_alert`, `load_balancing_health_alert`, `load_balancing_pool_enablement_alert`, `logo_match_alert`, `magic_tunnel_health_check_event`, `maintenance_event_notification`, `mtls_certificate_store_certificate_expiration_type`, `pages_event_alert`, `radar_notification`, `real_origin_monitoring`, `scriptmonitor_alert_new_code_change_detections`, `scriptmonitor_alert_new_hosts`, `scriptmonitor_alert_new_malicious_hosts`, `scriptmonitor_alert_new_malicious_scripts`, `scriptmonitor_alert_new_malicious_url`, `scriptmonitor_alert_new_max_length_resource_url`, `scriptmonitor_alert_new_resources`, `secondary_dns_all_primaries_failing`, `secondary_dns_primaries_failing`, `secondary_dns_zone_successfully_updated`, `secondary_dns_zone_validation_warning`, `sentinel_alert`, `stream_live_notifications`, `traffic_anomalies_alert`, `tunnel_health_event`, `tunnel_update_event`, `universal_ssl_event_type`, `web_analytics_metrics_update`, `weekly_account_overview`, `workers_alert`, `zone_aop_custom_certificate_expiration_type`.
    #[builder(into)]
    pub alert_type: pulumi_wasm_rust::Output<String>,
    /// Description of the notification policy.
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The email ID to which the notification should be dispatched.
    #[builder(into, default)]
    pub email_integrations: pulumi_wasm_rust::Output<
        Option<Vec<super::types::NotificationPolicyEmailIntegration>>,
    >,
    /// The status of the notification policy.
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// An optional nested block of filters that applies to the selected `alert_type`. A key-value map that specifies the type of filter and the values to match against (refer to the alert type block for available fields).
    #[builder(into, default)]
    pub filters: pulumi_wasm_rust::Output<
        Option<super::types::NotificationPolicyFilters>,
    >,
    /// The name of the notification policy.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The unique ID of a configured pagerduty endpoint to which the notification should be dispatched.
    #[builder(into, default)]
    pub pagerduty_integrations: pulumi_wasm_rust::Output<
        Option<Vec<super::types::NotificationPolicyPagerdutyIntegration>>,
    >,
    /// The unique ID of a configured webhooks endpoint to which the notification should be dispatched.
    #[builder(into, default)]
    pub webhooks_integrations: pulumi_wasm_rust::Output<
        Option<Vec<super::types::NotificationPolicyWebhooksIntegration>>,
    >,
}
pub struct NotificationPolicyResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The event type that will trigger the dispatch of a notification. See the developer documentation for descriptions of [available alert types](https://developers.cloudflare.com/fundamentals/notifications/notification-available/). Available values: `advanced_http_alert_error`, `access_custom_certificate_expiration_type`, `advanced_ddos_attack_l4_alert`, `advanced_ddos_attack_l7_alert`, `bgp_hijack_notification`, `billing_usage_alert`, `block_notification_block_removed`, `block_notification_new_block`, `block_notification_review_rejected`, `brand_protection_alert`, `brand_protection_digest`, `clickhouse_alert_fw_anomaly`, `clickhouse_alert_fw_ent_anomaly`, `custom_ssl_certificate_event_type`, `dedicated_ssl_certificate_event_type`, `dos_attack_l4`, `dos_attack_l7`, `expiring_service_token_alert`, `failing_logpush_job_disabled_alert`, `fbm_auto_advertisement`, `fbm_dosd_attack`, `fbm_volumetric_attack`, `health_check_status_notification`, `hostname_aop_custom_certificate_expiration_type`, `http_alert_edge_error`, `http_alert_origin_error`, `image_notification`, `incident_alert`, `load_balancing_health_alert`, `load_balancing_pool_enablement_alert`, `logo_match_alert`, `magic_tunnel_health_check_event`, `maintenance_event_notification`, `mtls_certificate_store_certificate_expiration_type`, `pages_event_alert`, `radar_notification`, `real_origin_monitoring`, `scriptmonitor_alert_new_code_change_detections`, `scriptmonitor_alert_new_hosts`, `scriptmonitor_alert_new_malicious_hosts`, `scriptmonitor_alert_new_malicious_scripts`, `scriptmonitor_alert_new_malicious_url`, `scriptmonitor_alert_new_max_length_resource_url`, `scriptmonitor_alert_new_resources`, `secondary_dns_all_primaries_failing`, `secondary_dns_primaries_failing`, `secondary_dns_zone_successfully_updated`, `secondary_dns_zone_validation_warning`, `sentinel_alert`, `stream_live_notifications`, `traffic_anomalies_alert`, `tunnel_health_event`, `tunnel_update_event`, `universal_ssl_event_type`, `web_analytics_metrics_update`, `weekly_account_overview`, `workers_alert`, `zone_aop_custom_certificate_expiration_type`.
    pub alert_type: pulumi_wasm_rust::Output<String>,
    /// When the notification policy was created.
    pub created: pulumi_wasm_rust::Output<String>,
    /// Description of the notification policy.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The email ID to which the notification should be dispatched.
    pub email_integrations: pulumi_wasm_rust::Output<
        Option<Vec<super::types::NotificationPolicyEmailIntegration>>,
    >,
    /// The status of the notification policy.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// An optional nested block of filters that applies to the selected `alert_type`. A key-value map that specifies the type of filter and the values to match against (refer to the alert type block for available fields).
    pub filters: pulumi_wasm_rust::Output<
        Option<super::types::NotificationPolicyFilters>,
    >,
    /// When the notification policy was last modified.
    pub modified: pulumi_wasm_rust::Output<String>,
    /// The name of the notification policy.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The unique ID of a configured pagerduty endpoint to which the notification should be dispatched.
    pub pagerduty_integrations: pulumi_wasm_rust::Output<
        Option<Vec<super::types::NotificationPolicyPagerdutyIntegration>>,
    >,
    /// The unique ID of a configured webhooks endpoint to which the notification should be dispatched.
    pub webhooks_integrations: pulumi_wasm_rust::Output<
        Option<Vec<super::types::NotificationPolicyWebhooksIntegration>>,
    >,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: NotificationPolicyArgs) -> NotificationPolicyResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let alert_type_binding = args.alert_type.get_inner();
    let description_binding = args.description.get_inner();
    let email_integrations_binding = args.email_integrations.get_inner();
    let enabled_binding = args.enabled.get_inner();
    let filters_binding = args.filters.get_inner();
    let name_binding = args.name.get_inner();
    let pagerduty_integrations_binding = args.pagerduty_integrations.get_inner();
    let webhooks_integrations_binding = args.webhooks_integrations.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/notificationPolicy:NotificationPolicy".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "alertType".into(),
                value: &alert_type_binding,
            },
            register_interface::ObjectField {
                name: "description".into(),
                value: &description_binding,
            },
            register_interface::ObjectField {
                name: "emailIntegrations".into(),
                value: &email_integrations_binding,
            },
            register_interface::ObjectField {
                name: "enabled".into(),
                value: &enabled_binding,
            },
            register_interface::ObjectField {
                name: "filters".into(),
                value: &filters_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "pagerdutyIntegrations".into(),
                value: &pagerduty_integrations_binding,
            },
            register_interface::ObjectField {
                name: "webhooksIntegrations".into(),
                value: &webhooks_integrations_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "alertType".into() },
            register_interface::ResultField { name : "created".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "emailIntegrations".into() },
            register_interface::ResultField { name : "enabled".into() },
            register_interface::ResultField { name : "filters".into() },
            register_interface::ResultField { name : "modified".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "pagerdutyIntegrations".into() },
            register_interface::ResultField { name : "webhooksIntegrations".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    NotificationPolicyResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        alert_type: into_domain(hashmap.remove("alertType").unwrap()),
        created: into_domain(hashmap.remove("created").unwrap()),
        description: into_domain(hashmap.remove("description").unwrap()),
        email_integrations: into_domain(hashmap.remove("emailIntegrations").unwrap()),
        enabled: into_domain(hashmap.remove("enabled").unwrap()),
        filters: into_domain(hashmap.remove("filters").unwrap()),
        modified: into_domain(hashmap.remove("modified").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        pagerduty_integrations: into_domain(
            hashmap.remove("pagerdutyIntegrations").unwrap(),
        ),
        webhooks_integrations: into_domain(
            hashmap.remove("webhooksIntegrations").unwrap(),
        ),
    }
}
