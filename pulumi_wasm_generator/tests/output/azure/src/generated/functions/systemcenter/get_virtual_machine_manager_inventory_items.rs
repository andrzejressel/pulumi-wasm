pub mod get_virtual_machine_manager_inventory_items {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualMachineManagerInventoryItemsArgs {
        /// The inventory type of the System Center Virtual Machine Manager Inventory Item. Possible values are `Cloud`, `VirtualMachine`, `VirtualMachineTemplate` and `VirtualNetwork`.
        #[builder(into)]
        pub inventory_type: pulumi_wasm_rust::Output<String>,
        /// The ID of the System Center Virtual Machine Manager Server.
        #[builder(into)]
        pub system_center_virtual_machine_manager_server_id: pulumi_wasm_rust::Output<
            String,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVirtualMachineManagerInventoryItemsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// One or more `inventory_items` blocks as defined below.
        pub inventory_items: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::systemcenter::GetVirtualMachineManagerInventoryItemsInventoryItem,
            >,
        >,
        pub inventory_type: pulumi_wasm_rust::Output<String>,
        pub system_center_virtual_machine_manager_server_id: pulumi_wasm_rust::Output<
            String,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetVirtualMachineManagerInventoryItemsArgs,
    ) -> GetVirtualMachineManagerInventoryItemsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let inventory_type_binding = args.inventory_type.get_inner();
        let system_center_virtual_machine_manager_server_id_binding = args
            .system_center_virtual_machine_manager_server_id
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:systemcenter/getVirtualMachineManagerInventoryItems:getVirtualMachineManagerInventoryItems"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "inventoryType".into(),
                    value: &inventory_type_binding,
                },
                register_interface::ObjectField {
                    name: "systemCenterVirtualMachineManagerServerId".into(),
                    value: &system_center_virtual_machine_manager_server_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "inventoryItems".into(),
                },
                register_interface::ResultField {
                    name: "inventoryType".into(),
                },
                register_interface::ResultField {
                    name: "systemCenterVirtualMachineManagerServerId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVirtualMachineManagerInventoryItemsResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            inventory_items: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inventoryItems").unwrap(),
            ),
            inventory_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inventoryType").unwrap(),
            ),
            system_center_virtual_machine_manager_server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("systemCenterVirtualMachineManagerServerId").unwrap(),
            ),
        }
    }
}