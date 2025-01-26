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
pub mod backup_plan_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPlanAssociationArgs {
        /// The BP with which resource needs to be created
        #[builder(into)]
        pub backup_plan: pulumi_wasm_rust::InputOrOutput<String>,
        /// The id of backupplan association
        ///
        ///
        /// - - -
        #[builder(into)]
        pub backup_plan_association_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The location for the backupplan association
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The resource for which BPA needs to be created
        #[builder(into)]
        pub resource: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource type of workload on which backupplan is applied
        #[builder(into)]
        pub resource_type: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackupPlanAssociationResult {
        /// The BP with which resource needs to be created
        pub backup_plan: pulumi_wasm_rust::Output<String>,
        /// The id of backupplan association
        ///
        ///
        /// - - -
        pub backup_plan_association_id: pulumi_wasm_rust::Output<String>,
        /// The time when the instance was created
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Resource name of data source which will be used as storage location for backups taken
        pub data_source: pulumi_wasm_rust::Output<String>,
        /// The point in time when the last successful backup was captured from the source
        pub last_successful_backup_consistency_time: pulumi_wasm_rust::Output<String>,
        /// The location for the backupplan association
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of backup plan association resource created
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The resource for which BPA needs to be created
        pub resource: pulumi_wasm_rust::Output<String>,
        /// The resource type of workload on which backupplan is applied
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// Message for rules config info
        /// Structure is documented below.
        pub rules_config_infos: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::backupdisasterrecovery::BackupPlanAssociationRulesConfigInfo,
            >,
        >,
        /// The time when the instance was updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BackupPlanAssociationArgs,
    ) -> BackupPlanAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_plan_binding = args.backup_plan.get_output(context).get_inner();
        let backup_plan_association_id_binding = args
            .backup_plan_association_id
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let resource_binding = args.resource.get_output(context).get_inner();
        let resource_type_binding = args.resource_type.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:backupdisasterrecovery/backupPlanAssociation:BackupPlanAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupPlan".into(),
                    value: &backup_plan_binding,
                },
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
                register_interface::ObjectField {
                    name: "resource".into(),
                    value: &resource_binding,
                },
                register_interface::ObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BackupPlanAssociationResult {
            backup_plan: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupPlan"),
            ),
            backup_plan_association_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupPlanAssociationId"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            data_source: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataSource"),
            ),
            last_successful_backup_consistency_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastSuccessfulBackupConsistencyTime"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            resource: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resource"),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceType"),
            ),
            rules_config_infos: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rulesConfigInfos"),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
