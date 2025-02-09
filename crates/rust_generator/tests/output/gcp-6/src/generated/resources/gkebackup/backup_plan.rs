/// Represents a Backup Plan instance.
///
///
/// To get more information about BackupPlan, see:
///
/// * [API documentation](https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/projects.locations.backupPlans)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke)
///
/// ## Example Usage
///
/// ### Gkebackup Backupplan Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = backup_plan::create(
///         "basic",
///         BackupPlanArgs::builder()
///             .backup_config(
///                 BackupPlanBackupConfig::builder()
///                     .allNamespaces(true)
///                     .includeSecrets(true)
///                     .includeVolumeData(true)
///                     .build_struct(),
///             )
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("basic-plan")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .addons_config(
///                 ClusterAddonsConfig::builder()
///                     .gkeBackupAgentConfig(
///                         ClusterAddonsConfigGkeBackupAgentConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("basic-cluster")
///             .network("default")
///             .subnetwork("default")
///             .workload_identity_config(
///                 ClusterWorkloadIdentityConfig::builder()
///                     .workloadPool("my-project-name.svc.id.goog")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkebackup Backupplan Autopilot
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let autopilot = backup_plan::create(
///         "autopilot",
///         BackupPlanArgs::builder()
///             .backup_config(
///                 BackupPlanBackupConfig::builder()
///                     .allNamespaces(true)
///                     .includeSecrets(true)
///                     .includeVolumeData(true)
///                     .build_struct(),
///             )
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("autopilot-plan")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .addons_config(
///                 ClusterAddonsConfig::builder()
///                     .gkeBackupAgentConfig(
///                         ClusterAddonsConfigGkeBackupAgentConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .enable_autopilot(true)
///             .ip_allocation_policy(ClusterIpAllocationPolicy::builder().build_struct())
///             .location("us-central1")
///             .name("autopilot-cluster")
///             .network("default")
///             .release_channel(
///                 ClusterReleaseChannel::builder().channel("RAPID").build_struct(),
///             )
///             .subnetwork("default")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkebackup Backupplan Cmek
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cmek = backup_plan::create(
///         "cmek",
///         BackupPlanArgs::builder()
///             .backup_config(
///                 BackupPlanBackupConfig::builder()
///                     .encryptionKey(
///                         BackupPlanBackupConfigEncryptionKey::builder()
///                             .gcpKmsEncryptionKey("${cryptoKey.id}")
///                             .build_struct(),
///                     )
///                     .includeSecrets(true)
///                     .includeVolumeData(true)
///                     .selectedNamespaces(
///                         BackupPlanBackupConfigSelectedNamespaces::builder()
///                             .namespaces(vec!["default", "test",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("cmek-plan")
///             .build_struct(),
///     );
///     let cryptoKey = crypto_key::create(
///         "cryptoKey",
///         CryptoKeyArgs::builder()
///             .key_ring("${keyRing.id}")
///             .name("backup-key")
///             .build_struct(),
///     );
///     let keyRing = key_ring::create(
///         "keyRing",
///         KeyRingArgs::builder().location("us-central1").name("backup-key").build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .addons_config(
///                 ClusterAddonsConfig::builder()
///                     .gkeBackupAgentConfig(
///                         ClusterAddonsConfigGkeBackupAgentConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("cmek-cluster")
///             .network("default")
///             .subnetwork("default")
///             .workload_identity_config(
///                 ClusterWorkloadIdentityConfig::builder()
///                     .workloadPool("my-project-name.svc.id.goog")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkebackup Backupplan Full
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let full = backup_plan::create(
///         "full",
///         BackupPlanArgs::builder()
///             .backup_config(
///                 BackupPlanBackupConfig::builder()
///                     .includeSecrets(true)
///                     .includeVolumeData(true)
///                     .selectedApplications(
///                         BackupPlanBackupConfigSelectedApplications::builder()
///                             .namespacedNames(
///                                 vec![
///                                     BackupPlanBackupConfigSelectedApplicationsNamespacedName::builder()
///                                     .name("app1").namespace("ns1").build_struct(),
///                                     BackupPlanBackupConfigSelectedApplicationsNamespacedName::builder()
///                                     .name("app2").namespace("ns2").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .backup_schedule(
///                 BackupPlanBackupSchedule::builder()
///                     .cronSchedule("0 9 * * 1")
///                     .build_struct(),
///             )
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("full-plan")
///             .retention_policy(
///                 BackupPlanRetentionPolicy::builder()
///                     .backupDeleteLockDays(30)
///                     .backupRetainDays(180)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .addons_config(
///                 ClusterAddonsConfig::builder()
///                     .gkeBackupAgentConfig(
///                         ClusterAddonsConfigGkeBackupAgentConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("full-cluster")
///             .network("default")
///             .subnetwork("default")
///             .workload_identity_config(
///                 ClusterWorkloadIdentityConfig::builder()
///                     .workloadPool("my-project-name.svc.id.goog")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkebackup Backupplan Permissive
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let permissive = backup_plan::create(
///         "permissive",
///         BackupPlanArgs::builder()
///             .backup_config(
///                 BackupPlanBackupConfig::builder()
///                     .includeSecrets(true)
///                     .includeVolumeData(true)
///                     .permissiveMode(true)
///                     .selectedApplications(
///                         BackupPlanBackupConfigSelectedApplications::builder()
///                             .namespacedNames(
///                                 vec![
///                                     BackupPlanBackupConfigSelectedApplicationsNamespacedName::builder()
///                                     .name("app1").namespace("ns1").build_struct(),
///                                     BackupPlanBackupConfigSelectedApplicationsNamespacedName::builder()
///                                     .name("app2").namespace("ns2").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .backup_schedule(
///                 BackupPlanBackupSchedule::builder()
///                     .cronSchedule("0 9 * * 1")
///                     .build_struct(),
///             )
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("permissive-plan")
///             .retention_policy(
///                 BackupPlanRetentionPolicy::builder()
///                     .backupDeleteLockDays(30)
///                     .backupRetainDays(180)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .addons_config(
///                 ClusterAddonsConfig::builder()
///                     .gkeBackupAgentConfig(
///                         ClusterAddonsConfigGkeBackupAgentConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("permissive-cluster")
///             .network("default")
///             .subnetwork("default")
///             .workload_identity_config(
///                 ClusterWorkloadIdentityConfig::builder()
///                     .workloadPool("my-project-name.svc.id.goog")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkebackup Backupplan Rpo Daily Window
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .addons_config(
///                 ClusterAddonsConfig::builder()
///                     .gkeBackupAgentConfig(
///                         ClusterAddonsConfigGkeBackupAgentConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("rpo-daily-cluster")
///             .network("default")
///             .subnetwork("default")
///             .workload_identity_config(
///                 ClusterWorkloadIdentityConfig::builder()
///                     .workloadPool("my-project-name.svc.id.goog")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let rpoDailyWindow = backup_plan::create(
///         "rpoDailyWindow",
///         BackupPlanArgs::builder()
///             .backup_config(
///                 BackupPlanBackupConfig::builder()
///                     .allNamespaces(true)
///                     .includeSecrets(true)
///                     .includeVolumeData(true)
///                     .build_struct(),
///             )
///             .backup_schedule(
///                 BackupPlanBackupSchedule::builder()
///                     .paused(true)
///                     .rpoConfig(
///                         BackupPlanBackupScheduleRpoConfig::builder()
///                             .exclusionWindows(
///                                 vec![
///                                     BackupPlanBackupScheduleRpoConfigExclusionWindow::builder()
///                                     .daily(true).duration("7200s")
///                                     .startTime(BackupPlanBackupScheduleRpoConfigExclusionWindowStartTime::builder()
///                                     .hours(12).build_struct()).build_struct(),
///                                     BackupPlanBackupScheduleRpoConfigExclusionWindow::builder()
///                                     .duration("3600s")
///                                     .singleOccurrenceDate(BackupPlanBackupScheduleRpoConfigExclusionWindowSingleOccurrenceDate::builder()
///                                     .day(16).month(3).year(2024).build_struct())
///                                     .startTime(BackupPlanBackupScheduleRpoConfigExclusionWindowStartTime::builder()
///                                     .hours(8).minutes(40).nanos(100).seconds(1).build_struct())
///                                     .build_struct(),
///                                 ],
///                             )
///                             .targetRpoMinutes(1440)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("rpo-daily-window")
///             .retention_policy(
///                 BackupPlanRetentionPolicy::builder()
///                     .backupDeleteLockDays(30)
///                     .backupRetainDays(180)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkebackup Backupplan Rpo Weekly Window
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .addons_config(
///                 ClusterAddonsConfig::builder()
///                     .gkeBackupAgentConfig(
///                         ClusterAddonsConfigGkeBackupAgentConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("rpo-weekly-cluster")
///             .network("default")
///             .subnetwork("default")
///             .workload_identity_config(
///                 ClusterWorkloadIdentityConfig::builder()
///                     .workloadPool("my-project-name.svc.id.goog")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let rpoWeeklyWindow = backup_plan::create(
///         "rpoWeeklyWindow",
///         BackupPlanArgs::builder()
///             .backup_config(
///                 BackupPlanBackupConfig::builder()
///                     .allNamespaces(true)
///                     .includeSecrets(true)
///                     .includeVolumeData(true)
///                     .build_struct(),
///             )
///             .backup_schedule(
///                 BackupPlanBackupSchedule::builder()
///                     .paused(true)
///                     .rpoConfig(
///                         BackupPlanBackupScheduleRpoConfig::builder()
///                             .exclusionWindows(
///                                 vec![
///                                     BackupPlanBackupScheduleRpoConfigExclusionWindow::builder()
///                                     .daysOfWeek(BackupPlanBackupScheduleRpoConfigExclusionWindowDaysOfWeek::builder()
///                                     .daysOfWeeks(vec!["MONDAY", "THURSDAY",]).build_struct())
///                                     .duration("1800s")
///                                     .startTime(BackupPlanBackupScheduleRpoConfigExclusionWindowStartTime::builder()
///                                     .hours(1).minutes(23).build_struct()).build_struct(),
///                                     BackupPlanBackupScheduleRpoConfigExclusionWindow::builder()
///                                     .duration("3600s")
///                                     .singleOccurrenceDate(BackupPlanBackupScheduleRpoConfigExclusionWindowSingleOccurrenceDate::builder()
///                                     .day(17).month(3).year(2024).build_struct())
///                                     .startTime(BackupPlanBackupScheduleRpoConfigExclusionWindowStartTime::builder()
///                                     .hours(12).build_struct()).build_struct(),
///                                     BackupPlanBackupScheduleRpoConfigExclusionWindow::builder()
///                                     .duration("600s")
///                                     .singleOccurrenceDate(BackupPlanBackupScheduleRpoConfigExclusionWindowSingleOccurrenceDate::builder()
///                                     .day(18).month(3).year(2024).build_struct())
///                                     .startTime(BackupPlanBackupScheduleRpoConfigExclusionWindowStartTime::builder()
///                                     .hours(8).minutes(40).build_struct()).build_struct(),
///                                 ],
///                             )
///                             .targetRpoMinutes(1440)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("rpo-weekly-window")
///             .retention_policy(
///                 BackupPlanRetentionPolicy::builder()
///                     .backupDeleteLockDays(30)
///                     .backupRetainDays(180)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// BackupPlan can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/backupPlans/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, BackupPlan can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkebackup/backupPlan:BackupPlan default projects/{{project}}/locations/{{location}}/backupPlans/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkebackup/backupPlan:BackupPlan default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkebackup/backupPlan:BackupPlan default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPlanArgs {
        /// Defines the configuration of Backups created via this BackupPlan.
        /// Structure is documented below.
        #[builder(into, default)]
        pub backup_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkebackup::BackupPlanBackupConfig>,
        >,
        /// Defines a schedule for automatic Backup creation via this BackupPlan.
        /// Structure is documented below.
        #[builder(into, default)]
        pub backup_schedule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkebackup::BackupPlanBackupSchedule>,
        >,
        /// The source cluster from which Backups will be created via this BackupPlan.
        #[builder(into)]
        pub cluster: pulumi_gestalt_rust::InputOrOutput<String>,
        /// This flag indicates whether this BackupPlan has been deactivated.
        /// Setting this field to True locks the BackupPlan such that no further updates will be allowed
        /// (except deletes), including the deactivated field itself. It also prevents any new Backups
        /// from being created via this BackupPlan (including scheduled Backups).
        #[builder(into, default)]
        pub deactivated: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// User specified descriptive string for this BackupPlan.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description: A set of custom labels supplied by the user.
        /// A list of key->value pairs.
        /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The region of the Backup Plan.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The full name of the BackupPlan Resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// RetentionPolicy governs lifecycle of Backups created under this plan.
        /// Structure is documented below.
        #[builder(into, default)]
        pub retention_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkebackup::BackupPlanRetentionPolicy>,
        >,
    }
    #[allow(dead_code)]
    pub struct BackupPlanResult {
        /// Defines the configuration of Backups created via this BackupPlan.
        /// Structure is documented below.
        pub backup_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkebackup::BackupPlanBackupConfig>,
        >,
        /// Defines a schedule for automatic Backup creation via this BackupPlan.
        /// Structure is documented below.
        pub backup_schedule: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkebackup::BackupPlanBackupSchedule>,
        >,
        /// The source cluster from which Backups will be created via this BackupPlan.
        pub cluster: pulumi_gestalt_rust::Output<String>,
        /// This flag indicates whether this BackupPlan has been deactivated.
        /// Setting this field to True locks the BackupPlan such that no further updates will be allowed
        /// (except deletes), including the deactivated field itself. It also prevents any new Backups
        /// from being created via this BackupPlan (including scheduled Backups).
        pub deactivated: pulumi_gestalt_rust::Output<bool>,
        /// User specified descriptive string for this BackupPlan.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// etag is used for optimistic concurrency control as a way to help prevent simultaneous
        /// updates of a backup plan from overwriting each other. It is strongly suggested that
        /// systems make use of the 'etag' in the read-modify-write cycle to perform BackupPlan updates
        /// in order to avoid race conditions: An etag is returned in the response to backupPlans.get,
        /// and systems are expected to put that etag in the request to backupPlans.patch or
        /// backupPlans.delete to ensure that their change will be applied to the same version of the resource.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Description: A set of custom labels supplied by the user.
        /// A list of key->value pairs.
        /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The region of the Backup Plan.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The full name of the BackupPlan Resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The number of Kubernetes Pods backed up in the last successful Backup created via this BackupPlan.
        pub protected_pod_count: pulumi_gestalt_rust::Output<i32>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// RetentionPolicy governs lifecycle of Backups created under this plan.
        /// Structure is documented below.
        pub retention_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkebackup::BackupPlanRetentionPolicy>,
        >,
        /// The State of the BackupPlan.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Detailed description of why BackupPlan is in its current state.
        pub state_reason: pulumi_gestalt_rust::Output<String>,
        /// Server generated, unique identifier of UUID format.
        pub uid: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupPlanArgs,
    ) -> BackupPlanResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_config_binding = args.backup_config.get_output(context);
        let backup_schedule_binding = args.backup_schedule.get_output(context);
        let cluster_binding = args.cluster.get_output(context);
        let deactivated_binding = args.deactivated.get_output(context);
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let retention_policy_binding = args.retention_policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:gkebackup/backupPlan:BackupPlan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupConfig".into(),
                    value: backup_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupSchedule".into(),
                    value: backup_schedule_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cluster".into(),
                    value: cluster_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deactivated".into(),
                    value: deactivated_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionPolicy".into(),
                    value: retention_policy_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackupPlanResult {
            backup_config: o.get_field("backupConfig"),
            backup_schedule: o.get_field("backupSchedule"),
            cluster: o.get_field("cluster"),
            deactivated: o.get_field("deactivated"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            protected_pod_count: o.get_field("protectedPodCount"),
            pulumi_labels: o.get_field("pulumiLabels"),
            retention_policy: o.get_field("retentionPolicy"),
            state: o.get_field("state"),
            state_reason: o.get_field("stateReason"),
            uid: o.get_field("uid"),
        }
    }
}
