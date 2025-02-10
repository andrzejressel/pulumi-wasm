/// Provides the ability to manage IP addresses that can be used by DNS records when
/// they are proxied through Cloudflare.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod address_map {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AddressMapArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If you have legacy TLS clients which do not send the TLS server name indicator, then you can specify one default SNI on the map.
        #[builder(into, default)]
        pub default_sni: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description of the address map.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the Address Map is enabled or not.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The set of IPs on the Address Map.
        #[builder(into, default)]
        pub ips: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::AddressMapIp>>,
        >,
        /// Zones and Accounts which will be assigned IPs on this Address Map.
        #[builder(into, default)]
        pub memberships: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::AddressMapMembership>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AddressMapResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// If set to false, then the Address Map cannot be deleted via API. This is true for Cloudflare-managed maps.
        pub can_delete: pulumi_gestalt_rust::Output<bool>,
        /// If set to false, then the IPs on the Address Map cannot be modified via the API. This is true for Cloudflare-managed maps.
        pub can_modify_ips: pulumi_gestalt_rust::Output<bool>,
        /// If you have legacy TLS clients which do not send the TLS server name indicator, then you can specify one default SNI on the map.
        pub default_sni: pulumi_gestalt_rust::Output<Option<String>>,
        /// Description of the address map.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the Address Map is enabled or not.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The set of IPs on the Address Map.
        pub ips: pulumi_gestalt_rust::Output<Option<Vec<super::types::AddressMapIp>>>,
        /// Zones and Accounts which will be assigned IPs on this Address Map.
        pub memberships: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::AddressMapMembership>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AddressMapArgs,
    ) -> AddressMapResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let default_sni_binding = args.default_sni.get_output(context);
        let description_binding = args.description.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let ips_binding = args.ips.get_output(context);
        let memberships_binding = args.memberships.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/addressMap:AddressMap".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultSni".into(),
                    value: default_sni_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ips".into(),
                    value: ips_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "memberships".into(),
                    value: memberships_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AddressMapResult {
            account_id: o.get_field("accountId"),
            can_delete: o.get_field("canDelete"),
            can_modify_ips: o.get_field("canModifyIps"),
            default_sni: o.get_field("defaultSni"),
            description: o.get_field("description"),
            enabled: o.get_field("enabled"),
            ips: o.get_field("ips"),
            memberships: o.get_field("memberships"),
        }
    }
}
