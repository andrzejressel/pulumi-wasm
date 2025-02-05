pub mod get_data_source {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataSourceArgs {
        #[builder(into)]
        pub backup_vault_id: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into)]
        pub data_source_id: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into)]
        pub project: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDataSourceArgs,
    ) -> GetDataSourceResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_vault_id_binding = args
            .backup_vault_id
            .get_output(context)
            .get_inner();
        let data_source_id_binding = args.data_source_id.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDataSourceResult {
            backup_config_infos: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupConfigInfos"),
            ),
            backup_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupCount"),
            ),
            backup_vault_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupVaultId"),
            ),
            config_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configState"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            data_source_backup_appliance_applications: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataSourceBackupApplianceApplications"),
            ),
            data_source_gcp_resources: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataSourceGcpResources"),
            ),
            data_source_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataSourceId"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            total_stored_bytes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("totalStoredBytes"),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
