/// Manages a Windows Virtual Machine.
///
/// ## Disclaimers
///
/// > **Note** This provider will automatically remove the OS Disk by default - this behaviour can be configured using the `features` setting within the Provider block.
///
/// > **Note** All arguments including the administrator login and password will be stored in the raw state as plain-text.
///
/// > **Note** This resource does not support Unmanaged Disks. If you need to use Unmanaged Disks you can continue to use the `azure.compute.VirtualMachine` resource instead.
///
/// > **Note** This resource does not support attaching existing OS Disks. You can instead capture an image of the OS Disk or continue to use the `azure.compute.VirtualMachine` resource instead.
///
/// > In this release there's a known issue where the `public_ip_address` and `public_ip_addresses` fields may not be fully populated for Dynamic Public IP's.
///
/// ## Example Usage
///
/// This example provisions a basic Windows Virtual Machine on an internal network.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleNetworkInterface = network_interface::create(
///         "exampleNetworkInterface",
///         NetworkInterfaceArgs::builder()
///             .ip_configurations(
///                 vec![
///                     NetworkInterfaceIpConfiguration::builder().name("internal")
///                     .privateIpAddressAllocation("Dynamic")
///                     .subnetId("${exampleSubnet.id}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example-nic")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .name("internal")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-network")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleWindowsVirtualMachine = windows_virtual_machine::create(
///         "exampleWindowsVirtualMachine",
///         WindowsVirtualMachineArgs::builder()
///             .admin_password("P@$$w0rd1234!")
///             .admin_username("adminuser")
///             .location("${example.location}")
///             .name("example-machine")
///             .network_interface_ids(vec!["${exampleNetworkInterface.id}",])
///             .os_disk(
///                 WindowsVirtualMachineOsDisk::builder()
///                     .caching("ReadWrite")
///                     .storageAccountType("Standard_LRS")
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .size("Standard_F2")
///             .source_image_reference(
///                 WindowsVirtualMachineSourceImageReference::builder()
///                     .offer("WindowsServer")
///                     .publisher("MicrosoftWindowsServer")
///                     .sku("2016-Datacenter")
///                     .version("latest")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Windows Virtual Machines can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/windowsVirtualMachine:WindowsVirtualMachine example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/virtualMachines/machine1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod windows_virtual_machine {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WindowsVirtualMachineArgs {
        /// A `additional_capabilities` block as defined below.
        #[builder(into, default)]
        pub additional_capabilities: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::WindowsVirtualMachineAdditionalCapabilities,
            >,
        >,
        /// One or more `additional_unattend_content` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub additional_unattend_contents: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::compute::WindowsVirtualMachineAdditionalUnattendContent,
                >,
            >,
        >,
        /// The Password which should be used for the local-administrator on this Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into)]
        pub admin_password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The username of the local administrator used for the Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into)]
        pub admin_username: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Should Extension Operations be allowed on this Virtual Machine? Defaults to `true`.
        #[builder(into, default)]
        pub allow_extension_operations: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the ID of the Availability Set in which the Virtual Machine should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub availability_set_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `boot_diagnostics` block as defined below.
        #[builder(into, default)]
        pub boot_diagnostics: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::WindowsVirtualMachineBootDiagnostics>,
        >,
        /// Specifies whether to skip platform scheduled patching when a user schedule is associated with the VM. Defaults to `false`.
        ///
        /// > **NOTE:** `bypass_platform_safety_checks_on_user_schedule_enabled` can only be set to `true` when `patch_mode` is set to `AutomaticByPlatform`.
        #[builder(into, default)]
        pub bypass_platform_safety_checks_on_user_schedule_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the ID of the Capacity Reservation Group which the Virtual Machine should be allocated to.
        ///
        /// > **NOTE:** `capacity_reservation_group_id` cannot be used with `availability_set_id` or `proximity_placement_group_id`
        #[builder(into, default)]
        pub capacity_reservation_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the Hostname which should be used for this Virtual Machine. If unspecified this defaults to the value for the `name` field. If the value of the `name` field is not a valid `computer_name`, then you must specify `computer_name`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub computer_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Base64-Encoded Custom Data which should be used for this Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub custom_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of a Dedicated Host Group that this Windows Virtual Machine should be run within. Conflicts with `dedicated_host_id`.
        #[builder(into, default)]
        pub dedicated_host_group_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of a Dedicated Host where this machine should be run on. Conflicts with `dedicated_host_group_id`.
        #[builder(into, default)]
        pub dedicated_host_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Disk Controller Type used for this Virtual Machine. Possible values are `SCSI` and `NVMe`.
        #[builder(into, default)]
        pub disk_controller_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Edge Zone within the Azure Region where this Windows Virtual Machine should exist. Changing this forces a new Windows Virtual Machine to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if Automatic Updates are Enabled for the Windows Virtual Machine. Changing this forces a new resource to be created. Defaults to `true`.
        #[builder(into, default)]
        pub enable_automatic_updates: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should all of the disks (including the temp disk) attached to this Virtual Machine be encrypted by enabling Encryption at Host?
        #[builder(into, default)]
        pub encryption_at_host_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies what should happen when the Virtual Machine is evicted for price reasons when using a Spot instance. Possible values are `Deallocate` and `Delete`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This can only be configured when `priority` is set to `Spot`.
        #[builder(into, default)]
        pub eviction_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the duration allocated for all extensions to start. The time duration should be between 15 minutes and 120 minutes (inclusive) and should be specified in ISO 8601 format. Defaults to `PT1H30M`.
        #[builder(into, default)]
        pub extensions_time_budget: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `gallery_application` blocks as defined below.
        ///
        /// > **Note** Gallery Application Assignments can be defined either directly on `azure.compute.WindowsVirtualMachine` resource, or using the `azure.compute.GalleryApplicationAssignment` resource - but the two approaches cannot be used together. If both are used with the same Virtual Machine, spurious changes will occur. If `azure.compute.GalleryApplicationAssignment` is used, it's recommended to use `ignore_changes` for the `gallery_application` block on the corresponding `azure.compute.WindowsVirtualMachine` resource, to avoid a persistent diff when using this resource.
        #[builder(into, default)]
        pub gallery_applications: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::compute::WindowsVirtualMachineGalleryApplication,
                >,
            >,
        >,
        /// Should the VM be patched without requiring a reboot? Possible values are `true` or `false`. Defaults to `false`. For more information about hot patching please see the [product documentation](https://docs.microsoft.com/azure/automanage/automanage-hotpatch).
        ///
        /// > **NOTE:** Hotpatching can only be enabled if the `patch_mode` is set to `AutomaticByPlatform`, the `provision_vm_agent` is set to `true`, your `source_image_reference` references a hotpatching enabled image, and the VM's `size` is set to a [Azure generation 2](https://docs.microsoft.com/azure/virtual-machines/generation-2#generation-2-vm-sizes) VM. An example of how to correctly configure a Windows Virtual Machine to use the `hotpatching_enabled` field can be found in the `./examples/virtual-machines/windows/hotpatching-enabled` directory within the GitHub Repository.
        #[builder(into, default)]
        pub hotpatching_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::WindowsVirtualMachineIdentity>,
        >,
        /// Specifies the type of on-premise license (also known as [Azure Hybrid Use Benefit](https://docs.microsoft.com/windows-server/get-started/azure-hybrid-benefit)) which should be used for this Virtual Machine. Possible values are `None`, `Windows_Client` and `Windows_Server`.
        #[builder(into, default)]
        pub license_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure location where the Windows Virtual Machine should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum price you're willing to pay for this Virtual Machine, in US Dollars; which must be greater than the current spot price. If this bid price falls below the current spot price the Virtual Machine will be evicted using the `eviction_policy`. Defaults to `-1`, which means that the Virtual Machine should not be evicted for price reasons.
        ///
        /// > **NOTE:** This can only be configured when `priority` is set to `Spot`.
        #[builder(into, default)]
        pub max_bid_price: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// The name of the Windows Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// . A list of Network Interface IDs which should be attached to this Virtual Machine. The first Network Interface ID in this list will be the Primary Network Interface on the Virtual Machine.
        #[builder(into)]
        pub network_interface_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A `os_disk` block as defined below.
        #[builder(into)]
        pub os_disk: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::WindowsVirtualMachineOsDisk,
        >,
        /// A `os_image_notification` block as defined below.
        #[builder(into, default)]
        pub os_image_notification: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::WindowsVirtualMachineOsImageNotification,
            >,
        >,
        /// Specifies the mode of VM Guest Patching for the Virtual Machine. Possible values are `AutomaticByPlatform` or `ImageDefault`. Defaults to `ImageDefault`.
        ///
        /// > **NOTE:** If the `patch_assessment_mode` is set to `AutomaticByPlatform` then the `provision_vm_agent` field must be set to `true`.
        #[builder(into, default)]
        pub patch_assessment_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the mode of in-guest patching to this Windows Virtual Machine. Possible values are `Manual`, `AutomaticByOS` and `AutomaticByPlatform`. Defaults to `AutomaticByOS`. For more information on patch modes please see the [product documentation](https://docs.microsoft.com/azure/virtual-machines/automatic-vm-guest-patching#patch-orchestration-modes).
        ///
        /// > **NOTE:** If `patch_mode` is set to `AutomaticByPlatform` then `provision_vm_agent` must also be set to `true`. If the Virtual Machine is using a hotpatching enabled image the `patch_mode` must always be set to `AutomaticByPlatform`.
        #[builder(into, default)]
        pub patch_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `plan` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::WindowsVirtualMachinePlan>,
        >,
        /// Specifies the Platform Fault Domain in which this Windows Virtual Machine should be created. Defaults to `-1`, which means this will be automatically assigned to a fault domain that best maintains balance across the available fault domains. Changing this forces a new Windows Virtual Machine to be created.
        #[builder(into, default)]
        pub platform_fault_domain: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the priority of this Virtual Machine. Possible values are `Regular` and `Spot`. Defaults to `Regular`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the Azure VM Agent be provisioned on this Virtual Machine? Defaults to `true`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If `provision_vm_agent` is set to `false` then `allow_extension_operations` must also be set to `false`.
        #[builder(into, default)]
        pub provision_vm_agent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Proximity Placement Group which the Virtual Machine should be assigned to.
        #[builder(into, default)]
        pub proximity_placement_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the reboot setting for platform scheduled patching. Possible values are `Always`, `IfRequired` and `Never`.
        ///
        /// > **NOTE:** `reboot_setting` can only be set when `patch_mode` is set to `AutomaticByPlatform`.
        #[builder(into, default)]
        pub reboot_setting: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group in which the Windows Virtual Machine should be exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `secret` blocks as defined below.
        #[builder(into, default)]
        pub secrets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::WindowsVirtualMachineSecret>>,
        >,
        /// Specifies if Secure Boot and Trusted Launch is enabled for the Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub secure_boot_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The SKU which should be used for this Virtual Machine, such as `Standard_F2`.
        #[builder(into)]
        pub size: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Image which this Virtual Machine should be created from. Changing this forces a new resource to be created. Possible Image ID types include `Image ID`s, `Shared Image ID`s, `Shared Image Version ID`s, `Community Gallery Image ID`s, `Community Gallery Image Version ID`s, `Shared Gallery Image ID`s and `Shared Gallery Image Version ID`s.
        ///
        /// > **NOTE:** One of either `source_image_id` or `source_image_reference` must be set.
        #[builder(into, default)]
        pub source_image_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `source_image_reference` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One of either `source_image_id` or `source_image_reference` must be set.
        #[builder(into, default)]
        pub source_image_reference: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::WindowsVirtualMachineSourceImageReference,
            >,
        >,
        /// A mapping of tags which should be assigned to this Virtual Machine.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `termination_notification` block as defined below.
        #[builder(into, default)]
        pub termination_notification: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::WindowsVirtualMachineTerminationNotification,
            >,
        >,
        /// Specifies the Time Zone which should be used by the Virtual Machine, [the possible values are defined here](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub timezone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Base64-Encoded User Data which should be used for this Virtual Machine.
        #[builder(into, default)]
        pub user_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Orchestrated Virtual Machine Scale Set that this Virtual Machine should be created within.
        ///
        /// > **NOTE:** To update `virtual_machine_scale_set_id` the Preview Feature `Microsoft.Compute/SingleFDAttachDetachVMToVmss` needs to be enabled, see [the documentation](https://review.learn.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-attach-detach-vm#enroll-in-the-preview) for more information.
        ///
        /// > **NOTE:** Orchestrated Virtual Machine Scale Sets can be provisioned using [the `azure.compute.OrchestratedVirtualMachineScaleSet` resource](https://www.terraform.io/docs/providers/azurerm/r/orchestrated_virtual_machine_scale_set.html).
        ///
        /// > **NOTE:** To attach an existing VM to a Virtual Machine Scale Set, the scale set must have `single_placement_group` set to `false`, see [the documentation](https://learn.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-attach-detach-vm?tabs=portal-1%2Cportal-2%2Cportal-3#limitations-for-attaching-an-existing-vm-to-a-scale-set) for more information.
        #[builder(into, default)]
        pub virtual_machine_scale_set_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies whether VMAgent Platform Updates is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub vm_agent_platform_updates_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies if vTPM (virtual Trusted Platform Module) and Trusted Launch is enabled for the Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub vtpm_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// One or more `winrm_listener` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub winrm_listeners: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::WindowsVirtualMachineWinrmListener>>,
        >,
        /// * `zones` - (Optional) Specifies the Availability Zone in which this Windows Virtual Machine should be located. Changing this forces a new Windows Virtual Machine to be created.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct WindowsVirtualMachineResult {
        /// A `additional_capabilities` block as defined below.
        pub additional_capabilities: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineAdditionalCapabilities,
            >,
        >,
        /// One or more `additional_unattend_content` blocks as defined below. Changing this forces a new resource to be created.
        pub additional_unattend_contents: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::WindowsVirtualMachineAdditionalUnattendContent,
                >,
            >,
        >,
        /// The Password which should be used for the local-administrator on this Virtual Machine. Changing this forces a new resource to be created.
        pub admin_password: pulumi_gestalt_rust::Output<String>,
        /// The username of the local administrator used for the Virtual Machine. Changing this forces a new resource to be created.
        pub admin_username: pulumi_gestalt_rust::Output<String>,
        /// Should Extension Operations be allowed on this Virtual Machine? Defaults to `true`.
        pub allow_extension_operations: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the ID of the Availability Set in which the Virtual Machine should exist. Changing this forces a new resource to be created.
        pub availability_set_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `boot_diagnostics` block as defined below.
        pub boot_diagnostics: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::WindowsVirtualMachineBootDiagnostics>,
        >,
        /// Specifies whether to skip platform scheduled patching when a user schedule is associated with the VM. Defaults to `false`.
        ///
        /// > **NOTE:** `bypass_platform_safety_checks_on_user_schedule_enabled` can only be set to `true` when `patch_mode` is set to `AutomaticByPlatform`.
        pub bypass_platform_safety_checks_on_user_schedule_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Specifies the ID of the Capacity Reservation Group which the Virtual Machine should be allocated to.
        ///
        /// > **NOTE:** `capacity_reservation_group_id` cannot be used with `availability_set_id` or `proximity_placement_group_id`
        pub capacity_reservation_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Hostname which should be used for this Virtual Machine. If unspecified this defaults to the value for the `name` field. If the value of the `name` field is not a valid `computer_name`, then you must specify `computer_name`. Changing this forces a new resource to be created.
        pub computer_name: pulumi_gestalt_rust::Output<String>,
        /// The Base64-Encoded Custom Data which should be used for this Virtual Machine. Changing this forces a new resource to be created.
        pub custom_data: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of a Dedicated Host Group that this Windows Virtual Machine should be run within. Conflicts with `dedicated_host_id`.
        pub dedicated_host_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of a Dedicated Host where this machine should be run on. Conflicts with `dedicated_host_group_id`.
        pub dedicated_host_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Disk Controller Type used for this Virtual Machine. Possible values are `SCSI` and `NVMe`.
        pub disk_controller_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Edge Zone within the Azure Region where this Windows Virtual Machine should exist. Changing this forces a new Windows Virtual Machine to be created.
        pub edge_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies if Automatic Updates are Enabled for the Windows Virtual Machine. Changing this forces a new resource to be created. Defaults to `true`.
        pub enable_automatic_updates: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should all of the disks (including the temp disk) attached to this Virtual Machine be encrypted by enabling Encryption at Host?
        pub encryption_at_host_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies what should happen when the Virtual Machine is evicted for price reasons when using a Spot instance. Possible values are `Deallocate` and `Delete`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This can only be configured when `priority` is set to `Spot`.
        pub eviction_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the duration allocated for all extensions to start. The time duration should be between 15 minutes and 120 minutes (inclusive) and should be specified in ISO 8601 format. Defaults to `PT1H30M`.
        pub extensions_time_budget: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `gallery_application` blocks as defined below.
        ///
        /// > **Note** Gallery Application Assignments can be defined either directly on `azure.compute.WindowsVirtualMachine` resource, or using the `azure.compute.GalleryApplicationAssignment` resource - but the two approaches cannot be used together. If both are used with the same Virtual Machine, spurious changes will occur. If `azure.compute.GalleryApplicationAssignment` is used, it's recommended to use `ignore_changes` for the `gallery_application` block on the corresponding `azure.compute.WindowsVirtualMachine` resource, to avoid a persistent diff when using this resource.
        pub gallery_applications: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::WindowsVirtualMachineGalleryApplication,
                >,
            >,
        >,
        /// Should the VM be patched without requiring a reboot? Possible values are `true` or `false`. Defaults to `false`. For more information about hot patching please see the [product documentation](https://docs.microsoft.com/azure/automanage/automanage-hotpatch).
        ///
        /// > **NOTE:** Hotpatching can only be enabled if the `patch_mode` is set to `AutomaticByPlatform`, the `provision_vm_agent` is set to `true`, your `source_image_reference` references a hotpatching enabled image, and the VM's `size` is set to a [Azure generation 2](https://docs.microsoft.com/azure/virtual-machines/generation-2#generation-2-vm-sizes) VM. An example of how to correctly configure a Windows Virtual Machine to use the `hotpatching_enabled` field can be found in the `./examples/virtual-machines/windows/hotpatching-enabled` directory within the GitHub Repository.
        pub hotpatching_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::WindowsVirtualMachineIdentity>,
        >,
        /// Specifies the type of on-premise license (also known as [Azure Hybrid Use Benefit](https://docs.microsoft.com/windows-server/get-started/azure-hybrid-benefit)) which should be used for this Virtual Machine. Possible values are `None`, `Windows_Client` and `Windows_Server`.
        pub license_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Azure location where the Windows Virtual Machine should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The maximum price you're willing to pay for this Virtual Machine, in US Dollars; which must be greater than the current spot price. If this bid price falls below the current spot price the Virtual Machine will be evicted using the `eviction_policy`. Defaults to `-1`, which means that the Virtual Machine should not be evicted for price reasons.
        ///
        /// > **NOTE:** This can only be configured when `priority` is set to `Spot`.
        pub max_bid_price: pulumi_gestalt_rust::Output<Option<f64>>,
        /// The name of the Windows Virtual Machine. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// . A list of Network Interface IDs which should be attached to this Virtual Machine. The first Network Interface ID in this list will be the Primary Network Interface on the Virtual Machine.
        pub network_interface_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A `os_disk` block as defined below.
        pub os_disk: pulumi_gestalt_rust::Output<
            super::super::types::compute::WindowsVirtualMachineOsDisk,
        >,
        /// A `os_image_notification` block as defined below.
        pub os_image_notification: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineOsImageNotification,
            >,
        >,
        /// Specifies the mode of VM Guest Patching for the Virtual Machine. Possible values are `AutomaticByPlatform` or `ImageDefault`. Defaults to `ImageDefault`.
        ///
        /// > **NOTE:** If the `patch_assessment_mode` is set to `AutomaticByPlatform` then the `provision_vm_agent` field must be set to `true`.
        pub patch_assessment_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the mode of in-guest patching to this Windows Virtual Machine. Possible values are `Manual`, `AutomaticByOS` and `AutomaticByPlatform`. Defaults to `AutomaticByOS`. For more information on patch modes please see the [product documentation](https://docs.microsoft.com/azure/virtual-machines/automatic-vm-guest-patching#patch-orchestration-modes).
        ///
        /// > **NOTE:** If `patch_mode` is set to `AutomaticByPlatform` then `provision_vm_agent` must also be set to `true`. If the Virtual Machine is using a hotpatching enabled image the `patch_mode` must always be set to `AutomaticByPlatform`.
        pub patch_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `plan` block as defined below. Changing this forces a new resource to be created.
        pub plan: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::WindowsVirtualMachinePlan>,
        >,
        /// Specifies the Platform Fault Domain in which this Windows Virtual Machine should be created. Defaults to `-1`, which means this will be automatically assigned to a fault domain that best maintains balance across the available fault domains. Changing this forces a new Windows Virtual Machine to be created.
        pub platform_fault_domain: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the priority of this Virtual Machine. Possible values are `Regular` and `Spot`. Defaults to `Regular`. Changing this forces a new resource to be created.
        pub priority: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Primary Private IP Address assigned to this Virtual Machine.
        pub private_ip_address: pulumi_gestalt_rust::Output<String>,
        /// A list of Private IP Addresses assigned to this Virtual Machine.
        pub private_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Should the Azure VM Agent be provisioned on this Virtual Machine? Defaults to `true`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If `provision_vm_agent` is set to `false` then `allow_extension_operations` must also be set to `false`.
        pub provision_vm_agent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Proximity Placement Group which the Virtual Machine should be assigned to.
        pub proximity_placement_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Primary Public IP Address assigned to this Virtual Machine.
        pub public_ip_address: pulumi_gestalt_rust::Output<String>,
        /// A list of the Public IP Addresses assigned to this Virtual Machine.
        pub public_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the reboot setting for platform scheduled patching. Possible values are `Always`, `IfRequired` and `Never`.
        ///
        /// > **NOTE:** `reboot_setting` can only be set when `patch_mode` is set to `AutomaticByPlatform`.
        pub reboot_setting: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the Windows Virtual Machine should be exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `secret` blocks as defined below.
        pub secrets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::WindowsVirtualMachineSecret>>,
        >,
        /// Specifies if Secure Boot and Trusted Launch is enabled for the Virtual Machine. Changing this forces a new resource to be created.
        pub secure_boot_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The SKU which should be used for this Virtual Machine, such as `Standard_F2`.
        pub size: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Image which this Virtual Machine should be created from. Changing this forces a new resource to be created. Possible Image ID types include `Image ID`s, `Shared Image ID`s, `Shared Image Version ID`s, `Community Gallery Image ID`s, `Community Gallery Image Version ID`s, `Shared Gallery Image ID`s and `Shared Gallery Image Version ID`s.
        ///
        /// > **NOTE:** One of either `source_image_id` or `source_image_reference` must be set.
        pub source_image_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `source_image_reference` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One of either `source_image_id` or `source_image_reference` must be set.
        pub source_image_reference: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineSourceImageReference,
            >,
        >,
        /// A mapping of tags which should be assigned to this Virtual Machine.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `termination_notification` block as defined below.
        pub termination_notification: pulumi_gestalt_rust::Output<
            super::super::types::compute::WindowsVirtualMachineTerminationNotification,
        >,
        /// Specifies the Time Zone which should be used by the Virtual Machine, [the possible values are defined here](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/). Changing this forces a new resource to be created.
        pub timezone: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Base64-Encoded User Data which should be used for this Virtual Machine.
        pub user_data: pulumi_gestalt_rust::Output<Option<String>>,
        /// A 128-bit identifier which uniquely identifies this Virtual Machine.
        pub virtual_machine_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Orchestrated Virtual Machine Scale Set that this Virtual Machine should be created within.
        ///
        /// > **NOTE:** To update `virtual_machine_scale_set_id` the Preview Feature `Microsoft.Compute/SingleFDAttachDetachVMToVmss` needs to be enabled, see [the documentation](https://review.learn.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-attach-detach-vm#enroll-in-the-preview) for more information.
        ///
        /// > **NOTE:** Orchestrated Virtual Machine Scale Sets can be provisioned using [the `azure.compute.OrchestratedVirtualMachineScaleSet` resource](https://www.terraform.io/docs/providers/azurerm/r/orchestrated_virtual_machine_scale_set.html).
        ///
        /// > **NOTE:** To attach an existing VM to a Virtual Machine Scale Set, the scale set must have `single_placement_group` set to `false`, see [the documentation](https://learn.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-attach-detach-vm?tabs=portal-1%2Cportal-2%2Cportal-3#limitations-for-attaching-an-existing-vm-to-a-scale-set) for more information.
        pub virtual_machine_scale_set_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether VMAgent Platform Updates is enabled. Defaults to `false`.
        pub vm_agent_platform_updates_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies if vTPM (virtual Trusted Platform Module) and Trusted Launch is enabled for the Virtual Machine. Changing this forces a new resource to be created.
        pub vtpm_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// One or more `winrm_listener` blocks as defined below. Changing this forces a new resource to be created.
        pub winrm_listeners: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::WindowsVirtualMachineWinrmListener>>,
        >,
        /// * `zones` - (Optional) Specifies the Availability Zone in which this Windows Virtual Machine should be located. Changing this forces a new Windows Virtual Machine to be created.
        pub zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WindowsVirtualMachineArgs,
    ) -> WindowsVirtualMachineResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_capabilities_binding = args
            .additional_capabilities
            .get_output(context);
        let additional_unattend_contents_binding = args
            .additional_unattend_contents
            .get_output(context);
        let admin_password_binding = args.admin_password.get_output(context);
        let admin_username_binding = args.admin_username.get_output(context);
        let allow_extension_operations_binding = args
            .allow_extension_operations
            .get_output(context);
        let availability_set_id_binding = args.availability_set_id.get_output(context);
        let boot_diagnostics_binding = args.boot_diagnostics.get_output(context);
        let bypass_platform_safety_checks_on_user_schedule_enabled_binding = args
            .bypass_platform_safety_checks_on_user_schedule_enabled
            .get_output(context);
        let capacity_reservation_group_id_binding = args
            .capacity_reservation_group_id
            .get_output(context);
        let computer_name_binding = args.computer_name.get_output(context);
        let custom_data_binding = args.custom_data.get_output(context);
        let dedicated_host_group_id_binding = args
            .dedicated_host_group_id
            .get_output(context);
        let dedicated_host_id_binding = args.dedicated_host_id.get_output(context);
        let disk_controller_type_binding = args.disk_controller_type.get_output(context);
        let edge_zone_binding = args.edge_zone.get_output(context);
        let enable_automatic_updates_binding = args
            .enable_automatic_updates
            .get_output(context);
        let encryption_at_host_enabled_binding = args
            .encryption_at_host_enabled
            .get_output(context);
        let eviction_policy_binding = args.eviction_policy.get_output(context);
        let extensions_time_budget_binding = args
            .extensions_time_budget
            .get_output(context);
        let gallery_applications_binding = args.gallery_applications.get_output(context);
        let hotpatching_enabled_binding = args.hotpatching_enabled.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let license_type_binding = args.license_type.get_output(context);
        let location_binding = args.location.get_output(context);
        let max_bid_price_binding = args.max_bid_price.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_interface_ids_binding = args
            .network_interface_ids
            .get_output(context);
        let os_disk_binding = args.os_disk.get_output(context);
        let os_image_notification_binding = args
            .os_image_notification
            .get_output(context);
        let patch_assessment_mode_binding = args
            .patch_assessment_mode
            .get_output(context);
        let patch_mode_binding = args.patch_mode.get_output(context);
        let plan_binding = args.plan.get_output(context);
        let platform_fault_domain_binding = args
            .platform_fault_domain
            .get_output(context);
        let priority_binding = args.priority.get_output(context);
        let provision_vm_agent_binding = args.provision_vm_agent.get_output(context);
        let proximity_placement_group_id_binding = args
            .proximity_placement_group_id
            .get_output(context);
        let reboot_setting_binding = args.reboot_setting.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let secrets_binding = args.secrets.get_output(context);
        let secure_boot_enabled_binding = args.secure_boot_enabled.get_output(context);
        let size_binding = args.size.get_output(context);
        let source_image_id_binding = args.source_image_id.get_output(context);
        let source_image_reference_binding = args
            .source_image_reference
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let termination_notification_binding = args
            .termination_notification
            .get_output(context);
        let timezone_binding = args.timezone.get_output(context);
        let user_data_binding = args.user_data.get_output(context);
        let virtual_machine_scale_set_id_binding = args
            .virtual_machine_scale_set_id
            .get_output(context);
        let vm_agent_platform_updates_enabled_binding = args
            .vm_agent_platform_updates_enabled
            .get_output(context);
        let vtpm_enabled_binding = args.vtpm_enabled.get_output(context);
        let winrm_listeners_binding = args.winrm_listeners.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/windowsVirtualMachine:WindowsVirtualMachine".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalCapabilities".into(),
                    value: additional_capabilities_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalUnattendContents".into(),
                    value: additional_unattend_contents_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminPassword".into(),
                    value: admin_password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminUsername".into(),
                    value: admin_username_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowExtensionOperations".into(),
                    value: allow_extension_operations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilitySetId".into(),
                    value: availability_set_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bootDiagnostics".into(),
                    value: boot_diagnostics_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bypassPlatformSafetyChecksOnUserScheduleEnabled".into(),
                    value: bypass_platform_safety_checks_on_user_schedule_enabled_binding
                        .get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacityReservationGroupId".into(),
                    value: capacity_reservation_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computerName".into(),
                    value: computer_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customData".into(),
                    value: custom_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dedicatedHostGroupId".into(),
                    value: dedicated_host_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dedicatedHostId".into(),
                    value: dedicated_host_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskControllerType".into(),
                    value: disk_controller_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "edgeZone".into(),
                    value: edge_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableAutomaticUpdates".into(),
                    value: enable_automatic_updates_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionAtHostEnabled".into(),
                    value: encryption_at_host_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "evictionPolicy".into(),
                    value: eviction_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extensionsTimeBudget".into(),
                    value: extensions_time_budget_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "galleryApplications".into(),
                    value: gallery_applications_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hotpatchingEnabled".into(),
                    value: hotpatching_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseType".into(),
                    value: license_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxBidPrice".into(),
                    value: max_bid_price_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaceIds".into(),
                    value: network_interface_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osDisk".into(),
                    value: os_disk_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osImageNotification".into(),
                    value: os_image_notification_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "patchAssessmentMode".into(),
                    value: patch_assessment_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "patchMode".into(),
                    value: patch_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plan".into(),
                    value: plan_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "platformFaultDomain".into(),
                    value: platform_fault_domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "provisionVmAgent".into(),
                    value: provision_vm_agent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "proximityPlacementGroupId".into(),
                    value: proximity_placement_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rebootSetting".into(),
                    value: reboot_setting_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secrets".into(),
                    value: secrets_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secureBootEnabled".into(),
                    value: secure_boot_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "size".into(),
                    value: size_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceImageId".into(),
                    value: source_image_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceImageReference".into(),
                    value: source_image_reference_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "terminationNotification".into(),
                    value: termination_notification_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timezone".into(),
                    value: timezone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userData".into(),
                    value: user_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualMachineScaleSetId".into(),
                    value: virtual_machine_scale_set_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vmAgentPlatformUpdatesEnabled".into(),
                    value: vm_agent_platform_updates_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vtpmEnabled".into(),
                    value: vtpm_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "winrmListeners".into(),
                    value: winrm_listeners_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WindowsVirtualMachineResult {
            additional_capabilities: o.get_field("additionalCapabilities"),
            additional_unattend_contents: o.get_field("additionalUnattendContents"),
            admin_password: o.get_field("adminPassword"),
            admin_username: o.get_field("adminUsername"),
            allow_extension_operations: o.get_field("allowExtensionOperations"),
            availability_set_id: o.get_field("availabilitySetId"),
            boot_diagnostics: o.get_field("bootDiagnostics"),
            bypass_platform_safety_checks_on_user_schedule_enabled: o
                .get_field("bypassPlatformSafetyChecksOnUserScheduleEnabled"),
            capacity_reservation_group_id: o.get_field("capacityReservationGroupId"),
            computer_name: o.get_field("computerName"),
            custom_data: o.get_field("customData"),
            dedicated_host_group_id: o.get_field("dedicatedHostGroupId"),
            dedicated_host_id: o.get_field("dedicatedHostId"),
            disk_controller_type: o.get_field("diskControllerType"),
            edge_zone: o.get_field("edgeZone"),
            enable_automatic_updates: o.get_field("enableAutomaticUpdates"),
            encryption_at_host_enabled: o.get_field("encryptionAtHostEnabled"),
            eviction_policy: o.get_field("evictionPolicy"),
            extensions_time_budget: o.get_field("extensionsTimeBudget"),
            gallery_applications: o.get_field("galleryApplications"),
            hotpatching_enabled: o.get_field("hotpatchingEnabled"),
            identity: o.get_field("identity"),
            license_type: o.get_field("licenseType"),
            location: o.get_field("location"),
            max_bid_price: o.get_field("maxBidPrice"),
            name: o.get_field("name"),
            network_interface_ids: o.get_field("networkInterfaceIds"),
            os_disk: o.get_field("osDisk"),
            os_image_notification: o.get_field("osImageNotification"),
            patch_assessment_mode: o.get_field("patchAssessmentMode"),
            patch_mode: o.get_field("patchMode"),
            plan: o.get_field("plan"),
            platform_fault_domain: o.get_field("platformFaultDomain"),
            priority: o.get_field("priority"),
            private_ip_address: o.get_field("privateIpAddress"),
            private_ip_addresses: o.get_field("privateIpAddresses"),
            provision_vm_agent: o.get_field("provisionVmAgent"),
            proximity_placement_group_id: o.get_field("proximityPlacementGroupId"),
            public_ip_address: o.get_field("publicIpAddress"),
            public_ip_addresses: o.get_field("publicIpAddresses"),
            reboot_setting: o.get_field("rebootSetting"),
            resource_group_name: o.get_field("resourceGroupName"),
            secrets: o.get_field("secrets"),
            secure_boot_enabled: o.get_field("secureBootEnabled"),
            size: o.get_field("size"),
            source_image_id: o.get_field("sourceImageId"),
            source_image_reference: o.get_field("sourceImageReference"),
            tags: o.get_field("tags"),
            termination_notification: o.get_field("terminationNotification"),
            timezone: o.get_field("timezone"),
            user_data: o.get_field("userData"),
            virtual_machine_id: o.get_field("virtualMachineId"),
            virtual_machine_scale_set_id: o.get_field("virtualMachineScaleSetId"),
            vm_agent_platform_updates_enabled: o
                .get_field("vmAgentPlatformUpdatesEnabled"),
            vtpm_enabled: o.get_field("vtpmEnabled"),
            winrm_listeners: o.get_field("winrmListeners"),
            zone: o.get_field("zone"),
        }
    }
}
