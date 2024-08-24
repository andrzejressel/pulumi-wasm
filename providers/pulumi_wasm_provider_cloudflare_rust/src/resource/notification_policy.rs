//! Provides a resource, that manages a notification policy for
//! Cloudflare's products. The delivery mechanisms supported are email,
//! webhooks, and PagerDuty.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! //## With Filters
//! const example = new cloudflare.NotificationPolicy("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     alertType: "health_check_status_notification",
//!     description: "Notification policy to alert on unhealthy Healthcheck status",
//!     emailIntegrations: [{
//!         id: "myemail@example.com",
//!     }],
//!     enabled: true,
//!     filters: {
//!         healthCheckIds: ["699d98642c564d2e855e9661899b7252"],
//!         statuses: ["Unhealthy"],
//!     },
//!     name: "Policy for Healthcheck notification",
//!     pagerdutyIntegrations: [{
//!         id: "850129d136459401860572c5d964d27k",
//!     }],
//!     webhooksIntegrations: [{
//!         id: "1860572c5d964d27aa0f379d13645940",
//!     }],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! ### With Filters
//! example = cloudflare.NotificationPolicy("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     alert_type="health_check_status_notification",
//!     description="Notification policy to alert on unhealthy Healthcheck status",
//!     email_integrations=[cloudflare.NotificationPolicyEmailIntegrationArgs(
//!         id="myemail@example.com",
//!     )],
//!     enabled=True,
//!     filters=cloudflare.NotificationPolicyFiltersArgs(
//!         health_check_ids=["699d98642c564d2e855e9661899b7252"],
//!         statuses=["Unhealthy"],
//!     ),
//!     name="Policy for Healthcheck notification",
//!     pagerduty_integrations=[cloudflare.NotificationPolicyPagerdutyIntegrationArgs(
//!         id="850129d136459401860572c5d964d27k",
//!     )],
//!     webhooks_integrations=[cloudflare.NotificationPolicyWebhooksIntegrationArgs(
//!         id="1860572c5d964d27aa0f379d13645940",
//!     )])
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
//!     //## With Filters
//!     var example = new Cloudflare.NotificationPolicy("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         AlertType = "health_check_status_notification",
//!         Description = "Notification policy to alert on unhealthy Healthcheck status",
//!         EmailIntegrations = new[]
//!         {
//!             new Cloudflare.Inputs.NotificationPolicyEmailIntegrationArgs
//!             {
//!                 Id = "myemail@example.com",
//!             },
//!         },
//!         Enabled = true,
//!         Filters = new Cloudflare.Inputs.NotificationPolicyFiltersArgs
//!         {
//!             HealthCheckIds = new[]
//!             {
//!                 "699d98642c564d2e855e9661899b7252",
//!             },
//!             Statuses = new[]
//!             {
//!                 "Unhealthy",
//!             },
//!         },
//!         Name = "Policy for Healthcheck notification",
//!         PagerdutyIntegrations = new[]
//!         {
//!             new Cloudflare.Inputs.NotificationPolicyPagerdutyIntegrationArgs
//!             {
//!                 Id = "850129d136459401860572c5d964d27k",
//!             },
//!         },
//!         WebhooksIntegrations = new[]
//!         {
//!             new Cloudflare.Inputs.NotificationPolicyWebhooksIntegrationArgs
//!             {
//!                 Id = "1860572c5d964d27aa0f379d13645940",
//!             },
//!         },
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
//! 		// ## With Filters
//! 		_, err := cloudflare.NewNotificationPolicy(ctx, "example", &cloudflare.NotificationPolicyArgs{
//! 			AccountId:   pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			AlertType:   pulumi.String("health_check_status_notification"),
//! 			Description: pulumi.String("Notification policy to alert on unhealthy Healthcheck status"),
//! 			EmailIntegrations: cloudflare.NotificationPolicyEmailIntegrationArray{
//! 				&cloudflare.NotificationPolicyEmailIntegrationArgs{
//! 					Id: pulumi.String("myemail@example.com"),
//! 				},
//! 			},
//! 			Enabled: pulumi.Bool(true),
//! 			Filters: &cloudflare.NotificationPolicyFiltersArgs{
//! 				HealthCheckIds: pulumi.StringArray{
//! 					pulumi.String("699d98642c564d2e855e9661899b7252"),
//! 				},
//! 				Statuses: pulumi.StringArray{
//! 					pulumi.String("Unhealthy"),
//! 				},
//! 			},
//! 			Name: pulumi.String("Policy for Healthcheck notification"),
//! 			PagerdutyIntegrations: cloudflare.NotificationPolicyPagerdutyIntegrationArray{
//! 				&cloudflare.NotificationPolicyPagerdutyIntegrationArgs{
//! 					Id: pulumi.String("850129d136459401860572c5d964d27k"),
//! 				},
//! 			},
//! 			WebhooksIntegrations: cloudflare.NotificationPolicyWebhooksIntegrationArray{
//! 				&cloudflare.NotificationPolicyWebhooksIntegrationArgs{
//! 					Id: pulumi.String("1860572c5d964d27aa0f379d13645940"),
//! 				},
//! 			},
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
//! import com.pulumi.cloudflare.NotificationPolicy;
//! import com.pulumi.cloudflare.NotificationPolicyArgs;
//! import com.pulumi.cloudflare.inputs.NotificationPolicyEmailIntegrationArgs;
//! import com.pulumi.cloudflare.inputs.NotificationPolicyFiltersArgs;
//! import com.pulumi.cloudflare.inputs.NotificationPolicyPagerdutyIntegrationArgs;
//! import com.pulumi.cloudflare.inputs.NotificationPolicyWebhooksIntegrationArgs;
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
//!         //## With Filters
//!         var example = new NotificationPolicy("example", NotificationPolicyArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .alertType("health_check_status_notification")
//!             .description("Notification policy to alert on unhealthy Healthcheck status")
//!             .emailIntegrations(NotificationPolicyEmailIntegrationArgs.builder()
//!                 .id("myemail@example.com")
//!                 .build())
//!             .enabled(true)
//!             .filters(NotificationPolicyFiltersArgs.builder()
//!                 .healthCheckIds("699d98642c564d2e855e9661899b7252")
//!                 .statuses("Unhealthy")
//!                 .build())
//!             .name("Policy for Healthcheck notification")
//!             .pagerdutyIntegrations(NotificationPolicyPagerdutyIntegrationArgs.builder()
//!                 .id("850129d136459401860572c5d964d27k")
//!                 .build())
//!             .webhooksIntegrations(NotificationPolicyWebhooksIntegrationArgs.builder()
//!                 .id("1860572c5d964d27aa0f379d13645940")
//!                 .build())
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   ## With Filters
//!   example:
//!     type: cloudflare:NotificationPolicy
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       alertType: health_check_status_notification
//!       description: Notification policy to alert on unhealthy Healthcheck status
//!       emailIntegrations:
//!         - id: myemail@example.com
//!       enabled: true
//!       filters:
//!         healthCheckIds:
//!           - 699d98642c564d2e855e9661899b7252
//!         statuses:
//!           - Unhealthy
//!       name: Policy for Healthcheck notification
//!       pagerdutyIntegrations:
//!         - id: 850129d136459401860572c5d964d27k
//!       webhooksIntegrations:
//!         - id: 1860572c5d964d27aa0f379d13645940
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/notificationPolicy:NotificationPolicy example <account_id>/<policy_id>
//! ```
//!

pub struct NotificationPolicyArgs {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The event type that will trigger the dispatch of a notification. See the developer documentation for descriptions of [available alert types](https://developers.cloudflare.com/fundamentals/notifications/notification-available/). Available values: `advanced_http_alert_error`, `access_custom_certificate_expiration_type`, `advanced_ddos_attack_l4_alert`, `advanced_ddos_attack_l7_alert`, `bgp_hijack_notification`, `billing_usage_alert`, `block_notification_block_removed`, `block_notification_new_block`, `block_notification_review_rejected`, `brand_protection_alert`, `brand_protection_digest`, `clickhouse_alert_fw_anomaly`, `clickhouse_alert_fw_ent_anomaly`, `custom_ssl_certificate_event_type`, `dedicated_ssl_certificate_event_type`, `dos_attack_l4`, `dos_attack_l7`, `expiring_service_token_alert`, `failing_logpush_job_disabled_alert`, `fbm_auto_advertisement`, `fbm_dosd_attack`, `fbm_volumetric_attack`, `health_check_status_notification`, `hostname_aop_custom_certificate_expiration_type`, `http_alert_edge_error`, `http_alert_origin_error`, `incident_alert`, `load_balancing_health_alert`, `load_balancing_pool_enablement_alert`, `logo_match_alert`, `magic_tunnel_health_check_event`, `maintenance_event_notification`, `mtls_certificate_store_certificate_expiration_type`, `pages_event_alert`, `radar_notification`, `real_origin_monitoring`, `scriptmonitor_alert_new_code_change_detections`, `scriptmonitor_alert_new_hosts`, `scriptmonitor_alert_new_malicious_hosts`, `scriptmonitor_alert_new_malicious_scripts`, `scriptmonitor_alert_new_malicious_url`, `scriptmonitor_alert_new_max_length_resource_url`, `scriptmonitor_alert_new_resources`, `secondary_dns_all_primaries_failing`, `secondary_dns_primaries_failing`, `secondary_dns_zone_successfully_updated`, `secondary_dns_zone_validation_warning`, `sentinel_alert`, `stream_live_notifications`, `traffic_anomalies_alert`, `tunnel_health_event`, `tunnel_update_event`, `universal_ssl_event_type`, `web_analytics_metrics_update`, `weekly_account_overview`, `workers_alert`, `zone_aop_custom_certificate_expiration_type`.
    pub alert_type: pulumi_wasm_rust::Output<String>,
    /// Description of the notification policy.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The email ID to which the notification should be dispatched.
    pub email_integrations:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyEmailIntegration>>>,
    /// State of the pool to alert on.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// An optional nested block of filters that applies to the selected `alert_type`. A key-value map that specifies the type of filter and the values to match against (refer to the alert type block for available fields).
    pub filters: pulumi_wasm_rust::Output<Option<crate::types::NotificationPolicyFilters>>,
    /// The name of the notification policy.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The unique ID of a configured pagerduty endpoint to which the notification should be dispatched.
    pub pagerduty_integrations:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyPagerdutyIntegration>>>,
    /// The unique ID of a configured webhooks endpoint to which the notification should be dispatched.
    pub webhooks_integrations:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyWebhooksIntegration>>>,
}

pub struct NotificationPolicyResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The event type that will trigger the dispatch of a notification. See the developer documentation for descriptions of [available alert types](https://developers.cloudflare.com/fundamentals/notifications/notification-available/). Available values: `advanced_http_alert_error`, `access_custom_certificate_expiration_type`, `advanced_ddos_attack_l4_alert`, `advanced_ddos_attack_l7_alert`, `bgp_hijack_notification`, `billing_usage_alert`, `block_notification_block_removed`, `block_notification_new_block`, `block_notification_review_rejected`, `brand_protection_alert`, `brand_protection_digest`, `clickhouse_alert_fw_anomaly`, `clickhouse_alert_fw_ent_anomaly`, `custom_ssl_certificate_event_type`, `dedicated_ssl_certificate_event_type`, `dos_attack_l4`, `dos_attack_l7`, `expiring_service_token_alert`, `failing_logpush_job_disabled_alert`, `fbm_auto_advertisement`, `fbm_dosd_attack`, `fbm_volumetric_attack`, `health_check_status_notification`, `hostname_aop_custom_certificate_expiration_type`, `http_alert_edge_error`, `http_alert_origin_error`, `incident_alert`, `load_balancing_health_alert`, `load_balancing_pool_enablement_alert`, `logo_match_alert`, `magic_tunnel_health_check_event`, `maintenance_event_notification`, `mtls_certificate_store_certificate_expiration_type`, `pages_event_alert`, `radar_notification`, `real_origin_monitoring`, `scriptmonitor_alert_new_code_change_detections`, `scriptmonitor_alert_new_hosts`, `scriptmonitor_alert_new_malicious_hosts`, `scriptmonitor_alert_new_malicious_scripts`, `scriptmonitor_alert_new_malicious_url`, `scriptmonitor_alert_new_max_length_resource_url`, `scriptmonitor_alert_new_resources`, `secondary_dns_all_primaries_failing`, `secondary_dns_primaries_failing`, `secondary_dns_zone_successfully_updated`, `secondary_dns_zone_validation_warning`, `sentinel_alert`, `stream_live_notifications`, `traffic_anomalies_alert`, `tunnel_health_event`, `tunnel_update_event`, `universal_ssl_event_type`, `web_analytics_metrics_update`, `weekly_account_overview`, `workers_alert`, `zone_aop_custom_certificate_expiration_type`.
    pub alert_type: pulumi_wasm_rust::Output<String>,
    /// When the notification policy was created.
    pub created: pulumi_wasm_rust::Output<String>,
    /// Description of the notification policy.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The email ID to which the notification should be dispatched.
    pub email_integrations:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyEmailIntegration>>>,
    /// State of the pool to alert on.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// An optional nested block of filters that applies to the selected `alert_type`. A key-value map that specifies the type of filter and the values to match against (refer to the alert type block for available fields).
    pub filters: pulumi_wasm_rust::Output<Option<crate::types::NotificationPolicyFilters>>,
    /// When the notification policy was last modified.
    pub modified: pulumi_wasm_rust::Output<String>,
    /// The name of the notification policy.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The unique ID of a configured pagerduty endpoint to which the notification should be dispatched.
    pub pagerduty_integrations:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyPagerdutyIntegration>>>,
    /// The unique ID of a configured webhooks endpoint to which the notification should be dispatched.
    pub webhooks_integrations:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyWebhooksIntegration>>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
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
