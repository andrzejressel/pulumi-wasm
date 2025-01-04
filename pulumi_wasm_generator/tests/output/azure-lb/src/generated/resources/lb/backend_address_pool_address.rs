/// Manages a Backend Address within a Backend Address Pool.
///
/// > **Note:** Backend Addresses can only be added to a `Standard` SKU Load Balancer.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleBackendAddressPoolAddress:
///     type: azure:lb:BackendAddressPoolAddress
///     name: example
///     properties:
///       name: example
///       backendAddressPoolId: ${exampleGetBackendAddressPool.id}
///       virtualNetworkId: ${example.id}
///       ipAddress: 10.0.0.1
///   example-1:
///     type: azure:lb:BackendAddressPoolAddress
///     properties:
///       name: address1
///       backendAddressPoolId: ${["backend-pool-cr"].id}
///       backendAddressIpConfigurationId: ${["backend-lb-R1"].frontendIpConfiguration[0].id}
///   example-2:
///     type: azure:lb:BackendAddressPoolAddress
///     properties:
///       name: address2
///       backendAddressPoolId: ${["backend-pool-cr"].id}
///       backendAddressIpConfigurationId: ${["backend-lb-R2"].frontendIpConfiguration[0].id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:network:getVirtualNetwork
///       arguments:
///         name: example-network
///         resourceGroupName: example-resources
///   exampleGetLB:
///     fn::invoke:
///       function: azure:lb:getLB
///       arguments:
///         name: example-lb
///         resourceGroupName: example-resources
///   exampleGetBackendAddressPool:
///     fn::invoke:
///       function: azure:lb:getBackendAddressPool
///       arguments:
///         name: first
///         loadbalancerId: ${exampleGetLB.id}
///   backend-pool-cr:
///     fn::invoke:
///       function: azure:lb:getBackendAddressPool
///       arguments:
///         name: globalLBBackendPool
///         loadbalancerId: ${exampleGetLB.id}
/// ```
///
/// ## Import
///
/// Backend Address Pool Addresses can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:lb/backendAddressPoolAddress:BackendAddressPoolAddress example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/loadBalancers/loadBalancer1/backendAddressPools/backendAddressPool1/addresses/address1
/// ```
///
pub mod backend_address_pool_address {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackendAddressPoolAddressArgs {
        /// The ip config ID of the regional load balancer that's added to the global load balancer's backend address pool.
        ///
        /// > **Note:** For cross-region load balancer, please append the name of the load balancers, virtual machines, and other resources in each region with a -R1 and -R2.
        #[builder(into, default)]
        pub backend_address_ip_configuration_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The ID of the Backend Address Pool. Changing this forces a new Backend Address Pool Address to be created.
        #[builder(into)]
        pub backend_address_pool_id: pulumi_wasm_rust::Output<String>,
        /// The Static IP Address which should be allocated to this Backend Address Pool.
        #[builder(into, default)]
        pub ip_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Backend Address Pool Address. Changing this forces a new Backend Address Pool Address to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Virtual Network within which the Backend Address Pool should exist.
        #[builder(into, default)]
        pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BackendAddressPoolAddressResult {
        /// The ip config ID of the regional load balancer that's added to the global load balancer's backend address pool.
        ///
        /// > **Note:** For cross-region load balancer, please append the name of the load balancers, virtual machines, and other resources in each region with a -R1 and -R2.
        pub backend_address_ip_configuration_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The ID of the Backend Address Pool. Changing this forces a new Backend Address Pool Address to be created.
        pub backend_address_pool_id: pulumi_wasm_rust::Output<String>,
        /// A list of `inbound_nat_rule_port_mapping` block as defined below.
        pub inbound_nat_rule_port_mappings: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::lb::BackendAddressPoolAddressInboundNatRulePortMapping,
            >,
        >,
        /// The Static IP Address which should be allocated to this Backend Address Pool.
        pub ip_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Backend Address Pool Address. Changing this forces a new Backend Address Pool Address to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Network within which the Backend Address Pool should exist.
        pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BackendAddressPoolAddressArgs,
    ) -> BackendAddressPoolAddressResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backend_address_ip_configuration_id_binding = args
            .backend_address_ip_configuration_id
            .get_inner();
        let backend_address_pool_id_binding = args.backend_address_pool_id.get_inner();
        let ip_address_binding = args.ip_address.get_inner();
        let name_binding = args.name.get_inner();
        let virtual_network_id_binding = args.virtual_network_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:lb/backendAddressPoolAddress:BackendAddressPoolAddress".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backendAddressIpConfigurationId".into(),
                    value: &backend_address_ip_configuration_id_binding,
                },
                register_interface::ObjectField {
                    name: "backendAddressPoolId".into(),
                    value: &backend_address_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddress".into(),
                    value: &ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkId".into(),
                    value: &virtual_network_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backendAddressIpConfigurationId".into(),
                },
                register_interface::ResultField {
                    name: "backendAddressPoolId".into(),
                },
                register_interface::ResultField {
                    name: "inboundNatRulePortMappings".into(),
                },
                register_interface::ResultField {
                    name: "ipAddress".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
        BackendAddressPoolAddressResult {
            backend_address_ip_configuration_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendAddressIpConfigurationId").unwrap(),
            ),
            backend_address_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendAddressPoolId").unwrap(),
            ),
            inbound_nat_rule_port_mappings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inboundNatRulePortMappings").unwrap(),
            ),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddress").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkId").unwrap(),
            ),
        }
    }
}
