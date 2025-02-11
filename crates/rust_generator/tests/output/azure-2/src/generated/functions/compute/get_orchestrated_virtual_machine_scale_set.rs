#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_orchestrated_virtual_machine_scale_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrchestratedVirtualMachineScaleSetArgs {
        /// The name of this Orchestrated Virtual Machine Scale Set.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Orchestrated Virtual Machine Scale Set exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOrchestratedVirtualMachineScaleSetResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetOrchestratedVirtualMachineScaleSetIdentity,
            >,
        >,
        /// The Azure Region in which this Orchestrated Virtual Machine Scale Set exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the public IP address configuration
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of `network_interface` blocks as defined below.
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetOrchestratedVirtualMachineScaleSetNetworkInterface,
            >,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOrchestratedVirtualMachineScaleSetArgs,
    ) -> GetOrchestratedVirtualMachineScaleSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:compute/getOrchestratedVirtualMachineScaleSet:getOrchestratedVirtualMachineScaleSet"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOrchestratedVirtualMachineScaleSetResult {
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network_interfaces: o.get_field("networkInterfaces"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
