pub mod get_virtual_machine {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualMachineArgs {
        /// Specifies the name of the Virtual Machine.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Virtual Machine is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualMachineResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetVirtualMachineIdentity>,
        >,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The power state of the virtual machine.
        pub power_state: pulumi_gestalt_rust::Output<String>,
        /// The Primary Private IP Address assigned to this Virtual Machine.
        pub private_ip_address: pulumi_gestalt_rust::Output<String>,
        /// A list of Private IP Addresses assigned to this Virtual Machine.
        pub private_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Primary Public IP Address assigned to this Virtual Machine.
        pub public_ip_address: pulumi_gestalt_rust::Output<String>,
        /// A list of the Public IP Addresses assigned to this Virtual Machine.
        pub public_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVirtualMachineArgs,
    ) -> GetVirtualMachineResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getVirtualMachine:getVirtualMachine".into(),
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
        GetVirtualMachineResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            power_state: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("powerState"),
            ),
            private_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpAddress"),
            ),
            private_ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpAddresses"),
            ),
            public_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicIpAddress"),
            ),
            public_ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicIpAddresses"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
