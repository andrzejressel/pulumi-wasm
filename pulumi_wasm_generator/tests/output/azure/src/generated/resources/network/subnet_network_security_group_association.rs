/// Associates a Network Security Group with a Subnet within a Virtual Network.
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
///     let exampleNetworkSecurityGroup = network_security_group::create(
///         "exampleNetworkSecurityGroup",
///         NetworkSecurityGroupArgs::builder()
///             .location("${example.location}")
///             .name("example-nsg")
///             .resource_group_name("${example.name}")
///             .security_rules(
///                 vec![
///                     NetworkSecurityGroupSecurityRule::builder().access("Allow")
///                     .destinationAddressPrefix("*").destinationPortRange("*")
///                     .direction("Inbound").name("test123").priority(100).protocol("Tcp")
///                     .sourceAddressPrefix("*").sourcePortRange("*").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .name("frontend")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleSubnetNetworkSecurityGroupAssociation = subnet_network_security_group_association::create(
///         "exampleSubnetNetworkSecurityGroupAssociation",
///         SubnetNetworkSecurityGroupAssociationArgs::builder()
///             .network_security_group_id("${exampleNetworkSecurityGroup.id}")
///             .subnet_id("${exampleSubnet.id}")
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
/// Subnet `<->` Network Security Group Associations can be imported using the `resource id` of the Subnet, e.g.
///
/// ```sh
/// $ pulumi import azure:network/subnetNetworkSecurityGroupAssociation:SubnetNetworkSecurityGroupAssociation association1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/virtualNetworks/myvnet1/subnets/mysubnet1
/// ```
///
pub mod subnet_network_security_group_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetNetworkSecurityGroupAssociationArgs {
        /// The ID of the Network Security Group which should be associated with the Subnet. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_security_group_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Subnet. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SubnetNetworkSecurityGroupAssociationResult {
        /// The ID of the Network Security Group which should be associated with the Subnet. Changing this forces a new resource to be created.
        pub network_security_group_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Subnet. Changing this forces a new resource to be created.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SubnetNetworkSecurityGroupAssociationArgs,
    ) -> SubnetNetworkSecurityGroupAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let network_security_group_id_binding = args
            .network_security_group_id
            .get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/subnetNetworkSecurityGroupAssociation:SubnetNetworkSecurityGroupAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "networkSecurityGroupId".into(),
                    value: &network_security_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "networkSecurityGroupId".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SubnetNetworkSecurityGroupAssociationResult {
            network_security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkSecurityGroupId").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
        }
    }
}