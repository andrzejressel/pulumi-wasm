#[allow(clippy::doc_lazy_continuation)]
pub mod get_virtual_machine_manager_inventory_items {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualMachineManagerInventoryItemsArgs {
        /// The inventory type of the System Center Virtual Machine Manager Inventory Item. Possible values are `Cloud`, `VirtualMachine`, `VirtualMachineTemplate` and `VirtualNetwork`.
        #[builder(into)]
        pub inventory_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the System Center Virtual Machine Manager Server.
        #[builder(into)]
        pub system_center_virtual_machine_manager_server_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVirtualMachineManagerInventoryItemsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// One or more `inventory_items` blocks as defined below.
        pub inventory_items: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::systemcenter::GetVirtualMachineManagerInventoryItemsInventoryItem,
            >,
        >,
        pub inventory_type: pulumi_gestalt_rust::Output<String>,
        pub system_center_virtual_machine_manager_server_id: pulumi_gestalt_rust::Output<
            String,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVirtualMachineManagerInventoryItemsArgs,
    ) -> GetVirtualMachineManagerInventoryItemsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let inventory_type_binding = args.inventory_type.get_output(context).get_inner();
        let system_center_virtual_machine_manager_server_id_binding = args
            .system_center_virtual_machine_manager_server_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:systemcenter/getVirtualMachineManagerInventoryItems:getVirtualMachineManagerInventoryItems"
                .into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVirtualMachineManagerInventoryItemsResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            inventory_items: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inventoryItems"),
            ),
            inventory_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inventoryType"),
            ),
            system_center_virtual_machine_manager_server_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("systemCenterVirtualMachineManagerServerId"),
            ),
        }
    }
}
