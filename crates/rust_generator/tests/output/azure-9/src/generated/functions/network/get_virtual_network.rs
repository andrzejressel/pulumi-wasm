#[allow(clippy::doc_lazy_continuation)]
pub mod get_virtual_network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualNetworkArgs {
        /// Specifies the name of the Virtual Network.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Virtual Network is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualNetworkResult {
        /// The list of address spaces used by the virtual network.
        pub address_spaces: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The list of DNS servers used by the virtual network.
        pub dns_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The GUID of the virtual network.
        pub guid: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Location of the virtual network.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The list of name of the subnets that are attached to this virtual network.
        pub subnets: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A mapping of tags to assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// A mapping of name - virtual network id of the virtual network peerings.
        pub vnet_peerings: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of virtual network peerings IP addresses.
        pub vnet_peerings_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVirtualNetworkArgs,
    ) -> GetVirtualNetworkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getVirtualNetwork:getVirtualNetwork".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVirtualNetworkResult {
            address_spaces: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addressSpaces"),
            ),
            dns_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsServers"),
            ),
            guid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("guid")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            subnets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnets"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            vnet_peerings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vnetPeerings"),
            ),
            vnet_peerings_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vnetPeeringsAddresses"),
            ),
        }
    }
}
