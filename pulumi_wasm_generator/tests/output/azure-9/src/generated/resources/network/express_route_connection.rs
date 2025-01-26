/// Manages an Express Route Connection.
///
/// > **NOTE:** The provider status of the Express Route Circuit must be set as provisioned while creating the Express Route Connection. See more details [here](https://docs.microsoft.com/azure/expressroute/expressroute-howto-circuit-portal-resource-manager#send-the-service-key-to-your-connectivity-provider-for-provisioning).
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
///     let exampleExpressRouteCircuit = express_route_circuit::create(
///         "exampleExpressRouteCircuit",
///         ExpressRouteCircuitArgs::builder()
///             .bandwidth_in_gbps(5)
///             .express_route_port_id("${exampleExpressRoutePort.id}")
///             .location("${example.location}")
///             .name("example-erc")
///             .resource_group_name("${example.name}")
///             .sku(
///                 ExpressRouteCircuitSku::builder()
///                     .family("MeteredData")
///                     .tier("Standard")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleExpressRouteCircuitPeering = express_route_circuit_peering::create(
///         "exampleExpressRouteCircuitPeering",
///         ExpressRouteCircuitPeeringArgs::builder()
///             .express_route_circuit_name("${exampleExpressRouteCircuit.name}")
///             .peer_asn(100)
///             .peering_type("AzurePrivatePeering")
///             .primary_peer_address_prefix("192.168.1.0/30")
///             .resource_group_name("${example.name}")
///             .secondary_peer_address_prefix("192.168.2.0/30")
///             .shared_key("ItsASecret")
///             .vlan_id(100)
///             .build_struct(),
///     );
///     let exampleExpressRouteConnection = express_route_connection::create(
///         "exampleExpressRouteConnection",
///         ExpressRouteConnectionArgs::builder()
///             .express_route_circuit_peering_id("${exampleExpressRouteCircuitPeering.id}")
///             .express_route_gateway_id("${exampleExpressRouteGateway.id}")
///             .name("example-expressrouteconn")
///             .build_struct(),
///     );
///     let exampleExpressRouteGateway = express_route_gateway::create(
///         "exampleExpressRouteGateway",
///         ExpressRouteGatewayArgs::builder()
///             .location("${example.location}")
///             .name("example-expressroutegateway")
///             .resource_group_name("${example.name}")
///             .scale_units(1)
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .build_struct(),
///     );
///     let exampleExpressRoutePort = express_route_port::create(
///         "exampleExpressRoutePort",
///         ExpressRoutePortArgs::builder()
///             .bandwidth_in_gbps(10)
///             .encapsulation("Dot1Q")
///             .location("${example.location}")
///             .name("example-erp")
///             .peering_location("Equinix-Seattle-SE2")
///             .resource_group_name("${example.name}")
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
/// }
/// ```
///
/// ## Import
///
/// Express Route Connections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/expressRouteConnection:ExpressRouteConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/expressRouteGateways/expressRouteGateway1/expressRouteConnections/connection1
/// ```
///
pub mod express_route_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExpressRouteConnectionArgs {
        /// The authorization key to establish the Express Route Connection.
        #[builder(into, default)]
        pub authorization_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Is Internet security enabled for this Express Route Connection?
        #[builder(into, default)]
        pub enable_internet_security: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Express Route Circuit Peering that this Express Route Connection connects with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub express_route_circuit_peering_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specified whether Fast Path is enabled for Virtual Wan Firewall Hub. Defaults to `false`.
        #[builder(into, default)]
        pub express_route_gateway_bypass_enabled: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The ID of the Express Route Gateway that this Express Route Connection connects with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub express_route_gateway_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this Express Route Connection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Bypass the Express Route gateway when accessing private-links. When enabled `express_route_gateway_bypass_enabled` must be set to `true`.
        #[builder(into, default)]
        pub private_link_fast_path_enabled: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A `routing` block as defined below.
        #[builder(into, default)]
        pub routing: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::network::ExpressRouteConnectionRouting>,
        >,
        /// The routing weight associated to the Express Route Connection. Possible value is between `0` and `32000`. Defaults to `0`.
        #[builder(into, default)]
        pub routing_weight: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct ExpressRouteConnectionResult {
        /// The authorization key to establish the Express Route Connection.
        pub authorization_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Is Internet security enabled for this Express Route Connection?
        pub enable_internet_security: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Express Route Circuit Peering that this Express Route Connection connects with. Changing this forces a new resource to be created.
        pub express_route_circuit_peering_id: pulumi_wasm_rust::Output<String>,
        /// Specified whether Fast Path is enabled for Virtual Wan Firewall Hub. Defaults to `false`.
        pub express_route_gateway_bypass_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Express Route Gateway that this Express Route Connection connects with. Changing this forces a new resource to be created.
        pub express_route_gateway_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Express Route Connection. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Bypass the Express Route gateway when accessing private-links. When enabled `express_route_gateway_bypass_enabled` must be set to `true`.
        pub private_link_fast_path_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `routing` block as defined below.
        pub routing: pulumi_wasm_rust::Output<
            super::super::types::network::ExpressRouteConnectionRouting,
        >,
        /// The routing weight associated to the Express Route Connection. Possible value is between `0` and `32000`. Defaults to `0`.
        pub routing_weight: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ExpressRouteConnectionArgs,
    ) -> ExpressRouteConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authorization_key_binding = args
            .authorization_key
            .get_output(context)
            .get_inner();
        let enable_internet_security_binding = args
            .enable_internet_security
            .get_output(context)
            .get_inner();
        let express_route_circuit_peering_id_binding = args
            .express_route_circuit_peering_id
            .get_output(context)
            .get_inner();
        let express_route_gateway_bypass_enabled_binding = args
            .express_route_gateway_bypass_enabled
            .get_output(context)
            .get_inner();
        let express_route_gateway_id_binding = args
            .express_route_gateway_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let private_link_fast_path_enabled_binding = args
            .private_link_fast_path_enabled
            .get_output(context)
            .get_inner();
        let routing_binding = args.routing.get_output(context).get_inner();
        let routing_weight_binding = args.routing_weight.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/expressRouteConnection:ExpressRouteConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authorizationKey".into(),
                    value: &authorization_key_binding,
                },
                register_interface::ObjectField {
                    name: "enableInternetSecurity".into(),
                    value: &enable_internet_security_binding,
                },
                register_interface::ObjectField {
                    name: "expressRouteCircuitPeeringId".into(),
                    value: &express_route_circuit_peering_id_binding,
                },
                register_interface::ObjectField {
                    name: "expressRouteGatewayBypassEnabled".into(),
                    value: &express_route_gateway_bypass_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "expressRouteGatewayId".into(),
                    value: &express_route_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateLinkFastPathEnabled".into(),
                    value: &private_link_fast_path_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "routing".into(),
                    value: &routing_binding,
                },
                register_interface::ObjectField {
                    name: "routingWeight".into(),
                    value: &routing_weight_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authorizationKey".into(),
                },
                register_interface::ResultField {
                    name: "enableInternetSecurity".into(),
                },
                register_interface::ResultField {
                    name: "expressRouteCircuitPeeringId".into(),
                },
                register_interface::ResultField {
                    name: "expressRouteGatewayBypassEnabled".into(),
                },
                register_interface::ResultField {
                    name: "expressRouteGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateLinkFastPathEnabled".into(),
                },
                register_interface::ResultField {
                    name: "routing".into(),
                },
                register_interface::ResultField {
                    name: "routingWeight".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ExpressRouteConnectionResult {
            authorization_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizationKey").unwrap(),
            ),
            enable_internet_security: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableInternetSecurity").unwrap(),
            ),
            express_route_circuit_peering_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expressRouteCircuitPeeringId").unwrap(),
            ),
            express_route_gateway_bypass_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expressRouteGatewayBypassEnabled").unwrap(),
            ),
            express_route_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expressRouteGatewayId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_link_fast_path_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateLinkFastPathEnabled").unwrap(),
            ),
            routing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routing").unwrap(),
            ),
            routing_weight: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routingWeight").unwrap(),
            ),
        }
    }
}
