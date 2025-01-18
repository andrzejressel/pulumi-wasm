/// Manages a System Center Virtual Machine Manager Availability Set.
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
///     let exampleVirtualMachineManagerAvailabilitySet = virtual_machine_manager_availability_set::create(
///         "exampleVirtualMachineManagerAvailabilitySet",
///         VirtualMachineManagerAvailabilitySetArgs::builder()
///             .custom_location_id("${exampleVirtualMachineManagerServer.customLocationId}")
///             .location("${example.location}")
///             .name("example-scvmmas")
///             .resource_group_name("${example.name}")
///             .system_center_virtual_machine_manager_server_id(
///                 "${exampleVirtualMachineManagerServer.id}",
///             )
///             .build_struct(),
///     );
///     let exampleVirtualMachineManagerServer = virtual_machine_manager_server::create(
///         "exampleVirtualMachineManagerServer",
///         VirtualMachineManagerServerArgs::builder()
///             .custom_location_id(
///                 "/subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.ExtendedLocation/customLocations/customLocation1",
///             )
///             .fqdn("example.labtest")
///             .location("${example.location}")
///             .name("example-scvmmms")
///             .password("H@Sh1CoR3!")
///             .resource_group_name("${example.name}")
///             .username("testUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// System Center Virtual Machine Manager Availability Sets can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:systemcenter/virtualMachineManagerAvailabilitySet:VirtualMachineManagerAvailabilitySet example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.ScVmm/availabilitySets/availabilitySet1
/// ```
///
pub mod virtual_machine_manager_availability_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineManagerAvailabilitySetArgs {
        /// The ID of the Custom Location for the System Center Virtual Machine Manager Availability Set. Changing this forces a new resource to be created.
        #[builder(into)]
        pub custom_location_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the System Center Virtual Machine Manager Availability Set should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the System Center Virtual Machine Manager Availability Set. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the System Center Virtual Machine Availability Set should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the System Center Virtual Machine Manager Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub system_center_virtual_machine_manager_server_id: pulumi_wasm_rust::Output<
            String,
        >,
        /// A mapping of tags which should be assigned to the System Center Virtual Machine Manager Availability Set.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineManagerAvailabilitySetResult {
        /// The ID of the Custom Location for the System Center Virtual Machine Manager Availability Set. Changing this forces a new resource to be created.
        pub custom_location_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the System Center Virtual Machine Manager Availability Set should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the System Center Virtual Machine Manager Availability Set. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the System Center Virtual Machine Availability Set should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the System Center Virtual Machine Manager Server. Changing this forces a new resource to be created.
        pub system_center_virtual_machine_manager_server_id: pulumi_wasm_rust::Output<
            String,
        >,
        /// A mapping of tags which should be assigned to the System Center Virtual Machine Manager Availability Set.
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
        args: VirtualMachineManagerAvailabilitySetArgs,
    ) -> VirtualMachineManagerAvailabilitySetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_location_id_binding = args.custom_location_id.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let system_center_virtual_machine_manager_server_id_binding = args
            .system_center_virtual_machine_manager_server_id
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:systemcenter/virtualMachineManagerAvailabilitySet:VirtualMachineManagerAvailabilitySet"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "systemCenterVirtualMachineManagerServerId".into(),
                    value: &system_center_virtual_machine_manager_server_id_binding,
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
                    name: "systemCenterVirtualMachineManagerServerId".into(),
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
        VirtualMachineManagerAvailabilitySetResult {
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
            system_center_virtual_machine_manager_server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("systemCenterVirtualMachineManagerServerId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
