/// Manages a Windows Virtual Machine Scale Set.
///
/// ## Disclaimers
///
/// > **Note:** This resource will only create Virtual Machine Scale Sets with the **Uniform** Orchestration Mode. For Virtual Machine Scale Sets with **Flexible** orchestration mode, use `azure.compute.OrchestratedVirtualMachineScaleSet`. Flexible orchestration mode is recommended for workloads on Azure.
///
/// > **Note:** All arguments including the administrator login and password will be stored in the raw state as plain-text. [Read more about sensitive data in state](https://www.terraform.io/docs/state/sensitive-data.html).
///
/// > **Note:** This provider will automatically update & reimage the nodes in the Scale Set (if Required) during an Update - this behaviour can be configured using the `features` setting within the Provider block.
///
/// > **Note:** This resource does not support Unmanaged Disks. If you need to use Unmanaged Disks you can continue to use the `azure.compute.ScaleSet` resource instead
///
/// ## Example Usage
///
/// This example provisions a basic Windows Virtual Machine Scale Set on an internal network.
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
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-network")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleWindowsVirtualMachineScaleSet = windows_virtual_machine_scale_set::create(
///         "exampleWindowsVirtualMachineScaleSet",
///         WindowsVirtualMachineScaleSetArgs::builder()
///             .admin_password("P@55w0rd1234!")
///             .admin_username("adminuser")
///             .computer_name_prefix("vm-")
///             .instances(1)
///             .location("${example.location}")
///             .name("example-vmss")
///             .network_interfaces(
///                 vec![
///                     WindowsVirtualMachineScaleSetNetworkInterface::builder()
///                     .ipConfigurations(vec![WindowsVirtualMachineScaleSetNetworkInterfaceIpConfiguration::builder()
///                     .name("internal").primary(true).subnetId("${internal.id}")
///                     .build_struct(),]).name("example").primary(true).build_struct(),
///                 ],
///             )
///             .os_disk(
///                 WindowsVirtualMachineScaleSetOsDisk::builder()
///                     .caching("ReadWrite")
///                     .storageAccountType("Standard_LRS")
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .sku("Standard_F2")
///             .source_image_reference(
///                 WindowsVirtualMachineScaleSetSourceImageReference::builder()
///                     .offer("WindowsServer")
///                     .publisher("MicrosoftWindowsServer")
///                     .sku("2016-Datacenter-Server-Core")
///                     .version("latest")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let internal = subnet::create(
///         "internal",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .name("internal")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Windows Virtual Machine Scale Sets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/windowsVirtualMachineScaleSet:WindowsVirtualMachineScaleSet example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/virtualMachineScaleSets/scaleset1
/// ```
///
pub mod windows_virtual_machine_scale_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WindowsVirtualMachineScaleSetArgs {
        /// An `additional_capabilities` block as defined below.
        #[builder(into, default)]
        pub additional_capabilities: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineScaleSetAdditionalCapabilities,
            >,
        >,
        /// One or more `additional_unattend_content` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub additional_unattend_contents: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::WindowsVirtualMachineScaleSetAdditionalUnattendContent,
                >,
            >,
        >,
        /// The Password which should be used for the local-administrator on this Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into)]
        pub admin_password: pulumi_wasm_rust::Output<String>,
        /// The username of the local administrator on each Virtual Machine Scale Set instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub admin_username: pulumi_wasm_rust::Output<String>,
        /// An `automatic_instance_repair` block as defined below. To enable the automatic instance repair, this Virtual Machine Scale Set must have a valid `health_probe_id` or an [Application Health Extension](https://docs.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-health-extension).
        ///
        /// > **Note:** For more information about Automatic Instance Repair, please refer to [this doc](https://docs.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-automatic-instance-repairs).
        #[builder(into, default)]
        pub automatic_instance_repair: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineScaleSetAutomaticInstanceRepair,
            >,
        >,
        /// An `automatic_os_upgrade_policy` block as defined below. This can only be specified when `upgrade_mode` is set to either `Automatic` or `Rolling`.
        #[builder(into, default)]
        pub automatic_os_upgrade_policy: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineScaleSetAutomaticOsUpgradePolicy,
            >,
        >,
        /// A `boot_diagnostics` block as defined below.
        #[builder(into, default)]
        pub boot_diagnostics: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineScaleSetBootDiagnostics,
            >,
        >,
        /// Specifies the ID of the Capacity Reservation Group which the Virtual Machine Scale Set should be allocated to. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `capacity_reservation_group_id` cannot be used with `proximity_placement_group_id`
        ///
        /// > **Note:** `single_placement_group` must be set to `false` when `capacity_reservation_group_id` is specified.
        #[builder(into, default)]
        pub capacity_reservation_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The prefix which should be used for the name of the Virtual Machines in this Scale Set. If unspecified this defaults to the value for the `name` field. If the value of the `name` field is not a valid `computer_name_prefix`, then you must specify `computer_name_prefix`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub computer_name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The Base64-Encoded Custom Data which should be used for this Virtual Machine Scale Set.
        ///
        /// > **Note:** When Custom Data has been configured, it's not possible to remove it without tainting the Virtual Machine Scale Set, due to a limitation of the Azure API.
        #[builder(into, default)]
        pub custom_data: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `data_disk` blocks as defined below.
        #[builder(into, default)]
        pub data_disks: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::WindowsVirtualMachineScaleSetDataDisk>,
            >,
        >,
        /// Should Virtual Machine Extensions be run on Overprovisioned Virtual Machines in the Scale Set? Defaults to `false`.
        #[builder(into, default)]
        pub do_not_run_extensions_on_overprovisioned_machines: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Specifies the Edge Zone within the Azure Region where this Windows Virtual Machine Scale Set should exist. Changing this forces a new Windows Virtual Machine Scale Set to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// Are automatic updates enabled for this Virtual Machine? Defaults to `true`.
        #[builder(into, default)]
        pub enable_automatic_updates: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should all of the disks (including the temp disk) attached to this Virtual Machine be encrypted by enabling Encryption at Host?
        #[builder(into, default)]
        pub encryption_at_host_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the eviction policy for Virtual Machines in this Scale Set. Possible values are `Deallocate` and `Delete`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This can only be configured when `priority` is set to `Spot`.
        #[builder(into, default)]
        pub eviction_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Should extension operations be allowed on the Virtual Machine Scale Set? Possible values are `true` or `false`. Defaults to `true`. Changing this forces a new Windows Virtual Machine Scale Set to be created.
        ///
        /// > **Note:** `extension_operations_enabled` may only be set to `false` if there are no extensions defined in the `extension` field.
        #[builder(into, default)]
        pub extension_operations_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `extension` blocks as defined below
        #[builder(into, default)]
        pub extensions: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::WindowsVirtualMachineScaleSetExtension>,
            >,
        >,
        /// Specifies the duration allocated for all extensions to start. The time duration should be between `15` minutes and `120` minutes (inclusive) and should be specified in ISO 8601 format. Defaults to `PT1H30M`.
        #[builder(into, default)]
        pub extensions_time_budget: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `gallery_application` blocks as defined below.
        #[builder(into, default)]
        pub gallery_applications: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::WindowsVirtualMachineScaleSetGalleryApplication,
                >,
            >,
        >,
        /// The ID of a Load Balancer Probe which should be used to determine the health of an instance. This is Required and can only be specified when `upgrade_mode` is set to `Automatic` or `Rolling`.
        #[builder(into, default)]
        pub health_probe_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the dedicated host group that the virtual machine scale set resides in. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub host_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::WindowsVirtualMachineScaleSetIdentity>,
        >,
        /// The number of Virtual Machines in the Scale Set.
        ///
        /// > **NOTE:** If you're using AutoScaling, you may wish to use [`Ignore Changes` functionality](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) to ignore changes to this field.
        #[builder(into)]
        pub instances: pulumi_wasm_rust::Output<i32>,
        /// Specifies the type of on-premise license (also known as [Azure Hybrid Use Benefit](https://docs.microsoft.com/azure/virtual-machines/virtual-machines-windows-hybrid-use-benefit-licensing)) which should be used for this Virtual Machine Scale Set. Possible values are `None`, `Windows_Client` and `Windows_Server`.
        #[builder(into, default)]
        pub license_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The Azure location where the Windows Virtual Machine Scale Set should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum price you're willing to pay for each Virtual Machine in this Scale Set, in US Dollars; which must be greater than the current spot price. If this bid price falls below the current spot price the Virtual Machines in the Scale Set will be evicted using the `eviction_policy`. Defaults to `-1`, which means that each Virtual Machine in the Scale Set should not be evicted for price reasons.
        ///
        /// > **Note:** This can only be configured when `priority` is set to `Spot`.
        #[builder(into, default)]
        pub max_bid_price: pulumi_wasm_rust::Output<Option<f64>>,
        /// The name of the Windows Virtual Machine Scale Set. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `network_interface` blocks as defined below.
        #[builder(into)]
        pub network_interfaces: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::compute::WindowsVirtualMachineScaleSetNetworkInterface,
            >,
        >,
        /// An `os_disk` block as defined below.
        #[builder(into)]
        pub os_disk: pulumi_wasm_rust::Output<
            super::super::types::compute::WindowsVirtualMachineScaleSetOsDisk,
        >,
        /// Should Azure over-provision Virtual Machines in this Scale Set? This means that multiple Virtual Machines will be provisioned and Azure will keep the instances which become available first - which improves provisioning success rates and improves deployment time. You're not billed for these over-provisioned VM's and they don't count towards the Subscription Quota. Defaults to `true`.
        #[builder(into, default)]
        pub overprovision: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `plan` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **Note:** When using an image from Azure Marketplace a `plan` must be specified.
        #[builder(into, default)]
        pub plan: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::WindowsVirtualMachineScaleSetPlan>,
        >,
        /// Specifies the number of fault domains that are used by this Linux Virtual Machine Scale Set. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub platform_fault_domain_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Priority of this Virtual Machine Scale Set. Possible values are `Regular` and `Spot`. Defaults to `Regular`. Changing this value forces a new resource.
        ///
        /// > **Note:** When `priority` is set to `Spot` an `eviction_policy` must be specified.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Azure VM Agent be provisioned on each Virtual Machine in the Scale Set? Defaults to `true`. Changing this value forces a new resource to be created.
        #[builder(into, default)]
        pub provision_vm_agent: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Proximity Placement Group in which the Virtual Machine Scale Set should be assigned to. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub proximity_placement_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the Windows Virtual Machine Scale Set should be exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `rolling_upgrade_policy` block as defined below. This is Required and can only be specified when `upgrade_mode` is set to `Automatic` or `Rolling`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub rolling_upgrade_policy: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineScaleSetRollingUpgradePolicy,
            >,
        >,
        /// A `scale_in` block as defined below.
        #[builder(into, default)]
        pub scale_in: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::WindowsVirtualMachineScaleSetScaleIn>,
        >,
        /// One or more `secret` blocks as defined below.
        #[builder(into, default)]
        pub secrets: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::WindowsVirtualMachineScaleSetSecret>,
            >,
        >,
        /// Specifies if Secure Boot and Trusted Launch is enabled for the Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub secure_boot_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should this Virtual Machine Scale Set be limited to a Single Placement Group, which means the number of instances will be capped at 100 Virtual Machines. Defaults to `true`.
        #[builder(into, default)]
        pub single_placement_group: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Virtual Machine SKU for the Scale Set, such as `Standard_F2`.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::Output<String>,
        /// The ID of an Image which each Virtual Machine in this Scale Set should be based on. Possible Image ID types include `Image ID`, `Shared Image ID`, `Shared Image Version ID`, `Community Gallery Image ID`, `Community Gallery Image Version ID`, `Shared Gallery Image ID` and `Shared Gallery Image Version ID`.
        ///
        /// > **Note:** One of either `source_image_id` or `source_image_reference` must be set.
        #[builder(into, default)]
        pub source_image_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `source_image_reference` block as defined below.
        ///
        /// > **Note:** One of either `source_image_id` or `source_image_reference` must be set.
        #[builder(into, default)]
        pub source_image_reference: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineScaleSetSourceImageReference,
            >,
        >,
        /// A `spot_restore` block as defined below.
        #[builder(into, default)]
        pub spot_restore: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineScaleSetSpotRestore,
            >,
        >,
        /// A mapping of tags which should be assigned to this Virtual Machine Scale Set.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `termination_notification` block as defined below.
        #[builder(into, default)]
        pub termination_notification: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineScaleSetTerminationNotification,
            >,
        >,
        /// Specifies the time zone of the virtual machine, [the possible values are defined here](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/).
        #[builder(into, default)]
        pub timezone: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub upgrade_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The Base64-Encoded User Data which should be used for this Virtual Machine Scale Set.
        #[builder(into, default)]
        pub user_data: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies if vTPM (Virtual Trusted Platform Module) and Trusted Launch is enabled for the Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub vtpm_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `winrm_listener` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub winrm_listeners: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::WindowsVirtualMachineScaleSetWinrmListener,
                >,
            >,
        >,
        /// Should the Virtual Machines in this Scale Set be strictly evenly distributed across Availability Zones? Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This can only be set to `true` when one or more `zones` are configured.
        #[builder(into, default)]
        pub zone_balance: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies a list of Availability Zones in which this Windows Virtual Machine Scale Set should be located.
        ///
        /// > **Note:** Updating `zones` to remove an existing zone forces a new Virtual Machine Scale Set to be created.
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct WindowsVirtualMachineScaleSetResult {
        /// An `additional_capabilities` block as defined below.
        pub additional_capabilities: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineScaleSetAdditionalCapabilities,
            >,
        >,
        /// One or more `additional_unattend_content` blocks as defined below. Changing this forces a new resource to be created.
        pub additional_unattend_contents: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::WindowsVirtualMachineScaleSetAdditionalUnattendContent,
                >,
            >,
        >,
        /// The Password which should be used for the local-administrator on this Virtual Machine. Changing this forces a new resource to be created.
        pub admin_password: pulumi_wasm_rust::Output<String>,
        /// The username of the local administrator on each Virtual Machine Scale Set instance. Changing this forces a new resource to be created.
        pub admin_username: pulumi_wasm_rust::Output<String>,
        /// An `automatic_instance_repair` block as defined below. To enable the automatic instance repair, this Virtual Machine Scale Set must have a valid `health_probe_id` or an [Application Health Extension](https://docs.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-health-extension).
        ///
        /// > **Note:** For more information about Automatic Instance Repair, please refer to [this doc](https://docs.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-automatic-instance-repairs).
        pub automatic_instance_repair: pulumi_wasm_rust::Output<
            super::super::types::compute::WindowsVirtualMachineScaleSetAutomaticInstanceRepair,
        >,
        /// An `automatic_os_upgrade_policy` block as defined below. This can only be specified when `upgrade_mode` is set to either `Automatic` or `Rolling`.
        pub automatic_os_upgrade_policy: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineScaleSetAutomaticOsUpgradePolicy,
            >,
        >,
        /// A `boot_diagnostics` block as defined below.
        pub boot_diagnostics: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineScaleSetBootDiagnostics,
            >,
        >,
        /// Specifies the ID of the Capacity Reservation Group which the Virtual Machine Scale Set should be allocated to. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `capacity_reservation_group_id` cannot be used with `proximity_placement_group_id`
        ///
        /// > **Note:** `single_placement_group` must be set to `false` when `capacity_reservation_group_id` is specified.
        pub capacity_reservation_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The prefix which should be used for the name of the Virtual Machines in this Scale Set. If unspecified this defaults to the value for the `name` field. If the value of the `name` field is not a valid `computer_name_prefix`, then you must specify `computer_name_prefix`. Changing this forces a new resource to be created.
        pub computer_name_prefix: pulumi_wasm_rust::Output<String>,
        /// The Base64-Encoded Custom Data which should be used for this Virtual Machine Scale Set.
        ///
        /// > **Note:** When Custom Data has been configured, it's not possible to remove it without tainting the Virtual Machine Scale Set, due to a limitation of the Azure API.
        pub custom_data: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `data_disk` blocks as defined below.
        pub data_disks: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::WindowsVirtualMachineScaleSetDataDisk>,
            >,
        >,
        /// Should Virtual Machine Extensions be run on Overprovisioned Virtual Machines in the Scale Set? Defaults to `false`.
        pub do_not_run_extensions_on_overprovisioned_machines: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Specifies the Edge Zone within the Azure Region where this Windows Virtual Machine Scale Set should exist. Changing this forces a new Windows Virtual Machine Scale Set to be created.
        pub edge_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// Are automatic updates enabled for this Virtual Machine? Defaults to `true`.
        pub enable_automatic_updates: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should all of the disks (including the temp disk) attached to this Virtual Machine be encrypted by enabling Encryption at Host?
        pub encryption_at_host_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the eviction policy for Virtual Machines in this Scale Set. Possible values are `Deallocate` and `Delete`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This can only be configured when `priority` is set to `Spot`.
        pub eviction_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Should extension operations be allowed on the Virtual Machine Scale Set? Possible values are `true` or `false`. Defaults to `true`. Changing this forces a new Windows Virtual Machine Scale Set to be created.
        ///
        /// > **Note:** `extension_operations_enabled` may only be set to `false` if there are no extensions defined in the `extension` field.
        pub extension_operations_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `extension` blocks as defined below
        pub extensions: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::WindowsVirtualMachineScaleSetExtension>,
        >,
        /// Specifies the duration allocated for all extensions to start. The time duration should be between `15` minutes and `120` minutes (inclusive) and should be specified in ISO 8601 format. Defaults to `PT1H30M`.
        pub extensions_time_budget: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `gallery_application` blocks as defined below.
        pub gallery_applications: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::WindowsVirtualMachineScaleSetGalleryApplication,
                >,
            >,
        >,
        /// The ID of a Load Balancer Probe which should be used to determine the health of an instance. This is Required and can only be specified when `upgrade_mode` is set to `Automatic` or `Rolling`.
        pub health_probe_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the dedicated host group that the virtual machine scale set resides in. Changing this forces a new resource to be created.
        pub host_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::WindowsVirtualMachineScaleSetIdentity>,
        >,
        /// The number of Virtual Machines in the Scale Set.
        ///
        /// > **NOTE:** If you're using AutoScaling, you may wish to use [`Ignore Changes` functionality](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) to ignore changes to this field.
        pub instances: pulumi_wasm_rust::Output<i32>,
        /// Specifies the type of on-premise license (also known as [Azure Hybrid Use Benefit](https://docs.microsoft.com/azure/virtual-machines/virtual-machines-windows-hybrid-use-benefit-licensing)) which should be used for this Virtual Machine Scale Set. Possible values are `None`, `Windows_Client` and `Windows_Server`.
        pub license_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The Azure location where the Windows Virtual Machine Scale Set should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The maximum price you're willing to pay for each Virtual Machine in this Scale Set, in US Dollars; which must be greater than the current spot price. If this bid price falls below the current spot price the Virtual Machines in the Scale Set will be evicted using the `eviction_policy`. Defaults to `-1`, which means that each Virtual Machine in the Scale Set should not be evicted for price reasons.
        ///
        /// > **Note:** This can only be configured when `priority` is set to `Spot`.
        pub max_bid_price: pulumi_wasm_rust::Output<Option<f64>>,
        /// The name of the Windows Virtual Machine Scale Set. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `network_interface` blocks as defined below.
        pub network_interfaces: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::compute::WindowsVirtualMachineScaleSetNetworkInterface,
            >,
        >,
        /// An `os_disk` block as defined below.
        pub os_disk: pulumi_wasm_rust::Output<
            super::super::types::compute::WindowsVirtualMachineScaleSetOsDisk,
        >,
        /// Should Azure over-provision Virtual Machines in this Scale Set? This means that multiple Virtual Machines will be provisioned and Azure will keep the instances which become available first - which improves provisioning success rates and improves deployment time. You're not billed for these over-provisioned VM's and they don't count towards the Subscription Quota. Defaults to `true`.
        pub overprovision: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `plan` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **Note:** When using an image from Azure Marketplace a `plan` must be specified.
        pub plan: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::WindowsVirtualMachineScaleSetPlan>,
        >,
        /// Specifies the number of fault domains that are used by this Linux Virtual Machine Scale Set. Changing this forces a new resource to be created.
        pub platform_fault_domain_count: pulumi_wasm_rust::Output<i32>,
        /// The Priority of this Virtual Machine Scale Set. Possible values are `Regular` and `Spot`. Defaults to `Regular`. Changing this value forces a new resource.
        ///
        /// > **Note:** When `priority` is set to `Spot` an `eviction_policy` must be specified.
        pub priority: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Azure VM Agent be provisioned on each Virtual Machine in the Scale Set? Defaults to `true`. Changing this value forces a new resource to be created.
        pub provision_vm_agent: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Proximity Placement Group in which the Virtual Machine Scale Set should be assigned to. Changing this forces a new resource to be created.
        pub proximity_placement_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the Windows Virtual Machine Scale Set should be exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `rolling_upgrade_policy` block as defined below. This is Required and can only be specified when `upgrade_mode` is set to `Automatic` or `Rolling`. Changing this forces a new resource to be created.
        pub rolling_upgrade_policy: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineScaleSetRollingUpgradePolicy,
            >,
        >,
        /// A `scale_in` block as defined below.
        pub scale_in: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::WindowsVirtualMachineScaleSetScaleIn>,
        >,
        /// One or more `secret` blocks as defined below.
        pub secrets: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::WindowsVirtualMachineScaleSetSecret>,
            >,
        >,
        /// Specifies if Secure Boot and Trusted Launch is enabled for the Virtual Machine. Changing this forces a new resource to be created.
        pub secure_boot_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should this Virtual Machine Scale Set be limited to a Single Placement Group, which means the number of instances will be capped at 100 Virtual Machines. Defaults to `true`.
        pub single_placement_group: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Virtual Machine SKU for the Scale Set, such as `Standard_F2`.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// The ID of an Image which each Virtual Machine in this Scale Set should be based on. Possible Image ID types include `Image ID`, `Shared Image ID`, `Shared Image Version ID`, `Community Gallery Image ID`, `Community Gallery Image Version ID`, `Shared Gallery Image ID` and `Shared Gallery Image Version ID`.
        ///
        /// > **Note:** One of either `source_image_id` or `source_image_reference` must be set.
        pub source_image_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `source_image_reference` block as defined below.
        ///
        /// > **Note:** One of either `source_image_id` or `source_image_reference` must be set.
        pub source_image_reference: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::WindowsVirtualMachineScaleSetSourceImageReference,
            >,
        >,
        /// A `spot_restore` block as defined below.
        pub spot_restore: pulumi_wasm_rust::Output<
            super::super::types::compute::WindowsVirtualMachineScaleSetSpotRestore,
        >,
        /// A mapping of tags which should be assigned to this Virtual Machine Scale Set.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `termination_notification` block as defined below.
        pub termination_notification: pulumi_wasm_rust::Output<
            super::super::types::compute::WindowsVirtualMachineScaleSetTerminationNotification,
        >,
        /// Specifies the time zone of the virtual machine, [the possible values are defined here](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/).
        pub timezone: pulumi_wasm_rust::Output<Option<String>>,
        /// The Unique ID for this Windows Virtual Machine Scale Set.
        pub unique_id: pulumi_wasm_rust::Output<String>,
        pub upgrade_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The Base64-Encoded User Data which should be used for this Virtual Machine Scale Set.
        pub user_data: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies if vTPM (Virtual Trusted Platform Module) and Trusted Launch is enabled for the Virtual Machine. Changing this forces a new resource to be created.
        pub vtpm_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `winrm_listener` blocks as defined below. Changing this forces a new resource to be created.
        pub winrm_listeners: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::WindowsVirtualMachineScaleSetWinrmListener,
                >,
            >,
        >,
        /// Should the Virtual Machines in this Scale Set be strictly evenly distributed across Availability Zones? Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This can only be set to `true` when one or more `zones` are configured.
        pub zone_balance: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies a list of Availability Zones in which this Windows Virtual Machine Scale Set should be located.
        ///
        /// > **Note:** Updating `zones` to remove an existing zone forces a new Virtual Machine Scale Set to be created.
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: WindowsVirtualMachineScaleSetArgs,
    ) -> WindowsVirtualMachineScaleSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_capabilities_binding = args.additional_capabilities.get_inner();
        let additional_unattend_contents_binding = args
            .additional_unattend_contents
            .get_inner();
        let admin_password_binding = args.admin_password.get_inner();
        let admin_username_binding = args.admin_username.get_inner();
        let automatic_instance_repair_binding = args
            .automatic_instance_repair
            .get_inner();
        let automatic_os_upgrade_policy_binding = args
            .automatic_os_upgrade_policy
            .get_inner();
        let boot_diagnostics_binding = args.boot_diagnostics.get_inner();
        let capacity_reservation_group_id_binding = args
            .capacity_reservation_group_id
            .get_inner();
        let computer_name_prefix_binding = args.computer_name_prefix.get_inner();
        let custom_data_binding = args.custom_data.get_inner();
        let data_disks_binding = args.data_disks.get_inner();
        let do_not_run_extensions_on_overprovisioned_machines_binding = args
            .do_not_run_extensions_on_overprovisioned_machines
            .get_inner();
        let edge_zone_binding = args.edge_zone.get_inner();
        let enable_automatic_updates_binding = args.enable_automatic_updates.get_inner();
        let encryption_at_host_enabled_binding = args
            .encryption_at_host_enabled
            .get_inner();
        let eviction_policy_binding = args.eviction_policy.get_inner();
        let extension_operations_enabled_binding = args
            .extension_operations_enabled
            .get_inner();
        let extensions_binding = args.extensions.get_inner();
        let extensions_time_budget_binding = args.extensions_time_budget.get_inner();
        let gallery_applications_binding = args.gallery_applications.get_inner();
        let health_probe_id_binding = args.health_probe_id.get_inner();
        let host_group_id_binding = args.host_group_id.get_inner();
        let identity_binding = args.identity.get_inner();
        let instances_binding = args.instances.get_inner();
        let license_type_binding = args.license_type.get_inner();
        let location_binding = args.location.get_inner();
        let max_bid_price_binding = args.max_bid_price.get_inner();
        let name_binding = args.name.get_inner();
        let network_interfaces_binding = args.network_interfaces.get_inner();
        let os_disk_binding = args.os_disk.get_inner();
        let overprovision_binding = args.overprovision.get_inner();
        let plan_binding = args.plan.get_inner();
        let platform_fault_domain_count_binding = args
            .platform_fault_domain_count
            .get_inner();
        let priority_binding = args.priority.get_inner();
        let provision_vm_agent_binding = args.provision_vm_agent.get_inner();
        let proximity_placement_group_id_binding = args
            .proximity_placement_group_id
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let rolling_upgrade_policy_binding = args.rolling_upgrade_policy.get_inner();
        let scale_in_binding = args.scale_in.get_inner();
        let secrets_binding = args.secrets.get_inner();
        let secure_boot_enabled_binding = args.secure_boot_enabled.get_inner();
        let single_placement_group_binding = args.single_placement_group.get_inner();
        let sku_binding = args.sku.get_inner();
        let source_image_id_binding = args.source_image_id.get_inner();
        let source_image_reference_binding = args.source_image_reference.get_inner();
        let spot_restore_binding = args.spot_restore.get_inner();
        let tags_binding = args.tags.get_inner();
        let termination_notification_binding = args.termination_notification.get_inner();
        let timezone_binding = args.timezone.get_inner();
        let upgrade_mode_binding = args.upgrade_mode.get_inner();
        let user_data_binding = args.user_data.get_inner();
        let vtpm_enabled_binding = args.vtpm_enabled.get_inner();
        let winrm_listeners_binding = args.winrm_listeners.get_inner();
        let zone_balance_binding = args.zone_balance.get_inner();
        let zones_binding = args.zones.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/windowsVirtualMachineScaleSet:WindowsVirtualMachineScaleSet"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalCapabilities".into(),
                    value: &additional_capabilities_binding,
                },
                register_interface::ObjectField {
                    name: "additionalUnattendContents".into(),
                    value: &additional_unattend_contents_binding,
                },
                register_interface::ObjectField {
                    name: "adminPassword".into(),
                    value: &admin_password_binding,
                },
                register_interface::ObjectField {
                    name: "adminUsername".into(),
                    value: &admin_username_binding,
                },
                register_interface::ObjectField {
                    name: "automaticInstanceRepair".into(),
                    value: &automatic_instance_repair_binding,
                },
                register_interface::ObjectField {
                    name: "automaticOsUpgradePolicy".into(),
                    value: &automatic_os_upgrade_policy_binding,
                },
                register_interface::ObjectField {
                    name: "bootDiagnostics".into(),
                    value: &boot_diagnostics_binding,
                },
                register_interface::ObjectField {
                    name: "capacityReservationGroupId".into(),
                    value: &capacity_reservation_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "computerNamePrefix".into(),
                    value: &computer_name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "customData".into(),
                    value: &custom_data_binding,
                },
                register_interface::ObjectField {
                    name: "dataDisks".into(),
                    value: &data_disks_binding,
                },
                register_interface::ObjectField {
                    name: "doNotRunExtensionsOnOverprovisionedMachines".into(),
                    value: &do_not_run_extensions_on_overprovisioned_machines_binding,
                },
                register_interface::ObjectField {
                    name: "edgeZone".into(),
                    value: &edge_zone_binding,
                },
                register_interface::ObjectField {
                    name: "enableAutomaticUpdates".into(),
                    value: &enable_automatic_updates_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionAtHostEnabled".into(),
                    value: &encryption_at_host_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "evictionPolicy".into(),
                    value: &eviction_policy_binding,
                },
                register_interface::ObjectField {
                    name: "extensionOperationsEnabled".into(),
                    value: &extension_operations_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "extensions".into(),
                    value: &extensions_binding,
                },
                register_interface::ObjectField {
                    name: "extensionsTimeBudget".into(),
                    value: &extensions_time_budget_binding,
                },
                register_interface::ObjectField {
                    name: "galleryApplications".into(),
                    value: &gallery_applications_binding,
                },
                register_interface::ObjectField {
                    name: "healthProbeId".into(),
                    value: &health_probe_id_binding,
                },
                register_interface::ObjectField {
                    name: "hostGroupId".into(),
                    value: &host_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "instances".into(),
                    value: &instances_binding,
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
                    name: "maxBidPrice".into(),
                    value: &max_bid_price_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterfaces".into(),
                    value: &network_interfaces_binding,
                },
                register_interface::ObjectField {
                    name: "osDisk".into(),
                    value: &os_disk_binding,
                },
                register_interface::ObjectField {
                    name: "overprovision".into(),
                    value: &overprovision_binding,
                },
                register_interface::ObjectField {
                    name: "plan".into(),
                    value: &plan_binding,
                },
                register_interface::ObjectField {
                    name: "platformFaultDomainCount".into(),
                    value: &platform_fault_domain_count_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "provisionVmAgent".into(),
                    value: &provision_vm_agent_binding,
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
                    name: "rollingUpgradePolicy".into(),
                    value: &rolling_upgrade_policy_binding,
                },
                register_interface::ObjectField {
                    name: "scaleIn".into(),
                    value: &scale_in_binding,
                },
                register_interface::ObjectField {
                    name: "secrets".into(),
                    value: &secrets_binding,
                },
                register_interface::ObjectField {
                    name: "secureBootEnabled".into(),
                    value: &secure_boot_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "singlePlacementGroup".into(),
                    value: &single_placement_group_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "sourceImageId".into(),
                    value: &source_image_id_binding,
                },
                register_interface::ObjectField {
                    name: "sourceImageReference".into(),
                    value: &source_image_reference_binding,
                },
                register_interface::ObjectField {
                    name: "spotRestore".into(),
                    value: &spot_restore_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "terminationNotification".into(),
                    value: &termination_notification_binding,
                },
                register_interface::ObjectField {
                    name: "timezone".into(),
                    value: &timezone_binding,
                },
                register_interface::ObjectField {
                    name: "upgradeMode".into(),
                    value: &upgrade_mode_binding,
                },
                register_interface::ObjectField {
                    name: "userData".into(),
                    value: &user_data_binding,
                },
                register_interface::ObjectField {
                    name: "vtpmEnabled".into(),
                    value: &vtpm_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "winrmListeners".into(),
                    value: &winrm_listeners_binding,
                },
                register_interface::ObjectField {
                    name: "zoneBalance".into(),
                    value: &zone_balance_binding,
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
                    name: "additionalUnattendContents".into(),
                },
                register_interface::ResultField {
                    name: "adminPassword".into(),
                },
                register_interface::ResultField {
                    name: "adminUsername".into(),
                },
                register_interface::ResultField {
                    name: "automaticInstanceRepair".into(),
                },
                register_interface::ResultField {
                    name: "automaticOsUpgradePolicy".into(),
                },
                register_interface::ResultField {
                    name: "bootDiagnostics".into(),
                },
                register_interface::ResultField {
                    name: "capacityReservationGroupId".into(),
                },
                register_interface::ResultField {
                    name: "computerNamePrefix".into(),
                },
                register_interface::ResultField {
                    name: "customData".into(),
                },
                register_interface::ResultField {
                    name: "dataDisks".into(),
                },
                register_interface::ResultField {
                    name: "doNotRunExtensionsOnOverprovisionedMachines".into(),
                },
                register_interface::ResultField {
                    name: "edgeZone".into(),
                },
                register_interface::ResultField {
                    name: "enableAutomaticUpdates".into(),
                },
                register_interface::ResultField {
                    name: "encryptionAtHostEnabled".into(),
                },
                register_interface::ResultField {
                    name: "evictionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "extensionOperationsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "extensions".into(),
                },
                register_interface::ResultField {
                    name: "extensionsTimeBudget".into(),
                },
                register_interface::ResultField {
                    name: "galleryApplications".into(),
                },
                register_interface::ResultField {
                    name: "healthProbeId".into(),
                },
                register_interface::ResultField {
                    name: "hostGroupId".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "instances".into(),
                },
                register_interface::ResultField {
                    name: "licenseType".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maxBidPrice".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaces".into(),
                },
                register_interface::ResultField {
                    name: "osDisk".into(),
                },
                register_interface::ResultField {
                    name: "overprovision".into(),
                },
                register_interface::ResultField {
                    name: "plan".into(),
                },
                register_interface::ResultField {
                    name: "platformFaultDomainCount".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "provisionVmAgent".into(),
                },
                register_interface::ResultField {
                    name: "proximityPlacementGroupId".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "rollingUpgradePolicy".into(),
                },
                register_interface::ResultField {
                    name: "scaleIn".into(),
                },
                register_interface::ResultField {
                    name: "secrets".into(),
                },
                register_interface::ResultField {
                    name: "secureBootEnabled".into(),
                },
                register_interface::ResultField {
                    name: "singlePlacementGroup".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "sourceImageId".into(),
                },
                register_interface::ResultField {
                    name: "sourceImageReference".into(),
                },
                register_interface::ResultField {
                    name: "spotRestore".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "terminationNotification".into(),
                },
                register_interface::ResultField {
                    name: "timezone".into(),
                },
                register_interface::ResultField {
                    name: "uniqueId".into(),
                },
                register_interface::ResultField {
                    name: "upgradeMode".into(),
                },
                register_interface::ResultField {
                    name: "userData".into(),
                },
                register_interface::ResultField {
                    name: "vtpmEnabled".into(),
                },
                register_interface::ResultField {
                    name: "winrmListeners".into(),
                },
                register_interface::ResultField {
                    name: "zoneBalance".into(),
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
        WindowsVirtualMachineScaleSetResult {
            additional_capabilities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalCapabilities").unwrap(),
            ),
            additional_unattend_contents: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalUnattendContents").unwrap(),
            ),
            admin_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminPassword").unwrap(),
            ),
            admin_username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminUsername").unwrap(),
            ),
            automatic_instance_repair: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticInstanceRepair").unwrap(),
            ),
            automatic_os_upgrade_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticOsUpgradePolicy").unwrap(),
            ),
            boot_diagnostics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootDiagnostics").unwrap(),
            ),
            capacity_reservation_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacityReservationGroupId").unwrap(),
            ),
            computer_name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computerNamePrefix").unwrap(),
            ),
            custom_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customData").unwrap(),
            ),
            data_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataDisks").unwrap(),
            ),
            do_not_run_extensions_on_overprovisioned_machines: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("doNotRunExtensionsOnOverprovisionedMachines").unwrap(),
            ),
            edge_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeZone").unwrap(),
            ),
            enable_automatic_updates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableAutomaticUpdates").unwrap(),
            ),
            encryption_at_host_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionAtHostEnabled").unwrap(),
            ),
            eviction_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("evictionPolicy").unwrap(),
            ),
            extension_operations_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extensionOperationsEnabled").unwrap(),
            ),
            extensions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extensions").unwrap(),
            ),
            extensions_time_budget: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extensionsTimeBudget").unwrap(),
            ),
            gallery_applications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("galleryApplications").unwrap(),
            ),
            health_probe_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthProbeId").unwrap(),
            ),
            host_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostGroupId").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            instances: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instances").unwrap(),
            ),
            license_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseType").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            max_bid_price: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxBidPrice").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_interfaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaces").unwrap(),
            ),
            os_disk: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osDisk").unwrap(),
            ),
            overprovision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("overprovision").unwrap(),
            ),
            plan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("plan").unwrap(),
            ),
            platform_fault_domain_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformFaultDomainCount").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            provision_vm_agent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisionVmAgent").unwrap(),
            ),
            proximity_placement_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proximityPlacementGroupId").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            rolling_upgrade_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rollingUpgradePolicy").unwrap(),
            ),
            scale_in: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scaleIn").unwrap(),
            ),
            secrets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secrets").unwrap(),
            ),
            secure_boot_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secureBootEnabled").unwrap(),
            ),
            single_placement_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("singlePlacementGroup").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            source_image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceImageId").unwrap(),
            ),
            source_image_reference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceImageReference").unwrap(),
            ),
            spot_restore: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spotRestore").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            termination_notification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terminationNotification").unwrap(),
            ),
            timezone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timezone").unwrap(),
            ),
            unique_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uniqueId").unwrap(),
            ),
            upgrade_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("upgradeMode").unwrap(),
            ),
            user_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userData").unwrap(),
            ),
            vtpm_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vtpmEnabled").unwrap(),
            ),
            winrm_listeners: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("winrmListeners").unwrap(),
            ),
            zone_balance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneBalance").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}
