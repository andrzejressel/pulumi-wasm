/// Manages a VPN Gateway NAT Rule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleVirtualHub = virtual_hub::create(
///         "exampleVirtualHub",
///         VirtualHubArgs::builder()
///             .address_prefix("10.0.1.0/24")
///             .location("${example.location}")
///             .name("example-vhub")
///             .resource_group_name("${example.name}")
///             .virtual_wan_id("${exampleVirtualWan.id}")
///             .build_struct(),
///     );
///     let exampleVirtualWan = virtual_wan::create(
///         "exampleVirtualWan",
///         VirtualWanArgs::builder()
///             .location("${example.location}")
///             .name("example-vwan")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVnpGatewayNatRule = vnp_gateway_nat_rule::create(
///         "exampleVnpGatewayNatRule",
///         VnpGatewayNatRuleArgs::builder()
///             .external_mappings(
///                 vec![
///                     VnpGatewayNatRuleExternalMapping::builder()
///                     .addressSpace("192.168.21.0/26").build_struct(),
///                 ],
///             )
///             .internal_mappings(
///                 vec![
///                     VnpGatewayNatRuleInternalMapping::builder()
///                     .addressSpace("10.4.0.0/26").build_struct(),
///                 ],
///             )
///             .name("example-vpngatewaynatrule")
///             .vpn_gateway_id("${exampleVpnGateway.id}")
///             .build_struct(),
///     );
///     let exampleVpnGateway = vpn_gateway::create(
///         "exampleVpnGateway",
///         VpnGatewayArgs::builder()
///             .location("${example.location}")
///             .name("example-vpngateway")
///             .resource_group_name("${example.name}")
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// VPN Gateway NAT Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/vnpGatewayNatRule:VnpGatewayNatRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.Network/vpnGateways/vpnGateway1/natRules/natRule1
/// ```
///
pub mod vnp_gateway_nat_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VnpGatewayNatRuleArgs {
        /// One of more `external_mapping` blocks as defined below.
        #[builder(into, default)]
        pub external_mappings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::VnpGatewayNatRuleExternalMapping>>,
        >,
        /// One of more `internal_mapping` blocks as defined below.
        #[builder(into, default)]
        pub internal_mappings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::VnpGatewayNatRuleInternalMapping>>,
        >,
        /// The ID of the IP Configuration this VPN Gateway NAT Rule applies to. Possible values are `Instance0` and `Instance1`.
        #[builder(into, default)]
        pub ip_configuration_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The source NAT direction of the VPN NAT. Possible values are `EgressSnat` and `IngressSnat`. Defaults to `EgressSnat`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this VPN Gateway NAT Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of the VPN Gateway NAT Rule. Possible values are `Dynamic` and `Static`. Defaults to `Static`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the VPN Gateway that this VPN Gateway NAT Rule belongs to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub vpn_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VnpGatewayNatRuleResult {
        /// One of more `external_mapping` blocks as defined below.
        pub external_mappings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::VnpGatewayNatRuleExternalMapping>>,
        >,
        /// One of more `internal_mapping` blocks as defined below.
        pub internal_mappings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::VnpGatewayNatRuleInternalMapping>>,
        >,
        /// The ID of the IP Configuration this VPN Gateway NAT Rule applies to. Possible values are `Instance0` and `Instance1`.
        pub ip_configuration_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The source NAT direction of the VPN NAT. Possible values are `EgressSnat` and `IngressSnat`. Defaults to `EgressSnat`. Changing this forces a new resource to be created.
        pub mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this VPN Gateway NAT Rule. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The type of the VPN Gateway NAT Rule. Possible values are `Dynamic` and `Static`. Defaults to `Static`. Changing this forces a new resource to be created.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the VPN Gateway that this VPN Gateway NAT Rule belongs to. Changing this forces a new resource to be created.
        pub vpn_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VnpGatewayNatRuleArgs) -> VnpGatewayNatRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let external_mappings_binding = args.external_mappings.get_inner();
        let internal_mappings_binding = args.internal_mappings.get_inner();
        let ip_configuration_id_binding = args.ip_configuration_id.get_inner();
        let mode_binding = args.mode.get_inner();
        let name_binding = args.name.get_inner();
        let type__binding = args.type_.get_inner();
        let vpn_gateway_id_binding = args.vpn_gateway_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/vnpGatewayNatRule:VnpGatewayNatRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "externalMappings".into(),
                    value: &external_mappings_binding,
                },
                register_interface::ObjectField {
                    name: "internalMappings".into(),
                    value: &internal_mappings_binding,
                },
                register_interface::ObjectField {
                    name: "ipConfigurationId".into(),
                    value: &ip_configuration_id_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "vpnGatewayId".into(),
                    value: &vpn_gateway_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "externalMappings".into(),
                },
                register_interface::ResultField {
                    name: "internalMappings".into(),
                },
                register_interface::ResultField {
                    name: "ipConfigurationId".into(),
                },
                register_interface::ResultField {
                    name: "mode".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "vpnGatewayId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VnpGatewayNatRuleResult {
            external_mappings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalMappings").unwrap(),
            ),
            internal_mappings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internalMappings").unwrap(),
            ),
            ip_configuration_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipConfigurationId").unwrap(),
            ),
            mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mode").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            vpn_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnGatewayId").unwrap(),
            ),
        }
    }
}
