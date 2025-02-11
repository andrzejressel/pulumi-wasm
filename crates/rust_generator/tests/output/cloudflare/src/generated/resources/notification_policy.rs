/// Provides a resource, that manages a notification policy for
/// Cloudflare's products. The delivery mechanisms supported are email,
/// webhooks, and PagerDuty.
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/notificationPolicy:NotificationPolicy example <account_id>/<policy_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod notification_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotificationPolicyArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The event type that will trigger the dispatch of a notification. See the developer documentation for descriptions of [available alert types](https://developers.cloudflare.com/fundamentals/notifications/notification-available/). Available values: `advanced_http_alert_error`, `access_custom_certificate_expiration_type`, `advanced_ddos_attack_l4_alert`, `advanced_ddos_attack_l7_alert`, `bgp_hijack_notification`, `billing_usage_alert`, `block_notification_block_removed`, `block_notification_new_block`, `block_notification_review_rejected`, `brand_protection_alert`, `brand_protection_digest`, `clickhouse_alert_fw_anomaly`, `clickhouse_alert_fw_ent_anomaly`, `custom_ssl_certificate_event_type`, `dedicated_ssl_certificate_event_type`, `dos_attack_l4`, `dos_attack_l7`, `expiring_service_token_alert`, `failing_logpush_job_disabled_alert`, `fbm_auto_advertisement`, `fbm_dosd_attack`, `fbm_volumetric_attack`, `health_check_status_notification`, `hostname_aop_custom_certificate_expiration_type`, `http_alert_edge_error`, `http_alert_origin_error`, `image_notification`, `incident_alert`, `load_balancing_health_alert`, `load_balancing_pool_enablement_alert`, `logo_match_alert`, `magic_tunnel_health_check_event`, `maintenance_event_notification`, `mtls_certificate_store_certificate_expiration_type`, `pages_event_alert`, `radar_notification`, `real_origin_monitoring`, `scriptmonitor_alert_new_code_change_detections`, `scriptmonitor_alert_new_hosts`, `scriptmonitor_alert_new_malicious_hosts`, `scriptmonitor_alert_new_malicious_scripts`, `scriptmonitor_alert_new_malicious_url`, `scriptmonitor_alert_new_max_length_resource_url`, `scriptmonitor_alert_new_resources`, `secondary_dns_all_primaries_failing`, `secondary_dns_primaries_failing`, `secondary_dns_zone_successfully_updated`, `secondary_dns_zone_validation_warning`, `sentinel_alert`, `stream_live_notifications`, `traffic_anomalies_alert`, `tunnel_health_event`, `tunnel_update_event`, `universal_ssl_event_type`, `web_analytics_metrics_update`, `weekly_account_overview`, `workers_alert`, `zone_aop_custom_certificate_expiration_type`.
        #[builder(into)]
        pub alert_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the notification policy.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The email ID to which the notification should be dispatched.
        #[builder(into, default)]
        pub email_integrations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::NotificationPolicyEmailIntegration>>,
        >,
        /// The status of the notification policy.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// An optional nested block of filters that applies to the selected `alert_type`. A key-value map that specifies the type of filter and the values to match against (refer to the alert type block for available fields).
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::NotificationPolicyFilters>,
        >,
        /// The name of the notification policy.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The unique ID of a configured pagerduty endpoint to which the notification should be dispatched.
        #[builder(into, default)]
        pub pagerduty_integrations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::NotificationPolicyPagerdutyIntegration>>,
        >,
        /// The unique ID of a configured webhooks endpoint to which the notification should be dispatched.
        #[builder(into, default)]
        pub webhooks_integrations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::NotificationPolicyWebhooksIntegration>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NotificationPolicyResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The event type that will trigger the dispatch of a notification. See the developer documentation for descriptions of [available alert types](https://developers.cloudflare.com/fundamentals/notifications/notification-available/). Available values: `advanced_http_alert_error`, `access_custom_certificate_expiration_type`, `advanced_ddos_attack_l4_alert`, `advanced_ddos_attack_l7_alert`, `bgp_hijack_notification`, `billing_usage_alert`, `block_notification_block_removed`, `block_notification_new_block`, `block_notification_review_rejected`, `brand_protection_alert`, `brand_protection_digest`, `clickhouse_alert_fw_anomaly`, `clickhouse_alert_fw_ent_anomaly`, `custom_ssl_certificate_event_type`, `dedicated_ssl_certificate_event_type`, `dos_attack_l4`, `dos_attack_l7`, `expiring_service_token_alert`, `failing_logpush_job_disabled_alert`, `fbm_auto_advertisement`, `fbm_dosd_attack`, `fbm_volumetric_attack`, `health_check_status_notification`, `hostname_aop_custom_certificate_expiration_type`, `http_alert_edge_error`, `http_alert_origin_error`, `image_notification`, `incident_alert`, `load_balancing_health_alert`, `load_balancing_pool_enablement_alert`, `logo_match_alert`, `magic_tunnel_health_check_event`, `maintenance_event_notification`, `mtls_certificate_store_certificate_expiration_type`, `pages_event_alert`, `radar_notification`, `real_origin_monitoring`, `scriptmonitor_alert_new_code_change_detections`, `scriptmonitor_alert_new_hosts`, `scriptmonitor_alert_new_malicious_hosts`, `scriptmonitor_alert_new_malicious_scripts`, `scriptmonitor_alert_new_malicious_url`, `scriptmonitor_alert_new_max_length_resource_url`, `scriptmonitor_alert_new_resources`, `secondary_dns_all_primaries_failing`, `secondary_dns_primaries_failing`, `secondary_dns_zone_successfully_updated`, `secondary_dns_zone_validation_warning`, `sentinel_alert`, `stream_live_notifications`, `traffic_anomalies_alert`, `tunnel_health_event`, `tunnel_update_event`, `universal_ssl_event_type`, `web_analytics_metrics_update`, `weekly_account_overview`, `workers_alert`, `zone_aop_custom_certificate_expiration_type`.
        pub alert_type: pulumi_gestalt_rust::Output<String>,
        /// When the notification policy was created.
        pub created: pulumi_gestalt_rust::Output<String>,
        /// Description of the notification policy.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The email ID to which the notification should be dispatched.
        pub email_integrations: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::NotificationPolicyEmailIntegration>>,
        >,
        /// The status of the notification policy.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// An optional nested block of filters that applies to the selected `alert_type`. A key-value map that specifies the type of filter and the values to match against (refer to the alert type block for available fields).
        pub filters: pulumi_gestalt_rust::Output<
            Option<super::types::NotificationPolicyFilters>,
        >,
        /// When the notification policy was last modified.
        pub modified: pulumi_gestalt_rust::Output<String>,
        /// The name of the notification policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The unique ID of a configured pagerduty endpoint to which the notification should be dispatched.
        pub pagerduty_integrations: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::NotificationPolicyPagerdutyIntegration>>,
        >,
        /// The unique ID of a configured webhooks endpoint to which the notification should be dispatched.
        pub webhooks_integrations: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::NotificationPolicyWebhooksIntegration>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NotificationPolicyArgs,
    ) -> NotificationPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let alert_type_binding = args.alert_type.get_output(context);
        let description_binding = args.description.get_output(context);
        let email_integrations_binding = args.email_integrations.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let filters_binding = args.filters.get_output(context);
        let name_binding = args.name.get_output(context);
        let pagerduty_integrations_binding = args
            .pagerduty_integrations
            .get_output(context);
        let webhooks_integrations_binding = args
            .webhooks_integrations
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/notificationPolicy:NotificationPolicy".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alertType".into(),
                    value: &alert_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailIntegrations".into(),
                    value: &email_integrations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pagerdutyIntegrations".into(),
                    value: &pagerduty_integrations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webhooksIntegrations".into(),
                    value: &webhooks_integrations_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NotificationPolicyResult {
            account_id: o.get_field("accountId"),
            alert_type: o.get_field("alertType"),
            created: o.get_field("created"),
            description: o.get_field("description"),
            email_integrations: o.get_field("emailIntegrations"),
            enabled: o.get_field("enabled"),
            filters: o.get_field("filters"),
            modified: o.get_field("modified"),
            name: o.get_field("name"),
            pagerduty_integrations: o.get_field("pagerdutyIntegrations"),
            webhooks_integrations: o.get_field("webhooksIntegrations"),
        }
    }
}
