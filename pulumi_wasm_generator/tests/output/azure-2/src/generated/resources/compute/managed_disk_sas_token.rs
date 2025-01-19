/// Manages a Disk SAS Token.
///
/// Use this resource to obtain a Shared Access Signature (SAS Token) for an existing Managed Disk.
///
/// Shared access signatures allow fine-grained, ephemeral access control to various aspects of Managed Disk similar to blob/storage account container.
///
/// With the help of this resource, data from the disk can be copied from managed disk to a storage blob or to some other system without the need of azcopy.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: azure:core:ResourceGroup
///     properties:
///       name: testrg
///       location: West Europe
///   testManagedDisk:
///     type: azure:compute:ManagedDisk
///     name: test
///     properties:
///       name: tst-disk-export
///       location: ${test.location}
///       resourceGroupName: ${test.name}
///       storageAccountType: Standard_LRS
///       createOption: Empty
///       diskSizeGb: '1'
///   testManagedDiskSasToken:
///     type: azure:compute:ManagedDiskSasToken
///     name: test
///     properties:
///       managedDiskId: ${testManagedDisk.id}
///       durationInSeconds: 300
///       accessLevel: Read
/// ```
///
/// ## Import
///
/// Disk SAS Token can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/managedDiskSasToken:ManagedDiskSasToken example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/disks/manageddisk1
/// ```
///
pub mod managed_disk_sas_token {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedDiskSasTokenArgs {
        /// The level of access required on the disk. Supported are Read, Write. Changing this forces a new resource to be created.
        ///
        /// Refer to the [SAS creation reference from Azure](https://docs.microsoft.com/rest/api/compute/disks/grant-access)
        /// for additional details on the fields above.
        #[builder(into)]
        pub access_level: pulumi_wasm_rust::Output<String>,
        /// The duration for which the export should be allowed. Should be between 30 & 4294967295 seconds. Changing this forces a new resource to be created.
        #[builder(into)]
        pub duration_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// The ID of an existing Managed Disk which should be exported. Changing this forces a new resource to be created.
        #[builder(into)]
        pub managed_disk_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedDiskSasTokenResult {
        /// The level of access required on the disk. Supported are Read, Write. Changing this forces a new resource to be created.
        ///
        /// Refer to the [SAS creation reference from Azure](https://docs.microsoft.com/rest/api/compute/disks/grant-access)
        /// for additional details on the fields above.
        pub access_level: pulumi_wasm_rust::Output<String>,
        /// The duration for which the export should be allowed. Should be between 30 & 4294967295 seconds. Changing this forces a new resource to be created.
        pub duration_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// The ID of an existing Managed Disk which should be exported. Changing this forces a new resource to be created.
        pub managed_disk_id: pulumi_wasm_rust::Output<String>,
        /// The computed Shared Access Signature (SAS) of the Managed Disk.
        pub sas_url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ManagedDiskSasTokenArgs,
    ) -> ManagedDiskSasTokenResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_level_binding = args.access_level.get_inner();
        let duration_in_seconds_binding = args.duration_in_seconds.get_inner();
        let managed_disk_id_binding = args.managed_disk_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/managedDiskSasToken:ManagedDiskSasToken".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessLevel".into(),
                    value: &access_level_binding,
                },
                register_interface::ObjectField {
                    name: "durationInSeconds".into(),
                    value: &duration_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "managedDiskId".into(),
                    value: &managed_disk_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessLevel".into(),
                },
                register_interface::ResultField {
                    name: "durationInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "managedDiskId".into(),
                },
                register_interface::ResultField {
                    name: "sasUrl".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedDiskSasTokenResult {
            access_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessLevel").unwrap(),
            ),
            duration_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("durationInSeconds").unwrap(),
            ),
            managed_disk_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedDiskId").unwrap(),
            ),
            sas_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sasUrl").unwrap(),
            ),
        }
    }
}
