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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetBackupPlanAssociationArgs,
    ) -> GetBackupPlanAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let backup_plan_association_id_binding_1 = args
            .backup_plan_association_id
            .get_output(context);
        let backup_plan_association_id_binding = backup_plan_association_id_binding_1
            .get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:backupdisasterrecovery/getBackupPlanAssociation:getBackupPlanAssociation"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupPlanAssociationId".into(),
                    value: &backup_plan_association_id_binding,
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
        GetBackupPlanAssociationResult {
            backup_plan: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupPlan"),
            ),
            backup_plan_association_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupPlanAssociationId"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            data_source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataSource"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            last_successful_backup_consistency_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastSuccessfulBackupConsistencyTime"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            resource: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resource"),
            ),
            resource_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceType"),
            ),
            rules_config_infos: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rulesConfigInfos"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
