#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_virtual_machine_scale_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualMachineScaleSetArgs {
        /// The name of this Virtual Machine Scale Set.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Virtual Machine Scale Set exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualMachineScaleSetResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetVirtualMachineScaleSetIdentity>,
        >,
        /// A list of `instances` blocks as defined below.
        pub instances: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetVirtualMachineScaleSetInstance>,
        >,
        /// The Azure Region in which this Virtual Machine Scale Set exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the public IP address configuration
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of `network_interface` blocks as defined below.
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetVirtualMachineScaleSetNetworkInterface,
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
        args: GetVirtualMachineScaleSetArgs,
    ) -> GetVirtualMachineScaleSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:compute/getVirtualMachineScaleSet:getVirtualMachineScaleSet"
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
        GetVirtualMachineScaleSetResult {
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            instances: o.get_field("instances"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network_interfaces: o.get_field("networkInterfaces"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
