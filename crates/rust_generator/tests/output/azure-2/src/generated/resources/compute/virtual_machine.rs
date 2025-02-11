/// Manages a Virtual Machine.
///
/// ## Disclaimers
///
/// > **Note:** The `azure.compute.VirtualMachine` resource has been superseded by the `azure.compute.LinuxVirtualMachine` and `azure.compute.WindowsVirtualMachine` resources. The existing `azure.compute.VirtualMachine` resource will continue to be available throughout the 2.x releases however is in a feature-frozen state to maintain compatibility - new functionality will instead be added to the `azure.compute.LinuxVirtualMachine` and `azure.compute.WindowsVirtualMachine` resources.
///
/// > **Note:** Data Disks can be attached either directly on the `azure.compute.VirtualMachine` resource, or using the `azure.compute.DataDiskAttachment` resource - but the two cannot be used together. If both are used against the same Virtual Machine, spurious changes will occur.
///
/// ## Example Usage
///
/// ### From An Azure Platform Image)
///
/// This example provisions a Virtual Machine with Managed Disks.
///
/// ```yaml
/// configuration:
///   prefix:
///     type: string
///     default: tfvmex
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: ${prefix}-resources
///       location: West Europe
///   main:
///     type: azure:network:VirtualNetwork
///     properties:
///       name: ${prefix}-network
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   internal:
///     type: azure:network:Subnet
///     properties:
///       name: internal
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${main.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   mainNetworkInterface:
///     type: azure:network:NetworkInterface
///     name: main
///     properties:
///       name: ${prefix}-nic
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       ipConfigurations:
///         - name: testconfiguration1
///           subnetId: ${internal.id}
///           privateIpAddressAllocation: Dynamic
///   mainVirtualMachine:
///     type: azure:compute:VirtualMachine
///     name: main
///     properties:
///       name: ${prefix}-vm
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       networkInterfaceIds:
///         - ${mainNetworkInterface.id}
///       vmSize: Standard_DS1_v2
///       storageImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///       storageOsDisk:
///         name: myosdisk1
///         caching: ReadWrite
///         createOption: FromImage
///         managedDiskType: Standard_LRS
///       osProfile:
///         computerName: hostname
///         adminUsername: testadmin
///         adminPassword: Password1234!
///       osProfileLinuxConfig:
///         disablePasswordAuthentication: false
///       tags:
///         environment: staging
/// ```
///
/// ## Import
///
/// Virtual Machines can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/virtualMachine:VirtualMachine example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/virtualMachines/machine1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_machine {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineArgs {
        /// An `additional_capabilities` block as defined below.
        #[builder(into, default)]
        pub additional_capabilities: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::VirtualMachineAdditionalCapabilities>,
        >,
        /// The ID of the Availability Set in which the Virtual Machine should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub availability_set_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `boot_diagnostics` block as defined below.
        #[builder(into, default)]
        pub boot_diagnostics: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::VirtualMachineBootDiagnostics>,
        >,
        /// Should the Data Disks (either the Managed Disks / VHD Blobs) be deleted when the Virtual Machine is destroyed? Defaults to `false`.
        ///
        /// > **Note:** This setting works when instance is deleted via the provider only and don't forget to delete disks manually if you deleted VM manually. It can increase spending.
        #[builder(into, default)]
        pub delete_data_disks_on_termination: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Should the OS Disk (either the Managed Disk / VHD Blob) be deleted when the Virtual Machine is destroyed? Defaults to `false`.
        ///
        /// > **Note:** This setting works when instance is deleted via the provider only and don't forget to delete disks manually if you deleted VM manually. It can increase spending.
        #[builder(into, default)]
        pub delete_os_disk_on_termination: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::VirtualMachineIdentity>,
        >,
        /// Specifies the BYOL Type for this Virtual Machine. This is only applicable to Windows Virtual Machines. Possible values are `Windows_Client` and `Windows_Server`.
        #[builder(into, default)]
        pub license_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Azure Region where the Virtual Machine exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of Network Interface IDs which should be associated with the Virtual Machine.
        #[builder(into)]
        pub network_interface_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// An `os_profile` block as defined below. Required when `create_option` in the `storage_os_disk` block is set to `FromImage`.
        #[builder(into, default)]
        pub os_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::VirtualMachineOsProfile>,
        >,
        /// (Required, when a Linux machine) An `os_profile_linux_config` block as defined below.
        #[builder(into, default)]
        pub os_profile_linux_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::VirtualMachineOsProfileLinuxConfig>,
        >,
        /// One or more `os_profile_secrets` blocks as defined below.
        #[builder(into, default)]
        pub os_profile_secrets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::VirtualMachineOsProfileSecret>>,
        >,
        /// (Required, when a Windows machine) An `os_profile_windows_config` block as defined below.
        #[builder(into, default)]
        pub os_profile_windows_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::VirtualMachineOsProfileWindowsConfig>,
        >,
        /// A `plan` block as defined below.
        #[builder(into, default)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::VirtualMachinePlan>,
        >,
        /// The ID of the Network Interface (which must be attached to the Virtual Machine) which should be the Primary Network Interface for this Virtual Machine.
        #[builder(into, default)]
        pub primary_network_interface_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the Proximity Placement Group to which this Virtual Machine should be assigned. Changing this forces a new resource to be created
        #[builder(into, default)]
        pub proximity_placement_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the Resource Group in which the Virtual Machine should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `storage_data_disk` blocks as defined below.
        ///
        /// > **Please Note:** Data Disks can also be attached either using this block or the `azure.compute.DataDiskAttachment` resource - but not both.
        #[builder(into, default)]
        pub storage_data_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::VirtualMachineStorageDataDisk>>,
        >,
        /// A `storage_image_reference` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub storage_image_reference: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::VirtualMachineStorageImageReference>,
        >,
        /// A `storage_os_disk` block as defined below.
        #[builder(into)]
        pub storage_os_disk: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::VirtualMachineStorageOsDisk,
        >,
        /// A mapping of tags to assign to the Virtual Machine.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the [size of the Virtual Machine](https://docs.microsoft.com/azure/virtual-machines/sizes-general). See also [Azure VM Naming Conventions](https://docs.microsoft.com/azure/virtual-machines/vm-naming-conventions).
        #[builder(into)]
        pub vm_size: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of a single item of the Availability Zone which the Virtual Machine should be allocated in. Changing this forces a new resource to be created.
        ///
        /// > **Please Note**: Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        ///
        /// For more information on the different example configurations, please check out the [Azure documentation](https://docs.microsoft.com/en-gb/rest/api/compute/virtualmachines/createorupdate#examples)
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineResult {
        /// An `additional_capabilities` block as defined below.
        pub additional_capabilities: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::VirtualMachineAdditionalCapabilities>,
        >,
        /// The ID of the Availability Set in which the Virtual Machine should exist. Changing this forces a new resource to be created.
        pub availability_set_id: pulumi_gestalt_rust::Output<String>,
        /// A `boot_diagnostics` block as defined below.
        pub boot_diagnostics: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::VirtualMachineBootDiagnostics>,
        >,
        /// Should the Data Disks (either the Managed Disks / VHD Blobs) be deleted when the Virtual Machine is destroyed? Defaults to `false`.
        ///
        /// > **Note:** This setting works when instance is deleted via the provider only and don't forget to delete disks manually if you deleted VM manually. It can increase spending.
        pub delete_data_disks_on_termination: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should the OS Disk (either the Managed Disk / VHD Blob) be deleted when the Virtual Machine is destroyed? Defaults to `false`.
        ///
        /// > **Note:** This setting works when instance is deleted via the provider only and don't forget to delete disks manually if you deleted VM manually. It can increase spending.
        pub delete_os_disk_on_termination: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::VirtualMachineIdentity>,
        >,
        /// Specifies the BYOL Type for this Virtual Machine. This is only applicable to Windows Virtual Machines. Possible values are `Windows_Client` and `Windows_Server`.
        pub license_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Azure Region where the Virtual Machine exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Virtual Machine. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of Network Interface IDs which should be associated with the Virtual Machine.
        pub network_interface_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// An `os_profile` block as defined below. Required when `create_option` in the `storage_os_disk` block is set to `FromImage`.
        pub os_profile: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::VirtualMachineOsProfile>,
        >,
        /// (Required, when a Linux machine) An `os_profile_linux_config` block as defined below.
        pub os_profile_linux_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::VirtualMachineOsProfileLinuxConfig>,
        >,
        /// One or more `os_profile_secrets` blocks as defined below.
        pub os_profile_secrets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::VirtualMachineOsProfileSecret>>,
        >,
        /// (Required, when a Windows machine) An `os_profile_windows_config` block as defined below.
        pub os_profile_windows_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::VirtualMachineOsProfileWindowsConfig>,
        >,
        /// A `plan` block as defined below.
        pub plan: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::VirtualMachinePlan>,
        >,
        /// The ID of the Network Interface (which must be attached to the Virtual Machine) which should be the Primary Network Interface for this Virtual Machine.
        pub primary_network_interface_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Proximity Placement Group to which this Virtual Machine should be assigned. Changing this forces a new resource to be created
        pub proximity_placement_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Resource Group in which the Virtual Machine should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `storage_data_disk` blocks as defined below.
        ///
        /// > **Please Note:** Data Disks can also be attached either using this block or the `azure.compute.DataDiskAttachment` resource - but not both.
        pub storage_data_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::VirtualMachineStorageDataDisk>,
        >,
        /// A `storage_image_reference` block as defined below. Changing this forces a new resource to be created.
        pub storage_image_reference: pulumi_gestalt_rust::Output<
            super::super::types::compute::VirtualMachineStorageImageReference,
        >,
        /// A `storage_os_disk` block as defined below.
        pub storage_os_disk: pulumi_gestalt_rust::Output<
            super::super::types::compute::VirtualMachineStorageOsDisk,
        >,
        /// A mapping of tags to assign to the Virtual Machine.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the [size of the Virtual Machine](https://docs.microsoft.com/azure/virtual-machines/sizes-general). See also [Azure VM Naming Conventions](https://docs.microsoft.com/azure/virtual-machines/vm-naming-conventions).
        pub vm_size: pulumi_gestalt_rust::Output<String>,
        /// A list of a single item of the Availability Zone which the Virtual Machine should be allocated in. Changing this forces a new resource to be created.
        ///
        /// > **Please Note**: Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        ///
        /// For more information on the different example configurations, please check out the [Azure documentation](https://docs.microsoft.com/en-gb/rest/api/compute/virtualmachines/createorupdate#examples)
        pub zones: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualMachineArgs,
    ) -> VirtualMachineResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_capabilities_binding = args
            .additional_capabilities
            .get_output(context);
        let availability_set_id_binding = args.availability_set_id.get_output(context);
        let boot_diagnostics_binding = args.boot_diagnostics.get_output(context);
        let delete_data_disks_on_termination_binding = args
            .delete_data_disks_on_termination
            .get_output(context);
        let delete_os_disk_on_termination_binding = args
            .delete_os_disk_on_termination
            .get_output(context);
        let identity_binding = args.identity.get_output(context);
        let license_type_binding = args.license_type.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_interface_ids_binding = args
            .network_interface_ids
            .get_output(context);
        let os_profile_binding = args.os_profile.get_output(context);
        let os_profile_linux_config_binding = args
            .os_profile_linux_config
            .get_output(context);
        let os_profile_secrets_binding = args.os_profile_secrets.get_output(context);
        let os_profile_windows_config_binding = args
            .os_profile_windows_config
            .get_output(context);
        let plan_binding = args.plan.get_output(context);
        let primary_network_interface_id_binding = args
            .primary_network_interface_id
            .get_output(context);
        let proximity_placement_group_id_binding = args
            .proximity_placement_group_id
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let storage_data_disks_binding = args.storage_data_disks.get_output(context);
        let storage_image_reference_binding = args
            .storage_image_reference
            .get_output(context);
        let storage_os_disk_binding = args.storage_os_disk.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vm_size_binding = args.vm_size.get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/virtualMachine:VirtualMachine".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalCapabilities".into(),
                    value: &additional_capabilities_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilitySetId".into(),
                    value: &availability_set_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bootDiagnostics".into(),
                    value: &boot_diagnostics_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deleteDataDisksOnTermination".into(),
                    value: &delete_data_disks_on_termination_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deleteOsDiskOnTermination".into(),
                    value: &delete_os_disk_on_termination_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseType".into(),
                    value: &license_type_binding.drop_type(),
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
                    name: "networkInterfaceIds".into(),
                    value: &network_interface_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osProfile".into(),
                    value: &os_profile_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osProfileLinuxConfig".into(),
                    value: &os_profile_linux_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osProfileSecrets".into(),
                    value: &os_profile_secrets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osProfileWindowsConfig".into(),
                    value: &os_profile_windows_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plan".into(),
                    value: &plan_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "primaryNetworkInterfaceId".into(),
                    value: &primary_network_interface_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "proximityPlacementGroupId".into(),
                    value: &proximity_placement_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageDataDisks".into(),
                    value: &storage_data_disks_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageImageReference".into(),
                    value: &storage_image_reference_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageOsDisk".into(),
                    value: &storage_os_disk_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vmSize".into(),
                    value: &vm_size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: &zones_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualMachineResult {
            additional_capabilities: o.get_field("additionalCapabilities"),
            availability_set_id: o.get_field("availabilitySetId"),
            boot_diagnostics: o.get_field("bootDiagnostics"),
            delete_data_disks_on_termination: o
                .get_field("deleteDataDisksOnTermination"),
            delete_os_disk_on_termination: o.get_field("deleteOsDiskOnTermination"),
            identity: o.get_field("identity"),
            license_type: o.get_field("licenseType"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network_interface_ids: o.get_field("networkInterfaceIds"),
            os_profile: o.get_field("osProfile"),
            os_profile_linux_config: o.get_field("osProfileLinuxConfig"),
            os_profile_secrets: o.get_field("osProfileSecrets"),
            os_profile_windows_config: o.get_field("osProfileWindowsConfig"),
            plan: o.get_field("plan"),
            primary_network_interface_id: o.get_field("primaryNetworkInterfaceId"),
            proximity_placement_group_id: o.get_field("proximityPlacementGroupId"),
            resource_group_name: o.get_field("resourceGroupName"),
            storage_data_disks: o.get_field("storageDataDisks"),
            storage_image_reference: o.get_field("storageImageReference"),
            storage_os_disk: o.get_field("storageOsDisk"),
            tags: o.get_field("tags"),
            vm_size: o.get_field("vmSize"),
            zones: o.get_field("zones"),
        }
    }
}
