/// Container to store and organize immutable and indelible backups.
///
///
///
/// ## Example Usage
///
/// ### Backup Dr Backup Vault Full
///
///
/// ```yaml
/// resources:
///   backup-vault-test:
///     type: gcp:backupdisasterrecovery:BackupVault
///     properties:
///       location: us-central1
///       backupVaultId: backup-vault-test
///       description: This is a second backup vault built by Terraform.
///       backupMinimumEnforcedRetentionDuration: 100000s
///       annotations:
///         annotations1: bar1
///         annotations2: baz1
///       labels:
///         foo: bar1
///         bar: baz1
///       forceUpdate: 'true'
///       accessRestriction: WITHIN_ORGANIZATION
///       ignoreInactiveDatasources: 'true'
///       ignoreBackupPlanReferences: 'true'
///       allowMissing: 'true'
/// ```
///
/// ## Import
///
/// BackupVault can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/backupVaults/{{backup_vault_id}}`
///
/// * `{{project}}/{{location}}/{{backup_vault_id}}`
///
/// * `{{location}}/{{backup_vault_id}}`
///
/// When using the `pulumi import` command, BackupVault can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:backupdisasterrecovery/backupVault:BackupVault default projects/{{project}}/locations/{{location}}/backupVaults/{{backup_vault_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:backupdisasterrecovery/backupVault:BackupVault default {{project}}/{{location}}/{{backup_vault_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:backupdisasterrecovery/backupVault:BackupVault default {{location}}/{{backup_vault_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup_vault {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupVaultArgs {
        /// Access restriction for the backup vault. Default value is `WITHIN_ORGANIZATION` if not provided during creation.
        /// Default value is `WITHIN_ORGANIZATION`.
        /// Possible values are: `ACCESS_RESTRICTION_UNSPECIFIED`, `WITHIN_PROJECT`, `WITHIN_ORGANIZATION`, `UNRESTRICTED`, `WITHIN_ORG_BUT_UNRESTRICTED_FOR_BA`.
        #[builder(into, default)]
        pub access_restriction: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Allow idempotent deletion of backup vault. The request will still succeed in case the backup vault does not exist.
        #[builder(into, default)]
        pub allow_missing: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Optional. User annotations. See https://google.aip.dev/128#annotations
        /// Stores small amounts of arbitrary data.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Required. The default and minimum enforced retention for each backup within the backup vault. The enforced retention for each backup can be extended.
        #[builder(into)]
        pub backup_minimum_enforced_retention_duration: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// Required. ID of the requesting object.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub backup_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. The description of the BackupVault instance (2048 characters or less).
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Time after which the BackupVault resource is locked.
        #[builder(into, default)]
        pub effective_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// (Optional, Deprecated)
        /// If set, the following restrictions against deletion of the backup vault instance can be overridden:
        /// * deletion of a backup vault instance containing no backups, but still containing empty datasources.
        /// * deletion of a backup vault instance that is being referenced by an active backup plan.
        ///
        /// > **Warning:** `force_delete` is deprecated and will be removed in a future major release. Use `ignore_inactive_datasources` instead.
        #[builder(into, default)]
        pub force_delete: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If set, allow update to extend the minimum enforced retention for backup vault. This overrides
        /// the restriction against conflicting retention periods. This conflict may occur when the
        /// expiration schedule defined by the associated backup plan is shorter than the minimum
        /// retention set by the backup vault.
        #[builder(into, default)]
        pub force_update: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If set, the following restrictions against deletion of the backup vault instance can be overridden:
        /// * deletion of a backup vault instance that is being referenced by an active backup plan.
        #[builder(into, default)]
        pub ignore_backup_plan_references: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// If set, the following restrictions against deletion of the backup vault instance can be overridden:
        /// * deletion of a backup vault instance containing no backups, but still containing empty datasources.
        #[builder(into, default)]
        pub ignore_inactive_datasources: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Optional. Resource labels to represent user provided metadata.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The GCP location for the backup vault.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BackupVaultResult {
        /// Access restriction for the backup vault. Default value is `WITHIN_ORGANIZATION` if not provided during creation.
        /// Default value is `WITHIN_ORGANIZATION`.
        /// Possible values are: `ACCESS_RESTRICTION_UNSPECIFIED`, `WITHIN_PROJECT`, `WITHIN_ORGANIZATION`, `UNRESTRICTED`, `WITHIN_ORG_BUT_UNRESTRICTED_FOR_BA`.
        pub access_restriction: pulumi_gestalt_rust::Output<Option<String>>,
        /// Allow idempotent deletion of backup vault. The request will still succeed in case the backup vault does not exist.
        pub allow_missing: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Optional. User annotations. See https://google.aip.dev/128#annotations
        /// Stores small amounts of arbitrary data.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Output only. The number of backups in this backup vault.
        pub backup_count: pulumi_gestalt_rust::Output<String>,
        /// Required. The default and minimum enforced retention for each backup within the backup vault. The enforced retention for each backup can be extended.
        pub backup_minimum_enforced_retention_duration: pulumi_gestalt_rust::Output<
            String,
        >,
        /// Required. ID of the requesting object.
        ///
        ///
        /// - - -
        pub backup_vault_id: pulumi_gestalt_rust::Output<String>,
        /// Output only. The time when the instance was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Output only. Set to true when there are no backups nested under this resource.
        pub deletable: pulumi_gestalt_rust::Output<bool>,
        /// Optional. The description of the BackupVault instance (2048 characters or less).
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Time after which the BackupVault resource is locked.
        pub effective_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. Server specified ETag for the backup vault resource to prevent simultaneous updates from overwiting each other.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// (Optional, Deprecated)
        /// If set, the following restrictions against deletion of the backup vault instance can be overridden:
        /// * deletion of a backup vault instance containing no backups, but still containing empty datasources.
        /// * deletion of a backup vault instance that is being referenced by an active backup plan.
        ///
        /// > **Warning:** `force_delete` is deprecated and will be removed in a future major release. Use `ignore_inactive_datasources` instead.
        pub force_delete: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If set, allow update to extend the minimum enforced retention for backup vault. This overrides
        /// the restriction against conflicting retention periods. This conflict may occur when the
        /// expiration schedule defined by the associated backup plan is shorter than the minimum
        /// retention set by the backup vault.
        pub force_update: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If set, the following restrictions against deletion of the backup vault instance can be overridden:
        /// * deletion of a backup vault instance that is being referenced by an active backup plan.
        pub ignore_backup_plan_references: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If set, the following restrictions against deletion of the backup vault instance can be overridden:
        /// * deletion of a backup vault instance containing no backups, but still containing empty datasources.
        pub ignore_inactive_datasources: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Optional. Resource labels to represent user provided metadata.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The GCP location for the backup vault.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Output only. Identifier. The resource name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Service account used by the BackupVault Service for this BackupVault.  The user should grant this account permissions in their workload project to enable the service to run backups and restores there.
        pub service_account: pulumi_gestalt_rust::Output<String>,
        /// Output only. The BackupVault resource instance state.
        /// Possible values:
        /// STATE_UNSPECIFIED
        /// CREATING
        /// ACTIVE
        /// DELETING
        /// ERROR
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Output only. Total size of the storage used by all backup resources.
        pub total_stored_bytes: pulumi_gestalt_rust::Output<String>,
        /// Output only. Output only Immutable after resource creation until resource deletion.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. The time when the instance was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupVaultArgs,
    ) -> BackupVaultResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_restriction_binding = args.access_restriction.get_output(context);
        let allow_missing_binding = args.allow_missing.get_output(context);
        let annotations_binding = args.annotations.get_output(context);
        let backup_minimum_enforced_retention_duration_binding = args
            .backup_minimum_enforced_retention_duration
            .get_output(context);
        let backup_vault_id_binding = args.backup_vault_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let effective_time_binding = args.effective_time.get_output(context);
        let force_delete_binding = args.force_delete.get_output(context);
        let force_update_binding = args.force_update.get_output(context);
        let ignore_backup_plan_references_binding = args
            .ignore_backup_plan_references
            .get_output(context);
        let ignore_inactive_datasources_binding = args
            .ignore_inactive_datasources
            .get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:backupdisasterrecovery/backupVault:BackupVault".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessRestriction".into(),
                    value: &access_restriction_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowMissing".into(),
                    value: &allow_missing_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupMinimumEnforcedRetentionDuration".into(),
                    value: &backup_minimum_enforced_retention_duration_binding
                        .drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupVaultId".into(),
                    value: &backup_vault_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "effectiveTime".into(),
                    value: &effective_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDelete".into(),
                    value: &force_delete_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceUpdate".into(),
                    value: &force_update_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ignoreBackupPlanReferences".into(),
                    value: &ignore_backup_plan_references_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ignoreInactiveDatasources".into(),
                    value: &ignore_inactive_datasources_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
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
        let o = context.register_resource(request);
        BackupVaultResult {
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
