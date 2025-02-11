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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod managed_disk {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
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
        pub create_option: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the disk access resource for using private endpoints on disks.
        ///
        /// > **Note:** `disk_access_id` is only supported when `network_access_policy` is set to `AllowPrivate`.
        #[builder(into, default)]
        pub disk_access_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of a Disk Encryption Set which should be used to encrypt this Managed Disk. Conflicts with `secure_vm_disk_encryption_set_id`.
        ///
        /// > **NOTE:** The Disk Encryption Set must have the `Reader` Role Assignment scoped on the Key Vault - in addition to an Access Policy to the Key Vault
        ///
        /// > **NOTE:** Disk Encryption Sets are in Public Preview in a limited set of regions
        #[builder(into, default)]
        pub disk_encryption_set_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of IOPS allowed across all VMs mounting the shared disk as read-only; only settable for UltraSSD disks and PremiumV2 disks with shared disk enabled. One operation can transfer between 4k and 256k bytes.
        #[builder(into, default)]
        pub disk_iops_read_only: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The number of IOPS allowed for this disk; only settable for UltraSSD disks and PremiumV2 disks. One operation can transfer between 4k and 256k bytes.
        #[builder(into, default)]
        pub disk_iops_read_write: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The bandwidth allowed across all VMs mounting the shared disk as read-only; only settable for UltraSSD disks and PremiumV2 disks with shared disk enabled. MBps means millions of bytes per second.
        #[builder(into, default)]
        pub disk_mbps_read_only: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The bandwidth allowed for this disk; only settable for UltraSSD disks and PremiumV2 disks. MBps means millions of bytes per second.
        #[builder(into, default)]
        pub disk_mbps_read_write: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub disk_size_gb: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the Edge Zone within the Azure Region where this Managed Disk should exist. Changing this forces a new Managed Disk to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `encryption_settings` block as defined below.
        ///
        /// > **NOTE:** Removing `encryption_settings` forces a new resource to be created.
        #[builder(into, default)]
        pub encryption_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::ManagedDiskEncryptionSettings>,
        >,
        /// ID of a Gallery Image Version to copy when `create_option` is `FromImage`. This field cannot be specified if image_reference_id is specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub gallery_image_reference_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The HyperV Generation of the Disk when the source of an `Import` or `Copy` operation targets a source that contains an operating system. Possible values are `V1` and `V2`. For `ImportSecure` it must be set to `V2`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub hyper_v_generation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of an existing platform/marketplace disk image to copy when `create_option` is `FromImage`. This field cannot be specified if gallery_image_reference_id is specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub image_reference_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specified the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Logical Sector Size. Possible values are: `512` and `4096`. Defaults to `4096`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Setting logical sector size is supported only with `UltraSSD_LRS` disks and `PremiumV2_LRS` disks.
        #[builder(into, default)]
        pub logical_sector_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The maximum number of VMs that can attach to the disk at the same time. Value greater than one indicates a disk that can be mounted on multiple VMs at the same time.
        ///
        /// > **Note:** Premium SSD maxShares limit: `P15` and `P20` disks: 2. `P30`,`P40`,`P50` disks: 5. `P60`,`P70`,`P80` disks: 10. For ultra disks the `max_shares` minimum value is 1 and the maximum is 5.
        #[builder(into, default)]
        pub max_shares: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the name of the Managed Disk. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Policy for accessing the disk via network. Allowed values are `AllowAll`, `AllowPrivate`, and `DenyAll`.
        #[builder(into, default)]
        pub network_access_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if On-Demand Bursting is enabled for the Managed Disk.
        ///
        /// > **Note:** Credit-Based Bursting is enabled by default on all eligible disks. More information on [Credit-Based and On-Demand Bursting can be found in the documentation](https://docs.microsoft.com/azure/virtual-machines/disk-bursting#disk-level-bursting).
        #[builder(into, default)]
        pub on_demand_bursting_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether this Managed Disk should be optimized for frequent disk attachments (where a disk is attached/detached more than 5 times in a day). Defaults to `false`.
        ///
        /// > **Note:** Setting `optimized_frequent_attach_enabled` to `true` causes the disks to not align with the fault domain of the Virtual Machine, which can have operational implications.
        #[builder(into, default)]
        pub optimized_frequent_attach_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specify a value when the source of an `Import`, `ImportSecure` or `Copy` operation targets a source that contains an operating system. Valid values are `Linux` or `Windows`.
        #[builder(into, default)]
        pub os_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether Performance Plus is enabled for this Managed Disk. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub performance_plus_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether it is allowed to access the disk via public network. Defaults to `true`.
        ///
        /// For more information on managed disks, such as sizing options and pricing, please check out the [Azure Documentation](https://docs.microsoft.com/azure/storage/storage-managed-disks-overview).
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the Resource Group where the Managed Disk should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Disk Encryption Set which should be used to Encrypt this OS Disk when the Virtual Machine is a Confidential VM. Conflicts with `disk_encryption_set_id`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `secure_vm_disk_encryption_set_id` can only be specified when `security_type` is set to `ConfidentialVM_DiskEncryptedWithCustomerKey`.
        #[builder(into, default)]
        pub secure_vm_disk_encryption_set_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Security Type of the Managed Disk when it is used for a Confidential VM. Possible values are `ConfidentialVM_VMGuestStateOnlyEncryptedWithPlatformKey`, `ConfidentialVM_DiskEncryptedWithPlatformKey` and `ConfidentialVM_DiskEncryptedWithCustomerKey`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** When `security_type` is set to `ConfidentialVM_DiskEncryptedWithCustomerKey` the value of `create_option` must be one of `FromImage` or `ImportSecure`.
        ///
        ///
        /// > **NOTE:** `security_type` cannot be specified when `trusted_launch_enabled` is set to true.
        ///
        /// > **NOTE:** `secure_vm_disk_encryption_set_id` must be specified when `security_type` is set to `ConfidentialVM_DiskEncryptedWithCustomerKey`.
        #[builder(into, default)]
        pub security_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of an existing Managed Disk or Snapshot to copy when `create_option` is `Copy` or the recovery point to restore when `create_option` is `Restore`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_resource_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URI to a valid VHD file to be used when `create_option` is `Import` or `ImportSecure`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Storage Account where the `source_uri` is located. Required when `create_option` is set to `Import` or `ImportSecure`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of storage to use for the managed disk. Possible values are `Standard_LRS`, `StandardSSD_ZRS`, `Premium_LRS`, `PremiumV2_LRS`, `Premium_ZRS`, `StandardSSD_LRS` or `UltraSSD_LRS`.
        ///
        /// > **Note:** Azure Ultra Disk Storage is only available in a region that support availability zones and can only enabled on the following VM series: `ESv3`, `DSv3`, `FSv3`, `LSv2`, `M` and `Mv2`. For more information see the `Azure Ultra Disk Storage` [product documentation](https://docs.microsoft.com/azure/virtual-machines/windows/disks-enable-ultra-ssd).
        #[builder(into)]
        pub storage_account_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if Trusted Launch is enabled for the Managed Disk. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Trusted Launch can only be enabled when `create_option` is `FromImage` or `Import`.
        #[builder(into, default)]
        pub trusted_launch_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the size of the managed disk to create in bytes. Required when `create_option` is `Upload`. The value must be equal to the source disk to be copied in bytes. Source disk size could be calculated with `ls -l` or `wc -c`. More information can be found at [Copy a managed disk](https://learn.microsoft.com/en-us/azure/virtual-machines/linux/disks-upload-vhd-to-managed-disk-cli#copy-a-managed-disk). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub upload_size_bytes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the Availability Zone in which this Managed Disk should be located. Changing this property forces a new resource to be created.
        ///
        /// > **Note:** Availability Zones are [only supported in select regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
        pub create_option: pulumi_gestalt_rust::Output<String>,
        /// The ID of the disk access resource for using private endpoints on disks.
        ///
        /// > **Note:** `disk_access_id` is only supported when `network_access_policy` is set to `AllowPrivate`.
        pub disk_access_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of a Disk Encryption Set which should be used to encrypt this Managed Disk. Conflicts with `secure_vm_disk_encryption_set_id`.
        ///
        /// > **NOTE:** The Disk Encryption Set must have the `Reader` Role Assignment scoped on the Key Vault - in addition to an Access Policy to the Key Vault
        ///
        /// > **NOTE:** Disk Encryption Sets are in Public Preview in a limited set of regions
        pub disk_encryption_set_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The number of IOPS allowed across all VMs mounting the shared disk as read-only; only settable for UltraSSD disks and PremiumV2 disks with shared disk enabled. One operation can transfer between 4k and 256k bytes.
        pub disk_iops_read_only: pulumi_gestalt_rust::Output<i32>,
        /// The number of IOPS allowed for this disk; only settable for UltraSSD disks and PremiumV2 disks. One operation can transfer between 4k and 256k bytes.
        pub disk_iops_read_write: pulumi_gestalt_rust::Output<i32>,
        /// The bandwidth allowed across all VMs mounting the shared disk as read-only; only settable for UltraSSD disks and PremiumV2 disks with shared disk enabled. MBps means millions of bytes per second.
        pub disk_mbps_read_only: pulumi_gestalt_rust::Output<i32>,
        /// The bandwidth allowed for this disk; only settable for UltraSSD disks and PremiumV2 disks. MBps means millions of bytes per second.
        pub disk_mbps_read_write: pulumi_gestalt_rust::Output<i32>,
        pub disk_size_gb: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the Edge Zone within the Azure Region where this Managed Disk should exist. Changing this forces a new Managed Disk to be created.
        pub edge_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `encryption_settings` block as defined below.
        ///
        /// > **NOTE:** Removing `encryption_settings` forces a new resource to be created.
        pub encryption_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::ManagedDiskEncryptionSettings>,
        >,
        /// ID of a Gallery Image Version to copy when `create_option` is `FromImage`. This field cannot be specified if image_reference_id is specified. Changing this forces a new resource to be created.
        pub gallery_image_reference_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The HyperV Generation of the Disk when the source of an `Import` or `Copy` operation targets a source that contains an operating system. Possible values are `V1` and `V2`. For `ImportSecure` it must be set to `V2`. Changing this forces a new resource to be created.
        pub hyper_v_generation: pulumi_gestalt_rust::Output<Option<String>>,
        /// ID of an existing platform/marketplace disk image to copy when `create_option` is `FromImage`. This field cannot be specified if gallery_image_reference_id is specified. Changing this forces a new resource to be created.
        pub image_reference_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specified the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Logical Sector Size. Possible values are: `512` and `4096`. Defaults to `4096`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Setting logical sector size is supported only with `UltraSSD_LRS` disks and `PremiumV2_LRS` disks.
        pub logical_sector_size: pulumi_gestalt_rust::Output<i32>,
        /// The maximum number of VMs that can attach to the disk at the same time. Value greater than one indicates a disk that can be mounted on multiple VMs at the same time.
        ///
        /// > **Note:** Premium SSD maxShares limit: `P15` and `P20` disks: 2. `P30`,`P40`,`P50` disks: 5. `P60`,`P70`,`P80` disks: 10. For ultra disks the `max_shares` minimum value is 1 and the maximum is 5.
        pub max_shares: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the name of the Managed Disk. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Policy for accessing the disk via network. Allowed values are `AllowAll`, `AllowPrivate`, and `DenyAll`.
        pub network_access_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies if On-Demand Bursting is enabled for the Managed Disk.
        ///
        /// > **Note:** Credit-Based Bursting is enabled by default on all eligible disks. More information on [Credit-Based and On-Demand Bursting can be found in the documentation](https://docs.microsoft.com/azure/virtual-machines/disk-bursting#disk-level-bursting).
        pub on_demand_bursting_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether this Managed Disk should be optimized for frequent disk attachments (where a disk is attached/detached more than 5 times in a day). Defaults to `false`.
        ///
        /// > **Note:** Setting `optimized_frequent_attach_enabled` to `true` causes the disks to not align with the fault domain of the Virtual Machine, which can have operational implications.
        pub optimized_frequent_attach_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specify a value when the source of an `Import`, `ImportSecure` or `Copy` operation targets a source that contains an operating system. Valid values are `Linux` or `Windows`.
        pub os_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether Performance Plus is enabled for this Managed Disk. Defaults to `false`. Changing this forces a new resource to be created.
        pub performance_plus_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether it is allowed to access the disk via public network. Defaults to `true`.
        ///
        /// For more information on managed disks, such as sizing options and pricing, please check out the [Azure Documentation](https://docs.microsoft.com/azure/storage/storage-managed-disks-overview).
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Resource Group where the Managed Disk should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Disk Encryption Set which should be used to Encrypt this OS Disk when the Virtual Machine is a Confidential VM. Conflicts with `disk_encryption_set_id`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `secure_vm_disk_encryption_set_id` can only be specified when `security_type` is set to `ConfidentialVM_DiskEncryptedWithCustomerKey`.
        pub secure_vm_disk_encryption_set_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Security Type of the Managed Disk when it is used for a Confidential VM. Possible values are `ConfidentialVM_VMGuestStateOnlyEncryptedWithPlatformKey`, `ConfidentialVM_DiskEncryptedWithPlatformKey` and `ConfidentialVM_DiskEncryptedWithCustomerKey`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** When `security_type` is set to `ConfidentialVM_DiskEncryptedWithCustomerKey` the value of `create_option` must be one of `FromImage` or `ImportSecure`.
        ///
        ///
        /// > **NOTE:** `security_type` cannot be specified when `trusted_launch_enabled` is set to true.
        ///
        /// > **NOTE:** `secure_vm_disk_encryption_set_id` must be specified when `security_type` is set to `ConfidentialVM_DiskEncryptedWithCustomerKey`.
        pub security_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of an existing Managed Disk or Snapshot to copy when `create_option` is `Copy` or the recovery point to restore when `create_option` is `Restore`. Changing this forces a new resource to be created.
        pub source_resource_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// URI to a valid VHD file to be used when `create_option` is `Import` or `ImportSecure`. Changing this forces a new resource to be created.
        pub source_uri: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Storage Account where the `source_uri` is located. Required when `create_option` is set to `Import` or `ImportSecure`. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of storage to use for the managed disk. Possible values are `Standard_LRS`, `StandardSSD_ZRS`, `Premium_LRS`, `PremiumV2_LRS`, `Premium_ZRS`, `StandardSSD_LRS` or `UltraSSD_LRS`.
        ///
        /// > **Note:** Azure Ultra Disk Storage is only available in a region that support availability zones and can only enabled on the following VM series: `ESv3`, `DSv3`, `FSv3`, `LSv2`, `M` and `Mv2`. For more information see the `Azure Ultra Disk Storage` [product documentation](https://docs.microsoft.com/azure/virtual-machines/windows/disks-enable-ultra-ssd).
        pub storage_account_type: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tier: pulumi_gestalt_rust::Output<String>,
        /// Specifies if Trusted Launch is enabled for the Managed Disk. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Trusted Launch can only be enabled when `create_option` is `FromImage` or `Import`.
        pub trusted_launch_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the size of the managed disk to create in bytes. Required when `create_option` is `Upload`. The value must be equal to the source disk to be copied in bytes. Source disk size could be calculated with `ls -l` or `wc -c`. More information can be found at [Copy a managed disk](https://learn.microsoft.com/en-us/azure/virtual-machines/linux/disks-upload-vhd-to-managed-disk-cli#copy-a-managed-disk). Changing this forces a new resource to be created.
        pub upload_size_bytes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the Availability Zone in which this Managed Disk should be located. Changing this property forces a new resource to be created.
        ///
        /// > **Note:** Availability Zones are [only supported in select regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        pub zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedDiskArgs,
    ) -> ManagedDiskResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let create_option_binding = args.create_option.get_output(context);
        let disk_access_id_binding = args.disk_access_id.get_output(context);
        let disk_encryption_set_id_binding = args
            .disk_encryption_set_id
            .get_output(context);
        let disk_iops_read_only_binding = args.disk_iops_read_only.get_output(context);
        let disk_iops_read_write_binding = args.disk_iops_read_write.get_output(context);
        let disk_mbps_read_only_binding = args.disk_mbps_read_only.get_output(context);
        let disk_mbps_read_write_binding = args.disk_mbps_read_write.get_output(context);
        let disk_size_gb_binding = args.disk_size_gb.get_output(context);
        let edge_zone_binding = args.edge_zone.get_output(context);
        let encryption_settings_binding = args.encryption_settings.get_output(context);
        let gallery_image_reference_id_binding = args
            .gallery_image_reference_id
            .get_output(context);
        let hyper_v_generation_binding = args.hyper_v_generation.get_output(context);
        let image_reference_id_binding = args.image_reference_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let logical_sector_size_binding = args.logical_sector_size.get_output(context);
        let max_shares_binding = args.max_shares.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_access_policy_binding = args
            .network_access_policy
            .get_output(context);
        let on_demand_bursting_enabled_binding = args
            .on_demand_bursting_enabled
            .get_output(context);
        let optimized_frequent_attach_enabled_binding = args
            .optimized_frequent_attach_enabled
            .get_output(context);
        let os_type_binding = args.os_type.get_output(context);
        let performance_plus_enabled_binding = args
            .performance_plus_enabled
            .get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let secure_vm_disk_encryption_set_id_binding = args
            .secure_vm_disk_encryption_set_id
            .get_output(context);
        let security_type_binding = args.security_type.get_output(context);
        let source_resource_id_binding = args.source_resource_id.get_output(context);
        let source_uri_binding = args.source_uri.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let storage_account_type_binding = args.storage_account_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tier_binding = args.tier.get_output(context);
        let trusted_launch_enabled_binding = args
            .trusted_launch_enabled
            .get_output(context);
        let upload_size_bytes_binding = args.upload_size_bytes.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/managedDisk:ManagedDisk".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createOption".into(),
                    value: &create_option_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskAccessId".into(),
                    value: &disk_access_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskEncryptionSetId".into(),
                    value: &disk_encryption_set_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskIopsReadOnly".into(),
                    value: &disk_iops_read_only_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskIopsReadWrite".into(),
                    value: &disk_iops_read_write_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskMbpsReadOnly".into(),
                    value: &disk_mbps_read_only_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskMbpsReadWrite".into(),
                    value: &disk_mbps_read_write_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskSizeGb".into(),
                    value: &disk_size_gb_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "edgeZone".into(),
                    value: &edge_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionSettings".into(),
                    value: &encryption_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "galleryImageReferenceId".into(),
                    value: &gallery_image_reference_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hyperVGeneration".into(),
                    value: &hyper_v_generation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageReferenceId".into(),
                    value: &image_reference_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logicalSectorSize".into(),
                    value: &logical_sector_size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxShares".into(),
                    value: &max_shares_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkAccessPolicy".into(),
                    value: &network_access_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "onDemandBurstingEnabled".into(),
                    value: &on_demand_bursting_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "optimizedFrequentAttachEnabled".into(),
                    value: &optimized_frequent_attach_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osType".into(),
                    value: &os_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "performancePlusEnabled".into(),
                    value: &performance_plus_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secureVmDiskEncryptionSetId".into(),
                    value: &secure_vm_disk_encryption_set_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityType".into(),
                    value: &security_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceResourceId".into(),
                    value: &source_resource_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceUri".into(),
                    value: &source_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountType".into(),
                    value: &storage_account_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tier".into(),
                    value: &tier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trustedLaunchEnabled".into(),
                    value: &trusted_launch_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "uploadSizeBytes".into(),
                    value: &upload_size_bytes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ManagedDiskResult {
            create_option: o.get_field("createOption"),
            disk_access_id: o.get_field("diskAccessId"),
            disk_encryption_set_id: o.get_field("diskEncryptionSetId"),
            disk_iops_read_only: o.get_field("diskIopsReadOnly"),
            disk_iops_read_write: o.get_field("diskIopsReadWrite"),
            disk_mbps_read_only: o.get_field("diskMbpsReadOnly"),
            disk_mbps_read_write: o.get_field("diskMbpsReadWrite"),
            disk_size_gb: o.get_field("diskSizeGb"),
            edge_zone: o.get_field("edgeZone"),
            encryption_settings: o.get_field("encryptionSettings"),
            gallery_image_reference_id: o.get_field("galleryImageReferenceId"),
            hyper_v_generation: o.get_field("hyperVGeneration"),
            image_reference_id: o.get_field("imageReferenceId"),
            location: o.get_field("location"),
            logical_sector_size: o.get_field("logicalSectorSize"),
            max_shares: o.get_field("maxShares"),
            name: o.get_field("name"),
            network_access_policy: o.get_field("networkAccessPolicy"),
            on_demand_bursting_enabled: o.get_field("onDemandBurstingEnabled"),
            optimized_frequent_attach_enabled: o
                .get_field("optimizedFrequentAttachEnabled"),
            os_type: o.get_field("osType"),
            performance_plus_enabled: o.get_field("performancePlusEnabled"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            secure_vm_disk_encryption_set_id: o.get_field("secureVmDiskEncryptionSetId"),
            security_type: o.get_field("securityType"),
            source_resource_id: o.get_field("sourceResourceId"),
            source_uri: o.get_field("sourceUri"),
            storage_account_id: o.get_field("storageAccountId"),
            storage_account_type: o.get_field("storageAccountType"),
            tags: o.get_field("tags"),
            tier: o.get_field("tier"),
            trusted_launch_enabled: o.get_field("trustedLaunchEnabled"),
            upload_size_bytes: o.get_field("uploadSizeBytes"),
            zone: o.get_field("zone"),
        }
    }
}
