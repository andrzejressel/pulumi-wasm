/// Associates a Route Table with a Subnet within a Virtual Network.
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
///     let exampleRouteTable = route_table::create(
///         "exampleRouteTable",
///         RouteTableArgs::builder()
///             .location("${example.location}")
///             .name("example-routetable")
///             .resource_group_name("${example.name}")
///             .routes(
///                 vec![
///                     RouteTableRoute::builder().addressPrefix("10.100.0.0/14")
///                     .name("example").nextHopInIpAddress("10.10.1.1")
///                     .nextHopType("VirtualAppliance").build_struct(),
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
///     let exampleSubnetRouteTableAssociation = subnet_route_table_association::create(
///         "exampleSubnetRouteTableAssociation",
///         SubnetRouteTableAssociationArgs::builder()
///             .route_table_id("${exampleRouteTable.id}")
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
/// Subnet Route Table Associations can be imported using the `resource id` of the Subnet, e.g.
///
/// ```sh
/// $ pulumi import azure:network/subnetRouteTableAssociation:SubnetRouteTableAssociation association1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/virtualNetworks/myvnet1/subnets/mysubnet1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod subnet_route_table_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetRouteTableAssociationArgs {
        /// The ID of the Route Table which should be associated with the Subnet. Changing this forces a new resource to be created.
        #[builder(into)]
        pub route_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Subnet. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubnetRouteTableAssociationResult {
        /// The ID of the Route Table which should be associated with the Subnet. Changing this forces a new resource to be created.
        pub route_table_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subnet. Changing this forces a new resource to be created.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SubnetRouteTableAssociationArgs,
    ) -> SubnetRouteTableAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let route_table_id_binding = args.route_table_id.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/subnetRouteTableAssociation:SubnetRouteTableAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeTableId".into(),
                    value: route_table_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: subnet_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SubnetRouteTableAssociationResult {
            route_table_id: o.get_field("routeTableId"),
            subnet_id: o.get_field("subnetId"),
        }
    }
}
