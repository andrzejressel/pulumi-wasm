pub mod get_lab {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLabArgs {
        /// The name of the Dev Test Lab.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group where the Dev Test Lab exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetLabResult {
        /// The ID of the Storage Account used for Artifact Storage.
        pub artifacts_storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Default Premium Storage Account for this Dev Test Lab.
        pub default_premium_storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Default Storage Account for this Dev Test Lab.
        pub default_storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Key used for this Dev Test Lab.
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// The Azure location where the Dev Test Lab exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Storage Account used for Storage of Premium Data Disk.
        pub premium_data_disk_storage_account_id: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The type of storage used by the Dev Test Lab.
        pub storage_type: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The unique immutable identifier of the Dev Test Lab.
        pub unique_identifier: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetLabArgs) -> GetLabResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:devtest/getLab:getLab".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "artifactsStorageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "defaultPremiumStorageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "defaultStorageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "premiumDataDiskStorageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "storageType".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "uniqueIdentifier".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLabResult {
            artifacts_storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("artifactsStorageAccountId").unwrap(),
            ),
            default_premium_storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPremiumStorageAccountId").unwrap(),
            ),
            default_storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultStorageAccountId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            premium_data_disk_storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("premiumDataDiskStorageAccountId").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            storage_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageType").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            unique_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uniqueIdentifier").unwrap(),
            ),
        }
    }
}
