/// Applies a Guest Configuration Policy to a Virtual Machine.
///
/// > **NOTE:** You can create Guest Configuration Policies without defining a `azure.compute.Extension` resource, however the policies will not be executed until a `azure.compute.Extension` has been provisioned to the virtual machine.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-gca
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       addressSpaces:
///         - 10.0.0.0/16
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
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       ipConfigurations:
///         - name: internal
///           subnetId: ${exampleSubnet.id}
///           privateIpAddressAllocation: Dynamic
///   exampleWindowsVirtualMachine:
///     type: azure:compute:WindowsVirtualMachine
///     name: example
///     properties:
///       name: examplevm
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       size: Standard_F2
///       adminUsername: adminuser
///       adminPassword: P@$$w0rd1234!
///       networkInterfaceIds:
///         - ${exampleNetworkInterface.id}
///       identity:
///         type: SystemAssigned
///       osDisk:
///         caching: ReadWrite
///         storageAccountType: Standard_LRS
///       sourceImageReference:
///         publisher: MicrosoftWindowsServer
///         offer: WindowsServer
///         sku: 2019-Datacenter
///         version: latest
///   exampleExtension:
///     type: azure:compute:Extension
///     name: example
///     properties:
///       name: AzurePolicyforWindows
///       virtualMachineId: ${exampleWindowsVirtualMachine.id}
///       publisher: Microsoft.GuestConfiguration
///       type: ConfigurationforWindows
///       typeHandlerVersion: '1.29'
///       autoUpgradeMinorVersion: 'true'
///   exampleVirtualMachineConfigurationAssignment:
///     type: azure:policy:VirtualMachineConfigurationAssignment
///     name: example
///     properties:
///       name: AzureWindowsBaseline
///       location: ${exampleWindowsVirtualMachine.location}
///       virtualMachineId: ${exampleWindowsVirtualMachine.id}
///       configuration:
///         assignmentType: ApplyAndMonitor
///         version: 1.*
///         parameters:
///           - name: Minimum Password Length;ExpectedValue
///             value: '16'
///           - name: Minimum Password Age;ExpectedValue
///             value: '0'
///           - name: Maximum Password Age;ExpectedValue
///             value: 30,45
///           - name: Enforce Password History;ExpectedValue
///             value: '10'
///           - name: Password Must Meet Complexity Requirements;ExpectedValue
///             value: '1'
/// ```
///
/// ## Import
///
/// Policy Virtual Machine Configuration Assignments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:policy/virtualMachineConfigurationAssignment:VirtualMachineConfigurationAssignment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Compute/virtualMachines/vm1/providers/Microsoft.GuestConfiguration/guestConfigurationAssignments/assignment1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_machine_configuration_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineConfigurationAssignmentArgs {
        /// A `configuration` block as defined below.
        #[builder(into)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::policy::VirtualMachineConfigurationAssignmentConfiguration,
        >,
        /// The Azure location where the Policy Virtual Machine Configuration Assignment should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Guest Configuration that will be assigned in this Guest Configuration Assignment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the Policy Virtual Machine which this Guest Configuration Assignment should apply to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_machine_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineConfigurationAssignmentResult {
        /// A `configuration` block as defined below.
        pub configuration: pulumi_gestalt_rust::Output<
            super::super::types::policy::VirtualMachineConfigurationAssignmentConfiguration,
        >,
        /// The Azure location where the Policy Virtual Machine Configuration Assignment should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Guest Configuration that will be assigned in this Guest Configuration Assignment. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the Policy Virtual Machine which this Guest Configuration Assignment should apply to. Changing this forces a new resource to be created.
        pub virtual_machine_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualMachineConfigurationAssignmentArgs,
    ) -> VirtualMachineConfigurationAssignmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_binding = args.configuration.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let virtual_machine_id_binding = args.virtual_machine_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:policy/virtualMachineConfigurationAssignment:VirtualMachineConfigurationAssignment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualMachineId".into(),
                    value: &virtual_machine_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualMachineConfigurationAssignmentResult {
            configuration: o.get_field("configuration"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            virtual_machine_id: o.get_field("virtualMachineId"),
        }
    }
}
