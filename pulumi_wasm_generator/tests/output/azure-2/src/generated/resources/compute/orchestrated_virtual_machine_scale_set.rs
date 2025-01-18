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
pub mod orchestrated_virtual_machine_scale_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrchestratedVirtualMachineScaleSetArgs {
        /// An `additional_capabilities` block as defined below.
        #[builder(into, default)]
        pub additional_capabilities: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetAdditionalCapabilities,
            >,
        >,
        /// An `automatic_instance_repair` block as defined below.
        ///
        /// > **Note:** To enable the `automatic_instance_repair`, the Orchestrated Virtual Machine Scale Set must have a valid [Application Health Extension](https://docs.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-health-extension).
        #[builder(into, default)]
        pub automatic_instance_repair: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetAutomaticInstanceRepair,
            >,
        >,
        /// A `boot_diagnostics` block as defined below.
        #[builder(into, default)]
        pub boot_diagnostics: pulumi_wasm_rust::Output<
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
        pub capacity_reservation_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `data_disk` blocks as defined below.
        #[builder(into, default)]
        pub data_disks: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::OrchestratedVirtualMachineScaleSetDataDisk,
                >,
            >,
        >,
        /// Should disks attached to this Virtual Machine Scale Set be encrypted by enabling Encryption at Host?
        #[builder(into, default)]
        pub encryption_at_host_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Policy which should be used by Spot Virtual Machines that are Evicted from the Scale Set. Possible values are `Deallocate` and `Delete`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub eviction_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Should extension operations be allowed on the Virtual Machine Scale Set? Possible values are `true` or `false`. Defaults to `true`. Changing this forces a new Virtual Machine Scale Set to be created.
        ///
        /// > **Note:** `extension_operations_enabled` may only be set to `false` if there are no extensions defined in the `extension` field.
        #[builder(into, default)]
        pub extension_operations_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `extension` blocks as defined below
        #[builder(into, default)]
        pub extensions: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::OrchestratedVirtualMachineScaleSetExtension,
                >,
            >,
        >,
        /// Specifies the time alloted for all extensions to start. The time duration should be between 15 minutes and 120 minutes (inclusive) and should be specified in ISO 8601 format. Defaults to `PT1H30M`.
        #[builder(into, default)]
        pub extensions_time_budget: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetIdentity,
            >,
        >,
        /// The number of Virtual Machines in the Virtual Machine Scale Set.
        #[builder(into, default)]
        pub instances: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the type of on-premise license (also known as Azure Hybrid Use Benefit) which should be used for this Virtual Machine Scale Set. Possible values are `None`, `Windows_Client` and `Windows_Server`.
        #[builder(into, default)]
        pub license_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The Azure location where the Virtual Machine Scale Set should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum price you're willing to pay for each Virtual Machine in this Scale Set, in US Dollars; which must be greater than the current spot price. If this bid price falls below the current spot price the Virtual Machines in the Scale Set will be evicted using the eviction_policy. Defaults to `-1`, which means that each Virtual Machine in the Scale Set should not be evicted for price reasons.
        #[builder(into, default)]
        pub max_bid_price: pulumi_wasm_rust::Output<Option<f64>>,
        /// The name of the Virtual Machine Scale Set. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `network_interface` blocks as defined below.
        #[builder(into, default)]
        pub network_interfaces: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::OrchestratedVirtualMachineScaleSetNetworkInterface,
                >,
            >,
        >,
        /// An `os_disk` block as defined below.
        #[builder(into, default)]
        pub os_disk: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetOsDisk,
            >,
        >,
        /// An `os_profile` block as defined below.
        #[builder(into, default)]
        pub os_profile: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfile,
            >,
        >,
        /// A `plan` block as documented below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub plan: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::OrchestratedVirtualMachineScaleSetPlan>,
        >,
        /// Specifies the number of fault domains that are used by this Virtual Machine Scale Set. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The number of Fault Domains varies depending on which Azure Region you're using. More information about update and fault domains and how they work can be found [here](https://learn.microsoft.com/en-us/azure/virtual-machines/availability-set-overview).
        #[builder(into)]
        pub platform_fault_domain_count: pulumi_wasm_rust::Output<i32>,
        /// The Priority of this Virtual Machine Scale Set. Possible values are `Regular` and `Spot`. Defaults to `Regular`. Changing this value forces a new resource.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::Output<Option<String>>,
        /// a `priority_mix` block as defined below
        #[builder(into, default)]
        pub priority_mix: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetPriorityMix,
            >,
        >,
        /// The ID of the Proximity Placement Group which the Virtual Machine should be assigned to. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub proximity_placement_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the Virtual Machine Scale Set should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Should this Virtual Machine Scale Set be limited to a Single Placement Group, which means the number of instances will be capped at 100 Virtual Machines. Possible values are `true` or `false`.
        ///
        /// > **Note:** `single_placement_group` behaves differently for Flexible orchestration Virtual Machine Scale Sets than it does for Uniform orchestration Virtual Machine Scale Sets. It is recommended that you do not define the `single_placement_group` field in your configuration file as the service will determine what this value should be based off of the value contained within the `sku_name` field of your configuration file. You may set the `single_placement_group` field to `true`, however once you set it to `false` you will not be able to revert it back to `true`.
        #[builder(into, default)]
        pub single_placement_group: pulumi_wasm_rust::Output<Option<bool>>,
        /// The `name` of the SKU to be used by this Virtual Machine Scale Set. Valid values include: any of the [General purpose](https://docs.microsoft.com/azure/virtual-machines/sizes-general), [Compute optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-compute), [Memory optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-memory), [Storage optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-storage), [GPU optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-gpu), [FPGA optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-field-programmable-gate-arrays), [High performance](https://docs.microsoft.com/azure/virtual-machines/sizes-hpc), or [Previous generation](https://docs.microsoft.com/azure/virtual-machines/sizes-previous-gen) virtual machine SKUs.
        #[builder(into, default)]
        pub sku_name: pulumi_wasm_rust::Output<Option<String>>,
        /// An `sku_profile` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **Note:** If `sku_profile` is specified the `sku_name` must be set to `Mix`.
        #[builder(into, default)]
        pub sku_profile: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetSkuProfile,
            >,
        >,
        /// The ID of an Image which each Virtual Machine in this Scale Set should be based on. Possible Image ID types include `Image ID`s, `Shared Image ID`s, `Shared Image Version ID`s, `Community Gallery Image ID`s, `Community Gallery Image Version ID`s, `Shared Gallery Image ID`s and `Shared Gallery Image Version ID`s.
        #[builder(into, default)]
        pub source_image_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `source_image_reference` block as defined below.
        #[builder(into, default)]
        pub source_image_reference: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetSourceImageReference,
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
                super::super::types::compute::OrchestratedVirtualMachineScaleSetTerminationNotification,
            >,
        >,
        /// The Base64-Encoded User Data which should be used for this Virtual Machine Scale Set.
        #[builder(into, default)]
        pub user_data_base64: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Virtual Machines in this Scale Set be strictly evenly distributed across Availability Zones? Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This can only be set to `true` when one or more `zones` are configured.
        #[builder(into, default)]
        pub zone_balance: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies a list of Availability Zones across which the Virtual Machine Scale Set will create instances.
        ///
        /// > **Note:** Updating `zones` to remove an existing zone forces a new Virtual Machine Scale Set to be created.
        ///
        /// > **Note:** Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct OrchestratedVirtualMachineScaleSetResult {
        /// An `additional_capabilities` block as defined below.
        pub additional_capabilities: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetAdditionalCapabilities,
            >,
        >,
        /// An `automatic_instance_repair` block as defined below.
        ///
        /// > **Note:** To enable the `automatic_instance_repair`, the Orchestrated Virtual Machine Scale Set must have a valid [Application Health Extension](https://docs.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-health-extension).
        pub automatic_instance_repair: pulumi_wasm_rust::Output<
            super::super::types::compute::OrchestratedVirtualMachineScaleSetAutomaticInstanceRepair,
        >,
        /// A `boot_diagnostics` block as defined below.
        pub boot_diagnostics: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetBootDiagnostics,
            >,
        >,
        /// Specifies the ID of the Capacity Reservation Group which the Virtual Machine Scale Set should be allocated to. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `capacity_reservation_group_id` cannot be specified with `proximity_placement_group_id`
        ///
        /// > **Note:** If `capacity_reservation_group_id` is specified the `single_placement_group` must be set to `false`.
        pub capacity_reservation_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `data_disk` blocks as defined below.
        pub data_disks: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::OrchestratedVirtualMachineScaleSetDataDisk,
                >,
            >,
        >,
        /// Should disks attached to this Virtual Machine Scale Set be encrypted by enabling Encryption at Host?
        pub encryption_at_host_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Policy which should be used by Spot Virtual Machines that are Evicted from the Scale Set. Possible values are `Deallocate` and `Delete`. Changing this forces a new resource to be created.
        pub eviction_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Should extension operations be allowed on the Virtual Machine Scale Set? Possible values are `true` or `false`. Defaults to `true`. Changing this forces a new Virtual Machine Scale Set to be created.
        ///
        /// > **Note:** `extension_operations_enabled` may only be set to `false` if there are no extensions defined in the `extension` field.
        pub extension_operations_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `extension` blocks as defined below
        pub extensions: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetExtension,
            >,
        >,
        /// Specifies the time alloted for all extensions to start. The time duration should be between 15 minutes and 120 minutes (inclusive) and should be specified in ISO 8601 format. Defaults to `PT1H30M`.
        pub extensions_time_budget: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetIdentity,
            >,
        >,
        /// The number of Virtual Machines in the Virtual Machine Scale Set.
        pub instances: pulumi_wasm_rust::Output<i32>,
        /// Specifies the type of on-premise license (also known as Azure Hybrid Use Benefit) which should be used for this Virtual Machine Scale Set. Possible values are `None`, `Windows_Client` and `Windows_Server`.
        pub license_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The Azure location where the Virtual Machine Scale Set should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The maximum price you're willing to pay for each Virtual Machine in this Scale Set, in US Dollars; which must be greater than the current spot price. If this bid price falls below the current spot price the Virtual Machines in the Scale Set will be evicted using the eviction_policy. Defaults to `-1`, which means that each Virtual Machine in the Scale Set should not be evicted for price reasons.
        pub max_bid_price: pulumi_wasm_rust::Output<Option<f64>>,
        /// The name of the Virtual Machine Scale Set. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `network_interface` blocks as defined below.
        pub network_interfaces: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::OrchestratedVirtualMachineScaleSetNetworkInterface,
                >,
            >,
        >,
        /// An `os_disk` block as defined below.
        pub os_disk: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetOsDisk,
            >,
        >,
        /// An `os_profile` block as defined below.
        pub os_profile: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfile,
            >,
        >,
        /// A `plan` block as documented below. Changing this forces a new resource to be created.
        pub plan: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::OrchestratedVirtualMachineScaleSetPlan>,
        >,
        /// Specifies the number of fault domains that are used by this Virtual Machine Scale Set. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The number of Fault Domains varies depending on which Azure Region you're using. More information about update and fault domains and how they work can be found [here](https://learn.microsoft.com/en-us/azure/virtual-machines/availability-set-overview).
        pub platform_fault_domain_count: pulumi_wasm_rust::Output<i32>,
        /// The Priority of this Virtual Machine Scale Set. Possible values are `Regular` and `Spot`. Defaults to `Regular`. Changing this value forces a new resource.
        pub priority: pulumi_wasm_rust::Output<Option<String>>,
        /// a `priority_mix` block as defined below
        pub priority_mix: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetPriorityMix,
            >,
        >,
        /// The ID of the Proximity Placement Group which the Virtual Machine should be assigned to. Changing this forces a new resource to be created.
        pub proximity_placement_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the Virtual Machine Scale Set should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Should this Virtual Machine Scale Set be limited to a Single Placement Group, which means the number of instances will be capped at 100 Virtual Machines. Possible values are `true` or `false`.
        ///
        /// > **Note:** `single_placement_group` behaves differently for Flexible orchestration Virtual Machine Scale Sets than it does for Uniform orchestration Virtual Machine Scale Sets. It is recommended that you do not define the `single_placement_group` field in your configuration file as the service will determine what this value should be based off of the value contained within the `sku_name` field of your configuration file. You may set the `single_placement_group` field to `true`, however once you set it to `false` you will not be able to revert it back to `true`.
        pub single_placement_group: pulumi_wasm_rust::Output<bool>,
        /// The `name` of the SKU to be used by this Virtual Machine Scale Set. Valid values include: any of the [General purpose](https://docs.microsoft.com/azure/virtual-machines/sizes-general), [Compute optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-compute), [Memory optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-memory), [Storage optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-storage), [GPU optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-gpu), [FPGA optimized](https://docs.microsoft.com/azure/virtual-machines/sizes-field-programmable-gate-arrays), [High performance](https://docs.microsoft.com/azure/virtual-machines/sizes-hpc), or [Previous generation](https://docs.microsoft.com/azure/virtual-machines/sizes-previous-gen) virtual machine SKUs.
        pub sku_name: pulumi_wasm_rust::Output<Option<String>>,
        /// An `sku_profile` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **Note:** If `sku_profile` is specified the `sku_name` must be set to `Mix`.
        pub sku_profile: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetSkuProfile,
            >,
        >,
        /// The ID of an Image which each Virtual Machine in this Scale Set should be based on. Possible Image ID types include `Image ID`s, `Shared Image ID`s, `Shared Image Version ID`s, `Community Gallery Image ID`s, `Community Gallery Image Version ID`s, `Shared Gallery Image ID`s and `Shared Gallery Image Version ID`s.
        pub source_image_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `source_image_reference` block as defined below.
        pub source_image_reference: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::OrchestratedVirtualMachineScaleSetSourceImageReference,
            >,
        >,
        /// A mapping of tags which should be assigned to this Virtual Machine Scale Set.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `termination_notification` block as defined below.
        pub termination_notification: pulumi_wasm_rust::Output<
            super::super::types::compute::OrchestratedVirtualMachineScaleSetTerminationNotification,
        >,
        /// The Unique ID for the Virtual Machine Scale Set.
        pub unique_id: pulumi_wasm_rust::Output<String>,
        /// The Base64-Encoded User Data which should be used for this Virtual Machine Scale Set.
        pub user_data_base64: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Virtual Machines in this Scale Set be strictly evenly distributed across Availability Zones? Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This can only be set to `true` when one or more `zones` are configured.
        pub zone_balance: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies a list of Availability Zones across which the Virtual Machine Scale Set will create instances.
        ///
        /// > **Note:** Updating `zones` to remove an existing zone forces a new Virtual Machine Scale Set to be created.
        ///
        /// > **Note:** Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OrchestratedVirtualMachineScaleSetArgs,
    ) -> OrchestratedVirtualMachineScaleSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_capabilities_binding = args.additional_capabilities.get_inner();
        let automatic_instance_repair_binding = args
            .automatic_instance_repair
            .get_inner();
        let boot_diagnostics_binding = args.boot_diagnostics.get_inner();
        let capacity_reservation_group_id_binding = args
            .capacity_reservation_group_id
            .get_inner();
        let data_disks_binding = args.data_disks.get_inner();
        let encryption_at_host_enabled_binding = args
            .encryption_at_host_enabled
            .get_inner();
        let eviction_policy_binding = args.eviction_policy.get_inner();
        let extension_operations_enabled_binding = args
            .extension_operations_enabled
            .get_inner();
        let extensions_binding = args.extensions.get_inner();
        let extensions_time_budget_binding = args.extensions_time_budget.get_inner();
        let identity_binding = args.identity.get_inner();
        let instances_binding = args.instances.get_inner();
        let license_type_binding = args.license_type.get_inner();
        let location_binding = args.location.get_inner();
        let max_bid_price_binding = args.max_bid_price.get_inner();
        let name_binding = args.name.get_inner();
        let network_interfaces_binding = args.network_interfaces.get_inner();
        let os_disk_binding = args.os_disk.get_inner();
        let os_profile_binding = args.os_profile.get_inner();
        let plan_binding = args.plan.get_inner();
        let platform_fault_domain_count_binding = args
            .platform_fault_domain_count
            .get_inner();
        let priority_binding = args.priority.get_inner();
        let priority_mix_binding = args.priority_mix.get_inner();
        let proximity_placement_group_id_binding = args
            .proximity_placement_group_id
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let single_placement_group_binding = args.single_placement_group.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let sku_profile_binding = args.sku_profile.get_inner();
        let source_image_id_binding = args.source_image_id.get_inner();
        let source_image_reference_binding = args.source_image_reference.get_inner();
        let tags_binding = args.tags.get_inner();
        let termination_notification_binding = args.termination_notification.get_inner();
        let user_data_base64_binding = args.user_data_base64.get_inner();
        let zone_balance_binding = args.zone_balance.get_inner();
        let zones_binding = args.zones.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/orchestratedVirtualMachineScaleSet:OrchestratedVirtualMachineScaleSet"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalCapabilities".into(),
                    value: &additional_capabilities_binding,
                },
                register_interface::ObjectField {
                    name: "automaticInstanceRepair".into(),
                    value: &automatic_instance_repair_binding,
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
                    name: "dataDisks".into(),
                    value: &data_disks_binding,
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
                    name: "osProfile".into(),
                    value: &os_profile_binding,
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
                    name: "priorityMix".into(),
                    value: &priority_mix_binding,
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
                    name: "singlePlacementGroup".into(),
                    value: &single_placement_group_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuProfile".into(),
                    value: &sku_profile_binding,
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "terminationNotification".into(),
                    value: &termination_notification_binding,
                },
                register_interface::ObjectField {
                    name: "userDataBase64".into(),
                    value: &user_data_base64_binding,
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
                    name: "automaticInstanceRepair".into(),
                },
                register_interface::ResultField {
                    name: "bootDiagnostics".into(),
                },
                register_interface::ResultField {
                    name: "capacityReservationGroupId".into(),
                },
                register_interface::ResultField {
                    name: "dataDisks".into(),
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
                    name: "osProfile".into(),
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
                    name: "priorityMix".into(),
                },
                register_interface::ResultField {
                    name: "proximityPlacementGroupId".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "singlePlacementGroup".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "skuProfile".into(),
                },
                register_interface::ResultField {
                    name: "sourceImageId".into(),
                },
                register_interface::ResultField {
                    name: "sourceImageReference".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "terminationNotification".into(),
                },
                register_interface::ResultField {
                    name: "uniqueId".into(),
                },
                register_interface::ResultField {
                    name: "userDataBase64".into(),
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
        OrchestratedVirtualMachineScaleSetResult {
            additional_capabilities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalCapabilities").unwrap(),
            ),
            automatic_instance_repair: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticInstanceRepair").unwrap(),
            ),
            boot_diagnostics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootDiagnostics").unwrap(),
            ),
            capacity_reservation_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacityReservationGroupId").unwrap(),
            ),
            data_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataDisks").unwrap(),
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
            os_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osProfile").unwrap(),
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
            priority_mix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priorityMix").unwrap(),
            ),
            proximity_placement_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proximityPlacementGroupId").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            single_placement_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("singlePlacementGroup").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            sku_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuProfile").unwrap(),
            ),
            source_image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceImageId").unwrap(),
            ),
            source_image_reference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceImageReference").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            termination_notification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terminationNotification").unwrap(),
            ),
            unique_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uniqueId").unwrap(),
            ),
            user_data_base64: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userDataBase64").unwrap(),
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
