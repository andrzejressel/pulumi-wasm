#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_backup_vault {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBackupVaultArgs {
        /// The id of Backup Vault resource.
        ///
        /// - - -
        #[builder(into)]
        pub backup_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location in which the Backup Vault resource belongs.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBackupVaultResult {
        pub access_restriction: pulumi_gestalt_rust::Output<String>,
        pub allow_missing: pulumi_gestalt_rust::Output<bool>,
        pub annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub backup_count: pulumi_gestalt_rust::Output<String>,
        pub backup_minimum_enforced_retention_duration: pulumi_gestalt_rust::Output<
            String,
        >,
        pub backup_vault_id: pulumi_gestalt_rust::Output<String>,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub deletable: pulumi_gestalt_rust::Output<bool>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub effective_time: pulumi_gestalt_rust::Output<String>,
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub force_delete: pulumi_gestalt_rust::Output<bool>,
        pub force_update: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ignore_backup_plan_references: pulumi_gestalt_rust::Output<bool>,
        pub ignore_inactive_datasources: pulumi_gestalt_rust::Output<bool>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub service_account: pulumi_gestalt_rust::Output<String>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub total_stored_bytes: pulumi_gestalt_rust::Output<String>,
        pub uid: pulumi_gestalt_rust::Output<String>,
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBackupVaultArgs,
    ) -> GetBackupVaultResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_vault_id_binding = args.backup_vault_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:backupdisasterrecovery/getBackupVault:getBackupVault".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupVaultId".into(),
                    value: &backup_vault_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBackupVaultResult {
            access_restriction: o.get_field("accessRestriction"),
            allow_missing: o.get_field("allowMissing"),
            annotations: o.get_field("annotations"),
            backup_count: o.get_field("backupCount"),
            backup_minimum_enforced_retention_duration: o
                .get_field("backupMinimumEnforcedRetentionDuration"),
            backup_vault_id: o.get_field("backupVaultId"),
            create_time: o.get_field("createTime"),
            deletable: o.get_field("deletable"),
            description: o.get_field("description"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            effective_time: o.get_field("effectiveTime"),
            etag: o.get_field("etag"),
            force_delete: o.get_field("forceDelete"),
            force_update: o.get_field("forceUpdate"),
            id: o.get_field("id"),
            ignore_backup_plan_references: o.get_field("ignoreBackupPlanReferences"),
            ignore_inactive_datasources: o.get_field("ignoreInactiveDatasources"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            service_account: o.get_field("serviceAccount"),
            state: o.get_field("state"),
            total_stored_bytes: o.get_field("totalStoredBytes"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
