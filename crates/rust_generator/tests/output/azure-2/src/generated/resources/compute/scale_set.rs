/// Manages a virtual machine scale set.
///
/// ## Example Usage
///
/// ### With Managed Disks (Recommended)
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
///       name: acctvn
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: acctsub
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: test
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       allocationMethod: Static
///       domainNameLabel: ${example.name}
///       tags:
///         environment: staging
///   exampleLoadBalancer:
///     type: azure:lb:LoadBalancer
///     name: example
///     properties:
///       name: test
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       frontendIpConfigurations:
///         - name: PublicIPAddress
///           publicIpAddressId: ${examplePublicIp.id}
///   bpepool:
///     type: azure:lb:BackendAddressPool
///     properties:
///       loadbalancerId: ${exampleLoadBalancer.id}
///       name: BackEndAddressPool
///   lbnatpool:
///     type: azure:lb:NatPool
///     properties:
///       resourceGroupName: ${example.name}
///       name: ssh
///       loadbalancerId: ${exampleLoadBalancer.id}
///       protocol: Tcp
///       frontendPortStart: 50000
///       frontendPortEnd: 50119
///       backendPort: 22
///       frontendIpConfigurationName: PublicIPAddress
///   exampleProbe:
///     type: azure:lb:Probe
///     name: example
///     properties:
///       loadbalancerId: ${exampleLoadBalancer.id}
///       name: http-probe
///       protocol: Http
///       requestPath: /health
///       port: 8080
///   exampleScaleSet:
///     type: azure:compute:ScaleSet
///     name: example
///     properties:
///       name: mytestscaleset-1
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       automaticOsUpgrade: true
///       upgradePolicyMode: Rolling
///       rollingUpgradePolicy:
///         maxBatchInstancePercent: 20
///         maxUnhealthyInstancePercent: 20
///         maxUnhealthyUpgradedInstancePercent: 5
///         pauseTimeBetweenBatches: PT0S
///       healthProbeId: ${exampleProbe.id}
///       sku:
///         name: Standard_F2
///         tier: Standard
///         capacity: 2
///       storageProfileImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///       storageProfileOsDisk:
///         name: ""
///         caching: ReadWrite
///         createOption: FromImage
///         managedDiskType: Standard_LRS
///       storageProfileDataDisks:
///         - lun: 0
///           caching: ReadWrite
///           createOption: Empty
///           diskSizeGb: 10
///       osProfile:
///         computerNamePrefix: testvm
///         adminUsername: myadmin
///       osProfileLinuxConfig:
///         disablePasswordAuthentication: true
///         sshKeys:
///           - path: /home/myadmin/.ssh/authorized_keys
///             keyData:
///               fn::invoke:
///                 function: std:file
///                 arguments:
///                   input: ~/.ssh/demo_key.pub
///                 return: result
///       networkProfiles:
///         - name: mynetworkprofile
///           primary: true
///           ipConfigurations:
///             - name: TestIPConfiguration
///               primary: true
///               subnetId: ${exampleSubnet.id}
///               loadBalancerBackendAddressPoolIds:
///                 - ${bpepool.id}
///               loadBalancerInboundNatRulesIds:
///                 - ${lbnatpool.id}
///       tags:
///         environment: staging
/// ```
///
///
/// ### With Unmanaged Disks
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
///       name: acctvn
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: acctsub
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: accsa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       tags:
///         environment: staging
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: vhds
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: private
///   exampleScaleSet:
///     type: azure:compute:ScaleSet
///     name: example
///     properties:
///       name: mytestscaleset-1
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       upgradePolicyMode: Manual
///       sku:
///         name: Standard_F2
///         tier: Standard
///         capacity: 2
///       osProfile:
///         computerNamePrefix: testvm
///         adminUsername: myadmin
///       osProfileLinuxConfig:
///         disablePasswordAuthentication: true
///         sshKeys:
///           - path: /home/myadmin/.ssh/authorized_keys
///             keyData:
///               fn::invoke:
///                 function: std:file
///                 arguments:
///                   input: ~/.ssh/demo_key.pub
///                 return: result
///       networkProfiles:
///         - name: TestNetworkProfile
///           primary: true
///           ipConfigurations:
///             - name: TestIPConfiguration
///               primary: true
///               subnetId: ${exampleSubnet.id}
///       storageProfileOsDisk:
///         name: osDiskProfile
///         caching: ReadWrite
///         createOption: FromImage
///         vhdContainers:
///           - ${exampleAccount.primaryBlobEndpoint}${exampleContainer.name}
///       storageProfileImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
/// ```
///
/// ## Example of storage_profile_image_reference with id
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = image::create(
///         "example",
///         ImageArgs::builder().name("test").build_struct(),
///     );
///     let exampleScaleSet = scale_set::create(
///         "exampleScaleSet",
///         ScaleSetArgs::builder()
///             .name("test")
///             .storage_profile_image_reference(
///                 ScaleSetStorageProfileImageReference::builder()
///                     .id("${example.id}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Virtual Machine Scale Sets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/scaleSet:ScaleSet scaleset1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/virtualMachineScaleSets/scaleset1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod scale_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScaleSetArgs {
        /// Automatic OS patches can be applied by Azure to your scaleset. This is particularly useful when `upgrade_policy_mode` is set to `Rolling`. Defaults to `false`.
        #[builder(into, default)]
        pub automatic_os_upgrade: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `boot_diagnostics` block as referenced below.
        #[builder(into, default)]
        pub boot_diagnostics: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::ScaleSetBootDiagnostics>,
        >,
        /// Specifies the eviction policy for Virtual Machines in this Scale Set. Possible values are `Deallocate` and `Delete`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `eviction_policy` can only be set when `priority` is set to `Low`.
        #[builder(into, default)]
        pub eviction_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Can be specified multiple times to add extension profiles to the scale set. Each `extension` block supports the fields documented below.
        #[builder(into, default)]
        pub extensions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::ScaleSetExtension>>,
        >,
        /// Specifies the identifier for the load balancer health probe. Required when using `Rolling` as your `upgrade_policy_mode`.
        #[builder(into, default)]
        pub health_probe_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::ScaleSetIdentity>,
        >,
        /// (Optional, when a Windows machine) Specifies the Windows OS license type. If supplied, the only allowed values are `Windows_Client` and `Windows_Server`.
        #[builder(into, default)]
        pub license_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the virtual machine scale set resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A collection of `network_profile` blocks as documented below.
        #[builder(into)]
        pub network_profiles: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::compute::ScaleSetNetworkProfile>,
        >,
        /// A `os_profile` block as documented below.
        #[builder(into)]
        pub os_profile: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::ScaleSetOsProfile,
        >,
        /// A `os_profile_linux_config` block as documented below.
        #[builder(into, default)]
        pub os_profile_linux_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::ScaleSetOsProfileLinuxConfig>,
        >,
        /// A collection of `os_profile_secrets` blocks as documented below.
        #[builder(into, default)]
        pub os_profile_secrets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::ScaleSetOsProfileSecret>>,
        >,
        /// A `os_profile_windows_config` block as documented below.
        #[builder(into, default)]
        pub os_profile_windows_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::ScaleSetOsProfileWindowsConfig>,
        >,
        /// Specifies whether the virtual machine scale set should be overprovisioned. Defaults to `true`.
        #[builder(into, default)]
        pub overprovision: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `plan` block as documented below.
        #[builder(into, default)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::ScaleSetPlan>,
        >,
        /// Specifies the priority for the Virtual Machines in the Scale Set. Possible values are `Low` and `Regular`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Proximity Placement Group to which this Virtual Machine should be assigned. Changing this forces a new resource to be created
        #[builder(into, default)]
        pub proximity_placement_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the resource group in which to create the virtual machine scale set. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `rolling_upgrade_policy` block as defined below. This is only applicable when the `upgrade_policy_mode` is `Rolling`.
        #[builder(into, default)]
        pub rolling_upgrade_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::ScaleSetRollingUpgradePolicy>,
        >,
        /// Specifies whether the scale set is limited to a single placement group with a maximum size of 100 virtual machines. If set to false, managed disks must be used. Changing this forces a new resource to be created. See [documentation](https://docs.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-placement-groups) for more information. Defaults to `true`.
        #[builder(into, default)]
        pub single_placement_group: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `sku` block as documented below.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::ScaleSetSku,
        >,
        /// A `storage_profile_data_disk` block as documented below.
        #[builder(into, default)]
        pub storage_profile_data_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::ScaleSetStorageProfileDataDisk>>,
        >,
        /// A `storage_profile_image_reference` block as documented below.
        #[builder(into, default)]
        pub storage_profile_image_reference: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::ScaleSetStorageProfileImageReference>,
        >,
        /// A `storage_profile_os_disk` block as documented below.
        #[builder(into)]
        pub storage_profile_os_disk: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::ScaleSetStorageProfileOsDisk,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the mode of an upgrade to virtual machines in the scale set. Possible values, `Rolling`, `Manual`, or `Automatic`. When choosing `Rolling`, you will need to set a health probe.
        #[builder(into)]
        pub upgrade_policy_mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A collection of availability zones to spread the Virtual Machines over. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ScaleSetResult {
        /// Automatic OS patches can be applied by Azure to your scaleset. This is particularly useful when `upgrade_policy_mode` is set to `Rolling`. Defaults to `false`.
        pub automatic_os_upgrade: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `boot_diagnostics` block as referenced below.
        pub boot_diagnostics: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::ScaleSetBootDiagnostics>,
        >,
        /// Specifies the eviction policy for Virtual Machines in this Scale Set. Possible values are `Deallocate` and `Delete`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `eviction_policy` can only be set when `priority` is set to `Low`.
        pub eviction_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Can be specified multiple times to add extension profiles to the scale set. Each `extension` block supports the fields documented below.
        pub extensions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::ScaleSetExtension>>,
        >,
        /// Specifies the identifier for the load balancer health probe. Required when using `Rolling` as your `upgrade_policy_mode`.
        pub health_probe_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::ScaleSetIdentity>,
        >,
        /// (Optional, when a Windows machine) Specifies the Windows OS license type. If supplied, the only allowed values are `Windows_Client` and `Windows_Server`.
        pub license_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the virtual machine scale set resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A collection of `network_profile` blocks as documented below.
        pub network_profiles: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::ScaleSetNetworkProfile>,
        >,
        /// A `os_profile` block as documented below.
        pub os_profile: pulumi_gestalt_rust::Output<
            super::super::types::compute::ScaleSetOsProfile,
        >,
        /// A `os_profile_linux_config` block as documented below.
        pub os_profile_linux_config: pulumi_gestalt_rust::Output<
            super::super::types::compute::ScaleSetOsProfileLinuxConfig,
        >,
        /// A collection of `os_profile_secrets` blocks as documented below.
        pub os_profile_secrets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::ScaleSetOsProfileSecret>>,
        >,
        /// A `os_profile_windows_config` block as documented below.
        pub os_profile_windows_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::ScaleSetOsProfileWindowsConfig>,
        >,
        /// Specifies whether the virtual machine scale set should be overprovisioned. Defaults to `true`.
        pub overprovision: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `plan` block as documented below.
        pub plan: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::ScaleSetPlan>,
        >,
        /// Specifies the priority for the Virtual Machines in the Scale Set. Possible values are `Low` and `Regular`. Changing this forces a new resource to be created.
        pub priority: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Proximity Placement Group to which this Virtual Machine should be assigned. Changing this forces a new resource to be created
        pub proximity_placement_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the virtual machine scale set. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `rolling_upgrade_policy` block as defined below. This is only applicable when the `upgrade_policy_mode` is `Rolling`.
        pub rolling_upgrade_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::ScaleSetRollingUpgradePolicy>,
        >,
        /// Specifies whether the scale set is limited to a single placement group with a maximum size of 100 virtual machines. If set to false, managed disks must be used. Changing this forces a new resource to be created. See [documentation](https://docs.microsoft.com/azure/virtual-machine-scale-sets/virtual-machine-scale-sets-placement-groups) for more information. Defaults to `true`.
        pub single_placement_group: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `sku` block as documented below.
        pub sku: pulumi_gestalt_rust::Output<super::super::types::compute::ScaleSetSku>,
        /// A `storage_profile_data_disk` block as documented below.
        pub storage_profile_data_disks: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::ScaleSetStorageProfileDataDisk>>,
        >,
        /// A `storage_profile_image_reference` block as documented below.
        pub storage_profile_image_reference: pulumi_gestalt_rust::Output<
            super::super::types::compute::ScaleSetStorageProfileImageReference,
        >,
        /// A `storage_profile_os_disk` block as documented below.
        pub storage_profile_os_disk: pulumi_gestalt_rust::Output<
            super::super::types::compute::ScaleSetStorageProfileOsDisk,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the mode of an upgrade to virtual machines in the scale set. Possible values, `Rolling`, `Manual`, or `Automatic`. When choosing `Rolling`, you will need to set a health probe.
        pub upgrade_policy_mode: pulumi_gestalt_rust::Output<String>,
        /// A collection of availability zones to spread the Virtual Machines over. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        pub zones: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScaleSetArgs,
    ) -> ScaleSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let automatic_os_upgrade_binding = args.automatic_os_upgrade.get_output(context);
        let boot_diagnostics_binding = args.boot_diagnostics.get_output(context);
        let eviction_policy_binding = args.eviction_policy.get_output(context);
        let extensions_binding = args.extensions.get_output(context);
        let health_probe_id_binding = args.health_probe_id.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let license_type_binding = args.license_type.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_profiles_binding = args.network_profiles.get_output(context);
        let os_profile_binding = args.os_profile.get_output(context);
        let os_profile_linux_config_binding = args
            .os_profile_linux_config
            .get_output(context);
        let os_profile_secrets_binding = args.os_profile_secrets.get_output(context);
        let os_profile_windows_config_binding = args
            .os_profile_windows_config
            .get_output(context);
        let overprovision_binding = args.overprovision.get_output(context);
        let plan_binding = args.plan.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let proximity_placement_group_id_binding = args
            .proximity_placement_group_id
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let rolling_upgrade_policy_binding = args
            .rolling_upgrade_policy
            .get_output(context);
        let single_placement_group_binding = args
            .single_placement_group
            .get_output(context);
        let sku_binding = args.sku.get_output(context);
        let storage_profile_data_disks_binding = args
            .storage_profile_data_disks
            .get_output(context);
        let storage_profile_image_reference_binding = args
            .storage_profile_image_reference
            .get_output(context);
        let storage_profile_os_disk_binding = args
            .storage_profile_os_disk
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let upgrade_policy_mode_binding = args.upgrade_policy_mode.get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/scaleSet:ScaleSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticOsUpgrade".into(),
                    value: automatic_os_upgrade_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bootDiagnostics".into(),
                    value: boot_diagnostics_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "evictionPolicy".into(),
                    value: eviction_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extensions".into(),
                    value: extensions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthProbeId".into(),
                    value: health_probe_id_binding.get_id(),
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
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkProfiles".into(),
                    value: network_profiles_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osProfile".into(),
                    value: os_profile_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osProfileLinuxConfig".into(),
                    value: os_profile_linux_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osProfileSecrets".into(),
                    value: os_profile_secrets_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osProfileWindowsConfig".into(),
                    value: os_profile_windows_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "overprovision".into(),
                    value: overprovision_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plan".into(),
                    value: plan_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
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
                    name: "rollingUpgradePolicy".into(),
                    value: rolling_upgrade_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "singlePlacementGroup".into(),
                    value: single_placement_group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: sku_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageProfileDataDisks".into(),
                    value: storage_profile_data_disks_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageProfileImageReference".into(),
                    value: storage_profile_image_reference_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageProfileOsDisk".into(),
                    value: storage_profile_os_disk_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "upgradePolicyMode".into(),
                    value: upgrade_policy_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: zones_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ScaleSetResult {
            automatic_os_upgrade: o.get_field("automaticOsUpgrade"),
            boot_diagnostics: o.get_field("bootDiagnostics"),
            eviction_policy: o.get_field("evictionPolicy"),
            extensions: o.get_field("extensions"),
            health_probe_id: o.get_field("healthProbeId"),
            identity: o.get_field("identity"),
            license_type: o.get_field("licenseType"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network_profiles: o.get_field("networkProfiles"),
            os_profile: o.get_field("osProfile"),
            os_profile_linux_config: o.get_field("osProfileLinuxConfig"),
            os_profile_secrets: o.get_field("osProfileSecrets"),
            os_profile_windows_config: o.get_field("osProfileWindowsConfig"),
            overprovision: o.get_field("overprovision"),
            plan: o.get_field("plan"),
            priority: o.get_field("priority"),
            proximity_placement_group_id: o.get_field("proximityPlacementGroupId"),
            resource_group_name: o.get_field("resourceGroupName"),
            rolling_upgrade_policy: o.get_field("rollingUpgradePolicy"),
            single_placement_group: o.get_field("singlePlacementGroup"),
            sku: o.get_field("sku"),
            storage_profile_data_disks: o.get_field("storageProfileDataDisks"),
            storage_profile_image_reference: o.get_field("storageProfileImageReference"),
            storage_profile_os_disk: o.get_field("storageProfileOsDisk"),
            tags: o.get_field("tags"),
            upgrade_policy_mode: o.get_field("upgradePolicyMode"),
            zones: o.get_field("zones"),
        }
    }
}
