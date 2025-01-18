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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPlanAssociationArgs {
        /// The BP with which resource needs to be created
        #[builder(into)]
        pub backup_plan: pulumi_wasm_rust::Output<String>,
        /// The id of backupplan association
        ///
        ///
        /// - - -
        #[builder(into)]
        pub backup_plan_association_id: pulumi_wasm_rust::Output<String>,
        /// The location for the backupplan association
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource for which BPA needs to be created
        #[builder(into)]
        pub resource: pulumi_wasm_rust::Output<String>,
        /// The resource type of workload on which backupplan is applied
        #[builder(into)]
        pub resource_type: pulumi_wasm_rust::Output<String>,
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
        name: &str,
        args: BackupPlanAssociationArgs,
    ) -> BackupPlanAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_plan_binding = args.backup_plan.get_inner();
        let backup_plan_association_id_binding = args
            .backup_plan_association_id
            .get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let resource_binding = args.resource.get_inner();
        let resource_type_binding = args.resource_type.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "backupPlan".into(),
                },
                register_interface::ResultField {
                    name: "backupPlanAssociationId".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dataSource".into(),
                },
                register_interface::ResultField {
                    name: "lastSuccessfulBackupConsistencyTime".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "resource".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
                register_interface::ResultField {
                    name: "rulesConfigInfos".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BackupPlanAssociationResult {
            backup_plan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupPlan").unwrap(),
            ),
            backup_plan_association_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupPlanAssociationId").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            data_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSource").unwrap(),
            ),
            last_successful_backup_consistency_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastSuccessfulBackupConsistencyTime").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            resource: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resource").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
            rules_config_infos: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rulesConfigInfos").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
