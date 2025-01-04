/// Manages a System Center Virtual Machine Manager Virtual Network.
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
///   exampleVirtualMachineManagerServer:
///     type: azure:systemcenter:VirtualMachineManagerServer
///     name: example
///     properties:
///       name: example-scvmmms
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       customLocationId: /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.ExtendedLocation/customLocations/customLocation1
///       fqdn: example.labtest
///       username: testUser
///       password: H@Sh1CoR3!
///   exampleVirtualMachineManagerVirtualNetwork:
///     type: azure:systemcenter:VirtualMachineManagerVirtualNetwork
///     name: example
///     properties:
///       name: example-scvmmvnet
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       customLocationId: ${exampleVirtualMachineManagerServer.customLocationId}
///       systemCenterVirtualMachineManagerServerInventoryItemId: ${example.inventoryItems[0].id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:systemcenter:getVirtualMachineManagerInventoryItems
///       arguments:
///         inventoryType: VirtualNetwork
///         systemCenterVirtualMachineManagerServerId: ${exampleVirtualMachineManagerServer.id}
/// ```
///
/// ## Import
///
/// System Center Virtual Machine Manager Virtual Networks can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:systemcenter/virtualMachineManagerVirtualNetwork:VirtualMachineManagerVirtualNetwork example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.ScVmm/virtualNetworks/virtualNetwork1
/// ```
///
pub mod virtual_machine_manager_virtual_network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineManagerVirtualNetworkArgs {
        /// The ID of the Custom Location for the System Center Virtual Machine Manager Virtual Network. Changing this forces a new resource to be created.
        #[builder(into)]
        pub custom_location_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the System Center Virtual Machine Manager Virtual Network should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the System Center Virtual Machine Manager Virtual Network. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the System Center Virtual Machine Virtual Network should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the System Center Virtual Machine Manager Server Inventory Item. Changing this forces a new resource to be created.
        #[builder(into)]
        pub system_center_virtual_machine_manager_server_inventory_item_id: pulumi_wasm_rust::Output<
            String,
        >,
        /// A mapping of tags which should be assigned to the System Center Virtual Machine Manager Virtual Network.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineManagerVirtualNetworkResult {
        /// The ID of the Custom Location for the System Center Virtual Machine Manager Virtual Network. Changing this forces a new resource to be created.
        pub custom_location_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the System Center Virtual Machine Manager Virtual Network should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the System Center Virtual Machine Manager Virtual Network. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the System Center Virtual Machine Virtual Network should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the System Center Virtual Machine Manager Server Inventory Item. Changing this forces a new resource to be created.
        pub system_center_virtual_machine_manager_server_inventory_item_id: pulumi_wasm_rust::Output<
            String,
        >,
        /// A mapping of tags which should be assigned to the System Center Virtual Machine Manager Virtual Network.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VirtualMachineManagerVirtualNetworkArgs,
    ) -> VirtualMachineManagerVirtualNetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_location_id_binding = args.custom_location_id.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let system_center_virtual_machine_manager_server_inventory_item_id_binding = args
            .system_center_virtual_machine_manager_server_inventory_item_id
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:systemcenter/virtualMachineManagerVirtualNetwork:VirtualMachineManagerVirtualNetwork"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customLocationId".into(),
                    value: &custom_location_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "systemCenterVirtualMachineManagerServerInventoryItemId"
                        .into(),
                    value: &system_center_virtual_machine_manager_server_inventory_item_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "customLocationId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "systemCenterVirtualMachineManagerServerInventoryItemId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualMachineManagerVirtualNetworkResult {
            custom_location_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customLocationId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            system_center_virtual_machine_manager_server_inventory_item_id: pulumi_wasm_rust::__private::into_domain(
                hashmap
                    .remove("systemCenterVirtualMachineManagerServerInventoryItemId")
                    .unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
