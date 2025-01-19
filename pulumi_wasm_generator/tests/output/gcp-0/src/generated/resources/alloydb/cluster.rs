/// ## Example Usage
///
/// ### Alloydb Cluster Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:alloydb:Cluster
///     properties:
///       clusterId: alloydb-cluster
///       location: us-central1
///       networkConfig:
///         network: ${defaultNetwork.id}
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: alloydb-cluster
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Alloydb Cluster Full
///
///
/// ```yaml
/// resources:
///   full:
///     type: gcp:alloydb:Cluster
///     properties:
///       clusterId: alloydb-cluster-full
///       location: us-central1
///       networkConfig:
///         network: ${default.id}
///       databaseVersion: POSTGRES_15
///       initialUser:
///         user: alloydb-cluster-full
///         password: alloydb-cluster-full
///       continuousBackupConfig:
///         enabled: true
///         recoveryWindowDays: 14
///       automatedBackupPolicy:
///         location: us-central1
///         backupWindow: 1800s
///         enabled: true
///         weeklySchedule:
///           daysOfWeeks:
///             - MONDAY
///           startTimes:
///             - hours: 23
///               minutes: 0
///               seconds: 0
///               nanos: 0
///         quantityBasedRetention:
///           count: 1
///         labels:
///           test: alloydb-cluster-full
///       labels:
///         test: alloydb-cluster-full
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: alloydb-cluster-full
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Alloydb Cluster Restore
///
///
/// ```yaml
/// resources:
///   source:
///     type: gcp:alloydb:Cluster
///     properties:
///       clusterId: alloydb-source-cluster
///       location: us-central1
///       network: ${default.id}
///       initialUser:
///         password: alloydb-source-cluster
///   sourceInstance:
///     type: gcp:alloydb:Instance
///     name: source
///     properties:
///       cluster: ${source.name}
///       instanceId: alloydb-instance
///       instanceType: PRIMARY
///       machineConfig:
///         cpuCount: 2
///     options:
///       dependsOn:
///         - ${vpcConnection}
///   sourceBackup:
///     type: gcp:alloydb:Backup
///     name: source
///     properties:
///       backupId: alloydb-backup
///       location: us-central1
///       clusterName: ${source.name}
///     options:
///       dependsOn:
///         - ${sourceInstance}
///   restoredFromBackup:
///     type: gcp:alloydb:Cluster
///     name: restored_from_backup
///     properties:
///       clusterId: alloydb-backup-restored
///       location: us-central1
///       networkConfig:
///         network: ${default.id}
///       restoreBackupSource:
///         backupName: ${sourceBackup.name}
///   restoredViaPitr:
///     type: gcp:alloydb:Cluster
///     name: restored_via_pitr
///     properties:
///       clusterId: alloydb-pitr-restored
///       location: us-central1
///       networkConfig:
///         network: ${default.id}
///       restoreContinuousBackupSource:
///         cluster: ${source.name}
///         pointInTime: 2023-08-03T19:19:00.094Z
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: alloydb-source-cluster
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 16
///       network: ${default.id}
///   vpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vpc_connection
///     properties:
///       network: ${default.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAlloc.name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
///   default:
///     fn::invoke:
///       function: gcp:compute:getNetwork
///       arguments:
///         name: alloydb-network
/// ```
/// ### Alloydb Secondary Cluster Basic
///
///
/// ```yaml
/// resources:
///   primary:
///     type: gcp:alloydb:Cluster
///     properties:
///       clusterId: alloydb-primary-cluster
///       location: us-central1
///       networkConfig:
///         network: ${default.id}
///   primaryInstance:
///     type: gcp:alloydb:Instance
///     name: primary
///     properties:
///       cluster: ${primary.name}
///       instanceId: alloydb-primary-instance
///       instanceType: PRIMARY
///       machineConfig:
///         cpuCount: 2
///     options:
///       dependsOn:
///         - ${vpcConnection}
///   secondary:
///     type: gcp:alloydb:Cluster
///     properties:
///       clusterId: alloydb-secondary-cluster
///       location: us-east1
///       networkConfig:
///         network: ${default.id}
///       clusterType: SECONDARY
///       continuousBackupConfig:
///         enabled: false
///       secondaryConfig:
///         primaryClusterName: ${primary.name}
///     options:
///       dependsOn:
///         - ${primaryInstance}
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: alloydb-secondary-cluster
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: alloydb-secondary-cluster
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 16
///       network: ${default.id}
///   vpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vpc_connection
///     properties:
///       network: ${default.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAlloc.name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Cluster can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/clusters/{{cluster_id}}`
///
/// * `{{project}}/{{location}}/{{cluster_id}}`
///
/// * `{{location}}/{{cluster_id}}`
///
/// * `{{cluster_id}}`
///
/// When using the `pulumi import` command, Cluster can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:alloydb/cluster:Cluster default projects/{{project}}/locations/{{location}}/clusters/{{cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:alloydb/cluster:Cluster default {{project}}/{{location}}/{{cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:alloydb/cluster:Cluster default {{location}}/{{cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:alloydb/cluster:Cluster default {{cluster_id}}
/// ```
///
pub mod cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The automated backup policy for this cluster. AutomatedBackupPolicy is disabled by default.
        /// Structure is documented below.
        #[builder(into, default)]
        pub automated_backup_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterAutomatedBackupPolicy>,
        >,
        /// The ID of the alloydb cluster.
        #[builder(into)]
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// The type of cluster. If not set, defaults to PRIMARY.
        /// Default value is `PRIMARY`.
        /// Possible values are: `PRIMARY`, `SECONDARY`.
        #[builder(into, default)]
        pub cluster_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The continuous backup config for this cluster.
        /// If no policy is provided then the default policy will be used. The default policy takes one backup a day and retains backups for 14 days.
        /// Structure is documented below.
        #[builder(into, default)]
        pub continuous_backup_config: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterContinuousBackupConfig>,
        >,
        /// The database engine major version. This is an optional field and it's populated at the Cluster creation time. This field cannot be changed after cluster creation.
        #[builder(into, default)]
        pub database_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Policy to determine if the cluster should be deleted forcefully.
        /// Deleting a cluster forcefully, deletes the cluster and all its associated instances within the cluster.
        /// Deleting a Secondary cluster with a secondary instance REQUIRES setting deletion_policy = "FORCE" otherwise an error is returned. This is needed as there is no support to delete just the secondary instance, and the only way to delete secondary instance is to delete the associated secondary cluster forcefully which also deletes the secondary instance.
        /// Possible values: DEFAULT, FORCE
        #[builder(into, default)]
        pub deletion_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// User-settable and human-readable display name for the Cluster.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// EncryptionConfig describes the encryption config of a cluster or a backup that is encrypted with a CMEK (customer-managed encryption key).
        /// Structure is documented below.
        #[builder(into, default)]
        pub encryption_config: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterEncryptionConfig>,
        >,
        /// For Resource freshness validation (https://google.aip.dev/154)
        #[builder(into, default)]
        pub etag: pulumi_wasm_rust::Output<Option<String>>,
        /// Initial user to setup during cluster creation.
        /// Structure is documented below.
        #[builder(into, default)]
        pub initial_user: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterInitialUser>,
        >,
        /// User-defined labels for the alloydb cluster.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the alloydb cluster should reside.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// MaintenanceUpdatePolicy defines the policy for system updates.
        /// Structure is documented below.
        #[builder(into, default)]
        pub maintenance_update_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterMaintenanceUpdatePolicy>,
        >,
        /// Metadata related to network configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub network_config: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterNetworkConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration for Private Service Connect (PSC) for the cluster.
        /// Structure is documented below.
        #[builder(into, default)]
        pub psc_config: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterPscConfig>,
        >,
        /// The source when restoring from a backup. Conflicts with 'restore_continuous_backup_source', both can't be set together.
        /// Structure is documented below.
        #[builder(into, default)]
        pub restore_backup_source: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterRestoreBackupSource>,
        >,
        /// The source when restoring via point in time recovery (PITR). Conflicts with 'restore_backup_source', both can't be set together.
        /// Structure is documented below.
        #[builder(into, default)]
        pub restore_continuous_backup_source: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterRestoreContinuousBackupSource>,
        >,
        /// Configuration of the secondary cluster for Cross Region Replication. This should be set if and only if the cluster is of type SECONDARY.
        /// Structure is documented below.
        #[builder(into, default)]
        pub secondary_config: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterSecondaryConfig>,
        >,
        /// The subscrition type of cluster.
        /// Possible values are: `TRIAL`, `STANDARD`.
        #[builder(into, default)]
        pub subscription_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The automated backup policy for this cluster. AutomatedBackupPolicy is disabled by default.
        /// Structure is documented below.
        pub automated_backup_policy: pulumi_wasm_rust::Output<
            super::super::types::alloydb::ClusterAutomatedBackupPolicy,
        >,
        /// Cluster created from backup.
        /// Structure is documented below.
        pub backup_sources: pulumi_wasm_rust::Output<
            Vec<super::super::types::alloydb::ClusterBackupSource>,
        >,
        /// The ID of the alloydb cluster.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// The type of cluster. If not set, defaults to PRIMARY.
        /// Default value is `PRIMARY`.
        /// Possible values are: `PRIMARY`, `SECONDARY`.
        pub cluster_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The continuous backup config for this cluster.
        /// If no policy is provided then the default policy will be used. The default policy takes one backup a day and retains backups for 14 days.
        /// Structure is documented below.
        pub continuous_backup_config: pulumi_wasm_rust::Output<
            super::super::types::alloydb::ClusterContinuousBackupConfig,
        >,
        /// ContinuousBackupInfo describes the continuous backup properties of a cluster.
        /// Structure is documented below.
        pub continuous_backup_infos: pulumi_wasm_rust::Output<
            Vec<super::super::types::alloydb::ClusterContinuousBackupInfo>,
        >,
        /// The database engine major version. This is an optional field and it's populated at the Cluster creation time. This field cannot be changed after cluster creation.
        pub database_version: pulumi_wasm_rust::Output<String>,
        /// Policy to determine if the cluster should be deleted forcefully.
        /// Deleting a cluster forcefully, deletes the cluster and all its associated instances within the cluster.
        /// Deleting a Secondary cluster with a secondary instance REQUIRES setting deletion_policy = "FORCE" otherwise an error is returned. This is needed as there is no support to delete just the secondary instance, and the only way to delete secondary instance is to delete the associated secondary cluster forcefully which also deletes the secondary instance.
        /// Possible values: DEFAULT, FORCE
        pub deletion_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// User-settable and human-readable display name for the Cluster.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// EncryptionConfig describes the encryption config of a cluster or a backup that is encrypted with a CMEK (customer-managed encryption key).
        /// Structure is documented below.
        pub encryption_config: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterEncryptionConfig>,
        >,
        /// (Output)
        /// Output only. The encryption information for the WALs and backups required for ContinuousBackup.
        /// Structure is documented below.
        pub encryption_infos: pulumi_wasm_rust::Output<
            Vec<super::super::types::alloydb::ClusterEncryptionInfo>,
        >,
        /// For Resource freshness validation (https://google.aip.dev/154)
        pub etag: pulumi_wasm_rust::Output<Option<String>>,
        /// Initial user to setup during cluster creation.
        /// Structure is documented below.
        pub initial_user: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterInitialUser>,
        >,
        /// User-defined labels for the alloydb cluster.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the alloydb cluster should reside.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// MaintenanceUpdatePolicy defines the policy for system updates.
        /// Structure is documented below.
        pub maintenance_update_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterMaintenanceUpdatePolicy>,
        >,
        /// Cluster created via DMS migration.
        /// Structure is documented below.
        pub migration_sources: pulumi_wasm_rust::Output<
            Vec<super::super::types::alloydb::ClusterMigrationSource>,
        >,
        /// The name of the cluster resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Metadata related to network configuration.
        /// Structure is documented below.
        pub network_config: pulumi_wasm_rust::Output<
            super::super::types::alloydb::ClusterNetworkConfig,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Configuration for Private Service Connect (PSC) for the cluster.
        /// Structure is documented below.
        pub psc_config: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterPscConfig>,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Reconciling (https://google.aip.dev/128#reconciliation).
        /// Set to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them.
        /// This can happen due to user-triggered updates or system actions like failover or maintenance.
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        /// The source when restoring from a backup. Conflicts with 'restore_continuous_backup_source', both can't be set together.
        /// Structure is documented below.
        pub restore_backup_source: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterRestoreBackupSource>,
        >,
        /// The source when restoring via point in time recovery (PITR). Conflicts with 'restore_backup_source', both can't be set together.
        /// Structure is documented below.
        pub restore_continuous_backup_source: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterRestoreContinuousBackupSource>,
        >,
        /// Configuration of the secondary cluster for Cross Region Replication. This should be set if and only if the cluster is of type SECONDARY.
        /// Structure is documented below.
        pub secondary_config: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::ClusterSecondaryConfig>,
        >,
        /// Output only. The current serving state of the cluster.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The subscrition type of cluster.
        /// Possible values are: `TRIAL`, `STANDARD`.
        pub subscription_type: pulumi_wasm_rust::Output<String>,
        /// Contains information and all metadata related to TRIAL clusters.
        /// Structure is documented below.
        pub trial_metadatas: pulumi_wasm_rust::Output<
            Vec<super::super::types::alloydb::ClusterTrialMetadata>,
        >,
        /// The system-generated UID of the resource.
        pub uid: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ClusterArgs) -> ClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_inner();
        let automated_backup_policy_binding = args.automated_backup_policy.get_inner();
        let cluster_id_binding = args.cluster_id.get_inner();
        let cluster_type_binding = args.cluster_type.get_inner();
        let continuous_backup_config_binding = args.continuous_backup_config.get_inner();
        let database_version_binding = args.database_version.get_inner();
        let deletion_policy_binding = args.deletion_policy.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let encryption_config_binding = args.encryption_config.get_inner();
        let etag_binding = args.etag.get_inner();
        let initial_user_binding = args.initial_user.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let maintenance_update_policy_binding = args
            .maintenance_update_policy
            .get_inner();
        let network_config_binding = args.network_config.get_inner();
        let project_binding = args.project.get_inner();
        let psc_config_binding = args.psc_config.get_inner();
        let restore_backup_source_binding = args.restore_backup_source.get_inner();
        let restore_continuous_backup_source_binding = args
            .restore_continuous_backup_source
            .get_inner();
        let secondary_config_binding = args.secondary_config.get_inner();
        let subscription_type_binding = args.subscription_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:alloydb/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "automatedBackupPolicy".into(),
                    value: &automated_backup_policy_binding,
                },
                register_interface::ObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "clusterType".into(),
                    value: &cluster_type_binding,
                },
                register_interface::ObjectField {
                    name: "continuousBackupConfig".into(),
                    value: &continuous_backup_config_binding,
                },
                register_interface::ObjectField {
                    name: "databaseVersion".into(),
                    value: &database_version_binding,
                },
                register_interface::ObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionConfig".into(),
                    value: &encryption_config_binding,
                },
                register_interface::ObjectField {
                    name: "etag".into(),
                    value: &etag_binding,
                },
                register_interface::ObjectField {
                    name: "initialUser".into(),
                    value: &initial_user_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceUpdatePolicy".into(),
                    value: &maintenance_update_policy_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfig".into(),
                    value: &network_config_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "pscConfig".into(),
                    value: &psc_config_binding,
                },
                register_interface::ObjectField {
                    name: "restoreBackupSource".into(),
                    value: &restore_backup_source_binding,
                },
                register_interface::ObjectField {
                    name: "restoreContinuousBackupSource".into(),
                    value: &restore_continuous_backup_source_binding,
                },
                register_interface::ObjectField {
                    name: "secondaryConfig".into(),
                    value: &secondary_config_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionType".into(),
                    value: &subscription_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "automatedBackupPolicy".into(),
                },
                register_interface::ResultField {
                    name: "backupSources".into(),
                },
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "clusterType".into(),
                },
                register_interface::ResultField {
                    name: "continuousBackupConfig".into(),
                },
                register_interface::ResultField {
                    name: "continuousBackupInfos".into(),
                },
                register_interface::ResultField {
                    name: "databaseVersion".into(),
                },
                register_interface::ResultField {
                    name: "deletionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "encryptionConfig".into(),
                },
                register_interface::ResultField {
                    name: "encryptionInfos".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "initialUser".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceUpdatePolicy".into(),
                },
                register_interface::ResultField {
                    name: "migrationSources".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkConfig".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pscConfig".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "reconciling".into(),
                },
                register_interface::ResultField {
                    name: "restoreBackupSource".into(),
                },
                register_interface::ResultField {
                    name: "restoreContinuousBackupSource".into(),
                },
                register_interface::ResultField {
                    name: "secondaryConfig".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionType".into(),
                },
                register_interface::ResultField {
                    name: "trialMetadatas".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            automated_backup_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automatedBackupPolicy").unwrap(),
            ),
            backup_sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupSources").unwrap(),
            ),
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            cluster_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterType").unwrap(),
            ),
            continuous_backup_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("continuousBackupConfig").unwrap(),
            ),
            continuous_backup_infos: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("continuousBackupInfos").unwrap(),
            ),
            database_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseVersion").unwrap(),
            ),
            deletion_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionPolicy").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            encryption_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfig").unwrap(),
            ),
            encryption_infos: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionInfos").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            initial_user: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("initialUser").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maintenance_update_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceUpdatePolicy").unwrap(),
            ),
            migration_sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("migrationSources").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkConfig").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            psc_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pscConfig").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reconciling").unwrap(),
            ),
            restore_backup_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restoreBackupSource").unwrap(),
            ),
            restore_continuous_backup_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restoreContinuousBackupSource").unwrap(),
            ),
            secondary_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryConfig").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            subscription_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionType").unwrap(),
            ),
            trial_metadatas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trialMetadatas").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(hashmap.remove("uid").unwrap()),
        }
    }
}
