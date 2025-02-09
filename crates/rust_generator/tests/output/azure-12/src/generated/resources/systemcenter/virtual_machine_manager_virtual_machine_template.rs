/// Manages a System Center Virtual Machine Manager Virtual Machine Template.
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
///   exampleVirtualMachineManagerVirtualMachineTemplate:
///     type: azure:systemcenter:VirtualMachineManagerVirtualMachineTemplate
///     name: example
///     properties:
///       name: example-scvmmvmtemplate
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       customLocationId: ${exampleVirtualMachineManagerServer.customLocationId}
///       systemCenterVirtualMachineManagerServerInventoryItemId: ${example.inventoryItems[0].id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:systemcenter:getVirtualMachineManagerInventoryItems
///       arguments:
///         inventoryType: VirtualMachineTemplate
///         systemCenterVirtualMachineManagerServerId: ${exampleVirtualMachineManagerServer.id}
/// ```
///
/// ## Import
///
/// System Center Virtual Machine Manager Virtual Machine Templates can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:systemcenter/virtualMachineManagerVirtualMachineTemplate:VirtualMachineManagerVirtualMachineTemplate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.ScVmm/virtualMachineTemplates/virtualMachineTemplate1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_machine_manager_virtual_machine_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineManagerVirtualMachineTemplateArgs {
        /// The ID of the Custom Location for the System Center Virtual Machine Manager Virtual Machine Template. Changing this forces a new resource to be created.
        #[builder(into)]
        pub custom_location_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region where the System Center Virtual Machine Manager Virtual Machine Template should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the System Center Virtual Machine Manager Virtual Machine Template. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the System Center Virtual Machine Manager Virtual Machine Template should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the System Center Virtual Machine Manager Server Inventory Item. Changing this forces a new resource to be created.
        #[builder(into)]
        pub system_center_virtual_machine_manager_server_inventory_item_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// A mapping of tags which should be assigned to the System Center Virtual Machine Manager Virtual Machine Template.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineManagerVirtualMachineTemplateResult {
        /// The ID of the Custom Location for the System Center Virtual Machine Manager Virtual Machine Template. Changing this forces a new resource to be created.
        pub custom_location_id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the System Center Virtual Machine Manager Virtual Machine Template should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the System Center Virtual Machine Manager Virtual Machine Template. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the System Center Virtual Machine Manager Virtual Machine Template should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the System Center Virtual Machine Manager Server Inventory Item. Changing this forces a new resource to be created.
        pub system_center_virtual_machine_manager_server_inventory_item_id: pulumi_gestalt_rust::Output<
            String,
        >,
        /// A mapping of tags which should be assigned to the System Center Virtual Machine Manager Virtual Machine Template.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualMachineManagerVirtualMachineTemplateArgs,
    ) -> VirtualMachineManagerVirtualMachineTemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_location_id_binding = args.custom_location_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let system_center_virtual_machine_manager_server_inventory_item_id_binding = args
            .system_center_virtual_machine_manager_server_inventory_item_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:systemcenter/virtualMachineManagerVirtualMachineTemplate:VirtualMachineManagerVirtualMachineTemplate"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customLocationId".into(),
                    value: custom_location_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "systemCenterVirtualMachineManagerServerInventoryItemId"
                        .into(),
                    value: system_center_virtual_machine_manager_server_inventory_item_id_binding
                        .get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualMachineManagerVirtualMachineTemplateResult {
            custom_location_id: o.get_field("customLocationId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            system_center_virtual_machine_manager_server_inventory_item_id: o
                .get_field("systemCenterVirtualMachineManagerServerInventoryItemId"),
            tags: o.get_field("tags"),
        }
    }
}
