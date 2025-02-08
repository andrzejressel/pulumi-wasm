/// Provides an Neptune Cluster Resource. A Cluster Resource defines attributes that are
/// applied to the entire cluster of Neptune Cluster Instances.
///
/// Changes to a Neptune Cluster can occur when you manually change a
/// parameter, such as `backup_retention_period`, and are reflected in the next maintenance
/// window. Because of this, this provider may report a difference in its planning
/// phase because a modification has not yet taken place. You can use the
/// `apply_immediately` flag to instruct the service to apply the change immediately
/// (see documentation below).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = cluster::create(
///         "default",
///         ClusterArgs::builder()
///             .apply_immediately(true)
///             .backup_retention_period(5)
///             .cluster_identifier("neptune-cluster-demo")
///             .engine("neptune")
///             .iam_database_authentication_enabled(true)
///             .preferred_backup_window("07:00-09:00")
///             .skip_final_snapshot(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// > **Note:** AWS Neptune does not support user name/password–based access control.
/// See the AWS [Docs](https://docs.aws.amazon.com/neptune/latest/userguide/limits.html) for more information.
///
/// ## Import
///
/// Using `pulumi import`, import `aws_neptune_cluster` using the cluster identifier. For example:
///
/// ```sh
/// $ pulumi import aws:neptune/cluster:Cluster example my-cluster
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// Specifies whether upgrades between different major versions are allowed. You must set it to `true` when providing an `engine_version` parameter that uses a different major version than the DB cluster's current version. Default is `false`.
        #[builder(into, default)]
        pub allow_major_version_upgrade: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies whether any cluster modifications are applied immediately, or during the next maintenance window. Default is `false`.
        #[builder(into, default)]
        pub apply_immediately: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of EC2 Availability Zones that instances in the Neptune cluster can be created in.
        #[builder(into, default)]
        pub availability_zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The days to retain backups for. Default `1`
        #[builder(into, default)]
        pub backup_retention_period: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The cluster identifier. If omitted, this provider will assign a random, unique identifier.
        #[builder(into, default)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique cluster identifier beginning with the specified prefix. Conflicts with `cluster_identifier`.
        #[builder(into, default)]
        pub cluster_identifier_prefix: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// If set to true, tags are copied to any snapshot of the DB cluster that is created.
        #[builder(into, default)]
        pub copy_tags_to_snapshot: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A value that indicates whether the DB cluster has deletion protection enabled.The database can't be deleted when deletion protection is enabled. By default, deletion protection is disabled.
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of the log types this DB cluster is configured to export to Cloudwatch Logs. Currently only supports `audit` and `slowquery`.
        #[builder(into, default)]
        pub enable_cloudwatch_logs_exports: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The name of the database engine to be used for this Neptune cluster. Defaults to `neptune`.
        #[builder(into, default)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The database engine version.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of your final Neptune snapshot when this Neptune cluster is deleted. If omitted, no final snapshot will be made.
        #[builder(into, default)]
        pub final_snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The global cluster identifier specified on `aws.neptune.GlobalCluster`.
        #[builder(into, default)]
        pub global_cluster_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies whether or not mappings of AWS Identity and Access Management (IAM) accounts to database accounts is enabled.
        #[builder(into, default)]
        pub iam_database_authentication_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A List of ARNs for the IAM roles to associate to the Neptune Cluster.
        #[builder(into, default)]
        pub iam_roles: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ARN for the KMS encryption key. When specifying `kms_key_arn`, `storage_encrypted` needs to be set to true.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A cluster parameter group to associate with the cluster.
        #[builder(into, default)]
        pub neptune_cluster_parameter_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the DB parameter group to apply to all instances of the DB cluster.
        #[builder(into, default)]
        pub neptune_instance_parameter_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A Neptune subnet group to associate with this Neptune instance.
        #[builder(into, default)]
        pub neptune_subnet_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The port on which the Neptune accepts connections. Default is `8182`.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The daily time range during which automated backups are created if automated backups are enabled using the BackupRetentionPeriod parameter. Time in UTC. Default: A 30-minute window selected at random from an 8-hour block of time per regionE.g., 04:00-09:00
        #[builder(into, default)]
        pub preferred_backup_window: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The weekly time range during which system maintenance can occur, in (UTC) e.g., wed:04:00-wed:04:30
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// ARN of a source Neptune cluster or Neptune instance if this Neptune cluster is to be created as a Read Replica.
        #[builder(into, default)]
        pub replication_source_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// If set, create the Neptune cluster as a serverless one. See Serverless for example block attributes.
        #[builder(into, default)]
        pub serverless_v2_scaling_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::neptune::ClusterServerlessV2ScalingConfiguration>,
        >,
        /// Determines whether a final Neptune snapshot is created before the Neptune cluster is deleted. If true is specified, no Neptune snapshot is created. If false is specified, a Neptune snapshot is created before the Neptune cluster is deleted, using the value from `final_snapshot_identifier`. Default is `false`.
        #[builder(into, default)]
        pub skip_final_snapshot: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether or not to create this cluster from a snapshot. You can use either the name or ARN when specifying a Neptune cluster snapshot, or the ARN when specifying a Neptune snapshot. Automated snapshots **should not** be used for this attribute, unless from a different cluster. Automated snapshots are deleted as part of cluster destruction when the resource is replaced.
        #[builder(into, default)]
        pub snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the Neptune cluster is encrypted. The default is `false` if not specified.
        #[builder(into, default)]
        pub storage_encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Storage type associated with the cluster `standard/iopt1`. Default: `standard`
        #[builder(into, default)]
        pub storage_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the Neptune cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of VPC security groups to associate with the Cluster
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// Specifies whether upgrades between different major versions are allowed. You must set it to `true` when providing an `engine_version` parameter that uses a different major version than the DB cluster's current version. Default is `false`.
        pub allow_major_version_upgrade: pulumi_gestalt_rust::Output<bool>,
        /// Specifies whether any cluster modifications are applied immediately, or during the next maintenance window. Default is `false`.
        pub apply_immediately: pulumi_gestalt_rust::Output<bool>,
        /// The Neptune Cluster Amazon Resource Name (ARN)
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A list of EC2 Availability Zones that instances in the Neptune cluster can be created in.
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The days to retain backups for. Default `1`
        pub backup_retention_period: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The cluster identifier. If omitted, this provider will assign a random, unique identifier.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique cluster identifier beginning with the specified prefix. Conflicts with `cluster_identifier`.
        pub cluster_identifier_prefix: pulumi_gestalt_rust::Output<String>,
        /// List of Neptune Instances that are a part of this cluster
        pub cluster_members: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Neptune Cluster Resource ID
        pub cluster_resource_id: pulumi_gestalt_rust::Output<String>,
        /// If set to true, tags are copied to any snapshot of the DB cluster that is created.
        pub copy_tags_to_snapshot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A value that indicates whether the DB cluster has deletion protection enabled.The database can't be deleted when deletion protection is enabled. By default, deletion protection is disabled.
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of the log types this DB cluster is configured to export to Cloudwatch Logs. Currently only supports `audit` and `slowquery`.
        pub enable_cloudwatch_logs_exports: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// The DNS address of the Neptune instance
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// The name of the database engine to be used for this Neptune cluster. Defaults to `neptune`.
        pub engine: pulumi_gestalt_rust::Output<Option<String>>,
        /// The database engine version.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The name of your final Neptune snapshot when this Neptune cluster is deleted. If omitted, no final snapshot will be made.
        pub final_snapshot_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// The global cluster identifier specified on `aws.neptune.GlobalCluster`.
        pub global_cluster_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Route53 Hosted Zone ID of the endpoint
        pub hosted_zone_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether or not mappings of AWS Identity and Access Management (IAM) accounts to database accounts is enabled.
        pub iam_database_authentication_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// A List of ARNs for the IAM roles to associate to the Neptune Cluster.
        pub iam_roles: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ARN for the KMS encryption key. When specifying `kms_key_arn`, `storage_encrypted` needs to be set to true.
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
        /// A cluster parameter group to associate with the cluster.
        pub neptune_cluster_parameter_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the DB parameter group to apply to all instances of the DB cluster.
        pub neptune_instance_parameter_group_name: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// A Neptune subnet group to associate with this Neptune instance.
        pub neptune_subnet_group_name: pulumi_gestalt_rust::Output<String>,
        /// The port on which the Neptune accepts connections. Default is `8182`.
        pub port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The daily time range during which automated backups are created if automated backups are enabled using the BackupRetentionPeriod parameter. Time in UTC. Default: A 30-minute window selected at random from an 8-hour block of time per regionE.g., 04:00-09:00
        pub preferred_backup_window: pulumi_gestalt_rust::Output<String>,
        /// The weekly time range during which system maintenance can occur, in (UTC) e.g., wed:04:00-wed:04:30
        pub preferred_maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// A read-only endpoint for the Neptune cluster, automatically load-balanced across replicas
        pub reader_endpoint: pulumi_gestalt_rust::Output<String>,
        /// ARN of a source Neptune cluster or Neptune instance if this Neptune cluster is to be created as a Read Replica.
        pub replication_source_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// If set, create the Neptune cluster as a serverless one. See Serverless for example block attributes.
        pub serverless_v2_scaling_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::neptune::ClusterServerlessV2ScalingConfiguration>,
        >,
        /// Determines whether a final Neptune snapshot is created before the Neptune cluster is deleted. If true is specified, no Neptune snapshot is created. If false is specified, a Neptune snapshot is created before the Neptune cluster is deleted, using the value from `final_snapshot_identifier`. Default is `false`.
        pub skip_final_snapshot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether or not to create this cluster from a snapshot. You can use either the name or ARN when specifying a Neptune cluster snapshot, or the ARN when specifying a Neptune snapshot. Automated snapshots **should not** be used for this attribute, unless from a different cluster. Automated snapshots are deleted as part of cluster destruction when the resource is replaced.
        pub snapshot_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether the Neptune cluster is encrypted. The default is `false` if not specified.
        pub storage_encrypted: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Storage type associated with the cluster `standard/iopt1`. Default: `standard`
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the Neptune cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// List of VPC security groups to associate with the Cluster
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        let copy_tags_to_snapshot_binding = args
            .copy_tags_to_snapshot
            .get_output(context)
            .get_inner();
        let deletion_protection_binding = args
            .deletion_protection
            .get_output(context)
            .get_inner();
        let enable_cloudwatch_logs_exports_binding = args
            .enable_cloudwatch_logs_exports
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
        let iam_database_authentication_enabled_binding = args
            .iam_database_authentication_enabled
            .get_output(context)
            .get_inner();
        let iam_roles_binding = args.iam_roles.get_output(context).get_inner();
        let kms_key_arn_binding = args.kms_key_arn.get_output(context).get_inner();
        let neptune_cluster_parameter_group_name_binding = args
            .neptune_cluster_parameter_group_name
            .get_output(context)
            .get_inner();
        let neptune_instance_parameter_group_name_binding = args
            .neptune_instance_parameter_group_name
            .get_output(context)
            .get_inner();
        let neptune_subnet_group_name_binding = args
            .neptune_subnet_group_name
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
        let replication_source_identifier_binding = args
            .replication_source_identifier
            .get_output(context)
            .get_inner();
        let serverless_v2_scaling_configuration_binding = args
            .serverless_v2_scaling_configuration
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
            type_: "aws:neptune/cluster:Cluster".into(),
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
                    name: "copyTagsToSnapshot".into(),
                    value: &copy_tags_to_snapshot_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "enableCloudwatchLogsExports".into(),
                    value: &enable_cloudwatch_logs_exports_binding,
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
                    name: "iamDatabaseAuthenticationEnabled".into(),
                    value: &iam_database_authentication_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "iamRoles".into(),
                    value: &iam_roles_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyArn".into(),
                    value: &kms_key_arn_binding,
                },
                register_interface::ObjectField {
                    name: "neptuneClusterParameterGroupName".into(),
                    value: &neptune_cluster_parameter_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "neptuneInstanceParameterGroupName".into(),
                    value: &neptune_instance_parameter_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "neptuneSubnetGroupName".into(),
                    value: &neptune_subnet_group_name_binding,
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
                    name: "replicationSourceIdentifier".into(),
                    value: &replication_source_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "serverlessV2ScalingConfiguration".into(),
                    value: &serverless_v2_scaling_configuration_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterResult {
            allow_major_version_upgrade: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowMajorVersionUpgrade"),
            ),
            apply_immediately: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applyImmediately"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            availability_zones: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZones"),
            ),
            backup_retention_period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupRetentionPeriod"),
            ),
            cluster_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterIdentifier"),
            ),
            cluster_identifier_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterIdentifierPrefix"),
            ),
            cluster_members: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterMembers"),
            ),
            cluster_resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterResourceId"),
            ),
            copy_tags_to_snapshot: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("copyTagsToSnapshot"),
            ),
            deletion_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            enable_cloudwatch_logs_exports: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableCloudwatchLogsExports"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            engine: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engine"),
            ),
            engine_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineVersion"),
            ),
            final_snapshot_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("finalSnapshotIdentifier"),
            ),
            global_cluster_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("globalClusterIdentifier"),
            ),
            hosted_zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostedZoneId"),
            ),
            iam_database_authentication_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iamDatabaseAuthenticationEnabled"),
            ),
            iam_roles: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iamRoles"),
            ),
            kms_key_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyArn"),
            ),
            neptune_cluster_parameter_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("neptuneClusterParameterGroupName"),
            ),
            neptune_instance_parameter_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("neptuneInstanceParameterGroupName"),
            ),
            neptune_subnet_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("neptuneSubnetGroupName"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            preferred_backup_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preferredBackupWindow"),
            ),
            preferred_maintenance_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preferredMaintenanceWindow"),
            ),
            reader_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readerEndpoint"),
            ),
            replication_source_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicationSourceIdentifier"),
            ),
            serverless_v2_scaling_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverlessV2ScalingConfiguration"),
            ),
            skip_final_snapshot: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skipFinalSnapshot"),
            ),
            snapshot_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotIdentifier"),
            ),
            storage_encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageEncrypted"),
            ),
            storage_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageType"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_security_group_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcSecurityGroupIds"),
            ),
        }
    }
}
