#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetBackupVaultArgs,
    ) -> GetBackupVaultResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let backup_vault_id_binding = args
            .backup_vault_id
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:backupdisasterrecovery/getBackupVault:getBackupVault".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupVaultId".into(),
                    value: &backup_vault_id_binding,
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
        GetBackupVaultResult {
            access_restriction: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessRestriction"),
            ),
            allow_missing: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowMissing"),
            ),
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            backup_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupCount"),
            ),
            backup_minimum_enforced_retention_duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupMinimumEnforcedRetentionDuration"),
            ),
            backup_vault_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupVaultId"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            deletable: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletable"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            effective_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveTime"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            force_delete: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceDelete"),
            ),
            force_update: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceUpdate"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ignore_backup_plan_references: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ignoreBackupPlanReferences"),
            ),
            ignore_inactive_datasources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ignoreInactiveDatasources"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            service_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceAccount"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            total_stored_bytes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("totalStoredBytes"),
            ),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
