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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// A value that indicates whether major version upgrades are allowed. Constraints: You must allow major version upgrades when specifying a value for the EngineVersion parameter that is a different major version than the DB cluster's current version.
        #[builder(into, default)]
        pub allow_major_version_upgrade: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies whether any cluster modifications
        /// are applied immediately, or during the next maintenance window. Default is
        /// `false`.
        #[builder(into, default)]
        pub apply_immediately: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of EC2 Availability Zones that
        /// instances in the DB cluster can be created in.
        #[builder(into, default)]
        pub availability_zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The days to retain backups for. Default `1`
        #[builder(into, default)]
        pub backup_retention_period: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The cluster identifier. If omitted, the provider will assign a random, unique identifier.
        #[builder(into, default)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique cluster identifier beginning with the specified prefix. Conflicts with `cluster_identifier`.
        #[builder(into, default)]
        pub cluster_identifier_prefix: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// List of DocumentDB Instances that are a part of this cluster
        #[builder(into, default)]
        pub cluster_members: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A cluster parameter group to associate with the cluster.
        #[builder(into, default)]
        pub db_cluster_parameter_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A DB subnet group to associate with this DB instance.
        #[builder(into, default)]
        pub db_subnet_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A boolean value that indicates whether the DB cluster has deletion protection enabled. The database can't be deleted when deletion protection is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of log types to export to cloudwatch. If omitted, no logs will be exported.
        /// The following log types are supported: `audit`, `profiler`.
        #[builder(into, default)]
        pub enabled_cloudwatch_logs_exports: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The name of the database engine to be used for this DB cluster. Defaults to `docdb`. Valid values: `docdb`.
        #[builder(into, default)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The database engine version. Updating this argument results in an outage.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of your final DB snapshot
        /// when this DB cluster is deleted. If omitted, no final snapshot will be
        /// made.
        #[builder(into, default)]
        pub final_snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The global cluster identifier specified on `aws.docdb.GlobalCluster`.
        #[builder(into, default)]
        pub global_cluster_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ARN for the KMS encryption key. When specifying `kms_key_id`, `storage_encrypted` needs to be set to true.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Password for the master DB user. Note that this may
        /// show up in logs, and it will be stored in the state file. Please refer to the DocumentDB Naming Constraints.
        #[builder(into, default)]
        pub master_password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Username for the master DB user.
        #[builder(into, default)]
        pub master_username: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The port on which the DB accepts connections
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The daily time range during which automated backups are created if automated backups are enabled using the BackupRetentionPeriod parameter.Time in UTC
        /// Default: A 30-minute window selected at random from an 8-hour block of time per regionE.g., 04:00-09:00
        #[builder(into, default)]
        pub preferred_backup_window: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The weekly time range during which system maintenance can occur, in (UTC) e.g., wed:04:00-wed:04:30
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A configuration block for restoring a DB instance to an arbitrary point in time. Requires the `identifier` argument to be set with the name of the new DB instance to be created. See Restore To Point In Time below for details.
        #[builder(into, default)]
        pub restore_to_point_in_time: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::docdb::ClusterRestoreToPointInTime>,
        >,
        /// Determines whether a final DB snapshot is created before the DB cluster is deleted. If true is specified, no DB snapshot is created. If false is specified, a DB snapshot is created before the DB cluster is deleted, using the value from `final_snapshot_identifier`. Default is `false`.
        #[builder(into, default)]
        pub skip_final_snapshot: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether or not to create this cluster from a snapshot. You can use either the name or ARN when specifying a DB cluster snapshot, or the ARN when specifying a DB snapshot. Automated snapshots **should not** be used for this attribute, unless from a different cluster. Automated snapshots are deleted as part of cluster destruction when the resource is replaced.
        #[builder(into, default)]
        pub snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the DB cluster is encrypted. The default is `false`.
        #[builder(into, default)]
        pub storage_encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The storage type to associate with the DB cluster. Valid values: `standard`, `iopt1`.
        #[builder(into, default)]
        pub storage_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the DB cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of VPC security groups to associate
        /// with the Cluster
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// A value that indicates whether major version upgrades are allowed. Constraints: You must allow major version upgrades when specifying a value for the EngineVersion parameter that is a different major version than the DB cluster's current version.
        pub allow_major_version_upgrade: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether any cluster modifications
        /// are applied immediately, or during the next maintenance window. Default is
        /// `false`.
        pub apply_immediately: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Amazon Resource Name (ARN) of cluster
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A list of EC2 Availability Zones that
        /// instances in the DB cluster can be created in.
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The days to retain backups for. Default `1`
        pub backup_retention_period: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The cluster identifier. If omitted, the provider will assign a random, unique identifier.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique cluster identifier beginning with the specified prefix. Conflicts with `cluster_identifier`.
        pub cluster_identifier_prefix: pulumi_gestalt_rust::Output<String>,
        /// List of DocumentDB Instances that are a part of this cluster
        pub cluster_members: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The DocumentDB Cluster Resource ID
        pub cluster_resource_id: pulumi_gestalt_rust::Output<String>,
        /// A cluster parameter group to associate with the cluster.
        pub db_cluster_parameter_group_name: pulumi_gestalt_rust::Output<String>,
        /// A DB subnet group to associate with this DB instance.
        pub db_subnet_group_name: pulumi_gestalt_rust::Output<String>,
        /// A boolean value that indicates whether the DB cluster has deletion protection enabled. The database can't be deleted when deletion protection is enabled. Defaults to `false`.
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// List of log types to export to cloudwatch. If omitted, no logs will be exported.
        /// The following log types are supported: `audit`, `profiler`.
        pub enabled_cloudwatch_logs_exports: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// The DNS address of the DocumentDB instance
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// The name of the database engine to be used for this DB cluster. Defaults to `docdb`. Valid values: `docdb`.
        pub engine: pulumi_gestalt_rust::Output<Option<String>>,
        /// The database engine version. Updating this argument results in an outage.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The name of your final DB snapshot
        /// when this DB cluster is deleted. If omitted, no final snapshot will be
        /// made.
        pub final_snapshot_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// The global cluster identifier specified on `aws.docdb.GlobalCluster`.
        pub global_cluster_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Route53 Hosted Zone ID of the endpoint
        pub hosted_zone_id: pulumi_gestalt_rust::Output<String>,
        /// The ARN for the KMS encryption key. When specifying `kms_key_id`, `storage_encrypted` needs to be set to true.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Password for the master DB user. Note that this may
        /// show up in logs, and it will be stored in the state file. Please refer to the DocumentDB Naming Constraints.
        pub master_password: pulumi_gestalt_rust::Output<Option<String>>,
        /// Username for the master DB user.
        pub master_username: pulumi_gestalt_rust::Output<String>,
        /// The port on which the DB accepts connections
        pub port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The daily time range during which automated backups are created if automated backups are enabled using the BackupRetentionPeriod parameter.Time in UTC
        /// Default: A 30-minute window selected at random from an 8-hour block of time per regionE.g., 04:00-09:00
        pub preferred_backup_window: pulumi_gestalt_rust::Output<String>,
        /// The weekly time range during which system maintenance can occur, in (UTC) e.g., wed:04:00-wed:04:30
        pub preferred_maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// A read-only endpoint for the DocumentDB cluster, automatically load-balanced across replicas
        pub reader_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A configuration block for restoring a DB instance to an arbitrary point in time. Requires the `identifier` argument to be set with the name of the new DB instance to be created. See Restore To Point In Time below for details.
        pub restore_to_point_in_time: pulumi_gestalt_rust::Output<
            Option<super::super::types::docdb::ClusterRestoreToPointInTime>,
        >,
        /// Determines whether a final DB snapshot is created before the DB cluster is deleted. If true is specified, no DB snapshot is created. If false is specified, a DB snapshot is created before the DB cluster is deleted, using the value from `final_snapshot_identifier`. Default is `false`.
        pub skip_final_snapshot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether or not to create this cluster from a snapshot. You can use either the name or ARN when specifying a DB cluster snapshot, or the ARN when specifying a DB snapshot. Automated snapshots **should not** be used for this attribute, unless from a different cluster. Automated snapshots are deleted as part of cluster destruction when the resource is replaced.
        pub snapshot_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether the DB cluster is encrypted. The default is `false`.
        pub storage_encrypted: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The storage type to associate with the DB cluster. Valid values: `standard`, `iopt1`.
        pub storage_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to assign to the DB cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// List of VPC security groups to associate
        /// with the Cluster
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_major_version_upgrade_binding = args
            .allow_major_version_upgrade
            .get_output(context);
        let apply_immediately_binding = args.apply_immediately.get_output(context);
        let availability_zones_binding = args.availability_zones.get_output(context);
        let backup_retention_period_binding = args
            .backup_retention_period
            .get_output(context);
        let cluster_identifier_binding = args.cluster_identifier.get_output(context);
        let cluster_identifier_prefix_binding = args
            .cluster_identifier_prefix
            .get_output(context);
        let cluster_members_binding = args.cluster_members.get_output(context);
        let db_cluster_parameter_group_name_binding = args
            .db_cluster_parameter_group_name
            .get_output(context);
        let db_subnet_group_name_binding = args.db_subnet_group_name.get_output(context);
        let deletion_protection_binding = args.deletion_protection.get_output(context);
        let enabled_cloudwatch_logs_exports_binding = args
            .enabled_cloudwatch_logs_exports
            .get_output(context);
        let engine_binding = args.engine.get_output(context);
        let engine_version_binding = args.engine_version.get_output(context);
        let final_snapshot_identifier_binding = args
            .final_snapshot_identifier
            .get_output(context);
        let global_cluster_identifier_binding = args
            .global_cluster_identifier
            .get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let master_password_binding = args.master_password.get_output(context);
        let master_username_binding = args.master_username.get_output(context);
        let port_binding = args.port.get_output(context);
        let preferred_backup_window_binding = args
            .preferred_backup_window
            .get_output(context);
        let preferred_maintenance_window_binding = args
            .preferred_maintenance_window
            .get_output(context);
        let restore_to_point_in_time_binding = args
            .restore_to_point_in_time
            .get_output(context);
        let skip_final_snapshot_binding = args.skip_final_snapshot.get_output(context);
        let snapshot_identifier_binding = args.snapshot_identifier.get_output(context);
        let storage_encrypted_binding = args.storage_encrypted.get_output(context);
        let storage_type_binding = args.storage_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:docdb/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowMajorVersionUpgrade".into(),
                    value: allow_major_version_upgrade_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applyImmediately".into(),
                    value: apply_immediately_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZones".into(),
                    value: availability_zones_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupRetentionPeriod".into(),
                    value: backup_retention_period_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifier".into(),
                    value: cluster_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifierPrefix".into(),
                    value: cluster_identifier_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterMembers".into(),
                    value: cluster_members_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbClusterParameterGroupName".into(),
                    value: db_cluster_parameter_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbSubnetGroupName".into(),
                    value: db_subnet_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtection".into(),
                    value: deletion_protection_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabledCloudwatchLogsExports".into(),
                    value: enabled_cloudwatch_logs_exports_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engine".into(),
                    value: engine_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineVersion".into(),
                    value: engine_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "finalSnapshotIdentifier".into(),
                    value: final_snapshot_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalClusterIdentifier".into(),
                    value: global_cluster_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: kms_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterPassword".into(),
                    value: master_password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterUsername".into(),
                    value: master_username_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredBackupWindow".into(),
                    value: preferred_backup_window_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredMaintenanceWindow".into(),
                    value: preferred_maintenance_window_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restoreToPointInTime".into(),
                    value: restore_to_point_in_time_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipFinalSnapshot".into(),
                    value: skip_final_snapshot_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotIdentifier".into(),
                    value: snapshot_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageEncrypted".into(),
                    value: storage_encrypted_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageType".into(),
                    value: storage_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: vpc_security_group_ids_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            allow_major_version_upgrade: o.get_field("allowMajorVersionUpgrade"),
            apply_immediately: o.get_field("applyImmediately"),
            arn: o.get_field("arn"),
            availability_zones: o.get_field("availabilityZones"),
            backup_retention_period: o.get_field("backupRetentionPeriod"),
            cluster_identifier: o.get_field("clusterIdentifier"),
            cluster_identifier_prefix: o.get_field("clusterIdentifierPrefix"),
            cluster_members: o.get_field("clusterMembers"),
            cluster_resource_id: o.get_field("clusterResourceId"),
            db_cluster_parameter_group_name: o.get_field("dbClusterParameterGroupName"),
            db_subnet_group_name: o.get_field("dbSubnetGroupName"),
            deletion_protection: o.get_field("deletionProtection"),
            enabled_cloudwatch_logs_exports: o.get_field("enabledCloudwatchLogsExports"),
            endpoint: o.get_field("endpoint"),
            engine: o.get_field("engine"),
            engine_version: o.get_field("engineVersion"),
            final_snapshot_identifier: o.get_field("finalSnapshotIdentifier"),
            global_cluster_identifier: o.get_field("globalClusterIdentifier"),
            hosted_zone_id: o.get_field("hostedZoneId"),
            kms_key_id: o.get_field("kmsKeyId"),
            master_password: o.get_field("masterPassword"),
            master_username: o.get_field("masterUsername"),
            port: o.get_field("port"),
            preferred_backup_window: o.get_field("preferredBackupWindow"),
            preferred_maintenance_window: o.get_field("preferredMaintenanceWindow"),
            reader_endpoint: o.get_field("readerEndpoint"),
            restore_to_point_in_time: o.get_field("restoreToPointInTime"),
            skip_final_snapshot: o.get_field("skipFinalSnapshot"),
            snapshot_identifier: o.get_field("snapshotIdentifier"),
            storage_encrypted: o.get_field("storageEncrypted"),
            storage_type: o.get_field("storageType"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
        }
    }
}
