/// Provides a Cloudflare Device Posture Rule resource. Device posture rules configure security policies for device posture checks.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let eaxmple = zero_trust_device_posture_rule::create(
///         "eaxmple",
///         ZeroTrustDevicePostureRuleArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .description("Device posture rule for corporate devices.")
///             .expiration("24h")
///             .inputs(
///                 vec![
///                     ZeroTrustDevicePostureRuleInput::builder()
///                     .id("${corporateDevices.id}").operator("<").osDistroName("ubuntu")
///                     .osDistroRevision("1.0.0").osVersionExtra("(a)").version("1.0.0")
///                     .build_struct(),
///                 ],
///             )
///             .matches(
///                 vec![
///                     ZeroTrustDevicePostureRuleMatch::builder().platform("linux")
///                     .build_struct(),
///                 ],
///             )
///             .name("Corporate devices posture rule")
///             .schedule("24h")
///             .type_("os_version")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustDevicePostureRule:ZeroTrustDevicePostureRule example <account_id>/<device_posture_rule_id>
/// ```
///
pub mod zero_trust_device_posture_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustDevicePostureRuleArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Expire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
        #[builder(into, default)]
        pub expiration: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Required for all rule types except `warp`, `gateway`, and `tanium`.
        #[builder(into, default)]
        pub inputs: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustDevicePostureRuleInput>>,
        >,
        /// The conditions that the client must match to run the rule.
        #[builder(into, default)]
        pub matches: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustDevicePostureRuleMatch>>,
        >,
        /// Name of the device posture rule.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Tells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
        #[builder(into, default)]
        pub schedule: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustDevicePostureRuleResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Expire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
        pub expiration: pulumi_wasm_rust::Output<Option<String>>,
        /// Required for all rule types except `warp`, `gateway`, and `tanium`.
        pub inputs: pulumi_wasm_rust::Output<
            Vec<super::types::ZeroTrustDevicePostureRuleInput>,
        >,
        /// The conditions that the client must match to run the rule.
        pub matches: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ZeroTrustDevicePostureRuleMatch>>,
        >,
        /// Name of the device posture rule.
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Tells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
        pub schedule: pulumi_wasm_rust::Output<Option<String>>,
        /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ZeroTrustDevicePostureRuleArgs,
    ) -> ZeroTrustDevicePostureRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let expiration_binding = args.expiration.get_output(context).get_inner();
        let inputs_binding = args.inputs.get_output(context).get_inner();
        let matches_binding = args.matches.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let schedule_binding = args.schedule.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustDevicePostureRule:ZeroTrustDevicePostureRule"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "expiration".into(),
                    value: &expiration_binding,
                },
                register_interface::ObjectField {
                    name: "inputs".into(),
                    value: &inputs_binding,
                },
                register_interface::ObjectField {
                    name: "matches".into(),
                    value: &matches_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "expiration".into(),
                },
                register_interface::ResultField {
                    name: "inputs".into(),
                },
                register_interface::ResultField {
                    name: "matches".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "schedule".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ZeroTrustDevicePostureRuleResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            expiration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiration").unwrap(),
            ),
            inputs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputs").unwrap(),
            ),
            matches: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("matches").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedule").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
