pub mod get_virtual_hub_route_table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualHubRouteTableArgs {
        /// The name of the Virtual Hub Route Table.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the Virtual Hub Route Table exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for Virtual Hub Route Table.
        #[builder(into)]
        pub virtual_hub_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualHubRouteTableResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of labels associated with this route table.
        pub labels: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name which is used for this route.
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `route` block as defined below.
        pub routes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetVirtualHubRouteTableRoute>,
        >,
        /// The ID of the Virtual Hub within which this route table is created
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
        pub virtual_hub_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetVirtualHubRouteTableArgs,
    ) -> GetVirtualHubRouteTableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let virtual_hub_name_binding = args
            .virtual_hub_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getVirtualHubRouteTable:getVirtualHubRouteTable"
                .into(),
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
                register_interface::ObjectField {
                    name: "virtualHubName".into(),
                    value: &virtual_hub_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVirtualHubRouteTableResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            routes: pulumi_wasm_rust::__private::into_domain(o.extract_field("routes")),
            virtual_hub_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualHubId"),
            ),
            virtual_hub_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualHubName"),
            ),
        }
    }
}
