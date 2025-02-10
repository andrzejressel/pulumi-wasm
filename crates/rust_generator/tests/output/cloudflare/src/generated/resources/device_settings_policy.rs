/// Provides a Cloudflare Device Settings Policy resource. Device policies configure settings applied to WARP devices.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let developerWarpPolicy = device_settings_policy::create(
///         "developerWarpPolicy",
///         DeviceSettingsPolicyArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .allow_mode_switch(true)
///             .allow_updates(true)
///             .allowed_to_leave(true)
///             .auto_connect(0)
///             .captive_portal(5)
///             .default(false)
///             .description("Developers WARP settings policy description")
///             .disable_auto_fallback(true)
///             .enabled(true)
///             .exclude_office_ips(false)
///             .match_("any(identity.groups.name[*] in {\"Developers\"})")
///             .name("Developers WARP settings policy")
///             .precedence(10)
///             .service_mode_v_2_mode("warp")
///             .service_mode_v_2_port(3000)
///             .support_url("https://cloudflare.com")
///             .switch_locked(true)
///             .tunnel_protocol("wireguard")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For default device settings policies you must use "default" as the policy ID.
///
/// ```sh
/// $ pulumi import cloudflare:index/deviceSettingsPolicy:DeviceSettingsPolicy example <account_id>/<device_policy_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod device_settings_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeviceSettingsPolicyArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to allow mode switch for this policy.
        #[builder(into, default)]
        pub allow_mode_switch: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to allow updates under this policy.
        #[builder(into, default)]
        pub allow_updates: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to allow devices to leave the organization. Defaults to `true`.
        #[builder(into, default)]
        pub allowed_to_leave: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The amount of time in seconds to reconnect after having been disabled.
        #[builder(into, default)]
        pub auto_connect: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The captive portal value for this policy. Defaults to `180`.
        #[builder(into, default)]
        pub captive_portal: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Whether the policy refers to the default account policy.
        #[builder(into, default)]
        pub default: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Description of Policy.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to disable auto fallback for this policy.
        #[builder(into, default)]
        pub disable_auto_fallback: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the policy is enabled (cannot be set for default policies). Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to add Microsoft IPs to split tunnel exclusions.
        #[builder(into, default)]
        pub exclude_office_ips: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Wirefilter expression to match a device against when evaluating whether this policy should take effect for that device.
        #[builder(into, default)]
        pub match_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the policy.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The precedence of the policy. Lower values indicate higher precedence.
        #[builder(into, default)]
        pub precedence: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The service mode. Available values: `1dot1`, `warp`, `proxy`, `posture_only`, `warp_tunnel_only`. Defaults to `warp`.
        #[builder(into, default)]
        pub service_mode_v2_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The port to use for the proxy service mode. Required when using `service_mode_v2_mode`.
        #[builder(into, default)]
        pub service_mode_v2_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The support URL that will be opened when sending feedback.
        #[builder(into, default)]
        pub support_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enablement of the ZT client switch lock.
        #[builder(into, default)]
        pub switch_locked: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Determines which tunnel protocol to use. Available values: `""`, `wireguard`, `masque`. Defaults to `wireguard`.
        #[builder(into, default)]
        pub tunnel_protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DeviceSettingsPolicyResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Whether to allow mode switch for this policy.
        pub allow_mode_switch: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to allow updates under this policy.
        pub allow_updates: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to allow devices to leave the organization. Defaults to `true`.
        pub allowed_to_leave: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The amount of time in seconds to reconnect after having been disabled.
        pub auto_connect: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The captive portal value for this policy. Defaults to `180`.
        pub captive_portal: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Whether the policy refers to the default account policy.
        pub default: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Description of Policy.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Whether to disable auto fallback for this policy.
        pub disable_auto_fallback: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether the policy is enabled (cannot be set for default policies). Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to add Microsoft IPs to split tunnel exclusions.
        pub exclude_office_ips: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Wirefilter expression to match a device against when evaluating whether this policy should take effect for that device.
        pub match_: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The precedence of the policy. Lower values indicate higher precedence.
        pub precedence: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The service mode. Available values: `1dot1`, `warp`, `proxy`, `posture_only`, `warp_tunnel_only`. Defaults to `warp`.
        pub service_mode_v2_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The port to use for the proxy service mode. Required when using `service_mode_v2_mode`.
        pub service_mode_v2_port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The support URL that will be opened when sending feedback.
        pub support_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// Enablement of the ZT client switch lock.
        pub switch_locked: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Determines which tunnel protocol to use. Available values: `""`, `wireguard`, `masque`. Defaults to `wireguard`.
        pub tunnel_protocol: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeviceSettingsPolicyArgs,
    ) -> DeviceSettingsPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let allow_mode_switch_binding = args.allow_mode_switch.get_output(context);
        let allow_updates_binding = args.allow_updates.get_output(context);
        let allowed_to_leave_binding = args.allowed_to_leave.get_output(context);
        let auto_connect_binding = args.auto_connect.get_output(context);
        let captive_portal_binding = args.captive_portal.get_output(context);
        let default_binding = args.default.get_output(context);
        let description_binding = args.description.get_output(context);
        let disable_auto_fallback_binding = args
            .disable_auto_fallback
            .get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let exclude_office_ips_binding = args.exclude_office_ips.get_output(context);
        let match__binding = args.match_.get_output(context);
        let name_binding = args.name.get_output(context);
        let precedence_binding = args.precedence.get_output(context);
        let service_mode_v2_mode_binding = args.service_mode_v2_mode.get_output(context);
        let service_mode_v2_port_binding = args.service_mode_v2_port.get_output(context);
        let support_url_binding = args.support_url.get_output(context);
        let switch_locked_binding = args.switch_locked.get_output(context);
        let tunnel_protocol_binding = args.tunnel_protocol.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/deviceSettingsPolicy:DeviceSettingsPolicy".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowModeSwitch".into(),
                    value: allow_mode_switch_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowUpdates".into(),
                    value: allow_updates_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowedToLeave".into(),
                    value: allowed_to_leave_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoConnect".into(),
                    value: auto_connect_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "captivePortal".into(),
                    value: captive_portal_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "default".into(),
                    value: default_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableAutoFallback".into(),
                    value: disable_auto_fallback_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludeOfficeIps".into(),
                    value: exclude_office_ips_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "match".into(),
                    value: match__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "precedence".into(),
                    value: precedence_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceModeV2Mode".into(),
                    value: service_mode_v2_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceModeV2Port".into(),
                    value: service_mode_v2_port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "supportUrl".into(),
                    value: support_url_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "switchLocked".into(),
                    value: switch_locked_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tunnelProtocol".into(),
                    value: tunnel_protocol_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DeviceSettingsPolicyResult {
            account_id: o.get_field("accountId"),
            allow_mode_switch: o.get_field("allowModeSwitch"),
            allow_updates: o.get_field("allowUpdates"),
            allowed_to_leave: o.get_field("allowedToLeave"),
            auto_connect: o.get_field("autoConnect"),
            captive_portal: o.get_field("captivePortal"),
            default: o.get_field("default"),
            description: o.get_field("description"),
            disable_auto_fallback: o.get_field("disableAutoFallback"),
            enabled: o.get_field("enabled"),
            exclude_office_ips: o.get_field("excludeOfficeIps"),
            match_: o.get_field("match"),
            name: o.get_field("name"),
            precedence: o.get_field("precedence"),
            service_mode_v2_mode: o.get_field("serviceModeV2Mode"),
            service_mode_v2_port: o.get_field("serviceModeV2Port"),
            support_url: o.get_field("supportUrl"),
            switch_locked: o.get_field("switchLocked"),
            tunnel_protocol: o.get_field("tunnelProtocol"),
        }
    }
}
