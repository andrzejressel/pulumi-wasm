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
pub mod virtual_machine {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineArgs {
        /// An `additional_capabilities` block as defined below.
        #[builder(into, default)]
        pub additional_capabilities: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::VirtualMachineAdditionalCapabilities>,
        >,
        /// The ID of the Availability Set in which the Virtual Machine should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub availability_set_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `boot_diagnostics` block as defined below.
        #[builder(into, default)]
        pub boot_diagnostics: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::VirtualMachineBootDiagnostics>,
        >,
        /// Should the Data Disks (either the Managed Disks / VHD Blobs) be deleted when the Virtual Machine is destroyed? Defaults to `false`.
        ///
        /// > **Note:** This setting works when instance is deleted via the provider only and don't forget to delete disks manually if you deleted VM manually. It can increase spending.
        #[builder(into, default)]
        pub delete_data_disks_on_termination: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the OS Disk (either the Managed Disk / VHD Blob) be deleted when the Virtual Machine is destroyed? Defaults to `false`.
        ///
        /// > **Note:** This setting works when instance is deleted via the provider only and don't forget to delete disks manually if you deleted VM manually. It can increase spending.
        #[builder(into, default)]
        pub delete_os_disk_on_termination: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::VirtualMachineIdentity>,
        >,
        /// Specifies the BYOL Type for this Virtual Machine. This is only applicable to Windows Virtual Machines. Possible values are `Windows_Client` and `Windows_Server`.
        #[builder(into, default)]
        pub license_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Virtual Machine exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of Network Interface IDs which should be associated with the Virtual Machine.
        #[builder(into)]
        pub network_interface_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// An `os_profile` block as defined below. Required when `create_option` in the `storage_os_disk` block is set to `FromImage`.
        #[builder(into, default)]
        pub os_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::VirtualMachineOsProfile>,
        >,
        /// (Required, when a Linux machine) An `os_profile_linux_config` block as defined below.
        #[builder(into, default)]
        pub os_profile_linux_config: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::VirtualMachineOsProfileLinuxConfig>,
        >,
        /// One or more `os_profile_secrets` blocks as defined below.
        #[builder(into, default)]
        pub os_profile_secrets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::VirtualMachineOsProfileSecret>>,
        >,
        /// (Required, when a Windows machine) An `os_profile_windows_config` block as defined below.
        #[builder(into, default)]
        pub os_profile_windows_config: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::VirtualMachineOsProfileWindowsConfig>,
        >,
        /// A `plan` block as defined below.
        #[builder(into, default)]
        pub plan: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::VirtualMachinePlan>,
        >,
        /// The ID of the Network Interface (which must be attached to the Virtual Machine) which should be the Primary Network Interface for this Virtual Machine.
        #[builder(into, default)]
        pub primary_network_interface_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Proximity Placement Group to which this Virtual Machine should be assigned. Changing this forces a new resource to be created
        #[builder(into, default)]
        pub proximity_placement_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Resource Group in which the Virtual Machine should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `storage_data_disk` blocks as defined below.
        ///
        /// > **Please Note:** Data Disks can also be attached either using this block or the `azure.compute.DataDiskAttachment` resource - but not both.
        #[builder(into, default)]
        pub storage_data_disks: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::VirtualMachineStorageDataDisk>>,
        >,
        /// A `storage_image_reference` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub storage_image_reference: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::VirtualMachineStorageImageReference>,
        >,
        /// A `storage_os_disk` block as defined below.
        #[builder(into)]
        pub storage_os_disk: pulumi_wasm_rust::Output<
            super::super::types::compute::VirtualMachineStorageOsDisk,
        >,
        /// A mapping of tags to assign to the Virtual Machine.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the [size of the Virtual Machine](https://docs.microsoft.com/azure/virtual-machines/sizes-general). See also [Azure VM Naming Conventions](https://docs.microsoft.com/azure/virtual-machines/vm-naming-conventions).
        #[builder(into)]
        pub vm_size: pulumi_wasm_rust::Output<String>,
        /// A list of a single item of the Availability Zone which the Virtual Machine should be allocated in. Changing this forces a new resource to be created.
        ///
        /// > **Please Note**: Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        ///
        /// For more information on the different example configurations, please check out the [Azure documentation](https://docs.microsoft.com/en-gb/rest/api/compute/virtualmachines/createorupdate#examples)
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineResult {
        /// An `additional_capabilities` block as defined below.
        pub additional_capabilities: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::VirtualMachineAdditionalCapabilities>,
        >,
        /// The ID of the Availability Set in which the Virtual Machine should exist. Changing this forces a new resource to be created.
        pub availability_set_id: pulumi_wasm_rust::Output<String>,
        /// A `boot_diagnostics` block as defined below.
        pub boot_diagnostics: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::VirtualMachineBootDiagnostics>,
        >,
        /// Should the Data Disks (either the Managed Disks / VHD Blobs) be deleted when the Virtual Machine is destroyed? Defaults to `false`.
        ///
        /// > **Note:** This setting works when instance is deleted via the provider only and don't forget to delete disks manually if you deleted VM manually. It can increase spending.
        pub delete_data_disks_on_termination: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the OS Disk (either the Managed Disk / VHD Blob) be deleted when the Virtual Machine is destroyed? Defaults to `false`.
        ///
        /// > **Note:** This setting works when instance is deleted via the provider only and don't forget to delete disks manually if you deleted VM manually. It can increase spending.
        pub delete_os_disk_on_termination: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::VirtualMachineIdentity>,
        >,
        /// Specifies the BYOL Type for this Virtual Machine. This is only applicable to Windows Virtual Machines. Possible values are `Windows_Client` and `Windows_Server`.
        pub license_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the Azure Region where the Virtual Machine exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Virtual Machine. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of Network Interface IDs which should be associated with the Virtual Machine.
        pub network_interface_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// An `os_profile` block as defined below. Required when `create_option` in the `storage_os_disk` block is set to `FromImage`.
        pub os_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::VirtualMachineOsProfile>,
        >,
        /// (Required, when a Linux machine) An `os_profile_linux_config` block as defined below.
        pub os_profile_linux_config: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::VirtualMachineOsProfileLinuxConfig>,
        >,
        /// One or more `os_profile_secrets` blocks as defined below.
        pub os_profile_secrets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::VirtualMachineOsProfileSecret>>,
        >,
        /// (Required, when a Windows machine) An `os_profile_windows_config` block as defined below.
        pub os_profile_windows_config: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::VirtualMachineOsProfileWindowsConfig>,
        >,
        /// A `plan` block as defined below.
        pub plan: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::VirtualMachinePlan>,
        >,
        /// The ID of the Network Interface (which must be attached to the Virtual Machine) which should be the Primary Network Interface for this Virtual Machine.
        pub primary_network_interface_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Proximity Placement Group to which this Virtual Machine should be assigned. Changing this forces a new resource to be created
        pub proximity_placement_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Resource Group in which the Virtual Machine should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `storage_data_disk` blocks as defined below.
        ///
        /// > **Please Note:** Data Disks can also be attached either using this block or the `azure.compute.DataDiskAttachment` resource - but not both.
        pub storage_data_disks: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::VirtualMachineStorageDataDisk>,
        >,
        /// A `storage_image_reference` block as defined below. Changing this forces a new resource to be created.
        pub storage_image_reference: pulumi_wasm_rust::Output<
            super::super::types::compute::VirtualMachineStorageImageReference,
        >,
        /// A `storage_os_disk` block as defined below.
        pub storage_os_disk: pulumi_wasm_rust::Output<
            super::super::types::compute::VirtualMachineStorageOsDisk,
        >,
        /// A mapping of tags to assign to the Virtual Machine.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the [size of the Virtual Machine](https://docs.microsoft.com/azure/virtual-machines/sizes-general). See also [Azure VM Naming Conventions](https://docs.microsoft.com/azure/virtual-machines/vm-naming-conventions).
        pub vm_size: pulumi_wasm_rust::Output<String>,
        /// A list of a single item of the Availability Zone which the Virtual Machine should be allocated in. Changing this forces a new resource to be created.
        ///
        /// > **Please Note**: Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        ///
        /// For more information on the different example configurations, please check out the [Azure documentation](https://docs.microsoft.com/en-gb/rest/api/compute/virtualmachines/createorupdate#examples)
        pub zones: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VirtualMachineArgs) -> VirtualMachineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_capabilities_binding = args.additional_capabilities.get_inner();
        let availability_set_id_binding = args.availability_set_id.get_inner();
        let boot_diagnostics_binding = args.boot_diagnostics.get_inner();
        let delete_data_disks_on_termination_binding = args
            .delete_data_disks_on_termination
            .get_inner();
        let delete_os_disk_on_termination_binding = args
            .delete_os_disk_on_termination
            .get_inner();
        let identity_binding = args.identity.get_inner();
        let license_type_binding = args.license_type.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let network_interface_ids_binding = args.network_interface_ids.get_inner();
        let os_profile_binding = args.os_profile.get_inner();
        let os_profile_linux_config_binding = args.os_profile_linux_config.get_inner();
        let os_profile_secrets_binding = args.os_profile_secrets.get_inner();
        let os_profile_windows_config_binding = args
            .os_profile_windows_config
            .get_inner();
        let plan_binding = args.plan.get_inner();
        let primary_network_interface_id_binding = args
            .primary_network_interface_id
            .get_inner();
        let proximity_placement_group_id_binding = args
            .proximity_placement_group_id
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let storage_data_disks_binding = args.storage_data_disks.get_inner();
        let storage_image_reference_binding = args.storage_image_reference.get_inner();
        let storage_os_disk_binding = args.storage_os_disk.get_inner();
        let tags_binding = args.tags.get_inner();
        let vm_size_binding = args.vm_size.get_inner();
        let zones_binding = args.zones.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/virtualMachine:VirtualMachine".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalCapabilities".into(),
                    value: &additional_capabilities_binding,
                },
                register_interface::ObjectField {
                    name: "availabilitySetId".into(),
                    value: &availability_set_id_binding,
                },
                register_interface::ObjectField {
                    name: "bootDiagnostics".into(),
                    value: &boot_diagnostics_binding,
                },
                register_interface::ObjectField {
                    name: "deleteDataDisksOnTermination".into(),
                    value: &delete_data_disks_on_termination_binding,
                },
                register_interface::ObjectField {
                    name: "deleteOsDiskOnTermination".into(),
                    value: &delete_os_disk_on_termination_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "licenseType".into(),
                    value: &license_type_binding,
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
                    name: "networkInterfaceIds".into(),
                    value: &network_interface_ids_binding,
                },
                register_interface::ObjectField {
                    name: "osProfile".into(),
                    value: &os_profile_binding,
                },
                register_interface::ObjectField {
                    name: "osProfileLinuxConfig".into(),
                    value: &os_profile_linux_config_binding,
                },
                register_interface::ObjectField {
                    name: "osProfileSecrets".into(),
                    value: &os_profile_secrets_binding,
                },
                register_interface::ObjectField {
                    name: "osProfileWindowsConfig".into(),
                    value: &os_profile_windows_config_binding,
                },
                register_interface::ObjectField {
                    name: "plan".into(),
                    value: &plan_binding,
                },
                register_interface::ObjectField {
                    name: "primaryNetworkInterfaceId".into(),
                    value: &primary_network_interface_id_binding,
                },
                register_interface::ObjectField {
                    name: "proximityPlacementGroupId".into(),
                    value: &proximity_placement_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageDataDisks".into(),
                    value: &storage_data_disks_binding,
                },
                register_interface::ObjectField {
                    name: "storageImageReference".into(),
                    value: &storage_image_reference_binding,
                },
                register_interface::ObjectField {
                    name: "storageOsDisk".into(),
                    value: &storage_os_disk_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vmSize".into(),
                    value: &vm_size_binding,
                },
                register_interface::ObjectField {
                    name: "zones".into(),
                    value: &zones_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalCapabilities".into(),
                },
                register_interface::ResultField {
                    name: "availabilitySetId".into(),
                },
                register_interface::ResultField {
                    name: "bootDiagnostics".into(),
                },
                register_interface::ResultField {
                    name: "deleteDataDisksOnTermination".into(),
                },
                register_interface::ResultField {
                    name: "deleteOsDiskOnTermination".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "licenseType".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceIds".into(),
                },
                register_interface::ResultField {
                    name: "osProfile".into(),
                },
                register_interface::ResultField {
                    name: "osProfileLinuxConfig".into(),
                },
                register_interface::ResultField {
                    name: "osProfileSecrets".into(),
                },
                register_interface::ResultField {
                    name: "osProfileWindowsConfig".into(),
                },
                register_interface::ResultField {
                    name: "plan".into(),
                },
                register_interface::ResultField {
                    name: "primaryNetworkInterfaceId".into(),
                },
                register_interface::ResultField {
                    name: "proximityPlacementGroupId".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "storageDataDisks".into(),
                },
                register_interface::ResultField {
                    name: "storageImageReference".into(),
                },
                register_interface::ResultField {
                    name: "storageOsDisk".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vmSize".into(),
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
        VirtualMachineResult {
            additional_capabilities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalCapabilities").unwrap(),
            ),
            availability_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilitySetId").unwrap(),
            ),
            boot_diagnostics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootDiagnostics").unwrap(),
            ),
            delete_data_disks_on_termination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteDataDisksOnTermination").unwrap(),
            ),
            delete_os_disk_on_termination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteOsDiskOnTermination").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            license_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseType").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_interface_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceIds").unwrap(),
            ),
            os_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osProfile").unwrap(),
            ),
            os_profile_linux_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osProfileLinuxConfig").unwrap(),
            ),
            os_profile_secrets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osProfileSecrets").unwrap(),
            ),
            os_profile_windows_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osProfileWindowsConfig").unwrap(),
            ),
            plan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("plan").unwrap(),
            ),
            primary_network_interface_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryNetworkInterfaceId").unwrap(),
            ),
            proximity_placement_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proximityPlacementGroupId").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            storage_data_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageDataDisks").unwrap(),
            ),
            storage_image_reference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageImageReference").unwrap(),
            ),
            storage_os_disk: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageOsDisk").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vm_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmSize").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}
