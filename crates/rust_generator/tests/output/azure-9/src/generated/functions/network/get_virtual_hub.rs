pub mod get_virtual_hub {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualHubArgs {
        /// The name of the Virtual Hub.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the Virtual Hub exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualHubResult {
        /// The Address Prefix used for this Virtual Hub.
        pub address_prefix: pulumi_gestalt_rust::Output<String>,
        /// The ID of the default Route Table in the Virtual Hub.
        pub default_route_table_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Virtual Hub exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Virtual Hub.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The Autonomous System Number of the Virtual Hub BGP router.
        pub virtual_router_asn: pulumi_gestalt_rust::Output<i32>,
        /// The IP addresses of the Virtual Hub BGP router.
        pub virtual_router_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the Virtual WAN within which the Virtual Hub exists.
        pub virtual_wan_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVirtualHubArgs,
    ) -> GetVirtualHubResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getVirtualHub:getVirtualHub".into(),
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
        GetVirtualHubResult {
            address_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addressPrefix"),
            ),
            default_route_table_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultRouteTableId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            virtual_router_asn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualRouterAsn"),
            ),
            virtual_router_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualRouterIps"),
            ),
            virtual_wan_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualWanId"),
            ),
        }
    }
}
