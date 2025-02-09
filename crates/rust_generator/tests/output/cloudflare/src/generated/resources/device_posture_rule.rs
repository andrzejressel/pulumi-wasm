/// Provides a Cloudflare Device Posture Rule resource. Device posture rules configure security policies for device posture checks.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let eaxmple = device_posture_rule::create(
///         "eaxmple",
///         DevicePostureRuleArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .description("Device posture rule for corporate devices.")
///             .expiration("24h")
///             .inputs(
///                 vec![
///                     DevicePostureRuleInput::builder().id("${corporateDevices.id}")
///                     .operator("<").osDistroName("ubuntu").osDistroRevision("1.0.0")
///                     .osVersionExtra("(a)").version("1.0.0").build_struct(),
///                 ],
///             )
///             .matches(
///                 vec![DevicePostureRuleMatch::builder().platform("linux").build_struct(),],
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
/// $ pulumi import cloudflare:index/devicePostureRule:DevicePostureRule example <account_id>/<device_posture_rule_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod device_posture_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DevicePostureRuleArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Expire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
        #[builder(into, default)]
        pub expiration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required for all rule types except `warp`, `gateway`, and `tanium`.
        #[builder(into, default)]
        pub inputs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::DevicePostureRuleInput>>,
        >,
        /// The conditions that the client must match to run the rule.
        #[builder(into, default)]
        pub matches: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::DevicePostureRuleMatch>>,
        >,
        /// Name of the device posture rule.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
        #[builder(into, default)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DevicePostureRuleResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Expire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
        pub expiration: pulumi_gestalt_rust::Output<Option<String>>,
        /// Required for all rule types except `warp`, `gateway`, and `tanium`.
        pub inputs: pulumi_gestalt_rust::Output<
            Vec<super::types::DevicePostureRuleInput>,
        >,
        /// The conditions that the client must match to run the rule.
        pub matches: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::DevicePostureRuleMatch>>,
        >,
        /// Name of the device posture rule.
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Tells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
        pub schedule: pulumi_gestalt_rust::Output<Option<String>>,
        /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DevicePostureRuleArgs,
    ) -> DevicePostureRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let expiration_binding = args.expiration.get_output(context);
        let inputs_binding = args.inputs.get_output(context);
        let matches_binding = args.matches.get_output(context);
        let name_binding = args.name.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/devicePostureRule:DevicePostureRule".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expiration".into(),
                    value: expiration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputs".into(),
                    value: inputs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "matches".into(),
                    value: matches_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: schedule_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DevicePostureRuleResult {
            account_id: o.get_field("accountId"),
            description: o.get_field("description"),
            expiration: o.get_field("expiration"),
            inputs: o.get_field("inputs"),
            matches: o.get_field("matches"),
            name: o.get_field("name"),
            schedule: o.get_field("schedule"),
            type_: o.get_field("type"),
        }
    }
}
