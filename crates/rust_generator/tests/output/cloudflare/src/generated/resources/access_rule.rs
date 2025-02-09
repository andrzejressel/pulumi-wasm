/// Provides a Cloudflare IP Firewall Access Rule resource. Access
/// control can be applied on basis of IP addresses, IP ranges, AS
/// numbers or countries.
///
/// ## Example Usage
///
/// ```yaml
/// configuration:
///   # Allowlist office's network IP ranges on all account zones (or other lists of
///   # resources).
///   myOffice:
///     type: list(string)
///     default:
///       - 192.0.2.0/24
///       - 198.51.100.0/24
///       - 2001:db8::/56
/// resources:
///   # Challenge requests coming from known Tor exit nodes.
///   torExitNodes:
///     type: cloudflare:AccessRule
///     name: tor_exit_nodes
///     properties:
///       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
///       notes: Requests coming from known Tor exit nodes
///       mode: challenge
///       configuration:
///         target: country
///         value: T1
///   # Allowlist requests coming from Antarctica, but only for single zone.
///   antarctica:
///     type: cloudflare:AccessRule
///     properties:
///       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
///       notes: Requests coming from Antarctica
///       mode: whitelist
///       configuration:
///         target: country
///         value: AQ
///   officeNetwork:
///     type: cloudflare:AccessRule
///     name: office_network
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       notes: Requests coming from office network
///       mode: whitelist
///       configuration:
///         target: ip_range
///         value:
///           fn::select:
///             - ${range.value}
///             - ${myOffice}
///     options: {}
/// ```
///
/// ## Import
///
/// User level access rule import.
///
/// ```sh
/// $ pulumi import cloudflare:index/accessRule:AccessRule default user/<user_id>/<rule_id>
/// ```
///
/// Zone level access rule import.
///
/// ```sh
/// $ pulumi import cloudflare:index/accessRule:AccessRule default zone/<zone_id>/<rule_id>
/// ```
///
/// Account level access rule import.
///
/// ```sh
/// $ pulumi import cloudflare:index/accessRule:AccessRule default account/<account_id>/<rule_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessRuleArgs {
        /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Rule configuration to apply to a matched request. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<
            super::types::AccessRuleConfiguration,
        >,
        /// The action to apply to a matched request. Available values: `block`, `challenge`, `whitelist`, `js_challenge`, `managed_challenge`.
        #[builder(into)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A personal note about the rule. Typically used as a reminder or explanation for the rule.
        #[builder(into, default)]
        pub notes: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccessRuleResult {
        /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Rule configuration to apply to a matched request. **Modifying this attribute will force creation of a new resource.**
        pub configuration: pulumi_gestalt_rust::Output<
            super::types::AccessRuleConfiguration,
        >,
        /// The action to apply to a matched request. Available values: `block`, `challenge`, `whitelist`, `js_challenge`, `managed_challenge`.
        pub mode: pulumi_gestalt_rust::Output<String>,
        /// A personal note about the rule. Typically used as a reminder or explanation for the rule.
        pub notes: pulumi_gestalt_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessRuleArgs,
    ) -> AccessRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let configuration_binding = args.configuration.get_output(context);
        let mode_binding = args.mode.get_output(context);
        let notes_binding = args.notes.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/accessRule:AccessRule".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mode".into(),
                    value: mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notes".into(),
                    value: notes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccessRuleResult {
            account_id: o.get_field("accountId"),
            configuration: o.get_field("configuration"),
            mode: o.get_field("mode"),
            notes: o.get_field("notes"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
