/// Manages an association between an Application Gateway for Containers and a Subnet.
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
///             .location("westeurope")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleLicationLoadBalancer = lication_load_balancer::create(
///         "exampleLicationLoadBalancer",
///         LicationLoadBalancerArgs::builder()
///             .location("${example.location}")
///             .name("example-alb")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLicationLoadBalancerSubnetAssociation = lication_load_balancer_subnet_association::create(
///         "exampleLicationLoadBalancerSubnetAssociation",
///         LicationLoadBalancerSubnetAssociationArgs::builder()
///             .application_load_balancer_id("${exampleLicationLoadBalancer.id}")
///             .name("example")
///             .subnet_id("${exampleSubnet.id}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.1.0/24",])
///             .delegations(
///                 vec![
///                     SubnetDelegation::builder().name("delegation")
///                     .serviceDelegation(SubnetDelegationServiceDelegation::builder()
///                     .actions(vec!["Microsoft.Network/virtualNetworks/subnets/join/action",])
///                     .name("Microsoft.ServiceNetworking/trafficControllers")
///                     .build_struct()).build_struct(),
///                 ],
///             )
///             .name("example-subnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-vnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Application Gateway for Containers Associations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appconfiguration/licationLoadBalancerSubnetAssociation:LicationLoadBalancerSubnetAssociation example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.ServiceNetworking/trafficControllers/alb1/associations/association1
/// ```
///
pub mod lication_load_balancer_subnet_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LicationLoadBalancerSubnetAssociationArgs {
        /// The ID of the Application Gateway for Containers. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_load_balancer_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Application Gateway for Containers Association. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the subnet which the Application Gateway for Containers associated to.
        ///
        /// > **Note:** The subnet to be used must have a delegation for  `Microsoft.ServiceNetworking/trafficControllers` as shown in the example above.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Application Gateway for Containers Association.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LicationLoadBalancerSubnetAssociationResult {
        /// The ID of the Application Gateway for Containers. Changing this forces a new resource to be created.
        pub application_load_balancer_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Application Gateway for Containers Association. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the subnet which the Application Gateway for Containers associated to.
        ///
        /// > **Note:** The subnet to be used must have a delegation for  `Microsoft.ServiceNetworking/trafficControllers` as shown in the example above.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Application Gateway for Containers Association.
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
        args: LicationLoadBalancerSubnetAssociationArgs,
    ) -> LicationLoadBalancerSubnetAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_load_balancer_id_binding = args
            .application_load_balancer_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appconfiguration/licationLoadBalancerSubnetAssociation:LicationLoadBalancerSubnetAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationLoadBalancerId".into(),
                    value: &application_load_balancer_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationLoadBalancerId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
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
        LicationLoadBalancerSubnetAssociationResult {
            application_load_balancer_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationLoadBalancerId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}