/// Manages a maintenance assignment to virtual machine.
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
///       name: example-network
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: internal
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleNetworkInterface:
///     type: azure:network:NetworkInterface
///     name: example
///     properties:
///       name: example-nic
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       ipConfigurations:
///         - name: internal
///           subnetId: ${exampleSubnet.id}
///           privateIpAddressAllocation: Dynamic
///   exampleLinuxVirtualMachine:
///     type: azure:compute:LinuxVirtualMachine
///     name: example
///     properties:
///       name: example-machine
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       size: Standard_F2
///       adminUsername: adminuser
///       networkInterfaceIds:
///         - ${exampleNetworkInterface.id}
///       adminSshKeys:
///         - username: adminuser
///           publicKey:
///             fn::invoke:
///               function: std:file
///               arguments:
///                 input: ~/.ssh/id_rsa.pub
///               return: result
///       osDisk:
///         caching: ReadWrite
///         storageAccountType: Standard_LRS
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///   exampleConfiguration:
///     type: azure:maintenance:Configuration
///     name: example
///     properties:
///       name: example-mc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       scope: All
///   exampleAssignmentVirtualMachine:
///     type: azure:maintenance:AssignmentVirtualMachine
///     name: example
///     properties:
///       location: ${example.location}
///       maintenanceConfigurationId: ${exampleConfiguration.id}
///       virtualMachineId: ${exampleLinuxVirtualMachine.id}
/// ```
///
/// ## Import
///
/// Maintenance Assignment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:maintenance/assignmentVirtualMachine:AssignmentVirtualMachine example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.Compute/virtualMachines/vm1/providers/Microsoft.Maintenance/configurationAssignments/assign1
/// ```
///
pub mod assignment_virtual_machine {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssignmentVirtualMachineArgs {
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the Maintenance Configuration Resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub maintenance_configuration_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Virtual Machine ID to which the Maintenance Configuration will be assigned. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_machine_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AssignmentVirtualMachineResult {
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the Maintenance Configuration Resource. Changing this forces a new resource to be created.
        pub maintenance_configuration_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Virtual Machine ID to which the Maintenance Configuration will be assigned. Changing this forces a new resource to be created.
        pub virtual_machine_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AssignmentVirtualMachineArgs,
    ) -> AssignmentVirtualMachineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let maintenance_configuration_id_binding = args
            .maintenance_configuration_id
            .get_inner();
        let virtual_machine_id_binding = args.virtual_machine_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:maintenance/assignmentVirtualMachine:AssignmentVirtualMachine"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceConfigurationId".into(),
                    value: &maintenance_configuration_id_binding,
                },
                register_interface::ObjectField {
                    name: "virtualMachineId".into(),
                    value: &virtual_machine_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceConfigurationId".into(),
                },
                register_interface::ResultField {
                    name: "virtualMachineId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AssignmentVirtualMachineResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maintenance_configuration_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceConfigurationId").unwrap(),
            ),
            virtual_machine_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualMachineId").unwrap(),
            ),
        }
    }
}