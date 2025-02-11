#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_backup_plan_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBackupPlanAssociationArgs {
        /// The id of Backupplan association resource.
        ///
        /// - - -
        #[builder(into)]
        pub backup_plan_association_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location in which the Backupplan association resource belongs.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBackupPlanAssociationResult {
        pub backup_plan: pulumi_gestalt_rust::Output<String>,
        pub backup_plan_association_id: pulumi_gestalt_rust::Output<String>,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub data_source: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub last_successful_backup_consistency_time: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub resource: pulumi_gestalt_rust::Output<String>,
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        pub rules_config_infos: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::backupdisasterrecovery::GetBackupPlanAssociationRulesConfigInfo,
            >,
        >,
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBackupPlanAssociationArgs,
    ) -> GetBackupPlanAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_plan_association_id_binding = args
            .backup_plan_association_id
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:backupdisasterrecovery/getBackupPlanAssociation:getBackupPlanAssociation"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupPlanAssociationId".into(),
                    value: &backup_plan_association_id_binding.drop_type(),
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
        GetBackupPlanAssociationResult {
            backup_plan: o.get_field("backupPlan"),
            backup_plan_association_id: o.get_field("backupPlanAssociationId"),
            create_time: o.get_field("createTime"),
            data_source: o.get_field("dataSource"),
            id: o.get_field("id"),
            last_successful_backup_consistency_time: o
                .get_field("lastSuccessfulBackupConsistencyTime"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            resource: o.get_field("resource"),
            resource_type: o.get_field("resourceType"),
            rules_config_infos: o.get_field("rulesConfigInfos"),
            update_time: o.get_field("updateTime"),
        }
    }
}
