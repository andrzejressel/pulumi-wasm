/// Manages a managed disk.
///
/// ## Example Usage
///
/// ### With Create Empty
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleManagedDisk:
///     type: azure:compute:ManagedDisk
///     name: example
///     properties:
///       name: acctestmd
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       storageAccountType: Standard_LRS
///       createOption: Empty
///       diskSizeGb: '1'
///       tags:
///         environment: staging
/// ```
///
///
/// ### With Create Copy
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   source:
///     type: azure:compute:ManagedDisk
///     properties:
///       name: acctestmd1
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       storageAccountType: Standard_LRS
///       createOption: Empty
///       diskSizeGb: '1'
///       tags:
///         environment: staging
///   copy:
///     type: azure:compute:ManagedDisk
///     properties:
///       name: acctestmd2
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       storageAccountType: Standard_LRS
///       createOption: Copy
///       sourceResourceId: ${source.id}
///       diskSizeGb: '1'
///       tags:
///         environment: staging
/// ```
///
/// ## Import
///
/// Managed Disks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/managedDisk:ManagedDisk example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/disks/manageddisk1
/// ```
///
pub mod managed_disk {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedDiskArgs {
        /// The method to use when creating the managed disk. Changing this forces a new resource to be created. Possible values include:
        /// * `Import` - Import a VHD file in to the managed disk (VHD specified with `source_uri`).
        /// * `ImportSecure` - Securely import a VHD file in to the managed disk (VHD specified with `source_uri`).
        /// * `Empty` - Create an empty managed disk.
        /// * `Copy` - Copy an existing managed disk or snapshot (specified with `source_resource_id`).
        /// * `FromImage` - Copy a Platform Image (specified with `image_reference_id`)
        /// * `Restore` - Set by Azure Backup or Site Recovery on a restored disk (specified with `source_resource_id`).
        /// * `Upload` - Upload a VHD disk with the help of SAS URL (to be used with `upload_size_bytes`).
        #[builder(into)]
        pub create_option: pulumi_wasm_rust::Output<String>,
        /// The ID of the disk access resource for using private endpoints on disks.
        ///
        /// > **Note:** `disk_access_id` is only supported when `network_access_policy` is set to `AllowPrivate`.
        #[builder(into, default)]
        pub disk_access_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of a Disk Encryption Set which should be used to encrypt this Managed Disk. Conflicts with `secure_vm_disk_encryption_set_id`.
        ///
        /// > **NOTE:** The Disk Encryption Set must have the `Reader` Role Assignment scoped on the Key Vault - in addition to an Access Policy to the Key Vault
        ///
        /// > **NOTE:** Disk Encryption Sets are in Public Preview in a limited set of regions
        #[builder(into, default)]
        pub disk_encryption_set_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of IOPS allowed across all VMs mounting the shared disk as read-only; only settable for UltraSSD disks and PremiumV2 disks with shared disk enabled. One operation can transfer between 4k and 256k bytes.
        #[builder(into, default)]
        pub disk_iops_read_only: pulumi_wasm_rust::Output<Option<i32>>,
        /// The number of IOPS allowed for this disk; only settable for UltraSSD disks and PremiumV2 disks. One operation can transfer between 4k and 256k bytes.
        #[builder(into, default)]
        pub disk_iops_read_write: pulumi_wasm_rust::Output<Option<i32>>,
        /// The bandwidth allowed across all VMs mounting the shared disk as read-only; only settable for UltraSSD disks and PremiumV2 disks with shared disk enabled. MBps means millions of bytes per second.
        #[builder(into, default)]
        pub disk_mbps_read_only: pulumi_wasm_rust::Output<Option<i32>>,
        /// The bandwidth allowed for this disk; only settable for UltraSSD disks and PremiumV2 disks. MBps means millions of bytes per second.
        #[builder(into, default)]
        pub disk_mbps_read_write: pulumi_wasm_rust::Output<Option<i32>>,
        #[builder(into, default)]
        pub disk_size_gb: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the Edge Zone within the Azure Region where this Managed Disk should exist. Changing this forces a new Managed Disk to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// A `encryption_settings` block as defined below.
        ///
        /// > **NOTE:** Removing `encryption_settings` forces a new resource to be created.
        #[builder(into, default)]
        pub encryption_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::ManagedDiskEncryptionSettings>,
        >,
        /// ID of a Gallery Image Version to copy when `create_option` is `FromImage`. This field cannot be specified if image_reference_id is specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub gallery_image_reference_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The HyperV Generation of the Disk when the source of an `Import` or `Copy` operation targets a source that contains an operating system. Possible values are `V1` and `V2`. For `ImportSecure` it must be set to `V2`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub hyper_v_generation: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of an existing platform/marketplace disk image to copy when `create_option` is `FromImage`. This field cannot be specified if gallery_image_reference_id is specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub image_reference_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specified the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Logical Sector Size. Possible values are: `512` and `4096`. Defaults to `4096`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Setting logical sector size is supported only with `UltraSSD_LRS` disks and `PremiumV2_LRS` disks.
        #[builder(into, default)]
        pub logical_sector_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// The maximum number of VMs that can attach to the disk at the same time. Value greater than one indicates a disk that can be mounted on multiple VMs at the same time.
        ///
        /// > **Note:** Premium SSD maxShares limit: `P15` and `P20` disks: 2. `P30`,`P40`,`P50` disks: 5. `P60`,`P70`,`P80` disks: 10. For ultra disks the `max_shares` minimum value is 1 and the maximum is 5.
        #[builder(into, default)]
        pub max_shares: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the Managed Disk. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Policy for accessing the disk via network. Allowed values are `AllowAll`, `AllowPrivate`, and `DenyAll`.
        #[builder(into, default)]
        pub network_access_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies if On-Demand Bursting is enabled for the Managed Disk.
        ///
        /// > **Note:** Credit-Based Bursting is enabled by default on all eligible disks. More information on [Credit-Based and On-Demand Bursting can be found in the documentation](https://docs.microsoft.com/azure/virtual-machines/disk-bursting#disk-level-bursting).
        #[builder(into, default)]
        pub on_demand_bursting_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether this Managed Disk should be optimized for frequent disk attachments (where a disk is attached/detached more than 5 times in a day). Defaults to `false`.
        ///
        /// > **Note:** Setting `optimized_frequent_attach_enabled` to `true` causes the disks to not align with the fault domain of the Virtual Machine, which can have operational implications.
        #[builder(into, default)]
        pub optimized_frequent_attach_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specify a value when the source of an `Import`, `ImportSecure` or `Copy` operation targets a source that contains an operating system. Valid values are `Linux` or `Windows`.
        #[builder(into, default)]
        pub os_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether Performance Plus is enabled for this Managed Disk. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub performance_plus_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether it is allowed to access the disk via public network. Defaults to `true`.
        ///
        /// For more information on managed disks, such as sizing options and pricing, please check out the [Azure Documentation](https://docs.microsoft.com/azure/storage/storage-managed-disks-overview).
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Resource Group where the Managed Disk should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Disk Encryption Set which should be used to Encrypt this OS Disk when the Virtual Machine is a Confidential VM. Conflicts with `disk_encryption_set_id`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `secure_vm_disk_encryption_set_id` can only be specified when `security_type` is set to `ConfidentialVM_DiskEncryptedWithCustomerKey`.
        #[builder(into, default)]
        pub secure_vm_disk_encryption_set_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Security Type of the Managed Disk when it is used for a Confidential VM. Possible values are `ConfidentialVM_VMGuestStateOnlyEncryptedWithPlatformKey`, `ConfidentialVM_DiskEncryptedWithPlatformKey` and `ConfidentialVM_DiskEncryptedWithCustomerKey`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** When `security_type` is set to `ConfidentialVM_DiskEncryptedWithCustomerKey` the value of `create_option` must be one of `FromImage` or `ImportSecure`.
        ///
        ///
        /// > **NOTE:** `security_type` cannot be specified when `trusted_launch_enabled` is set to true.
        ///
        /// > **NOTE:** `secure_vm_disk_encryption_set_id` must be specified when `security_type` is set to `ConfidentialVM_DiskEncryptedWithCustomerKey`.
        #[builder(into, default)]
        pub security_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of an existing Managed Disk or Snapshot to copy when `create_option` is `Copy` or the recovery point to restore when `create_option` is `Restore`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_resource_id: pulumi_wasm_rust::Output<Option<String>>,
        /// URI to a valid VHD file to be used when `create_option` is `Import` or `ImportSecure`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Storage Account where the `source_uri` is located. Required when `create_option` is set to `Import` or `ImportSecure`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub storage_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of storage to use for the managed disk. Possible values are `Standard_LRS`, `StandardSSD_ZRS`, `Premium_LRS`, `PremiumV2_LRS`, `Premium_ZRS`, `StandardSSD_LRS` or `UltraSSD_LRS`.
        ///
        /// > **Note:** Azure Ultra Disk Storage is only available in a region that support availability zones and can only enabled on the following VM series: `ESv3`, `DSv3`, `FSv3`, `LSv2`, `M` and `Mv2`. For more information see the `Azure Ultra Disk Storage` [product documentation](https://docs.microsoft.com/azure/virtual-machines/windows/disks-enable-ultra-ssd).
        #[builder(into)]
        pub storage_account_type: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub tier: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies if Trusted Launch is enabled for the Managed Disk. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Trusted Launch can only be enabled when `create_option` is `FromImage` or `Import`.
        #[builder(into, default)]
        pub trusted_launch_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the size of the managed disk to create in bytes. Required when `create_option` is `Upload`. The value must be equal to the source disk to be copied in bytes. Source disk size could be calculated with `ls -l` or `wc -c`. More information can be found at [Copy a managed disk](https://learn.microsoft.com/en-us/azure/virtual-machines/linux/disks-upload-vhd-to-managed-disk-cli#copy-a-managed-disk). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub upload_size_bytes: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the Availability Zone in which this Managed Disk should be located. Changing this property forces a new resource to be created.
        ///
        /// > **Note:** Availability Zones are [only supported in select regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ManagedDiskResult {
        /// The method to use when creating the managed disk. Changing this forces a new resource to be created. Possible values include:
        /// * `Import` - Import a VHD file in to the managed disk (VHD specified with `source_uri`).
        /// * `ImportSecure` - Securely import a VHD file in to the managed disk (VHD specified with `source_uri`).
        /// * `Empty` - Create an empty managed disk.
        /// * `Copy` - Copy an existing managed disk or snapshot (specified with `source_resource_id`).
        /// * `FromImage` - Copy a Platform Image (specified with `image_reference_id`)
        /// * `Restore` - Set by Azure Backup or Site Recovery on a restored disk (specified with `source_resource_id`).
        /// * `Upload` - Upload a VHD disk with the help of SAS URL (to be used with `upload_size_bytes`).
        pub create_option: pulumi_wasm_rust::Output<String>,
        /// The ID of the disk access resource for using private endpoints on disks.
        ///
        /// > **Note:** `disk_access_id` is only supported when `network_access_policy` is set to `AllowPrivate`.
        pub disk_access_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of a Disk Encryption Set which should be used to encrypt this Managed Disk. Conflicts with `secure_vm_disk_encryption_set_id`.
        ///
        /// > **NOTE:** The Disk Encryption Set must have the `Reader` Role Assignment scoped on the Key Vault - in addition to an Access Policy to the Key Vault
        ///
        /// > **NOTE:** Disk Encryption Sets are in Public Preview in a limited set of regions
        pub disk_encryption_set_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of IOPS allowed across all VMs mounting the shared disk as read-only; only settable for UltraSSD disks and PremiumV2 disks with shared disk enabled. One operation can transfer between 4k and 256k bytes.
        pub disk_iops_read_only: pulumi_wasm_rust::Output<i32>,
        /// The number of IOPS allowed for this disk; only settable for UltraSSD disks and PremiumV2 disks. One operation can transfer between 4k and 256k bytes.
        pub disk_iops_read_write: pulumi_wasm_rust::Output<i32>,
        /// The bandwidth allowed across all VMs mounting the shared disk as read-only; only settable for UltraSSD disks and PremiumV2 disks with shared disk enabled. MBps means millions of bytes per second.
        pub disk_mbps_read_only: pulumi_wasm_rust::Output<i32>,
        /// The bandwidth allowed for this disk; only settable for UltraSSD disks and PremiumV2 disks. MBps means millions of bytes per second.
        pub disk_mbps_read_write: pulumi_wasm_rust::Output<i32>,
        pub disk_size_gb: pulumi_wasm_rust::Output<i32>,
        /// Specifies the Edge Zone within the Azure Region where this Managed Disk should exist. Changing this forces a new Managed Disk to be created.
        pub edge_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// A `encryption_settings` block as defined below.
        ///
        /// > **NOTE:** Removing `encryption_settings` forces a new resource to be created.
        pub encryption_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::ManagedDiskEncryptionSettings>,
        >,
        /// ID of a Gallery Image Version to copy when `create_option` is `FromImage`. This field cannot be specified if image_reference_id is specified. Changing this forces a new resource to be created.
        pub gallery_image_reference_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The HyperV Generation of the Disk when the source of an `Import` or `Copy` operation targets a source that contains an operating system. Possible values are `V1` and `V2`. For `ImportSecure` it must be set to `V2`. Changing this forces a new resource to be created.
        pub hyper_v_generation: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of an existing platform/marketplace disk image to copy when `create_option` is `FromImage`. This field cannot be specified if gallery_image_reference_id is specified. Changing this forces a new resource to be created.
        pub image_reference_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specified the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Logical Sector Size. Possible values are: `512` and `4096`. Defaults to `4096`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Setting logical sector size is supported only with `UltraSSD_LRS` disks and `PremiumV2_LRS` disks.
        pub logical_sector_size: pulumi_wasm_rust::Output<i32>,
        /// The maximum number of VMs that can attach to the disk at the same time. Value greater than one indicates a disk that can be mounted on multiple VMs at the same time.
        ///
        /// > **Note:** Premium SSD maxShares limit: `P15` and `P20` disks: 2. `P30`,`P40`,`P50` disks: 5. `P60`,`P70`,`P80` disks: 10. For ultra disks the `max_shares` minimum value is 1 and the maximum is 5.
        pub max_shares: pulumi_wasm_rust::Output<i32>,
        /// Specifies the name of the Managed Disk. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Policy for accessing the disk via network. Allowed values are `AllowAll`, `AllowPrivate`, and `DenyAll`.
        pub network_access_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies if On-Demand Bursting is enabled for the Managed Disk.
        ///
        /// > **Note:** Credit-Based Bursting is enabled by default on all eligible disks. More information on [Credit-Based and On-Demand Bursting can be found in the documentation](https://docs.microsoft.com/azure/virtual-machines/disk-bursting#disk-level-bursting).
        pub on_demand_bursting_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether this Managed Disk should be optimized for frequent disk attachments (where a disk is attached/detached more than 5 times in a day). Defaults to `false`.
        ///
        /// > **Note:** Setting `optimized_frequent_attach_enabled` to `true` causes the disks to not align with the fault domain of the Virtual Machine, which can have operational implications.
        pub optimized_frequent_attach_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specify a value when the source of an `Import`, `ImportSecure` or `Copy` operation targets a source that contains an operating system. Valid values are `Linux` or `Windows`.
        pub os_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether Performance Plus is enabled for this Managed Disk. Defaults to `false`. Changing this forces a new resource to be created.
        pub performance_plus_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether it is allowed to access the disk via public network. Defaults to `true`.
        ///
        /// For more information on managed disks, such as sizing options and pricing, please check out the [Azure Documentation](https://docs.microsoft.com/azure/storage/storage-managed-disks-overview).
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Resource Group where the Managed Disk should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Disk Encryption Set which should be used to Encrypt this OS Disk when the Virtual Machine is a Confidential VM. Conflicts with `disk_encryption_set_id`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `secure_vm_disk_encryption_set_id` can only be specified when `security_type` is set to `ConfidentialVM_DiskEncryptedWithCustomerKey`.
        pub secure_vm_disk_encryption_set_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Security Type of the Managed Disk when it is used for a Confidential VM. Possible values are `ConfidentialVM_VMGuestStateOnlyEncryptedWithPlatformKey`, `ConfidentialVM_DiskEncryptedWithPlatformKey` and `ConfidentialVM_DiskEncryptedWithCustomerKey`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** When `security_type` is set to `ConfidentialVM_DiskEncryptedWithCustomerKey` the value of `create_option` must be one of `FromImage` or `ImportSecure`.
        ///
        ///
        /// > **NOTE:** `security_type` cannot be specified when `trusted_launch_enabled` is set to true.
        ///
        /// > **NOTE:** `secure_vm_disk_encryption_set_id` must be specified when `security_type` is set to `ConfidentialVM_DiskEncryptedWithCustomerKey`.
        pub security_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of an existing Managed Disk or Snapshot to copy when `create_option` is `Copy` or the recovery point to restore when `create_option` is `Restore`. Changing this forces a new resource to be created.
        pub source_resource_id: pulumi_wasm_rust::Output<Option<String>>,
        /// URI to a valid VHD file to be used when `create_option` is `Import` or `ImportSecure`. Changing this forces a new resource to be created.
        pub source_uri: pulumi_wasm_rust::Output<String>,
        /// The ID of the Storage Account where the `source_uri` is located. Required when `create_option` is set to `Import` or `ImportSecure`. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of storage to use for the managed disk. Possible values are `Standard_LRS`, `StandardSSD_ZRS`, `Premium_LRS`, `PremiumV2_LRS`, `Premium_ZRS`, `StandardSSD_LRS` or `UltraSSD_LRS`.
        ///
        /// > **Note:** Azure Ultra Disk Storage is only available in a region that support availability zones and can only enabled on the following VM series: `ESv3`, `DSv3`, `FSv3`, `LSv2`, `M` and `Mv2`. For more information see the `Azure Ultra Disk Storage` [product documentation](https://docs.microsoft.com/azure/virtual-machines/windows/disks-enable-ultra-ssd).
        pub storage_account_type: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tier: pulumi_wasm_rust::Output<String>,
        /// Specifies if Trusted Launch is enabled for the Managed Disk. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Trusted Launch can only be enabled when `create_option` is `FromImage` or `Import`.
        pub trusted_launch_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the size of the managed disk to create in bytes. Required when `create_option` is `Upload`. The value must be equal to the source disk to be copied in bytes. Source disk size could be calculated with `ls -l` or `wc -c`. More information can be found at [Copy a managed disk](https://learn.microsoft.com/en-us/azure/virtual-machines/linux/disks-upload-vhd-to-managed-disk-cli#copy-a-managed-disk). Changing this forces a new resource to be created.
        pub upload_size_bytes: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the Availability Zone in which this Managed Disk should be located. Changing this property forces a new resource to be created.
        ///
        /// > **Note:** Availability Zones are [only supported in select regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ManagedDiskArgs) -> ManagedDiskResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let create_option_binding = args.create_option.get_inner();
        let disk_access_id_binding = args.disk_access_id.get_inner();
        let disk_encryption_set_id_binding = args.disk_encryption_set_id.get_inner();
        let disk_iops_read_only_binding = args.disk_iops_read_only.get_inner();
        let disk_iops_read_write_binding = args.disk_iops_read_write.get_inner();
        let disk_mbps_read_only_binding = args.disk_mbps_read_only.get_inner();
        let disk_mbps_read_write_binding = args.disk_mbps_read_write.get_inner();
        let disk_size_gb_binding = args.disk_size_gb.get_inner();
        let edge_zone_binding = args.edge_zone.get_inner();
        let encryption_settings_binding = args.encryption_settings.get_inner();
        let gallery_image_reference_id_binding = args
            .gallery_image_reference_id
            .get_inner();
        let hyper_v_generation_binding = args.hyper_v_generation.get_inner();
        let image_reference_id_binding = args.image_reference_id.get_inner();
        let location_binding = args.location.get_inner();
        let logical_sector_size_binding = args.logical_sector_size.get_inner();
        let max_shares_binding = args.max_shares.get_inner();
        let name_binding = args.name.get_inner();
        let network_access_policy_binding = args.network_access_policy.get_inner();
        let on_demand_bursting_enabled_binding = args
            .on_demand_bursting_enabled
            .get_inner();
        let optimized_frequent_attach_enabled_binding = args
            .optimized_frequent_attach_enabled
            .get_inner();
        let os_type_binding = args.os_type.get_inner();
        let performance_plus_enabled_binding = args.performance_plus_enabled.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let secure_vm_disk_encryption_set_id_binding = args
            .secure_vm_disk_encryption_set_id
            .get_inner();
        let security_type_binding = args.security_type.get_inner();
        let source_resource_id_binding = args.source_resource_id.get_inner();
        let source_uri_binding = args.source_uri.get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let storage_account_type_binding = args.storage_account_type.get_inner();
        let tags_binding = args.tags.get_inner();
        let tier_binding = args.tier.get_inner();
        let trusted_launch_enabled_binding = args.trusted_launch_enabled.get_inner();
        let upload_size_bytes_binding = args.upload_size_bytes.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/managedDisk:ManagedDisk".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "createOption".into(),
                    value: &create_option_binding,
                },
                register_interface::ObjectField {
                    name: "diskAccessId".into(),
                    value: &disk_access_id_binding,
                },
                register_interface::ObjectField {
                    name: "diskEncryptionSetId".into(),
                    value: &disk_encryption_set_id_binding,
                },
                register_interface::ObjectField {
                    name: "diskIopsReadOnly".into(),
                    value: &disk_iops_read_only_binding,
                },
                register_interface::ObjectField {
                    name: "diskIopsReadWrite".into(),
                    value: &disk_iops_read_write_binding,
                },
                register_interface::ObjectField {
                    name: "diskMbpsReadOnly".into(),
                    value: &disk_mbps_read_only_binding,
                },
                register_interface::ObjectField {
                    name: "diskMbpsReadWrite".into(),
                    value: &disk_mbps_read_write_binding,
                },
                register_interface::ObjectField {
                    name: "diskSizeGb".into(),
                    value: &disk_size_gb_binding,
                },
                register_interface::ObjectField {
                    name: "edgeZone".into(),
                    value: &edge_zone_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionSettings".into(),
                    value: &encryption_settings_binding,
                },
                register_interface::ObjectField {
                    name: "galleryImageReferenceId".into(),
                    value: &gallery_image_reference_id_binding,
                },
                register_interface::ObjectField {
                    name: "hyperVGeneration".into(),
                    value: &hyper_v_generation_binding,
                },
                register_interface::ObjectField {
                    name: "imageReferenceId".into(),
                    value: &image_reference_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "logicalSectorSize".into(),
                    value: &logical_sector_size_binding,
                },
                register_interface::ObjectField {
                    name: "maxShares".into(),
                    value: &max_shares_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkAccessPolicy".into(),
                    value: &network_access_policy_binding,
                },
                register_interface::ObjectField {
                    name: "onDemandBurstingEnabled".into(),
                    value: &on_demand_bursting_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "optimizedFrequentAttachEnabled".into(),
                    value: &optimized_frequent_attach_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "osType".into(),
                    value: &os_type_binding,
                },
                register_interface::ObjectField {
                    name: "performancePlusEnabled".into(),
                    value: &performance_plus_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "secureVmDiskEncryptionSetId".into(),
                    value: &secure_vm_disk_encryption_set_id_binding,
                },
                register_interface::ObjectField {
                    name: "securityType".into(),
                    value: &security_type_binding,
                },
                register_interface::ObjectField {
                    name: "sourceResourceId".into(),
                    value: &source_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "sourceUri".into(),
                    value: &source_uri_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountType".into(),
                    value: &storage_account_type_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tier".into(),
                    value: &tier_binding,
                },
                register_interface::ObjectField {
                    name: "trustedLaunchEnabled".into(),
                    value: &trusted_launch_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "uploadSizeBytes".into(),
                    value: &upload_size_bytes_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createOption".into(),
                },
                register_interface::ResultField {
                    name: "diskAccessId".into(),
                },
                register_interface::ResultField {
                    name: "diskEncryptionSetId".into(),
                },
                register_interface::ResultField {
                    name: "diskIopsReadOnly".into(),
                },
                register_interface::ResultField {
                    name: "diskIopsReadWrite".into(),
                },
                register_interface::ResultField {
                    name: "diskMbpsReadOnly".into(),
                },
                register_interface::ResultField {
                    name: "diskMbpsReadWrite".into(),
                },
                register_interface::ResultField {
                    name: "diskSizeGb".into(),
                },
                register_interface::ResultField {
                    name: "edgeZone".into(),
                },
                register_interface::ResultField {
                    name: "encryptionSettings".into(),
                },
                register_interface::ResultField {
                    name: "galleryImageReferenceId".into(),
                },
                register_interface::ResultField {
                    name: "hyperVGeneration".into(),
                },
                register_interface::ResultField {
                    name: "imageReferenceId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "logicalSectorSize".into(),
                },
                register_interface::ResultField {
                    name: "maxShares".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkAccessPolicy".into(),
                },
                register_interface::ResultField {
                    name: "onDemandBurstingEnabled".into(),
                },
                register_interface::ResultField {
                    name: "optimizedFrequentAttachEnabled".into(),
                },
                register_interface::ResultField {
                    name: "osType".into(),
                },
                register_interface::ResultField {
                    name: "performancePlusEnabled".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secureVmDiskEncryptionSetId".into(),
                },
                register_interface::ResultField {
                    name: "securityType".into(),
                },
                register_interface::ResultField {
                    name: "sourceResourceId".into(),
                },
                register_interface::ResultField {
                    name: "sourceUri".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountType".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tier".into(),
                },
                register_interface::ResultField {
                    name: "trustedLaunchEnabled".into(),
                },
                register_interface::ResultField {
                    name: "uploadSizeBytes".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedDiskResult {
            create_option: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createOption").unwrap(),
            ),
            disk_access_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskAccessId").unwrap(),
            ),
            disk_encryption_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskEncryptionSetId").unwrap(),
            ),
            disk_iops_read_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskIopsReadOnly").unwrap(),
            ),
            disk_iops_read_write: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskIopsReadWrite").unwrap(),
            ),
            disk_mbps_read_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskMbpsReadOnly").unwrap(),
            ),
            disk_mbps_read_write: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskMbpsReadWrite").unwrap(),
            ),
            disk_size_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskSizeGb").unwrap(),
            ),
            edge_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeZone").unwrap(),
            ),
            encryption_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionSettings").unwrap(),
            ),
            gallery_image_reference_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("galleryImageReferenceId").unwrap(),
            ),
            hyper_v_generation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hyperVGeneration").unwrap(),
            ),
            image_reference_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageReferenceId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            logical_sector_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logicalSectorSize").unwrap(),
            ),
            max_shares: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxShares").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_access_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkAccessPolicy").unwrap(),
            ),
            on_demand_bursting_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("onDemandBurstingEnabled").unwrap(),
            ),
            optimized_frequent_attach_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("optimizedFrequentAttachEnabled").unwrap(),
            ),
            os_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osType").unwrap(),
            ),
            performance_plus_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("performancePlusEnabled").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secure_vm_disk_encryption_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secureVmDiskEncryptionSetId").unwrap(),
            ),
            security_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityType").unwrap(),
            ),
            source_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceResourceId").unwrap(),
            ),
            source_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceUri").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
            storage_account_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountType").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tier").unwrap(),
            ),
            trusted_launch_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustedLaunchEnabled").unwrap(),
            ),
            upload_size_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uploadSizeBytes").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
