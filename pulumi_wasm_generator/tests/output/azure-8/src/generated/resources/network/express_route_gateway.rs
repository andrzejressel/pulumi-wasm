/// Manages an ExpressRoute gateway.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleVirtualWan:
///     type: azure:network:VirtualWan
///     name: example
///     properties:
///       name: example-virtualwan
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///   exampleVirtualHub:
///     type: azure:network:VirtualHub
///     name: example
///     properties:
///       name: example-virtualhub
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       virtualWanId: ${exampleVirtualWan.id}
///       addressPrefix: 10.0.1.0/24
///   exampleExpressRouteGateway:
///     type: azure:network:ExpressRouteGateway
///     name: example
///     properties:
///       name: expressRoute1
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       virtualHubId: ${exampleVirtualHub.id}
///       scaleUnits: 1
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// ExpressRoute Gateways can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/expressRouteGateway:ExpressRouteGateway example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/expressRouteGateways/myExpressRouteGateway
/// ```
///
pub mod express_route_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExpressRouteGatewayArgs {
        /// Specified whether this gateway accept traffic from non-Virtual WAN networks. Defaults to `false`.
        #[builder(into, default)]
        pub allow_non_virtual_wan_traffic: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the ExpressRoute gateway. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the ExpressRoute gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The number of scale units with which to provision the ExpressRoute gateway. Each scale unit is equal to 2Gbps, with support for up to 10 scale units (20Gbps).
        #[builder(into)]
        pub scale_units: pulumi_wasm_rust::Output<i32>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of a Virtual HUB within which the ExpressRoute gateway should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ExpressRouteGatewayResult {
        /// Specified whether this gateway accept traffic from non-Virtual WAN networks. Defaults to `false`.
        pub allow_non_virtual_wan_traffic: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the ExpressRoute gateway. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the ExpressRoute gateway. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The number of scale units with which to provision the ExpressRoute gateway. Each scale unit is equal to 2Gbps, with support for up to 10 scale units (20Gbps).
        pub scale_units: pulumi_wasm_rust::Output<i32>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of a Virtual HUB within which the ExpressRoute gateway should be created. Changing this forces a new resource to be created.
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ExpressRouteGatewayArgs,
    ) -> ExpressRouteGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_non_virtual_wan_traffic_binding = args
            .allow_non_virtual_wan_traffic
            .get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let scale_units_binding = args.scale_units.get_inner();
        let tags_binding = args.tags.get_inner();
        let virtual_hub_id_binding = args.virtual_hub_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/expressRouteGateway:ExpressRouteGateway".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowNonVirtualWanTraffic".into(),
                    value: &allow_non_virtual_wan_traffic_binding,
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
                    name: "scaleUnits".into(),
                    value: &scale_units_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "virtualHubId".into(),
                    value: &virtual_hub_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowNonVirtualWanTraffic".into(),
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
                    name: "scaleUnits".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "virtualHubId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ExpressRouteGatewayResult {
            allow_non_virtual_wan_traffic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowNonVirtualWanTraffic").unwrap(),
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
            scale_units: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scaleUnits").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            virtual_hub_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualHubId").unwrap(),
            ),
        }
    }
}
