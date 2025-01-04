pub mod get_managed_disk {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedDiskArgs {
        /// Specifies the name of the Managed Disk.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group where this Managed Disk exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetManagedDiskResult {
        pub create_option: pulumi_wasm_rust::Output<String>,
        /// The ID of the disk access resource for using private endpoints on disks.
        pub disk_access_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Disk Encryption Set used to encrypt this Managed Disk.
        pub disk_encryption_set_id: pulumi_wasm_rust::Output<String>,
        /// The number of IOPS allowed for this disk, where one operation can transfer between 4k and 256k bytes.
        pub disk_iops_read_write: pulumi_wasm_rust::Output<i32>,
        /// The bandwidth allowed for this disk.
        pub disk_mbps_read_write: pulumi_wasm_rust::Output<i32>,
        /// The size of the Managed Disk in gigabytes.
        pub disk_size_gb: pulumi_wasm_rust::Output<i32>,
        /// A `encryption_settings` block as defined below.
        pub encryption_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetManagedDiskEncryptionSetting>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The ID of the source image used for creating this Managed Disk.
        pub image_reference_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Policy for accessing the disk via network.
        pub network_access_policy: pulumi_wasm_rust::Output<String>,
        /// The operating system used for this Managed Disk.
        pub os_type: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of an existing Managed Disk which this Disk was created from.
        pub source_resource_id: pulumi_wasm_rust::Output<String>,
        /// The Source URI for this Managed Disk.
        pub source_uri: pulumi_wasm_rust::Output<String>,
        /// The ID of the Storage Account where the `source_uri` is located.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The storage account type for the Managed Disk.
        pub storage_account_type: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// A list of Availability Zones where the Managed Disk exists.
        pub zones: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetManagedDiskArgs) -> GetManagedDiskResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getManagedDisk:getManagedDisk".into(),
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
                    name: "diskIopsReadWrite".into(),
                },
                register_interface::ResultField {
                    name: "diskMbpsReadWrite".into(),
                },
                register_interface::ResultField {
                    name: "diskSizeGb".into(),
                },
                register_interface::ResultField {
                    name: "encryptionSettings".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "imageReferenceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkAccessPolicy".into(),
                },
                register_interface::ResultField {
                    name: "osType".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
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
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetManagedDiskResult {
            create_option: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createOption").unwrap(),
            ),
            disk_access_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskAccessId").unwrap(),
            ),
            disk_encryption_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskEncryptionSetId").unwrap(),
            ),
            disk_iops_read_write: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskIopsReadWrite").unwrap(),
            ),
            disk_mbps_read_write: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskMbpsReadWrite").unwrap(),
            ),
            disk_size_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskSizeGb").unwrap(),
            ),
            encryption_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionSettings").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            image_reference_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageReferenceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_access_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkAccessPolicy").unwrap(),
            ),
            os_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osType").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
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
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}
