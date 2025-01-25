/// Manages a DocumentDB Cluster.
///
/// Changes to a DocumentDB Cluster can occur when you manually change a
/// parameter, such as `port`, and are reflected in the next maintenance
/// window. Because of this, this provider may report a difference in its planning
/// phase because a modification has not yet taken place. You can use the
/// `apply_immediately` flag to instruct the service to apply the change immediately
/// (see documentation below).
///
/// > **Note:** using `apply_immediately` can result in a brief downtime as the server reboots.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let docdb = cluster::create(
///         "docdb",
///         ClusterArgs::builder()
///             .backup_retention_period(5)
///             .cluster_identifier("my-docdb-cluster")
///             .engine("docdb")
///             .master_password("mustbeeightchars")
///             .master_username("foo")
///             .preferred_backup_window("07:00-09:00")
///             .skip_final_snapshot(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DocumentDB Clusters using the `cluster_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:docdb/cluster:Cluster docdb_cluster docdb-prod-cluster
/// ```
pub mod cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// A value that indicates whether major version upgrades are allowed. Constraints: You must allow major version upgrades when specifying a value for the EngineVersion parameter that is a different major version than the DB cluster's current version.
        #[builder(into, default)]
        pub allow_major_version_upgrade: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether any cluster modifications
        /// are applied immediately, or during the next maintenance window. Default is
        /// `false`.
        #[builder(into, default)]
        pub apply_immediately: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A list of EC2 Availability Zones that
        /// instances in the DB cluster can be created in.
        #[builder(into, default)]
        pub availability_zones: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The days to retain backups for. Default `1`
        #[builder(into, default)]
        pub backup_retention_period: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The cluster identifier. If omitted, the provider will assign a random, unique identifier.
        #[builder(into, default)]
        pub cluster_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Creates a unique cluster identifier beginning with the specified prefix. Conflicts with `cluster_identifier`.
        #[builder(into, default)]
        pub cluster_identifier_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of DocumentDB Instances that are a part of this cluster
        #[builder(into, default)]
        pub cluster_members: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A cluster parameter group to associate with the cluster.
        #[builder(into, default)]
        pub db_cluster_parameter_group_name: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// A DB subnet group to associate with this DB instance.
        #[builder(into, default)]
        pub db_subnet_group_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A boolean value that indicates whether the DB cluster has deletion protection enabled. The database can't be deleted when deletion protection is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub deletion_protection: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// List of log types to export to cloudwatch. If omitted, no logs will be exported.
        /// The following log types are supported: `audit`, `profiler`.
        #[builder(into, default)]
        pub enabled_cloudwatch_logs_exports: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The name of the database engine to be used for this DB cluster. Defaults to `docdb`. Valid values: `docdb`.
        #[builder(into, default)]
        pub engine: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The database engine version. Updating this argument results in an outage.
        #[builder(into, default)]
        pub engine_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of your final DB snapshot
        /// when this DB cluster is deleted. If omitted, no final snapshot will be
        /// made.
        #[builder(into, default)]
        pub final_snapshot_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The global cluster identifier specified on `aws.docdb.GlobalCluster`.
        #[builder(into, default)]
        pub global_cluster_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ARN for the KMS encryption key. When specifying `kms_key_id`, `storage_encrypted` needs to be set to true.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Password for the master DB user. Note that this may
        /// show up in logs, and it will be stored in the state file. Please refer to the DocumentDB Naming Constraints.
        #[builder(into, default)]
        pub master_password: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Username for the master DB user.
        #[builder(into, default)]
        pub master_username: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The port on which the DB accepts connections
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The daily time range during which automated backups are created if automated backups are enabled using the BackupRetentionPeriod parameter.Time in UTC
        /// Default: A 30-minute window selected at random from an 8-hour block of time per regionE.g., 04:00-09:00
        #[builder(into, default)]
        pub preferred_backup_window: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The weekly time range during which system maintenance can occur, in (UTC) e.g., wed:04:00-wed:04:30
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// A configuration block for restoring a DB instance to an arbitrary point in time. Requires the `identifier` argument to be set with the name of the new DB instance to be created. See Restore To Point In Time below for details.
        #[builder(into, default)]
        pub restore_to_point_in_time: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::docdb::ClusterRestoreToPointInTime>,
        >,
        /// Determines whether a final DB snapshot is created before the DB cluster is deleted. If true is specified, no DB snapshot is created. If false is specified, a DB snapshot is created before the DB cluster is deleted, using the value from `final_snapshot_identifier`. Default is `false`.
        #[builder(into, default)]
        pub skip_final_snapshot: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether or not to create this cluster from a snapshot. You can use either the name or ARN when specifying a DB cluster snapshot, or the ARN when specifying a DB snapshot. Automated snapshots **should not** be used for this attribute, unless from a different cluster. Automated snapshots are deleted as part of cluster destruction when the resource is replaced.
        #[builder(into, default)]
        pub snapshot_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the DB cluster is encrypted. The default is `false`.
        #[builder(into, default)]
        pub storage_encrypted: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The storage type to associate with the DB cluster. Valid values: `standard`, `iopt1`.
        #[builder(into, default)]
        pub storage_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the DB cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of VPC security groups to associate
        /// with the Cluster
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// A value that indicates whether major version upgrades are allowed. Constraints: You must allow major version upgrades when specifying a value for the EngineVersion parameter that is a different major version than the DB cluster's current version.
        pub allow_major_version_upgrade: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether any cluster modifications
        /// are applied immediately, or during the next maintenance window. Default is
        /// `false`.
        pub apply_immediately: pulumi_wasm_rust::Output<Option<bool>>,
        /// Amazon Resource Name (ARN) of cluster
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A list of EC2 Availability Zones that
        /// instances in the DB cluster can be created in.
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        /// The days to retain backups for. Default `1`
        pub backup_retention_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// The cluster identifier. If omitted, the provider will assign a random, unique identifier.
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// Creates a unique cluster identifier beginning with the specified prefix. Conflicts with `cluster_identifier`.
        pub cluster_identifier_prefix: pulumi_wasm_rust::Output<String>,
        /// List of DocumentDB Instances that are a part of this cluster
        pub cluster_members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The DocumentDB Cluster Resource ID
        pub cluster_resource_id: pulumi_wasm_rust::Output<String>,
        /// A cluster parameter group to associate with the cluster.
        pub db_cluster_parameter_group_name: pulumi_wasm_rust::Output<String>,
        /// A DB subnet group to associate with this DB instance.
        pub db_subnet_group_name: pulumi_wasm_rust::Output<String>,
        /// A boolean value that indicates whether the DB cluster has deletion protection enabled. The database can't be deleted when deletion protection is enabled. Defaults to `false`.
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of log types to export to cloudwatch. If omitted, no logs will be exported.
        /// The following log types are supported: `audit`, `profiler`.
        pub enabled_cloudwatch_logs_exports: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The DNS address of the DocumentDB instance
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// The name of the database engine to be used for this DB cluster. Defaults to `docdb`. Valid values: `docdb`.
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        /// The database engine version. Updating this argument results in an outage.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The name of your final DB snapshot
        /// when this DB cluster is deleted. If omitted, no final snapshot will be
        /// made.
        pub final_snapshot_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// The global cluster identifier specified on `aws.docdb.GlobalCluster`.
        pub global_cluster_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// The Route53 Hosted Zone ID of the endpoint
        pub hosted_zone_id: pulumi_wasm_rust::Output<String>,
        /// The ARN for the KMS encryption key. When specifying `kms_key_id`, `storage_encrypted` needs to be set to true.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// Password for the master DB user. Note that this may
        /// show up in logs, and it will be stored in the state file. Please refer to the DocumentDB Naming Constraints.
        pub master_password: pulumi_wasm_rust::Output<Option<String>>,
        /// Username for the master DB user.
        pub master_username: pulumi_wasm_rust::Output<String>,
        /// The port on which the DB accepts connections
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The daily time range during which automated backups are created if automated backups are enabled using the BackupRetentionPeriod parameter.Time in UTC
        /// Default: A 30-minute window selected at random from an 8-hour block of time per regionE.g., 04:00-09:00
        pub preferred_backup_window: pulumi_wasm_rust::Output<String>,
        /// The weekly time range during which system maintenance can occur, in (UTC) e.g., wed:04:00-wed:04:30
        pub preferred_maintenance_window: pulumi_wasm_rust::Output<String>,
        /// A read-only endpoint for the DocumentDB cluster, automatically load-balanced across replicas
        pub reader_endpoint: pulumi_wasm_rust::Output<String>,
        /// A configuration block for restoring a DB instance to an arbitrary point in time. Requires the `identifier` argument to be set with the name of the new DB instance to be created. See Restore To Point In Time below for details.
        pub restore_to_point_in_time: pulumi_wasm_rust::Output<
            Option<super::super::types::docdb::ClusterRestoreToPointInTime>,
        >,
        /// Determines whether a final DB snapshot is created before the DB cluster is deleted. If true is specified, no DB snapshot is created. If false is specified, a DB snapshot is created before the DB cluster is deleted, using the value from `final_snapshot_identifier`. Default is `false`.
        pub skip_final_snapshot: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether or not to create this cluster from a snapshot. You can use either the name or ARN when specifying a DB cluster snapshot, or the ARN when specifying a DB snapshot. Automated snapshots **should not** be used for this attribute, unless from a different cluster. Automated snapshots are deleted as part of cluster destruction when the resource is replaced.
        pub snapshot_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the DB cluster is encrypted. The default is `false`.
        pub storage_encrypted: pulumi_wasm_rust::Output<Option<bool>>,
        /// The storage type to associate with the DB cluster. Valid values: `standard`, `iopt1`.
        pub storage_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the DB cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// List of VPC security groups to associate
        /// with the Cluster
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_major_version_upgrade_binding = args
            .allow_major_version_upgrade
            .get_output(context)
            .get_inner();
        let apply_immediately_binding = args
            .apply_immediately
            .get_output(context)
            .get_inner();
        let availability_zones_binding = args
            .availability_zones
            .get_output(context)
            .get_inner();
        let backup_retention_period_binding = args
            .backup_retention_period
            .get_output(context)
            .get_inner();
        let cluster_identifier_binding = args
            .cluster_identifier
            .get_output(context)
            .get_inner();
        let cluster_identifier_prefix_binding = args
            .cluster_identifier_prefix
            .get_output(context)
            .get_inner();
        let cluster_members_binding = args
            .cluster_members
            .get_output(context)
            .get_inner();
        let db_cluster_parameter_group_name_binding = args
            .db_cluster_parameter_group_name
            .get_output(context)
            .get_inner();
        let db_subnet_group_name_binding = args
            .db_subnet_group_name
            .get_output(context)
            .get_inner();
        let deletion_protection_binding = args
            .deletion_protection
            .get_output(context)
            .get_inner();
        let enabled_cloudwatch_logs_exports_binding = args
            .enabled_cloudwatch_logs_exports
            .get_output(context)
            .get_inner();
        let engine_binding = args.engine.get_output(context).get_inner();
        let engine_version_binding = args.engine_version.get_output(context).get_inner();
        let final_snapshot_identifier_binding = args
            .final_snapshot_identifier
            .get_output(context)
            .get_inner();
        let global_cluster_identifier_binding = args
            .global_cluster_identifier
            .get_output(context)
            .get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let master_password_binding = args
            .master_password
            .get_output(context)
            .get_inner();
        let master_username_binding = args
            .master_username
            .get_output(context)
            .get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let preferred_backup_window_binding = args
            .preferred_backup_window
            .get_output(context)
            .get_inner();
        let preferred_maintenance_window_binding = args
            .preferred_maintenance_window
            .get_output(context)
            .get_inner();
        let restore_to_point_in_time_binding = args
            .restore_to_point_in_time
            .get_output(context)
            .get_inner();
        let skip_final_snapshot_binding = args
            .skip_final_snapshot
            .get_output(context)
            .get_inner();
        let snapshot_identifier_binding = args
            .snapshot_identifier
            .get_output(context)
            .get_inner();
        let storage_encrypted_binding = args
            .storage_encrypted
            .get_output(context)
            .get_inner();
        let storage_type_binding = args.storage_type.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:docdb/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowMajorVersionUpgrade".into(),
                    value: &allow_major_version_upgrade_binding,
                },
                register_interface::ObjectField {
                    name: "applyImmediately".into(),
                    value: &apply_immediately_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZones".into(),
                    value: &availability_zones_binding,
                },
                register_interface::ObjectField {
                    name: "backupRetentionPeriod".into(),
                    value: &backup_retention_period_binding,
                },
                register_interface::ObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "clusterIdentifierPrefix".into(),
                    value: &cluster_identifier_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "clusterMembers".into(),
                    value: &cluster_members_binding,
                },
                register_interface::ObjectField {
                    name: "dbClusterParameterGroupName".into(),
                    value: &db_cluster_parameter_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "dbSubnetGroupName".into(),
                    value: &db_subnet_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "enabledCloudwatchLogsExports".into(),
                    value: &enabled_cloudwatch_logs_exports_binding,
                },
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
                },
                register_interface::ObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding,
                },
                register_interface::ObjectField {
                    name: "finalSnapshotIdentifier".into(),
                    value: &final_snapshot_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "globalClusterIdentifier".into(),
                    value: &global_cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "masterPassword".into(),
                    value: &master_password_binding,
                },
                register_interface::ObjectField {
                    name: "masterUsername".into(),
                    value: &master_username_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "preferredBackupWindow".into(),
                    value: &preferred_backup_window_binding,
                },
                register_interface::ObjectField {
                    name: "preferredMaintenanceWindow".into(),
                    value: &preferred_maintenance_window_binding,
                },
                register_interface::ObjectField {
                    name: "restoreToPointInTime".into(),
                    value: &restore_to_point_in_time_binding,
                },
                register_interface::ObjectField {
                    name: "skipFinalSnapshot".into(),
                    value: &skip_final_snapshot_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotIdentifier".into(),
                    value: &snapshot_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "storageEncrypted".into(),
                    value: &storage_encrypted_binding,
                },
                register_interface::ObjectField {
                    name: "storageType".into(),
                    value: &storage_type_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: &vpc_security_group_ids_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowMajorVersionUpgrade".into(),
                },
                register_interface::ResultField {
                    name: "applyImmediately".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "backupRetentionPeriod".into(),
                },
                register_interface::ResultField {
                    name: "clusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "clusterIdentifierPrefix".into(),
                },
                register_interface::ResultField {
                    name: "clusterMembers".into(),
                },
                register_interface::ResultField {
                    name: "clusterResourceId".into(),
                },
                register_interface::ResultField {
                    name: "dbClusterParameterGroupName".into(),
                },
                register_interface::ResultField {
                    name: "dbSubnetGroupName".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "enabledCloudwatchLogsExports".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "finalSnapshotIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "globalClusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "hostedZoneId".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "masterPassword".into(),
                },
                register_interface::ResultField {
                    name: "masterUsername".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "preferredBackupWindow".into(),
                },
                register_interface::ResultField {
                    name: "preferredMaintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "readerEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "restoreToPointInTime".into(),
                },
                register_interface::ResultField {
                    name: "skipFinalSnapshot".into(),
                },
                register_interface::ResultField {
                    name: "snapshotIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "storageEncrypted".into(),
                },
                register_interface::ResultField {
                    name: "storageType".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcSecurityGroupIds".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterResult {
            allow_major_version_upgrade: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowMajorVersionUpgrade").unwrap(),
            ),
            apply_immediately: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applyImmediately").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZones").unwrap(),
            ),
            backup_retention_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupRetentionPeriod").unwrap(),
            ),
            cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterIdentifier").unwrap(),
            ),
            cluster_identifier_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterIdentifierPrefix").unwrap(),
            ),
            cluster_members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterMembers").unwrap(),
            ),
            cluster_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterResourceId").unwrap(),
            ),
            db_cluster_parameter_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbClusterParameterGroupName").unwrap(),
            ),
            db_subnet_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbSubnetGroupName").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            enabled_cloudwatch_logs_exports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabledCloudwatchLogsExports").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            final_snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("finalSnapshotIdentifier").unwrap(),
            ),
            global_cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalClusterIdentifier").unwrap(),
            ),
            hosted_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostedZoneId").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            master_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterPassword").unwrap(),
            ),
            master_username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterUsername").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            preferred_backup_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredBackupWindow").unwrap(),
            ),
            preferred_maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredMaintenanceWindow").unwrap(),
            ),
            reader_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readerEndpoint").unwrap(),
            ),
            restore_to_point_in_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restoreToPointInTime").unwrap(),
            ),
            skip_final_snapshot: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipFinalSnapshot").unwrap(),
            ),
            snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotIdentifier").unwrap(),
            ),
            storage_encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageEncrypted").unwrap(),
            ),
            storage_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageType").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcSecurityGroupIds").unwrap(),
            ),
        }
    }
}
