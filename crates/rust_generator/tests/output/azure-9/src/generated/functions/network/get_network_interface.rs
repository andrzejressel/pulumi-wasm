#[allow(clippy::doc_lazy_continuation)]
pub mod get_network_interface {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkInterfaceArgs {
        /// Specifies the name of the Network Interface.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Network Interface is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkInterfaceResult {
        /// Indicates if accelerated networking is set on the specified Network Interface.
        pub accelerated_networking_enabled: pulumi_gestalt_rust::Output<bool>,
        /// List of DNS servers applied to the specified Network Interface.
        pub applied_dns_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The list of DNS servers used by the specified Network Interface.
        pub dns_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The internal DNS name label of the specified Network Interface.
        pub internal_dns_name_label: pulumi_gestalt_rust::Output<String>,
        /// One or more `ip_configuration` blocks as defined below.
        pub ip_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetNetworkInterfaceIpConfiguration>,
        >,
        /// Indicate if IP forwarding is set on the specified Network Interface.
        pub ip_forwarding_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The location of the specified Network Interface.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The MAC address used by the specified Network Interface.
        pub mac_address: pulumi_gestalt_rust::Output<String>,
        /// The name of the IP Configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the network security group associated to the specified Network Interface.
        pub network_security_group_id: pulumi_gestalt_rust::Output<String>,
        /// The Private IP Address assigned to this Network Interface.
        pub private_ip_address: pulumi_gestalt_rust::Output<String>,
        /// The list of private IP addresses associates to the specified Network Interface.
        pub private_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// List the tags associated to the specified Network Interface.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The ID of the virtual machine that the specified Network Interface is attached to.
        pub virtual_machine_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetNetworkInterfaceArgs,
    ) -> GetNetworkInterfaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getNetworkInterface:getNetworkInterface".into(),
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
        GetNetworkInterfaceResult {
            accelerated_networking_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceleratedNetworkingEnabled"),
            ),
            applied_dns_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appliedDnsServers"),
            ),
            dns_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsServers"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            internal_dns_name_label: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("internalDnsNameLabel"),
            ),
            ip_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipConfigurations"),
            ),
            ip_forwarding_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipForwardingEnabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            mac_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("macAddress"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_security_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkSecurityGroupId"),
            ),
            private_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpAddress"),
            ),
            private_ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpAddresses"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            virtual_machine_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualMachineId"),
            ),
        }
    }
}
