/// Associates a Network Security Group with a Subnet within a Virtual Network.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod subnet_network_security_group_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetNetworkSecurityGroupAssociationArgs {
        /// The ID of the Network Security Group which should be associated with the Subnet. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_security_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Subnet. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubnetNetworkSecurityGroupAssociationResult {
        /// The ID of the Network Security Group which should be associated with the Subnet. Changing this forces a new resource to be created.
        pub network_security_group_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subnet. Changing this forces a new resource to be created.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SubnetNetworkSecurityGroupAssociationArgs,
    ) -> SubnetNetworkSecurityGroupAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let network_security_group_id_binding_1 = args
            .network_security_group_id
            .get_output(context);
        let network_security_group_id_binding = network_security_group_id_binding_1
            .get_inner();
        let subnet_id_binding_1 = args.subnet_id.get_output(context);
        let subnet_id_binding = subnet_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/subnetNetworkSecurityGroupAssociation:SubnetNetworkSecurityGroupAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        SubnetNetworkSecurityGroupAssociationResult {
            network_security_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkSecurityGroupId"),
            ),
            subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
        }
    }
}
