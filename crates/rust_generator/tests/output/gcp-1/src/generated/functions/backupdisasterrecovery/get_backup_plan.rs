#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_backup_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBackupPlanArgs {
        #[builder(into)]
        pub backup_plan_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBackupPlanResult {
        pub backup_plan_id: pulumi_gestalt_rust::Output<String>,
        pub backup_rules: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::backupdisasterrecovery::GetBackupPlanBackupRule,
            >,
        >,
        pub backup_vault: pulumi_gestalt_rust::Output<String>,
        pub backup_vault_service_account: pulumi_gestalt_rust::Output<String>,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBackupPlanArgs,
    ) -> GetBackupPlanResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_plan_id_binding = args.backup_plan_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:backupdisasterrecovery/getBackupPlan:getBackupPlan".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupPlanId".into(),
                    value: backup_plan_id_binding.get_id(),
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
        GetBackupPlanResult {
            backup_plan_id: o.get_field("backupPlanId"),
            backup_rules: o.get_field("backupRules"),
            backup_vault: o.get_field("backupVault"),
            backup_vault_service_account: o.get_field("backupVaultServiceAccount"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            resource_type: o.get_field("resourceType"),
            update_time: o.get_field("updateTime"),
        }
    }
}
