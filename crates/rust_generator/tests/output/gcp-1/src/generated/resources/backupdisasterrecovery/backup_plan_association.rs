/// ## Example Usage
///
/// ### Backup Dr Bpa
///
///
/// ```yaml
/// resources:
///   mySA:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-custom
///       displayName: Custom SA for VM Instance
///   myinstance:
///     type: gcp:compute:Instance
///     properties:
///       networkInterfaces:
///         - accessConfigs:
///             - {}
///           network: default
///       name: test-instance
///       machineType: n2-standard-2
///       zone: us-central1-a
///       bootDisk:
///         initializeParams:
///           image: debian-cloud/debian-11
///           labels:
///             my_label: value
///       scratchDisks:
///         - interface: NVME
///       serviceAccount:
///         email: ${mySA.email}
///         scopes:
///           - cloud-platform
///   bv1:
///     type: gcp:backupdisasterrecovery:BackupVault
///     properties:
///       location: us-central1
///       backupVaultId: bv-bpa
///       backupMinimumEnforcedRetentionDuration: 100000s
///       forceDelete: 'true'
///   bp1:
///     type: gcp:backupdisasterrecovery:BackupPlan
///     properties:
///       location: us-central1
///       backupPlanId: bp-bpa-test
///       resourceType: compute.googleapis.com/Instance
///       backupVault: ${bv1.id}
///       backupRules:
///         - ruleId: rule-1
///           backupRetentionDays: 2
///           standardSchedule:
///             recurrenceType: HOURLY
///             hourlyFrequency: 6
///             timeZone: UTC
///             backupWindow:
///               startHourOfDay: 12
///               endHourOfDay: 18
///   my-backup-plan-association:
///     type: gcp:backupdisasterrecovery:BackupPlanAssociation
///     properties:
///       location: us-central1
///       resourceType: compute.googleapis.com/Instance
///       backupPlanAssociationId: my-bpa
///       resource: ${myinstance.id}
///       backupPlan: ${bp1.name}
/// ```
///
/// ## Import
///
/// BackupPlanAssociation can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/backupPlanAssociations/{{backup_plan_association_id}}`
///
/// * `{{project}}/{{location}}/{{backup_plan_association_id}}`
///
/// * `{{location}}/{{backup_plan_association_id}}`
///
/// When using the `pulumi import` command, BackupPlanAssociation can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:backupdisasterrecovery/backupPlanAssociation:BackupPlanAssociation default projects/{{project}}/locations/{{location}}/backupPlanAssociations/{{backup_plan_association_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:backupdisasterrecovery/backupPlanAssociation:BackupPlanAssociation default {{project}}/{{location}}/{{backup_plan_association_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:backupdisasterrecovery/backupPlanAssociation:BackupPlanAssociation default {{location}}/{{backup_plan_association_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup_plan_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPlanAssociationArgs {
        /// The BP with which resource needs to be created
        #[builder(into)]
        pub backup_plan: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of backupplan association
        ///
        ///
        /// - - -
        #[builder(into)]
        pub backup_plan_association_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location for the backupplan association
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource for which BPA needs to be created
        #[builder(into)]
        pub resource: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource type of workload on which backupplan is applied
        #[builder(into)]
        pub resource_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackupPlanAssociationResult {
        /// The BP with which resource needs to be created
        pub backup_plan: pulumi_gestalt_rust::Output<String>,
        /// The id of backupplan association
        ///
        ///
        /// - - -
        pub backup_plan_association_id: pulumi_gestalt_rust::Output<String>,
        /// The time when the instance was created
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Resource name of data source which will be used as storage location for backups taken
        pub data_source: pulumi_gestalt_rust::Output<String>,
        /// The point in time when the last successful backup was captured from the source
        pub last_successful_backup_consistency_time: pulumi_gestalt_rust::Output<String>,
        /// The location for the backupplan association
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of backup plan association resource created
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The resource for which BPA needs to be created
        pub resource: pulumi_gestalt_rust::Output<String>,
        /// The resource type of workload on which backupplan is applied
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// Message for rules config info
        /// Structure is documented below.
        pub rules_config_infos: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::backupdisasterrecovery::BackupPlanAssociationRulesConfigInfo,
            >,
        >,
        /// The time when the instance was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupPlanAssociationArgs,
    ) -> BackupPlanAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_plan_binding = args.backup_plan.get_output(context);
        let backup_plan_association_id_binding = args
            .backup_plan_association_id
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let resource_binding = args.resource.get_output(context);
        let resource_type_binding = args.resource_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:backupdisasterrecovery/backupPlanAssociation:BackupPlanAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupPlan".into(),
                    value: &backup_plan_binding.drop_type(),
                },
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resource".into(),
                    value: &resource_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackupPlanAssociationResult {
            backup_plan: o.get_field("backupPlan"),
            backup_plan_association_id: o.get_field("backupPlanAssociationId"),
            create_time: o.get_field("createTime"),
            data_source: o.get_field("dataSource"),
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
