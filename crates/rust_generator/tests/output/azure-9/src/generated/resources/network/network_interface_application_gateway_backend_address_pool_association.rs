/// Manages the association between a Network Interface and a Application Gateway's Backend Address Pool.
///
/// ## Import
///
/// Associations between Network Interfaces and Application Gateway Backend Address Pools can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkInterfaceApplicationGatewayBackendAddressPoolAssociation:NetworkInterfaceApplicationGatewayBackendAddressPoolAssociation association1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/networkInterfaces/nic1/ipConfigurations/example|/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/applicationGateways/gateway1/backendAddressPools/pool1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_interface_application_gateway_backend_address_pool_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInterfaceApplicationGatewayBackendAddressPoolAssociationArgs {
        /// The ID of the Application Gateway's Backend Address Pool which this Network Interface which should be connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub backend_address_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the IP Configuration within the Network Interface which should be connected to the Backend Address Pool. Changing this forces a new resource to be created.
        #[builder(into)]
        pub ip_configuration_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Network Interface. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_interface_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkInterfaceApplicationGatewayBackendAddressPoolAssociationResult {
        /// The ID of the Application Gateway's Backend Address Pool which this Network Interface which should be connected to. Changing this forces a new resource to be created.
        pub backend_address_pool_id: pulumi_gestalt_rust::Output<String>,
        /// The Name of the IP Configuration within the Network Interface which should be connected to the Backend Address Pool. Changing this forces a new resource to be created.
        pub ip_configuration_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Network Interface. Changing this forces a new resource to be created.
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NetworkInterfaceApplicationGatewayBackendAddressPoolAssociationArgs,
    ) -> NetworkInterfaceApplicationGatewayBackendAddressPoolAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let backend_address_pool_id_binding = args
            .backend_address_pool_id
            .get_output(context)
            .get_inner();
        let ip_configuration_name_binding = args
            .ip_configuration_name
            .get_output(context)
            .get_inner();
        let network_interface_id_binding = args
            .network_interface_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkInterfaceApplicationGatewayBackendAddressPoolAssociation:NetworkInterfaceApplicationGatewayBackendAddressPoolAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backendAddressPoolId".into(),
                    value: &backend_address_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "ipConfigurationName".into(),
                    value: &ip_configuration_name_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterfaceId".into(),
                    value: &network_interface_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NetworkInterfaceApplicationGatewayBackendAddressPoolAssociationResult {
            backend_address_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendAddressPoolId"),
            ),
            ip_configuration_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipConfigurationName"),
            ),
            network_interface_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfaceId"),
            ),
        }
    }
}
