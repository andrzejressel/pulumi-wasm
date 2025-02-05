pub mod get_bastion_shareable_link {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBastionShareableLinkArgs {
        /// The name of the Bastion Host.
        #[builder(into)]
        pub bastion_host_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the resource group.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// List of VM references.
        #[builder(into, default)]
        pub vms: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::BastionShareableLink>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetBastionShareableLinkResult {
        /// The URL to get the next set of results.
        pub next_link: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetBastionShareableLinkArgs,
    ) -> GetBastionShareableLinkResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bastion_host_name_binding = args
            .bastion_host_name
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let vms_binding = args.vms.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "mypkg::getBastionShareableLink".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bastionHostName".into(),
                    value: &bastion_host_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "vms".into(),
                    value: &vms_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBastionShareableLinkResult {
            next_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nextLink"),
            ),
        }
    }
}
