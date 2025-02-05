/// Manages a subnet. Subnets represent network segments within the IP space defined by the virtual network.
///
/// > **NOTE on Virtual Networks and Subnet's:** This provider currently
/// provides both a standalone Subnet resource, and allows for Subnets to be defined in-line within the Virtual Network resource.
/// At this time you cannot use a Virtual Network with in-line Subnets in conjunction with any Subnet resources. Doing so will cause a conflict of Subnet configurations and will overwrite Subnets.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.1.0/24",])
///             .delegations(
///                 vec![
///                     SubnetDelegation::builder().name("delegation")
///                     .serviceDelegation(SubnetDelegationServiceDelegation::builder()
///                     .actions(vec!["Microsoft.Network/virtualNetworks/subnets/join/action",
///                     "Microsoft.Network/virtualNetworks/subnets/prepareNetworkPolicies/action",])
///                     .name("Microsoft.ContainerInstance/containerGroups").build_struct())
///                     .build_struct(),
///                 ],
///             )
///             .name("example-subnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-vnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Subnets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/subnet:Subnet exampleSubnet /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/virtualNetworks/myvnet1/subnets/mysubnet1
/// ```
///
pub mod subnet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetArgs {
        /// The address prefixes to use for the subnet.
        ///
        /// > **NOTE:** Currently only a single address prefix can be set as the [Multiple Subnet Address Prefixes Feature](https://github.com/Azure/azure-cli/issues/18194#issuecomment-880484269) is not yet in public preview or general availability.
        #[builder(into)]
        pub address_prefixes: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Enable default outbound access to the internet for the subnet. Defaults to `true`.
        #[builder(into, default)]
        pub default_outbound_access_enabled: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// One or more `delegation` blocks as defined below.
        #[builder(into, default)]
        pub delegations: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::network::SubnetDelegation>>,
        >,
        /// The name of the subnet. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Enable or Disable network policies for the private endpoint on the subnet. Possible values are `Disabled`, `Enabled`, `NetworkSecurityGroupEnabled` and `RouteTableEnabled`. Defaults to `Disabled`.
        ///
        /// > **NOTE:** If you don't want to use network policies like user-defined Routes and Network Security Groups, you need to set `private_endpoint_network_policies` in the subnet to `Disabled`. This setting only applies to Private Endpoints in the Subnet and affects all Private Endpoints in the Subnet. For other resources in the Subnet, access is controlled based via the Network Security Group which can be configured using the `azure.network.SubnetNetworkSecurityGroupAssociation` resource.
        ///
        /// > **NOTE:** If you want to use network policies like user-defined Routes and Network Security Groups, you need to set the `private_endpoint_network_policies` in the Subnet to `Enabled`/`NetworkSecurityGroupEnabled`/`RouteTableEnabled`. This setting only applies to Private Endpoints in the Subnet and affects all Private Endpoints in the Subnet. For other resources in the Subnet, access is controlled based via the Network Security Group which can be configured using the `azure.network.SubnetNetworkSecurityGroupAssociation` resource.
        ///
        /// > **NOTE:** See more details from [Manage network policies for Private Endpoints](https://learn.microsoft.com/en-gb/azure/private-link/disable-private-endpoint-network-policy?tabs=network-policy-portal).
        #[builder(into, default)]
        pub private_endpoint_network_policies: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Enable or Disable network policies for the private link service on the subnet. Defaults to `true`.
        ///
        /// > **NOTE:** When configuring Azure Private Link service, the explicit setting `private_link_service_network_policies_enabled` must be set to `false` in the subnet since Private Link Service does not support network policies like user-defined Routes and Network Security Groups. This setting only affects the Private Link service. For other resources in the subnet, access is controlled based on the Network Security Group which can be configured using the `azure.network.SubnetNetworkSecurityGroupAssociation` resource. See more details from [Manage network policies for Private Link Services](https://learn.microsoft.com/en-gb/azure/private-link/disable-private-link-service-network-policy?tabs=private-link-network-policy-powershell).
        #[builder(into, default)]
        pub private_link_service_network_policies_enabled: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the resource group in which to create the subnet. This must be the resource group that the virtual network resides in. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The list of IDs of Service Endpoint Policies to associate with the subnet.
        #[builder(into, default)]
        pub service_endpoint_policy_ids: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The list of Service endpoints to associate with the subnet. Possible values include: `Microsoft.AzureActiveDirectory`, `Microsoft.AzureCosmosDB`, `Microsoft.ContainerRegistry`, `Microsoft.EventHub`, `Microsoft.KeyVault`, `Microsoft.ServiceBus`, `Microsoft.Sql`, `Microsoft.Storage`, `Microsoft.Storage.Global` and `Microsoft.Web`.
        ///
        /// > **NOTE:** In order to use `Microsoft.Storage.Global` service endpoint (which allows access to virtual networks in other regions), you must enable the `AllowGlobalTagsForStorage` feature in your subscription. This is currently a preview feature, please see the [official documentation](https://learn.microsoft.com/en-us/azure/storage/common/storage-network-security?tabs=azure-cli#enabling-access-to-virtual-networks-in-other-regions-preview) for more information.
        #[builder(into, default)]
        pub service_endpoints: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the virtual network to which to attach the subnet. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_network_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubnetResult {
        /// The address prefixes to use for the subnet.
        ///
        /// > **NOTE:** Currently only a single address prefix can be set as the [Multiple Subnet Address Prefixes Feature](https://github.com/Azure/azure-cli/issues/18194#issuecomment-880484269) is not yet in public preview or general availability.
        pub address_prefixes: pulumi_wasm_rust::Output<Vec<String>>,
        /// Enable default outbound access to the internet for the subnet. Defaults to `true`.
        pub default_outbound_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `delegation` blocks as defined below.
        pub delegations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::SubnetDelegation>>,
        >,
        /// The name of the subnet. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Enable or Disable network policies for the private endpoint on the subnet. Possible values are `Disabled`, `Enabled`, `NetworkSecurityGroupEnabled` and `RouteTableEnabled`. Defaults to `Disabled`.
        ///
        /// > **NOTE:** If you don't want to use network policies like user-defined Routes and Network Security Groups, you need to set `private_endpoint_network_policies` in the subnet to `Disabled`. This setting only applies to Private Endpoints in the Subnet and affects all Private Endpoints in the Subnet. For other resources in the Subnet, access is controlled based via the Network Security Group which can be configured using the `azure.network.SubnetNetworkSecurityGroupAssociation` resource.
        ///
        /// > **NOTE:** If you want to use network policies like user-defined Routes and Network Security Groups, you need to set the `private_endpoint_network_policies` in the Subnet to `Enabled`/`NetworkSecurityGroupEnabled`/`RouteTableEnabled`. This setting only applies to Private Endpoints in the Subnet and affects all Private Endpoints in the Subnet. For other resources in the Subnet, access is controlled based via the Network Security Group which can be configured using the `azure.network.SubnetNetworkSecurityGroupAssociation` resource.
        ///
        /// > **NOTE:** See more details from [Manage network policies for Private Endpoints](https://learn.microsoft.com/en-gb/azure/private-link/disable-private-endpoint-network-policy?tabs=network-policy-portal).
        pub private_endpoint_network_policies: pulumi_wasm_rust::Output<Option<String>>,
        /// Enable or Disable network policies for the private link service on the subnet. Defaults to `true`.
        ///
        /// > **NOTE:** When configuring Azure Private Link service, the explicit setting `private_link_service_network_policies_enabled` must be set to `false` in the subnet since Private Link Service does not support network policies like user-defined Routes and Network Security Groups. This setting only affects the Private Link service. For other resources in the subnet, access is controlled based on the Network Security Group which can be configured using the `azure.network.SubnetNetworkSecurityGroupAssociation` resource. See more details from [Manage network policies for Private Link Services](https://learn.microsoft.com/en-gb/azure/private-link/disable-private-link-service-network-policy?tabs=private-link-network-policy-powershell).
        pub private_link_service_network_policies_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// The name of the resource group in which to create the subnet. This must be the resource group that the virtual network resides in. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The list of IDs of Service Endpoint Policies to associate with the subnet.
        pub service_endpoint_policy_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The list of Service endpoints to associate with the subnet. Possible values include: `Microsoft.AzureActiveDirectory`, `Microsoft.AzureCosmosDB`, `Microsoft.ContainerRegistry`, `Microsoft.EventHub`, `Microsoft.KeyVault`, `Microsoft.ServiceBus`, `Microsoft.Sql`, `Microsoft.Storage`, `Microsoft.Storage.Global` and `Microsoft.Web`.
        ///
        /// > **NOTE:** In order to use `Microsoft.Storage.Global` service endpoint (which allows access to virtual networks in other regions), you must enable the `AllowGlobalTagsForStorage` feature in your subscription. This is currently a preview feature, please see the [official documentation](https://learn.microsoft.com/en-us/azure/storage/common/storage-network-security?tabs=azure-cli#enabling-access-to-virtual-networks-in-other-regions-preview) for more information.
        pub service_endpoints: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the virtual network to which to attach the subnet. Changing this forces a new resource to be created.
        pub virtual_network_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SubnetArgs,
    ) -> SubnetResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let address_prefixes_binding = args
            .address_prefixes
            .get_output(context)
            .get_inner();
        let default_outbound_access_enabled_binding = args
            .default_outbound_access_enabled
            .get_output(context)
            .get_inner();
        let delegations_binding = args.delegations.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let private_endpoint_network_policies_binding = args
            .private_endpoint_network_policies
            .get_output(context)
            .get_inner();
        let private_link_service_network_policies_enabled_binding = args
            .private_link_service_network_policies_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let service_endpoint_policy_ids_binding = args
            .service_endpoint_policy_ids
            .get_output(context)
            .get_inner();
        let service_endpoints_binding = args
            .service_endpoints
            .get_output(context)
            .get_inner();
        let virtual_network_name_binding = args
            .virtual_network_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/subnet:Subnet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addressPrefixes".into(),
                    value: &address_prefixes_binding,
                },
                register_interface::ObjectField {
                    name: "defaultOutboundAccessEnabled".into(),
                    value: &default_outbound_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "delegations".into(),
                    value: &delegations_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateEndpointNetworkPolicies".into(),
                    value: &private_endpoint_network_policies_binding,
                },
                register_interface::ObjectField {
                    name: "privateLinkServiceNetworkPoliciesEnabled".into(),
                    value: &private_link_service_network_policies_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceEndpointPolicyIds".into(),
                    value: &service_endpoint_policy_ids_binding,
                },
                register_interface::ObjectField {
                    name: "serviceEndpoints".into(),
                    value: &service_endpoints_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkName".into(),
                    value: &virtual_network_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SubnetResult {
            address_prefixes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("addressPrefixes"),
            ),
            default_outbound_access_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultOutboundAccessEnabled"),
            ),
            delegations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("delegations"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            private_endpoint_network_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateEndpointNetworkPolicies"),
            ),
            private_link_service_network_policies_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateLinkServiceNetworkPoliciesEnabled"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            service_endpoint_policy_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceEndpointPolicyIds"),
            ),
            service_endpoints: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceEndpoints"),
            ),
            virtual_network_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualNetworkName"),
            ),
        }
    }
}
