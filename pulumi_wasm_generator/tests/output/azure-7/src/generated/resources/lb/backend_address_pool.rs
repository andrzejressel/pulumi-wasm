/// Manages a Load Balancer Backend Address Pool.
///
/// > **NOTE:** When using this resource, the Load Balancer needs to have a FrontEnd IP Configuration Attached
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
///             .name("LoadBalancerRG")
///             .build_struct(),
///     );
///     let exampleBackendAddressPool = backend_address_pool::create(
///         "exampleBackendAddressPool",
///         BackendAddressPoolArgs::builder()
///             .loadbalancer_id("${exampleLoadBalancer.id}")
///             .name("BackEndAddressPool")
///             .build_struct(),
///     );
///     let exampleLoadBalancer = load_balancer::create(
///         "exampleLoadBalancer",
///         LoadBalancerArgs::builder()
///             .frontend_ip_configurations(
///                 vec![
///                     LoadBalancerFrontendIpConfiguration::builder()
///                     .name("PublicIPAddress").publicIpAddressId("${examplePublicIp.id}")
///                     .build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("TestLoadBalancer")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("${example.location}")
///             .name("PublicIPForLB")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Load Balancer Backend Address Pools can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:lb/backendAddressPool:BackendAddressPool example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/loadBalancers/lb1/backendAddressPools/pool1
/// ```
///
pub mod backend_address_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackendAddressPoolArgs {
        /// The ID of the Load Balancer in which to create the Backend Address Pool. Changing this forces a new resource to be created.
        #[builder(into)]
        pub loadbalancer_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Backend Address Pool. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The backend address synchronous mode for the Backend Address Pool. Possible values are `Automatic` and `Manual`. This is required with `virtual_network_id`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `synchronous_mode` can set only for Load Balancer with `Standard` SKU.
        #[builder(into, default)]
        pub synchronous_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `tunnel_interface` blocks as defined below.
        #[builder(into, default)]
        pub tunnel_interfaces: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::lb::BackendAddressPoolTunnelInterface>>,
        >,
        /// The ID of the Virtual Network within which the Backend Address Pool should exist.
        #[builder(into, default)]
        pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BackendAddressPoolResult {
        /// The Backend IP Configurations associated with this Backend Address Pool.
        pub backend_ip_configurations: pulumi_wasm_rust::Output<Vec<String>>,
        /// An array of the Load Balancing Inbound NAT Rules associated with this Backend Address Pool.
        pub inbound_nat_rules: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Load Balancing Rules associated with this Backend Address Pool.
        pub load_balancing_rules: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the Load Balancer in which to create the Backend Address Pool. Changing this forces a new resource to be created.
        pub loadbalancer_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Backend Address Pool. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// An array of the Load Balancing Outbound Rules associated with this Backend Address Pool.
        pub outbound_rules: pulumi_wasm_rust::Output<Vec<String>>,
        /// The backend address synchronous mode for the Backend Address Pool. Possible values are `Automatic` and `Manual`. This is required with `virtual_network_id`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `synchronous_mode` can set only for Load Balancer with `Standard` SKU.
        pub synchronous_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `tunnel_interface` blocks as defined below.
        pub tunnel_interfaces: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::lb::BackendAddressPoolTunnelInterface>>,
        >,
        /// The ID of the Virtual Network within which the Backend Address Pool should exist.
        pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BackendAddressPoolArgs) -> BackendAddressPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let loadbalancer_id_binding = args.loadbalancer_id.get_inner();
        let name_binding = args.name.get_inner();
        let synchronous_mode_binding = args.synchronous_mode.get_inner();
        let tunnel_interfaces_binding = args.tunnel_interfaces.get_inner();
        let virtual_network_id_binding = args.virtual_network_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:lb/backendAddressPool:BackendAddressPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "loadbalancerId".into(),
                    value: &loadbalancer_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "synchronousMode".into(),
                    value: &synchronous_mode_binding,
                },
                register_interface::ObjectField {
                    name: "tunnelInterfaces".into(),
                    value: &tunnel_interfaces_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkId".into(),
                    value: &virtual_network_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backendIpConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "inboundNatRules".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancingRules".into(),
                },
                register_interface::ResultField {
                    name: "loadbalancerId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outboundRules".into(),
                },
                register_interface::ResultField {
                    name: "synchronousMode".into(),
                },
                register_interface::ResultField {
                    name: "tunnelInterfaces".into(),
                },
                register_interface::ResultField {
                    name: "virtualNetworkId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BackendAddressPoolResult {
            backend_ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendIpConfigurations").unwrap(),
            ),
            inbound_nat_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inboundNatRules").unwrap(),
            ),
            load_balancing_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancingRules").unwrap(),
            ),
            loadbalancer_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadbalancerId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            outbound_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outboundRules").unwrap(),
            ),
            synchronous_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("synchronousMode").unwrap(),
            ),
            tunnel_interfaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnelInterfaces").unwrap(),
            ),
            virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkId").unwrap(),
            ),
        }
    }
}
