/// Manages a Virtual Hub Routing Intent.
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
///     let exampleFirewall = firewall::create(
///         "exampleFirewall",
///         FirewallArgs::builder()
///             .location("${example.location}")
///             .name("example-fw")
///             .resource_group_name("${example.name}")
///             .sku_name("AZFW_Hub")
///             .sku_tier("Standard")
///             .virtual_hub(
///                 FirewallVirtualHub::builder()
///                     .publicIpCount(1)
///                     .virtualHubId("${exampleVirtualHub.id}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleRoutingIntent = routing_intent::create(
///         "exampleRoutingIntent",
///         RoutingIntentArgs::builder()
///             .name("example-routingintent")
///             .routing_policies(
///                 vec![
///                     RoutingIntentRoutingPolicy::builder().destinations(vec!["Internet",])
///                     .name("InternetTrafficPolicy").nextHop("${exampleFirewall.id}")
///                     .build_struct(),
///                 ],
///             )
///             .virtual_hub_id("${exampleVirtualHub.id}")
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
/// Virtual Hub Routing Intents can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/routingIntent:RoutingIntent example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Network/virtualHubs/virtualHub1/routingIntent/routingIntent1
/// ```
///
pub mod routing_intent {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RoutingIntentArgs {
        /// The name which should be used for this Virtual Hub Routing Intent. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `routing_policy` blocks as defined below.
        #[builder(into)]
        pub routing_policies: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::RoutingIntentRoutingPolicy>,
        >,
        /// The resource ID of the Virtual Hub. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RoutingIntentResult {
        /// The name which should be used for this Virtual Hub Routing Intent. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `routing_policy` blocks as defined below.
        pub routing_policies: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::RoutingIntentRoutingPolicy>,
        >,
        /// The resource ID of the Virtual Hub. Changing this forces a new resource to be created.
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RoutingIntentArgs) -> RoutingIntentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let routing_policies_binding = args.routing_policies.get_inner();
        let virtual_hub_id_binding = args.virtual_hub_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/routingIntent:RoutingIntent".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "routingPolicies".into(),
                    value: &routing_policies_binding,
                },
                register_interface::ObjectField {
                    name: "virtualHubId".into(),
                    value: &virtual_hub_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "routingPolicies".into(),
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
        RoutingIntentResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            routing_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routingPolicies").unwrap(),
            ),
            virtual_hub_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualHubId").unwrap(),
            ),
        }
    }
}
