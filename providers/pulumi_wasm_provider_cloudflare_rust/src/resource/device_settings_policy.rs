//! Provides a Cloudflare Device Settings Policy resource. Device policies configure settings applied to WARP devices.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const developerWarpPolicy = new cloudflare.DeviceSettingsPolicy("developer_warp_policy", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "Developers WARP settings policy",
//!     description: "Developers WARP settings policy description",
//!     precedence: 10,
//!     match: "any(identity.groups.name[*] in {\"Developers\"})",
//!     "default": false,
//!     enabled: true,
//!     allowModeSwitch: true,
//!     allowUpdates: true,
//!     allowedToLeave: true,
//!     autoConnect: 0,
//!     captivePortal: 5,
//!     disableAutoFallback: true,
//!     supportUrl: "https://cloudflare.com",
//!     switchLocked: true,
//!     serviceModeV2Mode: "warp",
//!     serviceModeV2Port: 3000,
//!     excludeOfficeIps: false,
//!     tunnelProtocol: "wireguard",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! developer_warp_policy = cloudflare.DeviceSettingsPolicy("developer_warp_policy",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="Developers WARP settings policy",
//!     description="Developers WARP settings policy description",
//!     precedence=10,
//!     match="any(identity.groups.name[*] in {\"Developers\"})",
//!     default=False,
//!     enabled=True,
//!     allow_mode_switch=True,
//!     allow_updates=True,
//!     allowed_to_leave=True,
//!     auto_connect=0,
//!     captive_portal=5,
//!     disable_auto_fallback=True,
//!     support_url="https://cloudflare.com",
//!     switch_locked=True,
//!     service_mode_v2_mode="warp",
//!     service_mode_v2_port=3000,
//!     exclude_office_ips=False,
//!     tunnel_protocol="wireguard")
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
//!     var developerWarpPolicy = new Cloudflare.DeviceSettingsPolicy("developer_warp_policy", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "Developers WARP settings policy",
//!         Description = "Developers WARP settings policy description",
//!         Precedence = 10,
//!         Match = "any(identity.groups.name[*] in {\"Developers\"})",
//!         Default = false,
//!         Enabled = true,
//!         AllowModeSwitch = true,
//!         AllowUpdates = true,
//!         AllowedToLeave = true,
//!         AutoConnect = 0,
//!         CaptivePortal = 5,
//!         DisableAutoFallback = true,
//!         SupportUrl = "https://cloudflare.com",
//!         SwitchLocked = true,
//!         ServiceModeV2Mode = "warp",
//!         ServiceModeV2Port = 3000,
//!         ExcludeOfficeIps = false,
//!         TunnelProtocol = "wireguard",
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
//! 		_, err := cloudflare.NewDeviceSettingsPolicy(ctx, "developer_warp_policy", &cloudflare.DeviceSettingsPolicyArgs{
//! 			AccountId:           pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:                pulumi.String("Developers WARP settings policy"),
//! 			Description:         pulumi.String("Developers WARP settings policy description"),
//! 			Precedence:          pulumi.Int(10),
//! 			Match:               pulumi.String("any(identity.groups.name[*] in {\"Developers\"})"),
//! 			Default:             pulumi.Bool(false),
//! 			Enabled:             pulumi.Bool(true),
//! 			AllowModeSwitch:     pulumi.Bool(true),
//! 			AllowUpdates:        pulumi.Bool(true),
//! 			AllowedToLeave:      pulumi.Bool(true),
//! 			AutoConnect:         pulumi.Int(0),
//! 			CaptivePortal:       pulumi.Int(5),
//! 			DisableAutoFallback: pulumi.Bool(true),
//! 			SupportUrl:          pulumi.String("https://cloudflare.com"),
//! 			SwitchLocked:        pulumi.Bool(true),
//! 			ServiceModeV2Mode:   pulumi.String("warp"),
//! 			ServiceModeV2Port:   pulumi.Int(3000),
//! 			ExcludeOfficeIps:    pulumi.Bool(false),
//! 			TunnelProtocol:      pulumi.String("wireguard"),
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
//! import com.pulumi.cloudflare.DeviceSettingsPolicy;
//! import com.pulumi.cloudflare.DeviceSettingsPolicyArgs;
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
//!         var developerWarpPolicy = new DeviceSettingsPolicy("developerWarpPolicy", DeviceSettingsPolicyArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("Developers WARP settings policy")
//!             .description("Developers WARP settings policy description")
//!             .precedence(10)
//!             .match("any(identity.groups.name[*] in {\"Developers\"})")
//!             .default_(false)
//!             .enabled(true)
//!             .allowModeSwitch(true)
//!             .allowUpdates(true)
//!             .allowedToLeave(true)
//!             .autoConnect(0)
//!             .captivePortal(5)
//!             .disableAutoFallback(true)
//!             .supportUrl("https://cloudflare.com")
//!             .switchLocked(true)
//!             .serviceModeV2Mode("warp")
//!             .serviceModeV2Port(3000)
//!             .excludeOfficeIps(false)
//!             .tunnelProtocol("wireguard")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   developerWarpPolicy:
//!     type: cloudflare:DeviceSettingsPolicy
//!     name: developer_warp_policy
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: Developers WARP settings policy
//!       description: Developers WARP settings policy description
//!       precedence: 10
//!       match: any(identity.groups.name[*] in {"Developers"})
//!       default: false
//!       enabled: true
//!       allowModeSwitch: true
//!       allowUpdates: true
//!       allowedToLeave: true
//!       autoConnect: 0
//!       captivePortal: 5
//!       disableAutoFallback: true
//!       supportUrl: https://cloudflare.com
//!       switchLocked: true
//!       serviceModeV2Mode: warp
//!       serviceModeV2Port: 3000
//!       excludeOfficeIps: false
//!       tunnelProtocol: wireguard
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! For default device settings policies you must use "default" as the policy ID.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/deviceSettingsPolicy:DeviceSettingsPolicy example <account_id>/<device_policy_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct DeviceSettingsPolicyArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether to allow mode switch for this policy.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub allow_mode_switch: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to allow updates under this policy.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub allow_updates: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to allow devices to leave the organization. Defaults to `true`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub allowed_to_leave: pulumi_wasm_rust::Output<Option<bool>>,
    /// The amount of time in seconds to reconnect after having been disabled.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub auto_connect: pulumi_wasm_rust::Output<Option<i32>>,
    /// The captive portal value for this policy. Defaults to `180`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub captive_portal: pulumi_wasm_rust::Output<Option<i32>>,
    /// Whether the policy refers to the default account policy.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub default: pulumi_wasm_rust::Output<Option<bool>>,
    /// Description of Policy.
    #[builder(into)]
    pub description: pulumi_wasm_rust::Output<String>,
    /// Whether to disable auto fallback for this policy.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub disable_auto_fallback: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether the policy is enabled (cannot be set for default policies). Defaults to `true`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to add Microsoft IPs to split tunnel exclusions.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub exclude_office_ips: pulumi_wasm_rust::Output<Option<bool>>,
    /// Wirefilter expression to match a device against when evaluating whether this policy should take effect for that device.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub match_: pulumi_wasm_rust::Output<Option<String>>,
    /// Name of the policy.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The precedence of the policy. Lower values indicate higher precedence.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub precedence: pulumi_wasm_rust::Output<Option<i32>>,
    /// The service mode. Available values: `1dot1`, `warp`, `proxy`, `posture_only`, `warp_tunnel_only`. Defaults to `warp`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub service_mode_v2_mode: pulumi_wasm_rust::Output<Option<String>>,
    /// The port to use for the proxy service mode. Required when using `service_mode_v2_mode`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub service_mode_v2_port: pulumi_wasm_rust::Output<Option<i32>>,
    /// The support URL that will be opened when sending feedback.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub support_url: pulumi_wasm_rust::Output<Option<String>>,
    /// Enablement of the ZT client switch lock.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub switch_locked: pulumi_wasm_rust::Output<Option<bool>>,
    /// Determines which tunnel protocol to use. Available values: `""`, `wireguard`, `masque`. Defaults to `wireguard`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub tunnel_protocol: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct DeviceSettingsPolicyResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether to allow mode switch for this policy.
    pub allow_mode_switch: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to allow updates under this policy.
    pub allow_updates: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to allow devices to leave the organization. Defaults to `true`.
    pub allowed_to_leave: pulumi_wasm_rust::Output<Option<bool>>,
    /// The amount of time in seconds to reconnect after having been disabled.
    pub auto_connect: pulumi_wasm_rust::Output<Option<i32>>,
    /// The captive portal value for this policy. Defaults to `180`.
    pub captive_portal: pulumi_wasm_rust::Output<Option<i32>>,
    /// Whether the policy refers to the default account policy.
    pub default: pulumi_wasm_rust::Output<Option<bool>>,
    /// Description of Policy.
    pub description: pulumi_wasm_rust::Output<String>,
    /// Whether to disable auto fallback for this policy.
    pub disable_auto_fallback: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether the policy is enabled (cannot be set for default policies). Defaults to `true`.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to add Microsoft IPs to split tunnel exclusions.
    pub exclude_office_ips: pulumi_wasm_rust::Output<Option<bool>>,
    /// Wirefilter expression to match a device against when evaluating whether this policy should take effect for that device.
    pub match_: pulumi_wasm_rust::Output<Option<String>>,
    /// Name of the policy.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The precedence of the policy. Lower values indicate higher precedence.
    pub precedence: pulumi_wasm_rust::Output<Option<i32>>,
    /// The service mode. Available values: `1dot1`, `warp`, `proxy`, `posture_only`, `warp_tunnel_only`. Defaults to `warp`.
    pub service_mode_v2_mode: pulumi_wasm_rust::Output<Option<String>>,
    /// The port to use for the proxy service mode. Required when using `service_mode_v2_mode`.
    pub service_mode_v2_port: pulumi_wasm_rust::Output<Option<i32>>,
    /// The support URL that will be opened when sending feedback.
    pub support_url: pulumi_wasm_rust::Output<Option<String>>,
    /// Enablement of the ZT client switch lock.
    pub switch_locked: pulumi_wasm_rust::Output<Option<bool>>,
    /// Determines which tunnel protocol to use. Available values: `""`, `wireguard`, `masque`. Defaults to `wireguard`.
    pub tunnel_protocol: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: DeviceSettingsPolicyArgs) -> DeviceSettingsPolicyResult {

    let result = crate::bindings::pulumi::cloudflare::device_settings_policy::invoke(name, &crate::bindings::pulumi::cloudflare::device_settings_policy::Args {
        account_id: &args.account_id.get_inner(),
        allow_mode_switch: &args.allow_mode_switch.get_inner(),
        allow_updates: &args.allow_updates.get_inner(),
        allowed_to_leave: &args.allowed_to_leave.get_inner(),
        auto_connect: &args.auto_connect.get_inner(),
        captive_portal: &args.captive_portal.get_inner(),
        default: &args.default.get_inner(),
        description: &args.description.get_inner(),
        disable_auto_fallback: &args.disable_auto_fallback.get_inner(),
        enabled: &args.enabled.get_inner(),
        exclude_office_ips: &args.exclude_office_ips.get_inner(),
        match_: &args.match_.get_inner(),
        name: &args.name.get_inner(),
        precedence: &args.precedence.get_inner(),
        service_mode_v2_mode: &args.service_mode_v2_mode.get_inner(),
        service_mode_v2_port: &args.service_mode_v2_port.get_inner(),
        support_url: &args.support_url.get_inner(),
        switch_locked: &args.switch_locked.get_inner(),
        tunnel_protocol: &args.tunnel_protocol.get_inner(),
    });

    DeviceSettingsPolicyResult {
        account_id: crate::into_domain(result.account_id),
        allow_mode_switch: crate::into_domain(result.allow_mode_switch),
        allow_updates: crate::into_domain(result.allow_updates),
        allowed_to_leave: crate::into_domain(result.allowed_to_leave),
        auto_connect: crate::into_domain(result.auto_connect),
        captive_portal: crate::into_domain(result.captive_portal),
        default: crate::into_domain(result.default),
        description: crate::into_domain(result.description),
        disable_auto_fallback: crate::into_domain(result.disable_auto_fallback),
        enabled: crate::into_domain(result.enabled),
        exclude_office_ips: crate::into_domain(result.exclude_office_ips),
        match_: crate::into_domain(result.match_),
        name: crate::into_domain(result.name),
        precedence: crate::into_domain(result.precedence),
        service_mode_v2_mode: crate::into_domain(result.service_mode_v2_mode),
        service_mode_v2_port: crate::into_domain(result.service_mode_v2_port),
        support_url: crate::into_domain(result.support_url),
        switch_locked: crate::into_domain(result.switch_locked),
        tunnel_protocol: crate::into_domain(result.tunnel_protocol),
    }
}
