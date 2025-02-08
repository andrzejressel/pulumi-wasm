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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetManagedDiskArgs,
    ) -> GetManagedDiskResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getManagedDisk:getManagedDisk".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetManagedDiskResult {
            create_option: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createOption"),
            ),
            disk_access_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskAccessId"),
            ),
            disk_encryption_set_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskEncryptionSetId"),
            ),
            disk_iops_read_write: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskIopsReadWrite"),
            ),
            disk_mbps_read_write: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskMbpsReadWrite"),
            ),
            disk_size_gb: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskSizeGb"),
            ),
            encryption_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionSettings"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            image_reference_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageReferenceId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_access_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkAccessPolicy"),
            ),
            os_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("osType"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            source_resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceResourceId"),
            ),
            source_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceUri"),
            ),
            storage_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
            storage_account_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountType"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            zones: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zones")),
        }
    }
}
