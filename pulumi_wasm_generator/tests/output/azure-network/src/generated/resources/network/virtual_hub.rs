/// Manages a Virtual Hub within a Virtual WAN.
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
///             .address_prefix("10.0.0.0/23")
///             .location("${example.location}")
///             .name("example-virtualhub")
///             .resource_group_name("${example.name}")
///             .virtual_wan_id("${exampleVirtualWan.id}")
///             .build_struct(),
///     );
///     let exampleVirtualWan = virtual_wan::create(
///         "exampleVirtualWan",
///         VirtualWanArgs::builder()
///             .location("${example.location}")
///             .name("example-virtualwan")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Virtual Hub's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/virtualHub:VirtualHub example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/virtualHubs/hub1
/// ```
///
pub mod virtual_hub {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualHubArgs {
        /// The Address Prefix which should be used for this Virtual Hub. Changing this forces a new resource to be created. [The address prefix subnet cannot be smaller than a `/24`. Azure recommends using a `/23`](https://docs.microsoft.com/azure/virtual-wan/virtual-wan-faq#what-is-the-recommended-hub-address-space-during-hub-creation).
        #[builder(into, default)]
        pub address_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The hub routing preference. Possible values are `ExpressRoute`, `ASPath` and `VpnGateway`. Defaults to `ExpressRoute`.
        #[builder(into, default)]
        pub hub_routing_preference: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the Virtual Hub should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Virtual Hub. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Resource Group where the Virtual Hub should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `route` blocks as defined below.
        #[builder(into, default)]
        pub routes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::VirtualHubRoute>>,
        >,
        /// The SKU of the Virtual Hub. Possible values are `Basic` and `Standard`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the Virtual Hub.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Minimum instance capacity for the scaling configuration of the Virtual Hub Router. Defaults to `2`.
        #[builder(into, default)]
        pub virtual_router_auto_scale_min_capacity: pulumi_wasm_rust::Output<
            Option<i32>,
        >,
        /// The ID of a Virtual WAN within which the Virtual Hub should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub virtual_wan_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VirtualHubResult {
        /// The Address Prefix which should be used for this Virtual Hub. Changing this forces a new resource to be created. [The address prefix subnet cannot be smaller than a `/24`. Azure recommends using a `/23`](https://docs.microsoft.com/azure/virtual-wan/virtual-wan-faq#what-is-the-recommended-hub-address-space-during-hub-creation).
        pub address_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the default Route Table in the Virtual Hub.
        pub default_route_table_id: pulumi_wasm_rust::Output<String>,
        /// The hub routing preference. Possible values are `ExpressRoute`, `ASPath` and `VpnGateway`. Defaults to `ExpressRoute`.
        pub hub_routing_preference: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the Virtual Hub should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Virtual Hub. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Virtual Hub should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `route` blocks as defined below.
        pub routes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::VirtualHubRoute>>,
        >,
        /// The SKU of the Virtual Hub. Possible values are `Basic` and `Standard`. Changing this forces a new resource to be created.
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the Virtual Hub.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Autonomous System Number of the Virtual Hub BGP router.
        pub virtual_router_asn: pulumi_wasm_rust::Output<i32>,
        /// Minimum instance capacity for the scaling configuration of the Virtual Hub Router. Defaults to `2`.
        pub virtual_router_auto_scale_min_capacity: pulumi_wasm_rust::Output<
            Option<i32>,
        >,
        /// The IP addresses of the Virtual Hub BGP router.
        pub virtual_router_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of a Virtual WAN within which the Virtual Hub should be created. Changing this forces a new resource to be created.
        pub virtual_wan_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VirtualHubArgs) -> VirtualHubResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let address_prefix_binding = args.address_prefix.get_inner();
        let hub_routing_preference_binding = args.hub_routing_preference.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let routes_binding = args.routes.get_inner();
        let sku_binding = args.sku.get_inner();
        let tags_binding = args.tags.get_inner();
        let virtual_router_auto_scale_min_capacity_binding = args
            .virtual_router_auto_scale_min_capacity
            .get_inner();
        let virtual_wan_id_binding = args.virtual_wan_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/virtualHub:VirtualHub".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addressPrefix".into(),
                    value: &address_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "hubRoutingPreference".into(),
                    value: &hub_routing_preference_binding,
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
                    name: "routes".into(),
                    value: &routes_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "virtualRouterAutoScaleMinCapacity".into(),
                    value: &virtual_router_auto_scale_min_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "virtualWanId".into(),
                    value: &virtual_wan_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addressPrefix".into(),
                },
                register_interface::ResultField {
                    name: "defaultRouteTableId".into(),
                },
                register_interface::ResultField {
                    name: "hubRoutingPreference".into(),
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
                    name: "routes".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "virtualRouterAsn".into(),
                },
                register_interface::ResultField {
                    name: "virtualRouterAutoScaleMinCapacity".into(),
                },
                register_interface::ResultField {
                    name: "virtualRouterIps".into(),
                },
                register_interface::ResultField {
                    name: "virtualWanId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualHubResult {
            address_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressPrefix").unwrap(),
            ),
            default_route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultRouteTableId").unwrap(),
            ),
            hub_routing_preference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hubRoutingPreference").unwrap(),
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
            routes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routes").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            virtual_router_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualRouterAsn").unwrap(),
            ),
            virtual_router_auto_scale_min_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualRouterAutoScaleMinCapacity").unwrap(),
            ),
            virtual_router_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualRouterIps").unwrap(),
            ),
            virtual_wan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualWanId").unwrap(),
            ),
        }
    }
}