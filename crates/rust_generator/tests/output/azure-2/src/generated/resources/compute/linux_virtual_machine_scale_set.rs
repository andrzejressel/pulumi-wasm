/// Manages a Linux Virtual Machine Scale Set.
///
/// ## Disclaimers
///
/// > **Note:** As of the **v2.86.0** (November 19, 2021) release of the provider this resource will only create Virtual Machine Scale Sets with the **Uniform** Orchestration Mode. For Virtual Machine Scale Sets with **Flexible** orchestration mode, use `azure.compute.OrchestratedVirtualMachineScaleSet`. Flexible orchestration mode is recommended for workloads on Azure.
/// rraform will automatically update & reimage the nodes in the Scale Set (if Required) during an Update - this behaviour can be configured using the `features` setting within the Provider block.
///
/// ## Example Usage
///
/// This example provisions a basic Linux Virtual Machine Scale Set on an internal network.
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
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       addressSpaces:
///         - 10.0.0.0/16
///   internal:
///     type: azure:network:Subnet
///     properties:
///       name: internal
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleLinuxVirtualMachineScaleSet:
///     type: azure:compute:LinuxVirtualMachineScaleSet
///     name: example
///     properties:
///       name: example-vmss
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: Standard_F2
///       instances: 1
///       adminUsername: adminuser
///       adminSshKeys:
///         - username: adminuser
///           publicKey: ${firstPublicKey}
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///       osDisk:
///         storageAccountType: Standard_LRS
///         caching: ReadWrite
///       networkInterfaces:
///         - name: example
///           primary: true
///           ipConfigurations:
///             - name: internal
///               primary: true
///               subnetId: ${internal.id}
/// variables:
///   firstPublicKey: ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQC+wWK73dCr+jgQOAxNsHAnNNNMEMWOHYEccp6wJm2gotpr9katuF/ZAdou5AaW1C61slRkHRkpRRX9FA9CYBiitZgvCCz+3nWNN7l/Up54Zps/pHWGZLHNJZRYyAB6j5yVLMVHIHriY49d/GZTZVNB8GoJv9Gakwc/fuEZYYl4YDFiGMBP///TzlI4jhiJzjKnEvqPFki5p2ZRJqcbCiF4pJrxUQR/RXqVFQdbRLZgYfJ8xGB878RENq3yQ39d8dVOkq4edbkzwcUmwwwkYVPIoDGsYLaRHnG+To7FvMeyO7xDVQkMKzopTQV8AuKpyvpqu0a9pWOMaiCyDytO7GGN you@me.com
/// ```
///
/// ## Import
///
/// Linux Virtual Machine Scale Sets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/linuxVirtualMachineScaleSet:LinuxVirtualMachineScaleSet example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/virtualMachineScaleSets/scaleset1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod linux_virtual_machine_scale_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinuxVirtualMachineScaleSetArgs {
        /// An `additional_capabilities` block as defined below.
        #[builder(into, default)]
        pub additional_capabilities: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::LinuxVirtualMachineScaleSetAdditionalCapabilities,
            >,
        >,
        /// The Password which should be used for the local-administrator on this Virtual Machine. Changing this forces a new resource to be created.
        ///
        /// > **Note:** When an `admin_password` is specified `disable_password_authentication` must be set to `false`.
        ///
        /// > **Note:** One of either `admin_password` or `admin_ssh_key` must be specified.
        #[builder(into, default)]
        pub admin_password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `admin_ssh_key` blocks as defined below.
        ///
        /// > **Note:** One of either `admin_password` or `admin_ssh_key` must be specified.
        #[builder(into, default)]
        pub admin_ssh_keys: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::compute::LinuxVirtualMachineScaleSetAdminSshKey>,
            >,
        >,
        /// The username of the local administrator on each Virtual Machine Scale Set instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub admin_username: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `automatic_instance_repair` block as defined below. To enable the automatic instance repair, this Virtual Machine Scale Set must have a valid `health_probe_id` or an [Application Health Extension](https://docs.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-health-extension).
        ///
        /// > **Note:** For more information about Automatic Instance Repair, please refer to the [product documentation](https://docs.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-automatic-instance-repairs).
        #[builder(into, default)]
        pub automatic_instance_repair: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::LinuxVirtualMachineScaleSetAutomaticInstanceRepair,
            >,
        >,
        /// An `automatic_os_upgrade_policy` block as defined below. This can only be specified when `upgrade_mode` is set to either `Automatic` or `Rolling`.
        #[builder(into, default)]
        pub automatic_os_upgrade_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::LinuxVirtualMachineScaleSetAutomaticOsUpgradePolicy,
            >,
        >,
        /// A `boot_diagnostics` block as defined below.
        #[builder(into, default)]
        pub boot_diagnostics: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::LinuxVirtualMachineScaleSetBootDiagnostics,
            >,
        >,
        /// Specifies the ID of the Capacity Reservation Group which the Virtual Machine Scale Set should be allocated to. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `capacity_reservation_group_id` cannot be used with `proximity_placement_group_id`
        ///
        /// > **Note:** `single_placement_group` must be set to `false` when `capacity_reservation_group_id` is specified.
        #[builder(into, default)]
        pub capacity_reservation_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The prefix which should be used for the name of the Virtual Machines in this Scale Set. If unspecified this defaults to the value for the `name` field. If the value of the `name` field is not a valid `computer_name_prefix`, then you must specify `computer_name_prefix`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub computer_name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Base64-Encoded Custom Data which should be used for this Virtual Machine Scale Set.
        ///
        /// > **Note:** When Custom Data has been configured, it's not possible to remove it without tainting the Virtual Machine Scale Set, due to a limitation of the Azure API.
        #[builder(into, default)]
        pub custom_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `data_disk` blocks as defined below.
        #[builder(into, default)]
        pub data_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::compute::LinuxVirtualMachineScaleSetDataDisk>,
            >,
        >,
        /// Should Password Authentication be disabled on this Virtual Machine Scale Set? Defaults to `true`.
        ///
        /// > In general we'd recommend using SSH Keys for authentication rather than Passwords - but there's tradeoff's to each - please [see this thread for more information](https://security.stackexchange.com/questions/69407/why-is-using-an-ssh-key-more-secure-than-using-passwords).
        ///
        /// > **Note:** When a `admin_password` is specified `disable_password_authentication` must be set to `false`.
        #[builder(into, default)]
        pub disable_password_authentication: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Should Virtual Machine Extensions be run on Overprovisioned Virtual Machines in the Scale Set? Defaults to `false`.
        #[builder(into, default)]
        pub do_not_run_extensions_on_overprovisioned_machines: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the Edge Zone within the Azure Region where this Linux Virtual Machine Scale Set should exist. Changing this forces a new Linux Virtual Machine Scale Set to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should all of the disks (including the temp disk) attached to this Virtual Machine be encrypted by enabling Encryption at Host?
        #[builder(into, default)]
        pub encryption_at_host_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the eviction policy for Virtual Machines in this Scale Set. Possible values are `Deallocate` and `Delete`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This can only be configured when `priority` is set to `Spot`.
        #[builder(into, default)]
        pub eviction_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should extension operations be allowed on the Virtual Machine Scale Set? Possible values are `true` or `false`. Defaults to `true`. Changing this forces a new Linux Virtual Machine Scale Set to be created.
        ///
        /// > **Note:** `extension_operations_enabled` may only be set to `false` if there are no extensions defined in the `extension` field.
        #[builder(into, default)]
        pub extension_operations_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// One or more `extension` blocks as defined below
        #[builder(into, default)]
        pub extensions: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::compute::LinuxVirtualMachineScaleSetExtension>,
            >,
        >,
        /// Specifies the duration allocated for all extensions to start. The time duration should be between `15` minutes and `120` minutes (inclusive) and should be specified in ISO 8601 format. Defaults to `PT1H30M`.
        #[builder(into, default)]
        pub extensions_time_budget: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `gallery_application` blocks as defined below.
        #[builder(into, default)]
        pub gallery_applications: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::compute::LinuxVirtualMachineScaleSetGalleryApplication,
                >,
            >,
        >,
        /// The ID of a Load Balancer Probe which should be used to determine the health of an instance. This is Required and can only be specified when `upgrade_mode` is set to `Automatic` or `Rolling`.
        #[builder(into, default)]
        pub health_probe_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the dedicated host group that the virtual machine scale set resides in. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub host_group_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::LinuxVirtualMachineScaleSetIdentity>,
        >,
        /// The number of Virtual Machines in the Scale Set. Defaults to `0`.
        ///
        /// > **NOTE:** If you're using AutoScaling, you may wish to use [`Ignore Changes` functionality](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) to ignore changes to this field.
        #[builder(into, default)]
        pub instances: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The Azure location where the Linux Virtual Machine Scale Set should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum price you're willing to pay for each Virtual Machine in this Scale Set, in US Dollars; which must be greater than the current spot price. If this bid price falls below the current spot price the Virtual Machines in the Scale Set will be evicted using the `eviction_policy`. Defaults to `-1`, which means that each Virtual Machine in this Scale Set should not be evicted for price reasons.
        ///
        /// > **Note:** This can only be configured when `priority` is set to `Spot`.
        #[builder(into, default)]
        pub max_bid_price: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// The name of the Linux Virtual Machine Scale Set. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `network_interface` blocks as defined below.
        #[builder(into)]
        pub network_interfaces: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::types::compute::LinuxVirtualMachineScaleSetNetworkInterface,
            >,
        >,
        /// An `os_disk` block as defined below.
        #[builder(into)]
        pub os_disk: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::LinuxVirtualMachineScaleSetOsDisk,
        >,
        /// Should Azure over-provision Virtual Machines in this Scale Set? This means that multiple Virtual Machines will be provisioned and Azure will keep the instances which become available first - which improves provisioning success rates and improves deployment time. You're not billed for these over-provisioned VM's and they don't count towards the Subscription Quota. Defaults to `true`.
        #[builder(into, default)]
        pub overprovision: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `plan` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **Note:** When using an image from Azure Marketplace a `plan` must be specified.
        #[builder(into, default)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::LinuxVirtualMachineScaleSetPlan>,
        >,
        /// Specifies the number of fault domains that are used by this Linux Virtual Machine Scale Set. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub platform_fault_domain_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The Priority of this Virtual Machine Scale Set. Possible values are `Regular` and `Spot`. Defaults to `Regular`. Changing this value forces a new resource.
        ///
        /// > **Note:** When `priority` is set to `Spot` an `eviction_policy` must be specified.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the Azure VM Agent be provisioned on each Virtual Machine in the Scale Set? Defaults to `true`. Changing this value forces a new resource to be created.
        #[builder(into, default)]
        pub provision_vm_agent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Proximity Placement Group in which the Virtual Machine Scale Set should be assigned to. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub proximity_placement_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the Resource Group in which the Linux Virtual Machine Scale Set should be exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `rolling_upgrade_policy` block as defined below. This is Required and can only be specified when `upgrade_mode` is set to `Automatic` or `Rolling`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub rolling_upgrade_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::LinuxVirtualMachineScaleSetRollingUpgradePolicy,
            >,
        >,
        /// A `scale_in` block as defined below.
        #[builder(into, default)]
        pub scale_in: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::LinuxVirtualMachineScaleSetScaleIn>,
        >,
        /// One or more `secret` blocks as defined below.
        #[builder(into, default)]
        pub secrets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::LinuxVirtualMachineScaleSetSecret>>,
        >,
        /// Specifies whether secure boot should be enabled on the virtual machine. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub secure_boot_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should this Virtual Machine Scale Set be limited to a Single Placement Group, which means the number of instances will be capped at 100 Virtual Machines. Defaults to `true`.
        #[builder(into, default)]
        pub single_placement_group: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Virtual Machine SKU for the Scale Set, such as `Standard_F2`.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of an Image which each Virtual Machine in this Scale Set should be based on. Possible Image ID types include `Image ID`, `Shared Image ID`, `Shared Image Version ID`, `Community Gallery Image ID`, `Community Gallery Image Version ID`, `Shared Gallery Image ID` and `Shared Gallery Image Version ID`.
        ///
        /// > **Note:** One of either `source_image_id` or `source_image_reference` must be set.
        #[builder(into, default)]
        pub source_image_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `source_image_reference` block as defined below.
        ///
        /// > **Note:** One of either `source_image_id` or `source_image_reference` must be set.
        #[builder(into, default)]
        pub source_image_reference: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::LinuxVirtualMachineScaleSetSourceImageReference,
            >,
        >,
        /// A `spot_restore` block as defined below.
        #[builder(into, default)]
        pub spot_restore: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::LinuxVirtualMachineScaleSetSpotRestore>,
        >,
        /// A mapping of tags which should be assigned to this Virtual Machine Scale Set.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `termination_notification` block as defined below.
        #[builder(into, default)]
        pub termination_notification: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::LinuxVirtualMachineScaleSetTerminationNotification,
            >,
        >,
        #[builder(into, default)]
        pub upgrade_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Base64-Encoded User Data which should be used for this Virtual Machine Scale Set.
        #[builder(into, default)]
        pub user_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether vTPM should be enabled on the virtual machine. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub vtpm_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should the Virtual Machines in this Scale Set be strictly evenly distributed across Availability Zones? Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This can only be set to `true` when one or more `zones` are configured.
        #[builder(into, default)]
        pub zone_balance: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies a list of Availability Zones in which this Linux Virtual Machine Scale Set should be located.
        ///
        /// > **Note:** Updating `zones` to remove an existing zone forces a new Virtual Machine Scale Set to be created.
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct LinuxVirtualMachineScaleSetResult {
        /// An `additional_capabilities` block as defined below.
        pub additional_capabilities: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::LinuxVirtualMachineScaleSetAdditionalCapabilities,
            >,
        >,
        /// The Password which should be used for the local-administrator on this Virtual Machine. Changing this forces a new resource to be created.
        ///
        /// > **Note:** When an `admin_password` is specified `disable_password_authentication` must be set to `false`.
        ///
        /// > **Note:** One of either `admin_password` or `admin_ssh_key` must be specified.
        pub admin_password: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `admin_ssh_key` blocks as defined below.
        ///
        /// > **Note:** One of either `admin_password` or `admin_ssh_key` must be specified.
        pub admin_ssh_keys: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::compute::LinuxVirtualMachineScaleSetAdminSshKey>,
            >,
        >,
        /// The username of the local administrator on each Virtual Machine Scale Set instance. Changing this forces a new resource to be created.
        pub admin_username: pulumi_gestalt_rust::Output<String>,
        /// An `automatic_instance_repair` block as defined below. To enable the automatic instance repair, this Virtual Machine Scale Set must have a valid `health_probe_id` or an [Application Health Extension](https://docs.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-health-extension).
        ///
        /// > **Note:** For more information about Automatic Instance Repair, please refer to the [product documentation](https://docs.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-automatic-instance-repairs).
        pub automatic_instance_repair: pulumi_gestalt_rust::Output<
            super::super::types::compute::LinuxVirtualMachineScaleSetAutomaticInstanceRepair,
        >,
        /// An `automatic_os_upgrade_policy` block as defined below. This can only be specified when `upgrade_mode` is set to either `Automatic` or `Rolling`.
        pub automatic_os_upgrade_policy: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::LinuxVirtualMachineScaleSetAutomaticOsUpgradePolicy,
            >,
        >,
        /// A `boot_diagnostics` block as defined below.
        pub boot_diagnostics: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::LinuxVirtualMachineScaleSetBootDiagnostics,
            >,
        >,
        /// Specifies the ID of the Capacity Reservation Group which the Virtual Machine Scale Set should be allocated to. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `capacity_reservation_group_id` cannot be used with `proximity_placement_group_id`
        ///
        /// > **Note:** `single_placement_group` must be set to `false` when `capacity_reservation_group_id` is specified.
        pub capacity_reservation_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The prefix which should be used for the name of the Virtual Machines in this Scale Set. If unspecified this defaults to the value for the `name` field. If the value of the `name` field is not a valid `computer_name_prefix`, then you must specify `computer_name_prefix`. Changing this forces a new resource to be created.
        pub computer_name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The Base64-Encoded Custom Data which should be used for this Virtual Machine Scale Set.
        ///
        /// > **Note:** When Custom Data has been configured, it's not possible to remove it without tainting the Virtual Machine Scale Set, due to a limitation of the Azure API.
        pub custom_data: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `data_disk` blocks as defined below.
        pub data_disks: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::compute::LinuxVirtualMachineScaleSetDataDisk>,
            >,
        >,
        /// Should Password Authentication be disabled on this Virtual Machine Scale Set? Defaults to `true`.
        ///
        /// > In general we'd recommend using SSH Keys for authentication rather than Passwords - but there's tradeoff's to each - please [see this thread for more information](https://security.stackexchange.com/questions/69407/why-is-using-an-ssh-key-more-secure-than-using-passwords).
        ///
        /// > **Note:** When a `admin_password` is specified `disable_password_authentication` must be set to `false`.
        pub disable_password_authentication: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should Virtual Machine Extensions be run on Overprovisioned Virtual Machines in the Scale Set? Defaults to `false`.
        pub do_not_run_extensions_on_overprovisioned_machines: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Specifies the Edge Zone within the Azure Region where this Linux Virtual Machine Scale Set should exist. Changing this forces a new Linux Virtual Machine Scale Set to be created.
        pub edge_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should all of the disks (including the temp disk) attached to this Virtual Machine be encrypted by enabling Encryption at Host?
        pub encryption_at_host_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the eviction policy for Virtual Machines in this Scale Set. Possible values are `Deallocate` and `Delete`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This can only be configured when `priority` is set to `Spot`.
        pub eviction_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should extension operations be allowed on the Virtual Machine Scale Set? Possible values are `true` or `false`. Defaults to `true`. Changing this forces a new Linux Virtual Machine Scale Set to be created.
        ///
        /// > **Note:** `extension_operations_enabled` may only be set to `false` if there are no extensions defined in the `extension` field.
        pub extension_operations_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// One or more `extension` blocks as defined below
        pub extensions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::LinuxVirtualMachineScaleSetExtension>,
        >,
        /// Specifies the duration allocated for all extensions to start. The time duration should be between `15` minutes and `120` minutes (inclusive) and should be specified in ISO 8601 format. Defaults to `PT1H30M`.
        pub extensions_time_budget: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `gallery_application` blocks as defined below.
        pub gallery_applications: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::LinuxVirtualMachineScaleSetGalleryApplication,
                >,
            >,
        >,
        /// The ID of a Load Balancer Probe which should be used to determine the health of an instance. This is Required and can only be specified when `upgrade_mode` is set to `Automatic` or `Rolling`.
        pub health_probe_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the ID of the dedicated host group that the virtual machine scale set resides in. Changing this forces a new resource to be created.
        pub host_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::LinuxVirtualMachineScaleSetIdentity>,
        >,
        /// The number of Virtual Machines in the Scale Set. Defaults to `0`.
        ///
        /// > **NOTE:** If you're using AutoScaling, you may wish to use [`Ignore Changes` functionality](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) to ignore changes to this field.
        pub instances: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The Azure location where the Linux Virtual Machine Scale Set should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The maximum price you're willing to pay for each Virtual Machine in this Scale Set, in US Dollars; which must be greater than the current spot price. If this bid price falls below the current spot price the Virtual Machines in the Scale Set will be evicted using the `eviction_policy`. Defaults to `-1`, which means that each Virtual Machine in this Scale Set should not be evicted for price reasons.
        ///
        /// > **Note:** This can only be configured when `priority` is set to `Spot`.
        pub max_bid_price: pulumi_gestalt_rust::Output<Option<f64>>,
        /// The name of the Linux Virtual Machine Scale Set. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `network_interface` blocks as defined below.
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::compute::LinuxVirtualMachineScaleSetNetworkInterface,
            >,
        >,
        /// An `os_disk` block as defined below.
        pub os_disk: pulumi_gestalt_rust::Output<
            super::super::types::compute::LinuxVirtualMachineScaleSetOsDisk,
        >,
        /// Should Azure over-provision Virtual Machines in this Scale Set? This means that multiple Virtual Machines will be provisioned and Azure will keep the instances which become available first - which improves provisioning success rates and improves deployment time. You're not billed for these over-provisioned VM's and they don't count towards the Subscription Quota. Defaults to `true`.
        pub overprovision: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `plan` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **Note:** When using an image from Azure Marketplace a `plan` must be specified.
        pub plan: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::LinuxVirtualMachineScaleSetPlan>,
        >,
        /// Specifies the number of fault domains that are used by this Linux Virtual Machine Scale Set. Changing this forces a new resource to be created.
        pub platform_fault_domain_count: pulumi_gestalt_rust::Output<i32>,
        /// The Priority of this Virtual Machine Scale Set. Possible values are `Regular` and `Spot`. Defaults to `Regular`. Changing this value forces a new resource.
        ///
        /// > **Note:** When `priority` is set to `Spot` an `eviction_policy` must be specified.
        pub priority: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should the Azure VM Agent be provisioned on each Virtual Machine in the Scale Set? Defaults to `true`. Changing this value forces a new resource to be created.
        pub provision_vm_agent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Proximity Placement Group in which the Virtual Machine Scale Set should be assigned to. Changing this forces a new resource to be created.
        pub proximity_placement_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the Linux Virtual Machine Scale Set should be exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `rolling_upgrade_policy` block as defined below. This is Required and can only be specified when `upgrade_mode` is set to `Automatic` or `Rolling`. Changing this forces a new resource to be created.
        pub rolling_upgrade_policy: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::LinuxVirtualMachineScaleSetRollingUpgradePolicy,
            >,
        >,
        /// A `scale_in` block as defined below.
        pub scale_in: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::LinuxVirtualMachineScaleSetScaleIn>,
        >,
        /// One or more `secret` blocks as defined below.
        pub secrets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::LinuxVirtualMachineScaleSetSecret>>,
        >,
        /// Specifies whether secure boot should be enabled on the virtual machine. Changing this forces a new resource to be created.
        pub secure_boot_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should this Virtual Machine Scale Set be limited to a Single Placement Group, which means the number of instances will be capped at 100 Virtual Machines. Defaults to `true`.
        pub single_placement_group: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Virtual Machine SKU for the Scale Set, such as `Standard_F2`.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// The ID of an Image which each Virtual Machine in this Scale Set should be based on. Possible Image ID types include `Image ID`, `Shared Image ID`, `Shared Image Version ID`, `Community Gallery Image ID`, `Community Gallery Image Version ID`, `Shared Gallery Image ID` and `Shared Gallery Image Version ID`.
        ///
        /// > **Note:** One of either `source_image_id` or `source_image_reference` must be set.
        pub source_image_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `source_image_reference` block as defined below.
        ///
        /// > **Note:** One of either `source_image_id` or `source_image_reference` must be set.
        pub source_image_reference: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::LinuxVirtualMachineScaleSetSourceImageReference,
            >,
        >,
        /// A `spot_restore` block as defined below.
        pub spot_restore: pulumi_gestalt_rust::Output<
            super::super::types::compute::LinuxVirtualMachineScaleSetSpotRestore,
        >,
        /// A mapping of tags which should be assigned to this Virtual Machine Scale Set.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `termination_notification` block as defined below.
        pub termination_notification: pulumi_gestalt_rust::Output<
            super::super::types::compute::LinuxVirtualMachineScaleSetTerminationNotification,
        >,
        /// The Unique ID for this Linux Virtual Machine Scale Set.
        pub unique_id: pulumi_gestalt_rust::Output<String>,
        pub upgrade_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Base64-Encoded User Data which should be used for this Virtual Machine Scale Set.
        pub user_data: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether vTPM should be enabled on the virtual machine. Changing this forces a new resource to be created.
        pub vtpm_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should the Virtual Machines in this Scale Set be strictly evenly distributed across Availability Zones? Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This can only be set to `true` when one or more `zones` are configured.
        pub zone_balance: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies a list of Availability Zones in which this Linux Virtual Machine Scale Set should be located.
        ///
        /// > **Note:** Updating `zones` to remove an existing zone forces a new Virtual Machine Scale Set to be created.
        pub zones: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LinuxVirtualMachineScaleSetArgs,
    ) -> LinuxVirtualMachineScaleSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_capabilities_binding = args
            .additional_capabilities
            .get_output(context);
        let admin_password_binding = args.admin_password.get_output(context);
        let admin_ssh_keys_binding = args.admin_ssh_keys.get_output(context);
        let admin_username_binding = args.admin_username.get_output(context);
        let automatic_instance_repair_binding = args
            .automatic_instance_repair
            .get_output(context);
        let automatic_os_upgrade_policy_binding = args
            .automatic_os_upgrade_policy
            .get_output(context);
        let boot_diagnostics_binding = args.boot_diagnostics.get_output(context);
        let capacity_reservation_group_id_binding = args
            .capacity_reservation_group_id
            .get_output(context);
        let computer_name_prefix_binding = args.computer_name_prefix.get_output(context);
        let custom_data_binding = args.custom_data.get_output(context);
        let data_disks_binding = args.data_disks.get_output(context);
        let disable_password_authentication_binding = args
            .disable_password_authentication
            .get_output(context);
        let do_not_run_extensions_on_overprovisioned_machines_binding = args
            .do_not_run_extensions_on_overprovisioned_machines
            .get_output(context);
        let edge_zone_binding = args.edge_zone.get_output(context);
        let encryption_at_host_enabled_binding = args
            .encryption_at_host_enabled
            .get_output(context);
        let eviction_policy_binding = args.eviction_policy.get_output(context);
        let extension_operations_enabled_binding = args
            .extension_operations_enabled
            .get_output(context);
        let extensions_binding = args.extensions.get_output(context);
        let extensions_time_budget_binding = args
            .extensions_time_budget
            .get_output(context);
        let gallery_applications_binding = args.gallery_applications.get_output(context);
        let health_probe_id_binding = args.health_probe_id.get_output(context);
        let host_group_id_binding = args.host_group_id.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let instances_binding = args.instances.get_output(context);
        let location_binding = args.location.get_output(context);
        let max_bid_price_binding = args.max_bid_price.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_interfaces_binding = args.network_interfaces.get_output(context);
        let os_disk_binding = args.os_disk.get_output(context);
        let overprovision_binding = args.overprovision.get_output(context);
        let plan_binding = args.plan.get_output(context);
        let platform_fault_domain_count_binding = args
            .platform_fault_domain_count
            .get_output(context);
        let priority_binding = args.priority.get_output(context);
        let provision_vm_agent_binding = args.provision_vm_agent.get_output(context);
        let proximity_placement_group_id_binding = args
            .proximity_placement_group_id
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let rolling_upgrade_policy_binding = args
            .rolling_upgrade_policy
            .get_output(context);
        let scale_in_binding = args.scale_in.get_output(context);
        let secrets_binding = args.secrets.get_output(context);
        let secure_boot_enabled_binding = args.secure_boot_enabled.get_output(context);
        let single_placement_group_binding = args
            .single_placement_group
            .get_output(context);
        let sku_binding = args.sku.get_output(context);
        let source_image_id_binding = args.source_image_id.get_output(context);
        let source_image_reference_binding = args
            .source_image_reference
            .get_output(context);
        let spot_restore_binding = args.spot_restore.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let termination_notification_binding = args
            .termination_notification
            .get_output(context);
        let upgrade_mode_binding = args.upgrade_mode.get_output(context);
        let user_data_binding = args.user_data.get_output(context);
        let vtpm_enabled_binding = args.vtpm_enabled.get_output(context);
        let zone_balance_binding = args.zone_balance.get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/linuxVirtualMachineScaleSet:LinuxVirtualMachineScaleSet"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalCapabilities".into(),
                    value: &additional_capabilities_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminPassword".into(),
                    value: &admin_password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminSshKeys".into(),
                    value: &admin_ssh_keys_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminUsername".into(),
                    value: &admin_username_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticInstanceRepair".into(),
                    value: &automatic_instance_repair_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticOsUpgradePolicy".into(),
                    value: &automatic_os_upgrade_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bootDiagnostics".into(),
                    value: &boot_diagnostics_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacityReservationGroupId".into(),
                    value: &capacity_reservation_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computerNamePrefix".into(),
                    value: &computer_name_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customData".into(),
                    value: &custom_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataDisks".into(),
                    value: &data_disks_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disablePasswordAuthentication".into(),
                    value: &disable_password_authentication_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "doNotRunExtensionsOnOverprovisionedMachines".into(),
                    value: &do_not_run_extensions_on_overprovisioned_machines_binding
                        .drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "edgeZone".into(),
                    value: &edge_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionAtHostEnabled".into(),
                    value: &encryption_at_host_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "evictionPolicy".into(),
                    value: &eviction_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extensionOperationsEnabled".into(),
                    value: &extension_operations_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extensions".into(),
                    value: &extensions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extensionsTimeBudget".into(),
                    value: &extensions_time_budget_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "galleryApplications".into(),
                    value: &gallery_applications_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthProbeId".into(),
                    value: &health_probe_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostGroupId".into(),
                    value: &host_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instances".into(),
                    value: &instances_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxBidPrice".into(),
                    value: &max_bid_price_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaces".into(),
                    value: &network_interfaces_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osDisk".into(),
                    value: &os_disk_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "overprovision".into(),
                    value: &overprovision_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plan".into(),
                    value: &plan_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "platformFaultDomainCount".into(),
                    value: &platform_fault_domain_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: &priority_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "provisionVmAgent".into(),
                    value: &provision_vm_agent_binding.drop_type(),
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
                    name: "rollingUpgradePolicy".into(),
                    value: &rolling_upgrade_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scaleIn".into(),
                    value: &scale_in_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secrets".into(),
                    value: &secrets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secureBootEnabled".into(),
                    value: &secure_boot_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "singlePlacementGroup".into(),
                    value: &single_placement_group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceImageId".into(),
                    value: &source_image_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceImageReference".into(),
                    value: &source_image_reference_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spotRestore".into(),
                    value: &spot_restore_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "terminationNotification".into(),
                    value: &termination_notification_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "upgradeMode".into(),
                    value: &upgrade_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userData".into(),
                    value: &user_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vtpmEnabled".into(),
                    value: &vtpm_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneBalance".into(),
                    value: &zone_balance_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: &zones_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LinuxVirtualMachineScaleSetResult {
            additional_capabilities: o.get_field("additionalCapabilities"),
            admin_password: o.get_field("adminPassword"),
            admin_ssh_keys: o.get_field("adminSshKeys"),
            admin_username: o.get_field("adminUsername"),
            automatic_instance_repair: o.get_field("automaticInstanceRepair"),
            automatic_os_upgrade_policy: o.get_field("automaticOsUpgradePolicy"),
            boot_diagnostics: o.get_field("bootDiagnostics"),
            capacity_reservation_group_id: o.get_field("capacityReservationGroupId"),
            computer_name_prefix: o.get_field("computerNamePrefix"),
            custom_data: o.get_field("customData"),
            data_disks: o.get_field("dataDisks"),
            disable_password_authentication: o
                .get_field("disablePasswordAuthentication"),
            do_not_run_extensions_on_overprovisioned_machines: o
                .get_field("doNotRunExtensionsOnOverprovisionedMachines"),
            edge_zone: o.get_field("edgeZone"),
            encryption_at_host_enabled: o.get_field("encryptionAtHostEnabled"),
            eviction_policy: o.get_field("evictionPolicy"),
            extension_operations_enabled: o.get_field("extensionOperationsEnabled"),
            extensions: o.get_field("extensions"),
            extensions_time_budget: o.get_field("extensionsTimeBudget"),
            gallery_applications: o.get_field("galleryApplications"),
            health_probe_id: o.get_field("healthProbeId"),
            host_group_id: o.get_field("hostGroupId"),
            identity: o.get_field("identity"),
            instances: o.get_field("instances"),
            location: o.get_field("location"),
            max_bid_price: o.get_field("maxBidPrice"),
            name: o.get_field("name"),
            network_interfaces: o.get_field("networkInterfaces"),
            os_disk: o.get_field("osDisk"),
            overprovision: o.get_field("overprovision"),
            plan: o.get_field("plan"),
            platform_fault_domain_count: o.get_field("platformFaultDomainCount"),
            priority: o.get_field("priority"),
            provision_vm_agent: o.get_field("provisionVmAgent"),
            proximity_placement_group_id: o.get_field("proximityPlacementGroupId"),
            resource_group_name: o.get_field("resourceGroupName"),
            rolling_upgrade_policy: o.get_field("rollingUpgradePolicy"),
            scale_in: o.get_field("scaleIn"),
            secrets: o.get_field("secrets"),
            secure_boot_enabled: o.get_field("secureBootEnabled"),
            single_placement_group: o.get_field("singlePlacementGroup"),
            sku: o.get_field("sku"),
            source_image_id: o.get_field("sourceImageId"),
            source_image_reference: o.get_field("sourceImageReference"),
            spot_restore: o.get_field("spotRestore"),
            tags: o.get_field("tags"),
            termination_notification: o.get_field("terminationNotification"),
            unique_id: o.get_field("uniqueId"),
            upgrade_mode: o.get_field("upgradeMode"),
            user_data: o.get_field("userData"),
            vtpm_enabled: o.get_field("vtpmEnabled"),
            zone_balance: o.get_field("zoneBalance"),
            zones: o.get_field("zones"),
        }
    }
}
