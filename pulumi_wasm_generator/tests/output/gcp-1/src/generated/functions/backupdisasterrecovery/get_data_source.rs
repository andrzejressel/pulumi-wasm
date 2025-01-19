pub mod get_data_source {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataSourceArgs {
        #[builder(into)]
        pub backup_vault_id: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub data_source_id: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub project: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetDataSourceResult {
        pub backup_config_infos: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::backupdisasterrecovery::GetDataSourceBackupConfigInfo,
            >,
        >,
        pub backup_count: pulumi_wasm_rust::Output<String>,
        pub backup_vault_id: pulumi_wasm_rust::Output<String>,
        pub config_state: pulumi_wasm_rust::Output<String>,
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub data_source_backup_appliance_applications: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::backupdisasterrecovery::GetDataSourceDataSourceBackupApplianceApplication,
            >,
        >,
        pub data_source_gcp_resources: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::backupdisasterrecovery::GetDataSourceDataSourceGcpResource,
            >,
        >,
        pub data_source_id: pulumi_wasm_rust::Output<String>,
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub total_stored_bytes: pulumi_wasm_rust::Output<String>,
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDataSourceArgs) -> GetDataSourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_vault_id_binding = args.backup_vault_id.get_inner();
        let data_source_id_binding = args.data_source_id.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:backupdisasterrecovery/getDataSource:getDataSource".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupVaultId".into(),
                    value: &backup_vault_id_binding,
                },
                register_interface::ObjectField {
                    name: "dataSourceId".into(),
                    value: &data_source_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backupConfigInfos".into(),
                },
                register_interface::ResultField {
                    name: "backupCount".into(),
                },
                register_interface::ResultField {
                    name: "backupVaultId".into(),
                },
                register_interface::ResultField {
                    name: "configState".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dataSourceBackupApplianceApplications".into(),
                },
                register_interface::ResultField {
                    name: "dataSourceGcpResources".into(),
                },
                register_interface::ResultField {
                    name: "dataSourceId".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "totalStoredBytes".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDataSourceResult {
            backup_config_infos: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupConfigInfos").unwrap(),
            ),
            backup_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupCount").unwrap(),
            ),
            backup_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupVaultId").unwrap(),
            ),
            config_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configState").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            data_source_backup_appliance_applications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSourceBackupApplianceApplications").unwrap(),
            ),
            data_source_gcp_resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSourceGcpResources").unwrap(),
            ),
            data_source_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSourceId").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            total_stored_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalStoredBytes").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
