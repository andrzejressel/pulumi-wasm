pub mod get_registry {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegistryArgs {
        /// The name of the Container Registry.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where this Container Registry exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRegistryResult {
        /// Is the Administrator account enabled for this Container Registry.
        pub admin_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Password associated with the Container Registry Admin account - if the admin account is enabled.
        pub admin_password: pulumi_wasm_rust::Output<String>,
        /// The Username associated with the Container Registry Admin account - if the admin account is enabled.
        pub admin_username: pulumi_wasm_rust::Output<String>,
        /// Whether dedicated data endpoints for this Container Registry are enabled?
        pub data_endpoint_enabled: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region in which this Container Registry exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The URL that can be used to log into the container registry.
        pub login_server: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU of this Container Registry, such as `Basic`.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A map of tags assigned to the Container Registry.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRegistryArgs,
    ) -> GetRegistryResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:containerservice/getRegistry:getRegistry".into(),
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
        GetRegistryResult {
            admin_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminEnabled"),
            ),
            admin_password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminPassword"),
            ),
            admin_username: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminUsername"),
            ),
            data_endpoint_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataEndpointEnabled"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            login_server: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loginServer"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(o.extract_field("sku")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
