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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backend_address_pool_address {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackendAddressPoolAddressArgs {
        /// The ip config ID of the regional load balancer that's added to the global load balancer's backend address pool.
        ///
        /// > **Note:** For cross-region load balancer, please append the name of the load balancers, virtual machines, and other resources in each region with a -R1 and -R2.
        #[builder(into, default)]
        pub backend_address_ip_configuration_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the Backend Address Pool. Changing this forces a new Backend Address Pool Address to be created.
        #[builder(into)]
        pub backend_address_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Static IP Address which should be allocated to this Backend Address Pool.
        #[builder(into, default)]
        pub ip_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Backend Address Pool Address. Changing this forces a new Backend Address Pool Address to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Virtual Network within which the Backend Address Pool should exist.
        #[builder(into, default)]
        pub virtual_network_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BackendAddressPoolAddressResult {
        /// The ip config ID of the regional load balancer that's added to the global load balancer's backend address pool.
        ///
        /// > **Note:** For cross-region load balancer, please append the name of the load balancers, virtual machines, and other resources in each region with a -R1 and -R2.
        pub backend_address_ip_configuration_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The ID of the Backend Address Pool. Changing this forces a new Backend Address Pool Address to be created.
        pub backend_address_pool_id: pulumi_gestalt_rust::Output<String>,
        /// A list of `inbound_nat_rule_port_mapping` block as defined below.
        pub inbound_nat_rule_port_mappings: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::lb::BackendAddressPoolAddressInboundNatRulePortMapping,
            >,
        >,
        /// The Static IP Address which should be allocated to this Backend Address Pool.
        pub ip_address: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Backend Address Pool Address. Changing this forces a new Backend Address Pool Address to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Virtual Network within which the Backend Address Pool should exist.
        pub virtual_network_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BackendAddressPoolAddressArgs,
    ) -> BackendAddressPoolAddressResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let backend_address_ip_configuration_id_binding_1 = args
            .backend_address_ip_configuration_id
            .get_output(context);
        let backend_address_ip_configuration_id_binding = backend_address_ip_configuration_id_binding_1
            .get_inner();
        let backend_address_pool_id_binding_1 = args
            .backend_address_pool_id
            .get_output(context);
        let backend_address_pool_id_binding = backend_address_pool_id_binding_1
            .get_inner();
        let ip_address_binding_1 = args.ip_address.get_output(context);
        let ip_address_binding = ip_address_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let virtual_network_id_binding_1 = args.virtual_network_id.get_output(context);
        let virtual_network_id_binding = virtual_network_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:lb/backendAddressPoolAddress:BackendAddressPoolAddress".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        BackendAddressPoolAddressResult {
            backend_address_ip_configuration_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendAddressIpConfigurationId"),
            ),
            backend_address_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendAddressPoolId"),
            ),
            inbound_nat_rule_port_mappings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inboundNatRulePortMappings"),
            ),
            ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddress"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            virtual_network_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualNetworkId"),
            ),
        }
    }
}
