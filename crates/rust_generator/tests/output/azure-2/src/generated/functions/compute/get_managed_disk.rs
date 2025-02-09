#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_managed_disk {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedDiskArgs {
        /// Specifies the name of the Managed Disk.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group where this Managed Disk exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetManagedDiskResult {
        pub create_option: pulumi_gestalt_rust::Output<String>,
        /// The ID of the disk access resource for using private endpoints on disks.
        pub disk_access_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Disk Encryption Set used to encrypt this Managed Disk.
        pub disk_encryption_set_id: pulumi_gestalt_rust::Output<String>,
        /// The number of IOPS allowed for this disk, where one operation can transfer between 4k and 256k bytes.
        pub disk_iops_read_write: pulumi_gestalt_rust::Output<i32>,
        /// The bandwidth allowed for this disk.
        pub disk_mbps_read_write: pulumi_gestalt_rust::Output<i32>,
        /// The size of the Managed Disk in gigabytes.
        pub disk_size_gb: pulumi_gestalt_rust::Output<i32>,
        /// A `encryption_settings` block as defined below.
        pub encryption_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetManagedDiskEncryptionSetting>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the source image used for creating this Managed Disk.
        pub image_reference_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Policy for accessing the disk via network.
        pub network_access_policy: pulumi_gestalt_rust::Output<String>,
        /// The operating system used for this Managed Disk.
        pub os_type: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of an existing Managed Disk which this Disk was created from.
        pub source_resource_id: pulumi_gestalt_rust::Output<String>,
        /// The Source URI for this Managed Disk.
        pub source_uri: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Storage Account where the `source_uri` is located.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// The storage account type for the Managed Disk.
        pub storage_account_type: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// A list of Availability Zones where the Managed Disk exists.
        pub zones: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetManagedDiskArgs,
    ) -> GetManagedDiskResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:compute/getManagedDisk:getManagedDisk".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetManagedDiskResult {
            create_option: o.get_field("createOption"),
            disk_access_id: o.get_field("diskAccessId"),
            disk_encryption_set_id: o.get_field("diskEncryptionSetId"),
            disk_iops_read_write: o.get_field("diskIopsReadWrite"),
            disk_mbps_read_write: o.get_field("diskMbpsReadWrite"),
            disk_size_gb: o.get_field("diskSizeGb"),
            encryption_settings: o.get_field("encryptionSettings"),
            id: o.get_field("id"),
            image_reference_id: o.get_field("imageReferenceId"),
            name: o.get_field("name"),
            network_access_policy: o.get_field("networkAccessPolicy"),
            os_type: o.get_field("osType"),
            resource_group_name: o.get_field("resourceGroupName"),
            source_resource_id: o.get_field("sourceResourceId"),
            source_uri: o.get_field("sourceUri"),
            storage_account_id: o.get_field("storageAccountId"),
            storage_account_type: o.get_field("storageAccountType"),
            tags: o.get_field("tags"),
            zones: o.get_field("zones"),
        }
    }
}
