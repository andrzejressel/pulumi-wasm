/// Manages an Azure Stack HCI Virtual Hard Disk.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: West Europe
///   exampleHciStoragePath:
///     type: azure:stack:HciStoragePath
///     name: example
///     properties:
///       name: example-sp
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       customLocationId: /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.ExtendedLocation/customLocations/cl1
///       path: C:\ClusterStorage\UserStorage_2\sp-example
///       tags:
///         foo: bar
///   exampleHciVirtualHardDisk:
///     type: azure:stack:HciVirtualHardDisk
///     name: example
///     properties:
///       name: example-vhd
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       customLocationId: /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.ExtendedLocation/customLocations/cl1
///       diskSizeInGb: 2
///       storagePathId: ${exampleHciStoragePath.id}
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Azure Stack HCI Virtual Hard Disks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:stack/hciVirtualHardDisk:HciVirtualHardDisk example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.AzureStackHCI/virtualHardDisks/disk1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hci_virtual_hard_disk {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HciVirtualHardDiskArgs {
        /// The block size of the disk in bytes. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        #[builder(into, default)]
        pub block_size_in_bytes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the Custom Location where the Azure Stack HCI Virtual Hard Disk should exist. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        #[builder(into)]
        pub custom_location_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The format of the disk file. Possible values are `vhdx` and `vhd`. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        #[builder(into, default)]
        pub disk_file_format: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The size of the disk in GB. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        #[builder(into)]
        pub disk_size_in_gb: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Whether to enable dynamic sizing for the Azure Stack HCI Virtual Hard Disk. Defaults to `false`. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        #[builder(into, default)]
        pub dynamic_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The hypervisor generation of the Azure Stack HCI Virtual Hard Disk. Possible values are `V1` and `V2`. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        #[builder(into, default)]
        pub hyperv_generation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure Region where the Azure Stack HCI Virtual Hard Disk should exist. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The logical sector size of the disk in bytes. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        #[builder(into, default)]
        pub logical_sector_in_bytes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name which should be used for this Azure Stack HCI Virtual Hard Disk. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The physical sector size of the disk in bytes. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        #[builder(into, default)]
        pub physical_sector_in_bytes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the Resource Group where the Azure Stack HCI Virtual Hard Disk should exist. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Azure Stack HCI Storage Path used for this Virtual Hard Disk. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        ///
        /// > **Note:** If `storage_path_id` is not specified, the Virtual Hard Disk will be placed in a high availability Storage Path. If you experience a diff you may need to add this to `ignore_changes`.
        #[builder(into, default)]
        pub storage_path_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Azure Stack HCI Virtual Hard Disk.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct HciVirtualHardDiskResult {
        /// The block size of the disk in bytes. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        pub block_size_in_bytes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the Custom Location where the Azure Stack HCI Virtual Hard Disk should exist. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        pub custom_location_id: pulumi_gestalt_rust::Output<String>,
        /// The format of the disk file. Possible values are `vhdx` and `vhd`. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        pub disk_file_format: pulumi_gestalt_rust::Output<Option<String>>,
        /// The size of the disk in GB. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        pub disk_size_in_gb: pulumi_gestalt_rust::Output<i32>,
        /// Whether to enable dynamic sizing for the Azure Stack HCI Virtual Hard Disk. Defaults to `false`. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        pub dynamic_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The hypervisor generation of the Azure Stack HCI Virtual Hard Disk. Possible values are `V1` and `V2`. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        pub hyperv_generation: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Azure Region where the Azure Stack HCI Virtual Hard Disk should exist. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The logical sector size of the disk in bytes. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        pub logical_sector_in_bytes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name which should be used for this Azure Stack HCI Virtual Hard Disk. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The physical sector size of the disk in bytes. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        pub physical_sector_in_bytes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the Resource Group where the Azure Stack HCI Virtual Hard Disk should exist. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Azure Stack HCI Storage Path used for this Virtual Hard Disk. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        ///
        /// > **Note:** If `storage_path_id` is not specified, the Virtual Hard Disk will be placed in a high availability Storage Path. If you experience a diff you may need to add this to `ignore_changes`.
        pub storage_path_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Azure Stack HCI Virtual Hard Disk.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HciVirtualHardDiskArgs,
    ) -> HciVirtualHardDiskResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let block_size_in_bytes_binding = args.block_size_in_bytes.get_output(context);
        let custom_location_id_binding = args.custom_location_id.get_output(context);
        let disk_file_format_binding = args.disk_file_format.get_output(context);
        let disk_size_in_gb_binding = args.disk_size_in_gb.get_output(context);
        let dynamic_enabled_binding = args.dynamic_enabled.get_output(context);
        let hyperv_generation_binding = args.hyperv_generation.get_output(context);
        let location_binding = args.location.get_output(context);
        let logical_sector_in_bytes_binding = args
            .logical_sector_in_bytes
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let physical_sector_in_bytes_binding = args
            .physical_sector_in_bytes
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let storage_path_id_binding = args.storage_path_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:stack/hciVirtualHardDisk:HciVirtualHardDisk".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockSizeInBytes".into(),
                    value: block_size_in_bytes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customLocationId".into(),
                    value: custom_location_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskFileFormat".into(),
                    value: disk_file_format_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskSizeInGb".into(),
                    value: disk_size_in_gb_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dynamicEnabled".into(),
                    value: dynamic_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hypervGeneration".into(),
                    value: hyperv_generation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logicalSectorInBytes".into(),
                    value: logical_sector_in_bytes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "physicalSectorInBytes".into(),
                    value: physical_sector_in_bytes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storagePathId".into(),
                    value: storage_path_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        HciVirtualHardDiskResult {
            block_size_in_bytes: o.get_field("blockSizeInBytes"),
            custom_location_id: o.get_field("customLocationId"),
            disk_file_format: o.get_field("diskFileFormat"),
            disk_size_in_gb: o.get_field("diskSizeInGb"),
            dynamic_enabled: o.get_field("dynamicEnabled"),
            hyperv_generation: o.get_field("hypervGeneration"),
            location: o.get_field("location"),
            logical_sector_in_bytes: o.get_field("logicalSectorInBytes"),
            name: o.get_field("name"),
            physical_sector_in_bytes: o.get_field("physicalSectorInBytes"),
            resource_group_name: o.get_field("resourceGroupName"),
            storage_path_id: o.get_field("storagePathId"),
            tags: o.get_field("tags"),
        }
    }
}
