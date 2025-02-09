/// Manages an Virtual Machine Scale Set in Flexible Orchestration Mode.
///
/// ## Disclaimers
///
/// > **Note:** As of the **v2.86.0** (November 19, 2021) release of the provider this resource will only create Virtual Machine Scale Sets with the **Flexible** Orchestration Mode.
///
/// > **Note:** All arguments including the administrator login and password will be stored in the raw state as plain-text. [Read more about sensitive data in state](https://www.terraform.io/docs/state/sensitive-data.html).
///
/// ## Example Usage
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
///     let exampleOrchestratedVirtualMachineScaleSet = orchestrated_virtual_machine_scale_set::create(
///         "exampleOrchestratedVirtualMachineScaleSet",
///         OrchestratedVirtualMachineScaleSetArgs::builder()
///             .location("${example.location}")
///             .name("example-VMSS")
///             .platform_fault_domain_count(1)
///             .resource_group_name("${example.name}")
///             .zones(vec!["1",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// An Virtual Machine Scale Set can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/orchestratedVirtualMachineScaleSet:OrchestratedVirtualMachineScaleSet example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Compute/virtualMachineScaleSets/scaleset1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod orchestrated_virtual_machine_scale_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrchestratedVirtualMachineScaleSetArgs {
        /// An `additional_capabilities` block as defined below.
        #[builder(into, default)]
        pub additional_capabilities: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetAdditionalCapabilities,
            >,
        >,
        /// An `automatic_instance_repair` block as defined below.
        ///
        /// > **Note:** To enable the `automatic_instance_repair`, the Orchestrated Virtual Machine Scale Set must have a valid [Application Health Extension](https://docs.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-health-extension).
        #[builder(into, default)]
        pub automatic_instance_repair: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetAutomaticInstanceRepair,
            >,
        >,
        /// A `boot_diagnostics` block as defined below.
        #[builder(into, default)]
        pub boot_diagnostics: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetBootDiagnostics,
            >,
        >,
        /// Specifies the ID of the Capacity Reservation Group which the Virtual Machine Scale Set should be allocated to. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `capacity_reservation_group_id` cannot be specified with `proximity_placement_group_id`
        ///
        /// > **Note:** If `capacity_reservation_group_id` is specified the `single_placement_group` must be set to `false`.
        #[builder(into, default)]
        pub capacity_reservation_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// One or more `data_disk` blocks as defined below.
        #[builder(into, default)]
        pub data_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::compute::OrchestratedVirtualMachineScaleSetDataDisk,
                >,
            >,
        >,
        /// Should disks attached to this Virtual Machine Scale Set be encrypted by enabling Encryption at Host?
        #[builder(into, default)]
        pub encryption_at_host_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Policy which should be used by Spot Virtual Machines that are Evicted from the Scale Set. Possible values are `Deallocate` and `Delete`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub eviction_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should extension operations be allowed on the Virtual Machine Scale Set? Possible values are `true` or `false`. Defaults to `true`. Changing this forces a new Virtual Machine Scale Set to be created.
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
                Vec<
                    super::super::types::compute::OrchestratedVirtualMachineScaleSetExtension,
                >,
            >,
        >,
        /// Specifies the time alloted for all extensions to start. The time duration should be between 15 minutes and 120 minutes (inclusive) and should be specified in ISO 8601 format. Defaults to `PT1H30M`.
        #[builder(into, default)]
        pub extensions_time_budget: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetIdentity,
            >,
        >,
        /// The number of Virtual Machines in the Virtual Machine Scale Set.
        #[builder(into, default)]
        pub instances: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the type of on-premise license (also known as Azure Hybrid Use Benefit) which should be used for this Virtual Machine Scale Set. Possible values are `None`, `Windows_Client` and `Windows_Server`.
        #[builder(into, default)]
        pub license_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure location where the Virtual Machine Scale Set should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum price you're willing to pay for each Virtual Machine in this Scale Set, in US Dollars; which must be greater than the current spot price. If this bid price falls below the current spot price the Virtual Machines in the Scale Set will be evicted using the eviction_policy. Defaults to `-1`, which means that each Virtual Machine in the Scale Set should not be evicted for price reasons.
        #[builder(into, default)]
        pub max_bid_price: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// The name of the Virtual Machine Scale Set. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `network_interface` blocks as defined below.
        #[builder(into, default)]
        pub network_interfaces: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::compute::OrchestratedVirtualMachineScaleSetNetworkInterface,
                >,
            >,
        >,
        /// An `os_disk` block as defined below.
        #[builder(into, default)]
        pub os_disk: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetOsDisk,
            >,
        >,
        /// An `os_profile` block as defined below.
        #[builder(into, default)]
        pub os_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfile,
            >,
        >,
        /// A `plan` block as documented below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::OrchestratedVirtualMachineScaleSetPlan>,
        >,
        /// Specifies the number of fault domains that are used by this Virtual Machine Scale Set. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The number of Fault Domains varies depending on which Azure Region you're using. More information about update and fault domains and how they work can be found [here](https://learn.microsoft.com/en-us/azure/virtual-machines/availability-set-overview).
        #[builder(into)]
        pub platform_fault_domain_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The Priority of this Virtual Machine Scale Set. Possible values are `Regular` and `Spot`. Defaults to `Regular`. Changing this value forces a new resource.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// a `priority_mix` block as defined below
        #[builder(into, default)]
        pub priority_mix: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetPriorityMix,
            >,
        >,
        /// The ID of the Proximity Placement Group which the Virtual Machine should be assigned to. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub proximity_placement_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the Resource Group in which the Virtual Machine Scale Set should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Should this Virtual Machine Scale Set be limited to a Single Placement Group, which means the number of instances will be capped at 100 Virtual Machines. Possible values are `true` or `false`.
        ///
        /// > **Note:** `single_placement_group` behaves differently for Flexible orchestration Virtual Machine Scale Sets than it does for Uniform orchestration Virtual Machine Scale Sets. It is recommended that you do not define the `single_placement_group` field in your configuration file as the service will determine what this value should be based off of the value contained within the `sku_name` field of your configuration file. You may set the `single_placement_group` field to `true`, however once you set it to `false` you will not be able to revert it back to `true`.
        #[builder(into, default)]
        pub single_placement_group: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The `name` of the SKU to be used by this Virtual Machine Scale Set. Valid values include: any of the [General purpose](https://docs.microsoft.com/azure/virtual-machines/sizes-general), [Compute optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-compute), [Memory optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-memory), [Storage optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-storage), [GPU optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-gpu), [FPGA optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-field-programmable-gate-arrays), [High performance](https://docs.microsoft.com/azure/virtual-machines/sizes-hpc), or [Previous generation](https://docs.microsoft.com/azure/virtual-machines/sizes-previous-gen) virtual machine SKUs.
        #[builder(into, default)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `sku_profile` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **Note:** If `sku_profile` is specified the `sku_name` must be set to `Mix`.
        #[builder(into, default)]
        pub sku_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetSkuProfile,
            >,
        >,
        /// The ID of an Image which each Virtual Machine in this Scale Set should be based on. Possible Image ID types include `Image ID`s, `Shared Image ID`s, `Shared Image Version ID`s, `Community Gallery Image ID`s, `Community Gallery Image Version ID`s, `Shared Gallery Image ID`s and `Shared Gallery Image Version ID`s.
        #[builder(into, default)]
        pub source_image_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `source_image_reference` block as defined below.
        #[builder(into, default)]
        pub source_image_reference: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetSourceImageReference,
            >,
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
                super::super::types::compute::OrchestratedVirtualMachineScaleSetTerminationNotification,
            >,
        >,
        /// The Base64-Encoded User Data which should be used for this Virtual Machine Scale Set.
        #[builder(into, default)]
        pub user_data_base64: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the Virtual Machines in this Scale Set be strictly evenly distributed across Availability Zones? Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This can only be set to `true` when one or more `zones` are configured.
        #[builder(into, default)]
        pub zone_balance: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies a list of Availability Zones across which the Virtual Machine Scale Set will create instances.
        ///
        /// > **Note:** Updating `zones` to remove an existing zone forces a new Virtual Machine Scale Set to be created.
        ///
        /// > **Note:** Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct OrchestratedVirtualMachineScaleSetResult {
        /// An `additional_capabilities` block as defined below.
        pub additional_capabilities: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetAdditionalCapabilities,
            >,
        >,
        /// An `automatic_instance_repair` block as defined below.
        ///
        /// > **Note:** To enable the `automatic_instance_repair`, the Orchestrated Virtual Machine Scale Set must have a valid [Application Health Extension](https://docs.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-health-extension).
        pub automatic_instance_repair: pulumi_gestalt_rust::Output<
            super::super::types::compute::OrchestratedVirtualMachineScaleSetAutomaticInstanceRepair,
        >,
        /// A `boot_diagnostics` block as defined below.
        pub boot_diagnostics: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetBootDiagnostics,
            >,
        >,
        /// Specifies the ID of the Capacity Reservation Group which the Virtual Machine Scale Set should be allocated to. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `capacity_reservation_group_id` cannot be specified with `proximity_placement_group_id`
        ///
        /// > **Note:** If `capacity_reservation_group_id` is specified the `single_placement_group` must be set to `false`.
        pub capacity_reservation_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `data_disk` blocks as defined below.
        pub data_disks: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::OrchestratedVirtualMachineScaleSetDataDisk,
                >,
            >,
        >,
        /// Should disks attached to this Virtual Machine Scale Set be encrypted by enabling Encryption at Host?
        pub encryption_at_host_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Policy which should be used by Spot Virtual Machines that are Evicted from the Scale Set. Possible values are `Deallocate` and `Delete`. Changing this forces a new resource to be created.
        pub eviction_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should extension operations be allowed on the Virtual Machine Scale Set? Possible values are `true` or `false`. Defaults to `true`. Changing this forces a new Virtual Machine Scale Set to be created.
        ///
        /// > **Note:** `extension_operations_enabled` may only be set to `false` if there are no extensions defined in the `extension` field.
        pub extension_operations_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// One or more `extension` blocks as defined below
        pub extensions: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetExtension,
            >,
        >,
        /// Specifies the time alloted for all extensions to start. The time duration should be between 15 minutes and 120 minutes (inclusive) and should be specified in ISO 8601 format. Defaults to `PT1H30M`.
        pub extensions_time_budget: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetIdentity,
            >,
        >,
        /// The number of Virtual Machines in the Virtual Machine Scale Set.
        pub instances: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the type of on-premise license (also known as Azure Hybrid Use Benefit) which should be used for this Virtual Machine Scale Set. Possible values are `None`, `Windows_Client` and `Windows_Server`.
        pub license_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Azure location where the Virtual Machine Scale Set should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The maximum price you're willing to pay for each Virtual Machine in this Scale Set, in US Dollars; which must be greater than the current spot price. If this bid price falls below the current spot price the Virtual Machines in the Scale Set will be evicted using the eviction_policy. Defaults to `-1`, which means that each Virtual Machine in the Scale Set should not be evicted for price reasons.
        pub max_bid_price: pulumi_gestalt_rust::Output<Option<f64>>,
        /// The name of the Virtual Machine Scale Set. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `network_interface` blocks as defined below.
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::OrchestratedVirtualMachineScaleSetNetworkInterface,
                >,
            >,
        >,
        /// An `os_disk` block as defined below.
        pub os_disk: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetOsDisk,
            >,
        >,
        /// An `os_profile` block as defined below.
        pub os_profile: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfile,
            >,
        >,
        /// A `plan` block as documented below. Changing this forces a new resource to be created.
        pub plan: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::OrchestratedVirtualMachineScaleSetPlan>,
        >,
        /// Specifies the number of fault domains that are used by this Virtual Machine Scale Set. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The number of Fault Domains varies depending on which Azure Region you're using. More information about update and fault domains and how they work can be found [here](https://learn.microsoft.com/en-us/azure/virtual-machines/availability-set-overview).
        pub platform_fault_domain_count: pulumi_gestalt_rust::Output<i32>,
        /// The Priority of this Virtual Machine Scale Set. Possible values are `Regular` and `Spot`. Defaults to `Regular`. Changing this value forces a new resource.
        pub priority: pulumi_gestalt_rust::Output<Option<String>>,
        /// a `priority_mix` block as defined below
        pub priority_mix: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetPriorityMix,
            >,
        >,
        /// The ID of the Proximity Placement Group which the Virtual Machine should be assigned to. Changing this forces a new resource to be created.
        pub proximity_placement_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the Virtual Machine Scale Set should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Should this Virtual Machine Scale Set be limited to a Single Placement Group, which means the number of instances will be capped at 100 Virtual Machines. Possible values are `true` or `false`.
        ///
        /// > **Note:** `single_placement_group` behaves differently for Flexible orchestration Virtual Machine Scale Sets than it does for Uniform orchestration Virtual Machine Scale Sets. It is recommended that you do not define the `single_placement_group` field in your configuration file as the service will determine what this value should be based off of the value contained within the `sku_name` field of your configuration file. You may set the `single_placement_group` field to `true`, however once you set it to `false` you will not be able to revert it back to `true`.
        pub single_placement_group: pulumi_gestalt_rust::Output<bool>,
        /// The `name` of the SKU to be used by this Virtual Machine Scale Set. Valid values include: any of the [General purpose](https://docs.microsoft.com/azure/virtual-machines/sizes-general), [Compute optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-compute), [Memory optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-memory), [Storage optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-storage), [GPU optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-gpu), [FPGA optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-field-programmable-gate-arrays), [High performance](https://docs.microsoft.com/azure/virtual-machines/sizes-hpc), or [Previous generation](https://docs.microsoft.com/azure/virtual-machines/sizes-previous-gen) virtual machine SKUs.
        pub sku_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `sku_profile` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **Note:** If `sku_profile` is specified the `sku_name` must be set to `Mix`.
        pub sku_profile: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetSkuProfile,
            >,
        >,
        /// The ID of an Image which each Virtual Machine in this Scale Set should be based on. Possible Image ID types include `Image ID`s, `Shared Image ID`s, `Shared Image Version ID`s, `Community Gallery Image ID`s, `Community Gallery Image Version ID`s, `Shared Gallery Image ID`s and `Shared Gallery Image Version ID`s.
        pub source_image_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `source_image_reference` block as defined below.
        pub source_image_reference: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetSourceImageReference,
            >,
        >,
        /// A mapping of tags which should be assigned to this Virtual Machine Scale Set.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `termination_notification` block as defined below.
        pub termination_notification: pulumi_gestalt_rust::Output<
            super::super::types::compute::OrchestratedVirtualMachineScaleSetTerminationNotification,
        >,
        /// The Unique ID for the Virtual Machine Scale Set.
        pub unique_id: pulumi_gestalt_rust::Output<String>,
        /// The Base64-Encoded User Data which should be used for this Virtual Machine Scale Set.
        pub user_data_base64: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should the Virtual Machines in this Scale Set be strictly evenly distributed across Availability Zones? Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This can only be set to `true` when one or more `zones` are configured.
        pub zone_balance: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies a list of Availability Zones across which the Virtual Machine Scale Set will create instances.
        ///
        /// > **Note:** Updating `zones` to remove an existing zone forces a new Virtual Machine Scale Set to be created.
        ///
        /// > **Note:** Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        pub zones: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrchestratedVirtualMachineScaleSetArgs,
    ) -> OrchestratedVirtualMachineScaleSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_capabilities_binding = args
            .additional_capabilities
            .get_output(context);
        let automatic_instance_repair_binding = args
            .automatic_instance_repair
            .get_output(context);
        let boot_diagnostics_binding = args.boot_diagnostics.get_output(context);
        let capacity_reservation_group_id_binding = args
            .capacity_reservation_group_id
            .get_output(context);
        let data_disks_binding = args.data_disks.get_output(context);
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
        let identity_binding = args.identity.get_output(context);
        let instances_binding = args.instances.get_output(context);
        let license_type_binding = args.license_type.get_output(context);
        let location_binding = args.location.get_output(context);
        let max_bid_price_binding = args.max_bid_price.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_interfaces_binding = args.network_interfaces.get_output(context);
        let os_disk_binding = args.os_disk.get_output(context);
        let os_profile_binding = args.os_profile.get_output(context);
        let plan_binding = args.plan.get_output(context);
        let platform_fault_domain_count_binding = args
            .platform_fault_domain_count
            .get_output(context);
        let priority_binding = args.priority.get_output(context);
        let priority_mix_binding = args.priority_mix.get_output(context);
        let proximity_placement_group_id_binding = args
            .proximity_placement_group_id
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let single_placement_group_binding = args
            .single_placement_group
            .get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let sku_profile_binding = args.sku_profile.get_output(context);
        let source_image_id_binding = args.source_image_id.get_output(context);
        let source_image_reference_binding = args
            .source_image_reference
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let termination_notification_binding = args
            .termination_notification
            .get_output(context);
        let user_data_base64_binding = args.user_data_base64.get_output(context);
        let zone_balance_binding = args.zone_balance.get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/orchestratedVirtualMachineScaleSet:OrchestratedVirtualMachineScaleSet"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalCapabilities".into(),
                    value: additional_capabilities_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticInstanceRepair".into(),
                    value: automatic_instance_repair_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bootDiagnostics".into(),
                    value: boot_diagnostics_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacityReservationGroupId".into(),
                    value: capacity_reservation_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataDisks".into(),
                    value: data_disks_binding.get_id(),
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
                    name: "extensionOperationsEnabled".into(),
                    value: extension_operations_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extensions".into(),
                    value: extensions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extensionsTimeBudget".into(),
                    value: extensions_time_budget_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instances".into(),
                    value: instances_binding.get_id(),
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
                    name: "networkInterfaces".into(),
                    value: network_interfaces_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osDisk".into(),
                    value: os_disk_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osProfile".into(),
                    value: os_profile_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plan".into(),
                    value: plan_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "platformFaultDomainCount".into(),
                    value: platform_fault_domain_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priorityMix".into(),
                    value: priority_mix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "proximityPlacementGroupId".into(),
                    value: proximity_placement_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "singlePlacementGroup".into(),
                    value: single_placement_group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: sku_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuProfile".into(),
                    value: sku_profile_binding.get_id(),
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
                    name: "userDataBase64".into(),
                    value: user_data_base64_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneBalance".into(),
                    value: zone_balance_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: zones_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrchestratedVirtualMachineScaleSetResult {
            additional_capabilities: o.get_field("additionalCapabilities"),
            automatic_instance_repair: o.get_field("automaticInstanceRepair"),
            boot_diagnostics: o.get_field("bootDiagnostics"),
            capacity_reservation_group_id: o.get_field("capacityReservationGroupId"),
            data_disks: o.get_field("dataDisks"),
            encryption_at_host_enabled: o.get_field("encryptionAtHostEnabled"),
            eviction_policy: o.get_field("evictionPolicy"),
            extension_operations_enabled: o.get_field("extensionOperationsEnabled"),
            extensions: o.get_field("extensions"),
            extensions_time_budget: o.get_field("extensionsTimeBudget"),
            identity: o.get_field("identity"),
            instances: o.get_field("instances"),
            license_type: o.get_field("licenseType"),
            location: o.get_field("location"),
            max_bid_price: o.get_field("maxBidPrice"),
            name: o.get_field("name"),
            network_interfaces: o.get_field("networkInterfaces"),
            os_disk: o.get_field("osDisk"),
            os_profile: o.get_field("osProfile"),
            plan: o.get_field("plan"),
            platform_fault_domain_count: o.get_field("platformFaultDomainCount"),
            priority: o.get_field("priority"),
            priority_mix: o.get_field("priorityMix"),
            proximity_placement_group_id: o.get_field("proximityPlacementGroupId"),
            resource_group_name: o.get_field("resourceGroupName"),
            single_placement_group: o.get_field("singlePlacementGroup"),
            sku_name: o.get_field("skuName"),
            sku_profile: o.get_field("skuProfile"),
            source_image_id: o.get_field("sourceImageId"),
            source_image_reference: o.get_field("sourceImageReference"),
            tags: o.get_field("tags"),
            termination_notification: o.get_field("terminationNotification"),
            unique_id: o.get_field("uniqueId"),
            user_data_base64: o.get_field("userDataBase64"),
            zone_balance: o.get_field("zoneBalance"),
            zones: o.get_field("zones"),
        }
    }
}
