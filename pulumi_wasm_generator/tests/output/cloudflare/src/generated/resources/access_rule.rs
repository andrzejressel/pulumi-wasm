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
pub mod access_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessRuleArgs {
        /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Rule configuration to apply to a matched request. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub configuration: pulumi_wasm_rust::Output<
            super::types::AccessRuleConfiguration,
        >,
        /// The action to apply to a matched request. Available values: `block`, `challenge`, `whitelist`, `js_challenge`, `managed_challenge`.
        #[builder(into)]
        pub mode: pulumi_wasm_rust::Output<String>,
        /// A personal note about the rule. Typically used as a reminder or explanation for the rule.
        #[builder(into, default)]
        pub notes: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccessRuleResult {
        /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Rule configuration to apply to a matched request. **Modifying this attribute will force creation of a new resource.**
        pub configuration: pulumi_wasm_rust::Output<
            super::types::AccessRuleConfiguration,
        >,
        /// The action to apply to a matched request. Available values: `block`, `challenge`, `whitelist`, `js_challenge`, `managed_challenge`.
        pub mode: pulumi_wasm_rust::Output<String>,
        /// A personal note about the rule. Typically used as a reminder or explanation for the rule.
        pub notes: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccessRuleArgs) -> AccessRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let configuration_binding = args.configuration.get_inner();
        let mode_binding = args.mode.get_inner();
        let notes_binding = args.notes.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/accessRule:AccessRule".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "notes".into(),
                    value: &notes_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "configuration".into(),
                },
                register_interface::ResultField {
                    name: "mode".into(),
                },
                register_interface::ResultField {
                    name: "notes".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessRuleResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configuration").unwrap(),
            ),
            mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mode").unwrap(),
            ),
            notes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notes").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
