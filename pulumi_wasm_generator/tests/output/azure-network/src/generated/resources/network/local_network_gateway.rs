/// Manages a local network gateway connection over which specific connections can be configured.
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
///             .name("localNetworkGWTest")
///             .build_struct(),
///     );
///     let home = local_network_gateway::create(
///         "home",
///         LocalNetworkGatewayArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .gateway_address("12.13.14.15")
///             .location("${example.location}")
///             .name("backHome")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Local Network Gateways can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/localNetworkGateway:LocalNetworkGateway lng1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/localNetworkGateways/lng1
/// ```
///
pub mod local_network_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalNetworkGatewayArgs {
        /// The list of string CIDRs representing the address spaces the gateway exposes.
        #[builder(into, default)]
        pub address_spaces: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `bgp_settings` block as defined below containing the Local Network Gateway's BGP speaker settings.
        #[builder(into, default)]
        pub bgp_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::network::LocalNetworkGatewayBgpSettings>,
        >,
        /// The gateway IP address to connect with.
        #[builder(into, default)]
        pub gateway_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The gateway FQDN to connect with.
        ///
        /// > **NOTE:** Either `gateway_address` or `gateway_fqdn` should be specified.
        #[builder(into, default)]
        pub gateway_fqdn: pulumi_wasm_rust::Output<Option<String>>,
        /// The location/region where the local network gateway is created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the local network gateway. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the local network gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LocalNetworkGatewayResult {
        /// The list of string CIDRs representing the address spaces the gateway exposes.
        pub address_spaces: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `bgp_settings` block as defined below containing the Local Network Gateway's BGP speaker settings.
        pub bgp_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::network::LocalNetworkGatewayBgpSettings>,
        >,
        /// The gateway IP address to connect with.
        pub gateway_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The gateway FQDN to connect with.
        ///
        /// > **NOTE:** Either `gateway_address` or `gateway_fqdn` should be specified.
        pub gateway_fqdn: pulumi_wasm_rust::Output<Option<String>>,
        /// The location/region where the local network gateway is created. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the local network gateway. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the local network gateway. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: LocalNetworkGatewayArgs,
    ) -> LocalNetworkGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let address_spaces_binding = args.address_spaces.get_inner();
        let bgp_settings_binding = args.bgp_settings.get_inner();
        let gateway_address_binding = args.gateway_address.get_inner();
        let gateway_fqdn_binding = args.gateway_fqdn.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/localNetworkGateway:LocalNetworkGateway".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addressSpaces".into(),
                    value: &address_spaces_binding,
                },
                register_interface::ObjectField {
                    name: "bgpSettings".into(),
                    value: &bgp_settings_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayAddress".into(),
                    value: &gateway_address_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayFqdn".into(),
                    value: &gateway_fqdn_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addressSpaces".into(),
                },
                register_interface::ResultField {
                    name: "bgpSettings".into(),
                },
                register_interface::ResultField {
                    name: "gatewayAddress".into(),
                },
                register_interface::ResultField {
                    name: "gatewayFqdn".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LocalNetworkGatewayResult {
            address_spaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressSpaces").unwrap(),
            ),
            bgp_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpSettings").unwrap(),
            ),
            gateway_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayAddress").unwrap(),
            ),
            gateway_fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayFqdn").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}