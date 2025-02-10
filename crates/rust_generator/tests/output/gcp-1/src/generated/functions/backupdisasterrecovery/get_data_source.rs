#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_data_source {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataSourceArgs {
        #[builder(into)]
        pub backup_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub data_source_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub project: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDataSourceResult {
        pub backup_config_infos: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::backupdisasterrecovery::GetDataSourceBackupConfigInfo,
            >,
        >,
        pub backup_count: pulumi_gestalt_rust::Output<String>,
        pub backup_vault_id: pulumi_gestalt_rust::Output<String>,
        pub config_state: pulumi_gestalt_rust::Output<String>,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub data_source_backup_appliance_applications: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::backupdisasterrecovery::GetDataSourceDataSourceBackupApplianceApplication,
            >,
        >,
        pub data_source_gcp_resources: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::backupdisasterrecovery::GetDataSourceDataSourceGcpResource,
            >,
        >,
        pub data_source_id: pulumi_gestalt_rust::Output<String>,
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub total_stored_bytes: pulumi_gestalt_rust::Output<String>,
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDataSourceArgs,
    ) -> GetDataSourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_vault_id_binding = args.backup_vault_id.get_output(context);
        let data_source_id_binding = args.data_source_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:backupdisasterrecovery/getDataSource:getDataSource".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupVaultId".into(),
                    value: backup_vault_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataSourceId".into(),
                    value: data_source_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDataSourceResult {
            backup_config_infos: o.get_field("backupConfigInfos"),
            backup_count: o.get_field("backupCount"),
            backup_vault_id: o.get_field("backupVaultId"),
            config_state: o.get_field("configState"),
            create_time: o.get_field("createTime"),
            data_source_backup_appliance_applications: o
                .get_field("dataSourceBackupApplianceApplications"),
            data_source_gcp_resources: o.get_field("dataSourceGcpResources"),
            data_source_id: o.get_field("dataSourceId"),
            etag: o.get_field("etag"),
            id: o.get_field("id"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            state: o.get_field("state"),
            total_stored_bytes: o.get_field("totalStoredBytes"),
            update_time: o.get_field("updateTime"),
        }
    }
}
