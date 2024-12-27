/// Provides a Cloudflare Device Settings Policy resource. Device policies configure settings applied to WARP devices.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod device_settings_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeviceSettingsPolicyArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Whether to allow mode switch for this policy.
        #[builder(into, default)]
        pub allow_mode_switch: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to allow updates under this policy.
        #[builder(into, default)]
        pub allow_updates: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to allow devices to leave the organization. Defaults to `true`.
        #[builder(into, default)]
        pub allowed_to_leave: pulumi_wasm_rust::Output<Option<bool>>,
        /// The amount of time in seconds to reconnect after having been disabled.
        #[builder(into, default)]
        pub auto_connect: pulumi_wasm_rust::Output<Option<i32>>,
        /// The captive portal value for this policy. Defaults to `180`.
        #[builder(into, default)]
        pub captive_portal: pulumi_wasm_rust::Output<Option<i32>>,
        /// Whether the policy refers to the default account policy.
        #[builder(into, default)]
        pub default: pulumi_wasm_rust::Output<Option<bool>>,
        /// Description of Policy.
        #[builder(into)]
        pub description: pulumi_wasm_rust::Output<String>,
        /// Whether to disable auto fallback for this policy.
        #[builder(into, default)]
        pub disable_auto_fallback: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the policy is enabled (cannot be set for default policies). Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to add Microsoft IPs to split tunnel exclusions.
        #[builder(into, default)]
        pub exclude_office_ips: pulumi_wasm_rust::Output<Option<bool>>,
        /// Wirefilter expression to match a device against when evaluating whether this policy should take effect for that device.
        #[builder(into, default)]
        pub match_: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the policy.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The precedence of the policy. Lower values indicate higher precedence.
        #[builder(into, default)]
        pub precedence: pulumi_wasm_rust::Output<Option<i32>>,
        /// The service mode. Available values: `1dot1`, `warp`, `proxy`, `posture_only`, `warp_tunnel_only`. Defaults to `warp`.
        #[builder(into, default)]
        pub service_mode_v2_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The port to use for the proxy service mode. Required when using `service_mode_v2_mode`.
        #[builder(into, default)]
        pub service_mode_v2_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The support URL that will be opened when sending feedback.
        #[builder(into, default)]
        pub support_url: pulumi_wasm_rust::Output<Option<String>>,
        /// Enablement of the ZT client switch lock.
        #[builder(into, default)]
        pub switch_locked: pulumi_wasm_rust::Output<Option<bool>>,
        /// Determines which tunnel protocol to use. Available values: `""`, `wireguard`, `masque`. Defaults to `wireguard`.
        #[builder(into, default)]
        pub tunnel_protocol: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DeviceSettingsPolicyArgs,
    ) -> DeviceSettingsPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let allow_mode_switch_binding = args.allow_mode_switch.get_inner();
        let allow_updates_binding = args.allow_updates.get_inner();
        let allowed_to_leave_binding = args.allowed_to_leave.get_inner();
        let auto_connect_binding = args.auto_connect.get_inner();
        let captive_portal_binding = args.captive_portal.get_inner();
        let default_binding = args.default.get_inner();
        let description_binding = args.description.get_inner();
        let disable_auto_fallback_binding = args.disable_auto_fallback.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let exclude_office_ips_binding = args.exclude_office_ips.get_inner();
        let match__binding = args.match_.get_inner();
        let name_binding = args.name.get_inner();
        let precedence_binding = args.precedence.get_inner();
        let service_mode_v2_mode_binding = args.service_mode_v2_mode.get_inner();
        let service_mode_v2_port_binding = args.service_mode_v2_port.get_inner();
        let support_url_binding = args.support_url.get_inner();
        let switch_locked_binding = args.switch_locked.get_inner();
        let tunnel_protocol_binding = args.tunnel_protocol.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/deviceSettingsPolicy:DeviceSettingsPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "allowModeSwitch".into(),
                    value: &allow_mode_switch_binding,
                },
                register_interface::ObjectField {
                    name: "allowUpdates".into(),
                    value: &allow_updates_binding,
                },
                register_interface::ObjectField {
                    name: "allowedToLeave".into(),
                    value: &allowed_to_leave_binding,
                },
                register_interface::ObjectField {
                    name: "autoConnect".into(),
                    value: &auto_connect_binding,
                },
                register_interface::ObjectField {
                    name: "captivePortal".into(),
                    value: &captive_portal_binding,
                },
                register_interface::ObjectField {
                    name: "default".into(),
                    value: &default_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disableAutoFallback".into(),
                    value: &disable_auto_fallback_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "excludeOfficeIps".into(),
                    value: &exclude_office_ips_binding,
                },
                register_interface::ObjectField {
                    name: "match".into(),
                    value: &match__binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "precedence".into(),
                    value: &precedence_binding,
                },
                register_interface::ObjectField {
                    name: "serviceModeV2Mode".into(),
                    value: &service_mode_v2_mode_binding,
                },
                register_interface::ObjectField {
                    name: "serviceModeV2Port".into(),
                    value: &service_mode_v2_port_binding,
                },
                register_interface::ObjectField {
                    name: "supportUrl".into(),
                    value: &support_url_binding,
                },
                register_interface::ObjectField {
                    name: "switchLocked".into(),
                    value: &switch_locked_binding,
                },
                register_interface::ObjectField {
                    name: "tunnelProtocol".into(),
                    value: &tunnel_protocol_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "allowModeSwitch".into(),
                },
                register_interface::ResultField {
                    name: "allowUpdates".into(),
                },
                register_interface::ResultField {
                    name: "allowedToLeave".into(),
                },
                register_interface::ResultField {
                    name: "autoConnect".into(),
                },
                register_interface::ResultField {
                    name: "captivePortal".into(),
                },
                register_interface::ResultField {
                    name: "default".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disableAutoFallback".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "excludeOfficeIps".into(),
                },
                register_interface::ResultField {
                    name: "match".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "precedence".into(),
                },
                register_interface::ResultField {
                    name: "serviceModeV2Mode".into(),
                },
                register_interface::ResultField {
                    name: "serviceModeV2Port".into(),
                },
                register_interface::ResultField {
                    name: "supportUrl".into(),
                },
                register_interface::ResultField {
                    name: "switchLocked".into(),
                },
                register_interface::ResultField {
                    name: "tunnelProtocol".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DeviceSettingsPolicyResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            allow_mode_switch: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowModeSwitch").unwrap(),
            ),
            allow_updates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowUpdates").unwrap(),
            ),
            allowed_to_leave: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedToLeave").unwrap(),
            ),
            auto_connect: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoConnect").unwrap(),
            ),
            captive_portal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("captivePortal").unwrap(),
            ),
            default: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("default").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disable_auto_fallback: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableAutoFallback").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            exclude_office_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("excludeOfficeIps").unwrap(),
            ),
            match_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("match").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            precedence: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("precedence").unwrap(),
            ),
            service_mode_v2_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceModeV2Mode").unwrap(),
            ),
            service_mode_v2_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceModeV2Port").unwrap(),
            ),
            support_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportUrl").unwrap(),
            ),
            switch_locked: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("switchLocked").unwrap(),
            ),
            tunnel_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnelProtocol").unwrap(),
            ),
        }
    }
}
