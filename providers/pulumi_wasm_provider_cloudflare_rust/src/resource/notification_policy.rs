//! Provides a resource, that manages a notification policy for
//! Cloudflare's products. The delivery mechanisms supported are email,
//! webhooks, and PagerDuty.
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/notificationPolicy:NotificationPolicy example <account_id>/<policy_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct NotificationPolicyArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The event type that will trigger the dispatch of a notification. See the developer documentation for descriptions of [available alert types](https://developers.cloudflare.com/fundamentals/notifications/notification-available/). Available values: `advanced_http_alert_error`, `access_custom_certificate_expiration_type`, `advanced_ddos_attack_l4_alert`, `advanced_ddos_attack_l7_alert`, `bgp_hijack_notification`, `billing_usage_alert`, `block_notification_block_removed`, `block_notification_new_block`, `block_notification_review_rejected`, `brand_protection_alert`, `brand_protection_digest`, `clickhouse_alert_fw_anomaly`, `clickhouse_alert_fw_ent_anomaly`, `custom_ssl_certificate_event_type`, `dedicated_ssl_certificate_event_type`, `dos_attack_l4`, `dos_attack_l7`, `expiring_service_token_alert`, `failing_logpush_job_disabled_alert`, `fbm_auto_advertisement`, `fbm_dosd_attack`, `fbm_volumetric_attack`, `health_check_status_notification`, `hostname_aop_custom_certificate_expiration_type`, `http_alert_edge_error`, `http_alert_origin_error`, `image_notification`, `incident_alert`, `load_balancing_health_alert`, `load_balancing_pool_enablement_alert`, `logo_match_alert`, `magic_tunnel_health_check_event`, `maintenance_event_notification`, `mtls_certificate_store_certificate_expiration_type`, `pages_event_alert`, `radar_notification`, `real_origin_monitoring`, `scriptmonitor_alert_new_code_change_detections`, `scriptmonitor_alert_new_hosts`, `scriptmonitor_alert_new_malicious_hosts`, `scriptmonitor_alert_new_malicious_scripts`, `scriptmonitor_alert_new_malicious_url`, `scriptmonitor_alert_new_max_length_resource_url`, `scriptmonitor_alert_new_resources`, `secondary_dns_all_primaries_failing`, `secondary_dns_primaries_failing`, `secondary_dns_zone_successfully_updated`, `secondary_dns_zone_validation_warning`, `sentinel_alert`, `stream_live_notifications`, `traffic_anomalies_alert`, `tunnel_health_event`, `tunnel_update_event`, `universal_ssl_event_type`, `web_analytics_metrics_update`, `weekly_account_overview`, `workers_alert`, `zone_aop_custom_certificate_expiration_type`.
    #[builder(into)]
    pub alert_type: pulumi_wasm_rust::Output<String>,
    /// Description of the notification policy.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The email ID to which the notification should be dispatched.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub email_integrations: pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyEmailIntegration>>>,
    /// The status of the notification policy.
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// An optional nested block of filters that applies to the selected `alert_type`. A key-value map that specifies the type of filter and the values to match against (refer to the alert type block for available fields).
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub filters: pulumi_wasm_rust::Output<Option<crate::types::NotificationPolicyFilters>>,
    /// The name of the notification policy.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The unique ID of a configured pagerduty endpoint to which the notification should be dispatched.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub pagerduty_integrations: pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyPagerdutyIntegration>>>,
    /// The unique ID of a configured webhooks endpoint to which the notification should be dispatched.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub webhooks_integrations: pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyWebhooksIntegration>>>,
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
    pub email_integrations: pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyEmailIntegration>>>,
    /// The status of the notification policy.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// An optional nested block of filters that applies to the selected `alert_type`. A key-value map that specifies the type of filter and the values to match against (refer to the alert type block for available fields).
    pub filters: pulumi_wasm_rust::Output<Option<crate::types::NotificationPolicyFilters>>,
    /// When the notification policy was last modified.
    pub modified: pulumi_wasm_rust::Output<String>,
    /// The name of the notification policy.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The unique ID of a configured pagerduty endpoint to which the notification should be dispatched.
    pub pagerduty_integrations: pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyPagerdutyIntegration>>>,
    /// The unique ID of a configured webhooks endpoint to which the notification should be dispatched.
    pub webhooks_integrations: pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyWebhooksIntegration>>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: NotificationPolicyArgs) -> NotificationPolicyResult {

    let result = crate::bindings::pulumi::cloudflare::notification_policy::invoke(name, &crate::bindings::pulumi::cloudflare::notification_policy::Args {
        account_id: &args.account_id.get_inner(),
        alert_type: &args.alert_type.get_inner(),
        description: &args.description.get_inner(),
        email_integrations: &args.email_integrations.get_inner(),
        enabled: &args.enabled.get_inner(),
        filters: &args.filters.get_inner(),
        name: &args.name.get_inner(),
        pagerduty_integrations: &args.pagerduty_integrations.get_inner(),
        webhooks_integrations: &args.webhooks_integrations.get_inner(),
    });

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
