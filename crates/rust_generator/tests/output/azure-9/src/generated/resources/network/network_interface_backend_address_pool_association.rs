/// Manages the association between a Network Interface and a Load Balancer's Backend Address Pool.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleBackendAddressPool = backend_address_pool::create(
///         "exampleBackendAddressPool",
///         BackendAddressPoolArgs::builder()
///             .loadbalancer_id("${exampleLoadBalancer.id}")
///             .name("acctestpool")
///             .build_struct(),
///     );
///     let exampleLoadBalancer = load_balancer::create(
///         "exampleLoadBalancer",
///         LoadBalancerArgs::builder()
///             .frontend_ip_configurations(
///                 vec![
///                     LoadBalancerFrontendIpConfiguration::builder().name("primary")
///                     .publicIpAddressId("${examplePublicIp.id}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example-lb")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleNetworkInterface = network_interface::create(
///         "exampleNetworkInterface",
///         NetworkInterfaceArgs::builder()
///             .ip_configurations(
///                 vec![
///                     NetworkInterfaceIpConfiguration::builder().name("testconfiguration1")
///                     .privateIpAddressAllocation("Dynamic")
///                     .subnetId("${exampleSubnet.id}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example-nic")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleNetworkInterfaceBackendAddressPoolAssociation = network_interface_backend_address_pool_association::create(
///         "exampleNetworkInterfaceBackendAddressPoolAssociation",
///         NetworkInterfaceBackendAddressPoolAssociationArgs::builder()
///             .backend_address_pool_id("${exampleBackendAddressPool.id}")
///             .ip_configuration_name("testconfiguration1")
///             .network_interface_id("${exampleNetworkInterface.id}")
///             .build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("${example.location}")
///             .name("example-pip")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .name("internal")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-network")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Associations between Network Interfaces and Load Balancer Backend Address Pools can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkInterfaceBackendAddressPoolAssociation:NetworkInterfaceBackendAddressPoolAssociation association1 "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/networkInterfaces/nic1/ipConfigurations/example|/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/loadBalancers/lb1/backendAddressPools/pool1"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_interface_backend_address_pool_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInterfaceBackendAddressPoolAssociationArgs {
        /// The ID of the Load Balancer Backend Address Pool which this Network Interface should be connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub backend_address_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the IP Configuration within the Network Interface which should be connected to the Backend Address Pool. Changing this forces a new resource to be created.
        #[builder(into)]
        pub ip_configuration_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Network Interface. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_interface_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkInterfaceBackendAddressPoolAssociationResult {
        /// The ID of the Load Balancer Backend Address Pool which this Network Interface should be connected to. Changing this forces a new resource to be created.
        pub backend_address_pool_id: pulumi_gestalt_rust::Output<String>,
        /// The Name of the IP Configuration within the Network Interface which should be connected to the Backend Address Pool. Changing this forces a new resource to be created.
        pub ip_configuration_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Network Interface. Changing this forces a new resource to be created.
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NetworkInterfaceBackendAddressPoolAssociationArgs,
    ) -> NetworkInterfaceBackendAddressPoolAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let backend_address_pool_id_binding_1 = args
            .backend_address_pool_id
            .get_output(context);
        let backend_address_pool_id_binding = backend_address_pool_id_binding_1
            .get_inner();
        let ip_configuration_name_binding_1 = args
            .ip_configuration_name
            .get_output(context);
        let ip_configuration_name_binding = ip_configuration_name_binding_1.get_inner();
        let network_interface_id_binding_1 = args
            .network_interface_id
            .get_output(context);
        let network_interface_id_binding = network_interface_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkInterfaceBackendAddressPoolAssociation:NetworkInterfaceBackendAddressPoolAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backendAddressPoolId".into(),
                    value: &backend_address_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "ipConfigurationName".into(),
                    value: &ip_configuration_name_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterfaceId".into(),
                    value: &network_interface_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NetworkInterfaceBackendAddressPoolAssociationResult {
            backend_address_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendAddressPoolId"),
            ),
            ip_configuration_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipConfigurationName"),
            ),
            network_interface_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfaceId"),
            ),
        }
    }
}
