/// Provides the ability to manage IP addresses that can be used by DNS records when
/// they are proxied through Cloudflare.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = address_map::create(
///         "example",
///         AddressMapArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .default_sni("*.example.com")
///             .description("My address map")
///             .enabled(true)
///             .ips(
///                 vec![
///                     AddressMapIp::builder().ip("192.0.2.1").build_struct(),
///                     AddressMapIp::builder().ip("203.0.113.1").build_struct(),
///                 ],
///             )
///             .memberships(
///                 vec![
///                     AddressMapMembership::builder()
///                     .identifier("92f17202ed8bd63d69a66b86a49a8f6b").kind("account")
///                     .build_struct(), AddressMapMembership::builder()
///                     .identifier("023e105f4ecef8ad9ca31a8372d0c353").kind("zone")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/addressMap:AddressMap example <account_id>/<address_map_id>
/// ```
///
pub mod address_map {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AddressMapArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// If you have legacy TLS clients which do not send the TLS server name indicator, then you can specify one default SNI on the map.
        #[builder(into, default)]
        pub default_sni: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Description of the address map.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether the Address Map is enabled or not.
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<bool>,
        /// The set of IPs on the Address Map.
        #[builder(into, default)]
        pub ips: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::AddressMapIp>>,
        >,
        /// Zones and Accounts which will be assigned IPs on this Address Map.
        #[builder(into, default)]
        pub memberships: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::AddressMapMembership>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AddressMapResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// If set to false, then the Address Map cannot be deleted via API. This is true for Cloudflare-managed maps.
        pub can_delete: pulumi_wasm_rust::Output<bool>,
        /// If set to false, then the IPs on the Address Map cannot be modified via the API. This is true for Cloudflare-managed maps.
        pub can_modify_ips: pulumi_wasm_rust::Output<bool>,
        /// If you have legacy TLS clients which do not send the TLS server name indicator, then you can specify one default SNI on the map.
        pub default_sni: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the address map.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the Address Map is enabled or not.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The set of IPs on the Address Map.
        pub ips: pulumi_wasm_rust::Output<Option<Vec<super::types::AddressMapIp>>>,
        /// Zones and Accounts which will be assigned IPs on this Address Map.
        pub memberships: pulumi_wasm_rust::Output<
            Option<Vec<super::types::AddressMapMembership>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AddressMapArgs,
    ) -> AddressMapResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let default_sni_binding = args.default_sni.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let ips_binding = args.ips.get_output(context).get_inner();
        let memberships_binding = args.memberships.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/addressMap:AddressMap".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "defaultSni".into(),
                    value: &default_sni_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "ips".into(),
                    value: &ips_binding,
                },
                register_interface::ObjectField {
                    name: "memberships".into(),
                    value: &memberships_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AddressMapResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            can_delete: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("canDelete"),
            ),
            can_modify_ips: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("canModifyIps"),
            ),
            default_sni: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultSni"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            ips: pulumi_wasm_rust::__private::into_domain(o.extract_field("ips")),
            memberships: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("memberships"),
            ),
        }
    }
}
