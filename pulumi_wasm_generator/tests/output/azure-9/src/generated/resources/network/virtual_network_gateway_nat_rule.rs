/// Manages a Virtual Network Gateway Nat Rule.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       addressSpaces:
///         - 10.0.0.0/16
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: GatewaySubnet
///       resourceGroupName: ${exampleResourceGroup.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: example-pip
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       allocationMethod: Dynamic
///   exampleVirtualNetworkGateway:
///     type: azure:network:VirtualNetworkGateway
///     name: example
///     properties:
///       name: example-vnetgw
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       type: Vpn
///       vpnType: RouteBased
///       sku: Basic
///       ipConfigurations:
///         - publicIpAddressId: ${examplePublicIp.id}
///           privateIpAddressAllocation: Dynamic
///           subnetId: ${exampleSubnet.id}
///   exampleVirtualNetworkGatewayNatRule:
///     type: azure:network:VirtualNetworkGatewayNatRule
///     name: example
///     properties:
///       name: example-vnetgwnatrule
///       resourceGroupName: ${exampleResourceGroup.name}
///       virtualNetworkGatewayId: ${example.id}
///       mode: EgressSnat
///       type: Dynamic
///       ipConfigurationId: ${example.ipConfigurations[0].id}
///       externalMappings:
///         - addressSpace: 10.2.0.0/26
///           portRange: '200'
///       internalMappings:
///         - addressSpace: 10.4.0.0/26
///           portRange: '400'
/// variables:
///   example:
///     fn::invoke:
///       function: azure:network:getVirtualNetworkGateway
///       arguments:
///         name: ${exampleVirtualNetworkGateway.name}
///         resourceGroupName: ${exampleVirtualNetworkGateway.resourceGroupName}
/// ```
///
/// ## Import
///
/// Virtual Network Gateway Nat Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/virtualNetworkGatewayNatRule:VirtualNetworkGatewayNatRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.Network/virtualNetworkGateways/gw1/natRules/rule1
/// ```
///
pub mod virtual_network_gateway_nat_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkGatewayNatRuleArgs {
        /// One or more `external_mapping` blocks as documented below.
        #[builder(into)]
        pub external_mappings: pulumi_wasm_rust::InputOrOutput<
            Vec<
                super::super::types::network::VirtualNetworkGatewayNatRuleExternalMapping,
            >,
        >,
        /// One or more `internal_mapping` blocks as documented below.
        #[builder(into)]
        pub internal_mappings: pulumi_wasm_rust::InputOrOutput<
            Vec<
                super::super::types::network::VirtualNetworkGatewayNatRuleInternalMapping,
            >,
        >,
        /// The ID of the IP Configuration this Virtual Network Gateway Nat Rule applies to.
        #[builder(into, default)]
        pub ip_configuration_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The source Nat direction of the Virtual Network Gateway Nat. Possible values are `EgressSnat` and `IngressSnat`. Defaults to `EgressSnat`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Virtual Network Gateway Nat Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Name of the Resource Group in which this Virtual Network Gateway Nat Rule should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of the Virtual Network Gateway Nat Rule. Possible values are `Dynamic` and `Static`. Defaults to `Static`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Virtual Network Gateway that this Virtual Network Gateway Nat Rule belongs to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_network_gateway_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkGatewayNatRuleResult {
        /// One or more `external_mapping` blocks as documented below.
        pub external_mappings: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::network::VirtualNetworkGatewayNatRuleExternalMapping,
            >,
        >,
        /// One or more `internal_mapping` blocks as documented below.
        pub internal_mappings: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::network::VirtualNetworkGatewayNatRuleInternalMapping,
            >,
        >,
        /// The ID of the IP Configuration this Virtual Network Gateway Nat Rule applies to.
        pub ip_configuration_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The source Nat direction of the Virtual Network Gateway Nat. Possible values are `EgressSnat` and `IngressSnat`. Defaults to `EgressSnat`. Changing this forces a new resource to be created.
        pub mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Virtual Network Gateway Nat Rule. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group in which this Virtual Network Gateway Nat Rule should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The type of the Virtual Network Gateway Nat Rule. Possible values are `Dynamic` and `Static`. Defaults to `Static`. Changing this forces a new resource to be created.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Virtual Network Gateway that this Virtual Network Gateway Nat Rule belongs to. Changing this forces a new resource to be created.
        pub virtual_network_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VirtualNetworkGatewayNatRuleArgs,
    ) -> VirtualNetworkGatewayNatRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let external_mappings_binding = args
            .external_mappings
            .get_output(context)
            .get_inner();
        let internal_mappings_binding = args
            .internal_mappings
            .get_output(context)
            .get_inner();
        let ip_configuration_id_binding = args
            .ip_configuration_id
            .get_output(context)
            .get_inner();
        let mode_binding = args.mode.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let virtual_network_gateway_id_binding = args
            .virtual_network_gateway_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/virtualNetworkGatewayNatRule:VirtualNetworkGatewayNatRule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "externalMappings".into(),
                    value: &external_mappings_binding,
                },
                register_interface::ObjectField {
                    name: "internalMappings".into(),
                    value: &internal_mappings_binding,
                },
                register_interface::ObjectField {
                    name: "ipConfigurationId".into(),
                    value: &ip_configuration_id_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkGatewayId".into(),
                    value: &virtual_network_gateway_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VirtualNetworkGatewayNatRuleResult {
            external_mappings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("externalMappings"),
            ),
            internal_mappings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("internalMappings"),
            ),
            ip_configuration_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipConfigurationId"),
            ),
            mode: pulumi_wasm_rust::__private::into_domain(o.extract_field("mode")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            virtual_network_gateway_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualNetworkGatewayId"),
            ),
        }
    }
}
