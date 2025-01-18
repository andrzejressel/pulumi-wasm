/// Manages a Dedicated Hardware Security Module.
///
/// > **Note:** Before using this resource, it's required to submit the request of registering the providers and features with Azure CLI `az provider register --namespace Microsoft.HardwareSecurityModules && az feature register --namespace Microsoft.HardwareSecurityModules --name AzureDedicatedHSM && az provider register --namespace Microsoft.Network && az feature register --namespace Microsoft.Network --name AllowBaremetalServers` and ask service team (hsmrequest@microsoft.com) to approve. See more details from <https://docs.microsoft.com/azure/dedicated-hsm/tutorial-deploy-hsm-cli#prerequisites>.
///
/// > **Note:** If the quota is not enough in some region, please submit the quota request to service team.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.2.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example-compute
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.2.0.0/24
///   example2:
///     type: azure:network:Subnet
///     properties:
///       name: example-hsmsubnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.2.1.0/24
///       delegations:
///         - name: first
///           serviceDelegation:
///             name: Microsoft.HardwareSecurityModules/dedicatedHSMs
///             actions:
///               - Microsoft.Network/networkinterfaces/*
///               - Microsoft.Network/virtualNetworks/subnets/join/action
///   example3:
///     type: azure:network:Subnet
///     properties:
///       name: gatewaysubnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.2.255.0/26
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: example-pip
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       allocationMethod: Static
///   exampleVirtualNetworkGateway:
///     type: azure:network:VirtualNetworkGateway
///     name: example
///     properties:
///       name: example-vnetgateway
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       type: ExpressRoute
///       vpnType: PolicyBased
///       sku: Standard
///       ipConfigurations:
///         - publicIpAddressId: ${examplePublicIp.id}
///           privateIpAddressAllocation: Dynamic
///           subnetId: ${example3.id}
///   exampleModule:
///     type: azure:hsm:Module
///     name: example
///     properties:
///       name: example-hsm
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: payShield10K_LMK1_CPS60
///       managementNetworkProfile:
///         networkInterfacePrivateIpAddresses:
///           - 10.2.1.7
///         subnetId: ${example2.id}
///       networkProfile:
///         networkInterfacePrivateIpAddresses:
///           - 10.2.1.8
///         subnetId: ${example2.id}
///       stampId: stamp2
///       tags:
///         env: Test
///     options:
///       dependsOn:
///         - ${exampleVirtualNetworkGateway}
/// ```
///
/// ## Import
///
/// Dedicated Hardware Security Module can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:hsm/module:Module example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.HardwareSecurityModules/dedicatedHSMs/hsm1
/// ```
///
pub mod module {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ModuleArgs {
        /// The Azure Region where the Dedicated Hardware Security Module should exist. Changing this forces a new Dedicated Hardware Security Module to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// A `management_network_profile` block as defined below.
        ///
        /// ->**NOTE:**  The `management_network_profile` should not be specified when `sku_name` is `SafeNet Luna Network HSM A790`.
        #[builder(into, default)]
        pub management_network_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::hsm::ModuleManagementNetworkProfile>,
        >,
        /// The name which should be used for this Dedicated Hardware Security Module. Changing this forces a new Dedicated Hardware Security Module to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `network_profile` block as defined below.
        #[builder(into)]
        pub network_profile: pulumi_wasm_rust::Output<
            super::super::types::hsm::ModuleNetworkProfile,
        >,
        /// The name of the Resource Group where the Dedicated Hardware Security Module should exist. Changing this forces a new Dedicated Hardware Security Module to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU name of the dedicated hardware security module. Possible values are `payShield10K_LMK1_CPS60`,`payShield10K_LMK1_CPS250`,`payShield10K_LMK1_CPS2500`,`payShield10K_LMK2_CPS60`,`payShield10K_LMK2_CPS250`,`payShield10K_LMK2_CPS2500` and `SafeNet Luna Network HSM A790`. Changing this forces a new Dedicated Hardware Security Module to be created.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the stamp. Possible values are `stamp1` or `stamp2`. Changing this forces a new Dedicated Hardware Security Module to be created.
        #[builder(into, default)]
        pub stamp_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Dedicated Hardware Security Module.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a list of Availability Zones in which this Dedicated Hardware Security Module should be located. Changing this forces a new Dedicated Hardware Security Module to be created.
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ModuleResult {
        /// The Azure Region where the Dedicated Hardware Security Module should exist. Changing this forces a new Dedicated Hardware Security Module to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `management_network_profile` block as defined below.
        ///
        /// ->**NOTE:**  The `management_network_profile` should not be specified when `sku_name` is `SafeNet Luna Network HSM A790`.
        pub management_network_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::hsm::ModuleManagementNetworkProfile>,
        >,
        /// The name which should be used for this Dedicated Hardware Security Module. Changing this forces a new Dedicated Hardware Security Module to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `network_profile` block as defined below.
        pub network_profile: pulumi_wasm_rust::Output<
            super::super::types::hsm::ModuleNetworkProfile,
        >,
        /// The name of the Resource Group where the Dedicated Hardware Security Module should exist. Changing this forces a new Dedicated Hardware Security Module to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU name of the dedicated hardware security module. Possible values are `payShield10K_LMK1_CPS60`,`payShield10K_LMK1_CPS250`,`payShield10K_LMK1_CPS2500`,`payShield10K_LMK2_CPS60`,`payShield10K_LMK2_CPS250`,`payShield10K_LMK2_CPS2500` and `SafeNet Luna Network HSM A790`. Changing this forces a new Dedicated Hardware Security Module to be created.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the stamp. Possible values are `stamp1` or `stamp2`. Changing this forces a new Dedicated Hardware Security Module to be created.
        pub stamp_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Dedicated Hardware Security Module.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a list of Availability Zones in which this Dedicated Hardware Security Module should be located. Changing this forces a new Dedicated Hardware Security Module to be created.
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ModuleArgs) -> ModuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let management_network_profile_binding = args
            .management_network_profile
            .get_inner();
        let name_binding = args.name.get_inner();
        let network_profile_binding = args.network_profile.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let stamp_id_binding = args.stamp_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let zones_binding = args.zones.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:hsm/module:Module".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managementNetworkProfile".into(),
                    value: &management_network_profile_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkProfile".into(),
                    value: &network_profile_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "stampId".into(),
                    value: &stamp_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "zones".into(),
                    value: &zones_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managementNetworkProfile".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkProfile".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "stampId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ModuleResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            management_network_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementNetworkProfile").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkProfile").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            stamp_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stampId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}
