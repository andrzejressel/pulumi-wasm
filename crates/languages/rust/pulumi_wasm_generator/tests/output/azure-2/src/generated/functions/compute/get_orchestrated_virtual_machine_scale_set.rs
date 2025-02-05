pub mod get_orchestrated_virtual_machine_scale_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrchestratedVirtualMachineScaleSetArgs {
        /// The name of this Orchestrated Virtual Machine Scale Set.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Orchestrated Virtual Machine Scale Set exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOrchestratedVirtualMachineScaleSetResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetOrchestratedVirtualMachineScaleSetIdentity,
            >,
        >,
        /// The Azure Region in which this Orchestrated Virtual Machine Scale Set exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the public IP address configuration
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of `network_interface` blocks as defined below.
        pub network_interfaces: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetOrchestratedVirtualMachineScaleSetNetworkInterface,
            >,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetOrchestratedVirtualMachineScaleSetArgs,
    ) -> GetOrchestratedVirtualMachineScaleSetResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getOrchestratedVirtualMachineScaleSet:getOrchestratedVirtualMachineScaleSet"
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
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOrchestratedVirtualMachineScaleSetResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network_interfaces: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkInterfaces"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
