pub mod get_snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotArgs {
        /// Specifies the name of the Snapshot.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the resource group the Snapshot is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotResult {
        pub creation_option: pulumi_wasm_rust::Output<String>,
        /// The size of the Snapshotted Disk in GB.
        pub disk_size_gb: pulumi_wasm_rust::Output<i32>,
        pub encryption_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetSnapshotEncryptionSetting>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub os_type: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The reference to an existing snapshot.
        pub source_resource_id: pulumi_wasm_rust::Output<String>,
        /// The URI to a Managed or Unmanaged Disk.
        pub source_uri: pulumi_wasm_rust::Output<String>,
        /// The ID of an storage account.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        pub time_created: pulumi_wasm_rust::Output<String>,
        /// Whether Trusted Launch is enabled for the Snapshot.
        pub trusted_launch_enabled: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSnapshotArgs) -> GetSnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getSnapshot:getSnapshot".into(),
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
                    name: "creationOption".into(),
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
                    name: "name".into(),
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
                    name: "timeCreated".into(),
                },
                register_interface::ResultField {
                    name: "trustedLaunchEnabled".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSnapshotResult {
            creation_option: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationOption").unwrap(),
            ),
            disk_size_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskSizeGb").unwrap(),
            ),
            encryption_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionSettings").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
            time_created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeCreated").unwrap(),
            ),
            trusted_launch_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustedLaunchEnabled").unwrap(),
            ),
        }
    }
}
