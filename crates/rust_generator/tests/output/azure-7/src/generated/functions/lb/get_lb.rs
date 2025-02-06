pub mod get_lb {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLbArgs {
        /// Specifies the name of the Load Balancer.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Load Balancer exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLbResult {
        /// A `frontend_ip_configuration` block as documented below.
        pub frontend_ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::lb::GetLbFrontendIpConfiguration>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure location where the Load Balancer exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Frontend IP Configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Private IP Address to assign to the Load Balancer.
        pub private_ip_address: pulumi_wasm_rust::Output<String>,
        /// The list of private IP address assigned to the load balancer in `frontend_ip_configuration` blocks, if any.
        pub private_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU of the Load Balancer.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetLbArgs,
    ) -> GetLbResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:lb/getLB:getLB".into(),
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
        GetLbResult {
            frontend_ip_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("frontendIpConfigurations"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            private_ip_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateIpAddress"),
            ),
            private_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateIpAddresses"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(o.extract_field("sku")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
