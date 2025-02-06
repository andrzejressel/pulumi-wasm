pub mod get_ip_groups {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIpGroupsArgs {
        /// A substring to match some number of IP Groups.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the IP Groups exist.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetIpGroupsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of IP Group IDs.
        pub ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of IP Group Names.
        pub names: pulumi_wasm_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetIpGroupsArgs,
    ) -> GetIpGroupsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getIpGroups:getIpGroups".into(),
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
        GetIpGroupsResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ids: pulumi_wasm_rust::__private::into_domain(o.extract_field("ids")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            names: pulumi_wasm_rust::__private::into_domain(o.extract_field("names")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
