/// Provides an RDS Cluster Instance Resource. A Cluster Instance Resource defines
/// attributes that are specific to a single instance in a RDS Cluster,
/// specifically running Amazon Aurora.
///
/// Unlike other RDS resources that support replication, with Amazon Aurora you do
/// not designate a primary and subsequent replicas. Instead, you simply add RDS
/// Instances and Aurora manages the replication. You can use the [count][5]
/// meta-parameter to make multiple instances and join them all to the same RDS
/// Cluster, or you may specify different Cluster Instance resources with various
/// `instance_class` sizes.
///
/// For more information on Amazon Aurora, see [Aurora on Amazon RDS](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Aurora.html) in the Amazon RDS User Guide.
///
/// > **NOTE:** Deletion Protection from the RDS service can only be enabled at the cluster level, not for individual cluster instances. You can still add the [`protect` CustomResourceOption](https://www.pulumi.com/docs/intro/concepts/programming-model/#protect) to this resource configuration if you desire protection from accidental deletion.
///
/// > **NOTE:** `aurora` is no longer a valid `engine` because of [Amazon Aurora's MySQL-Compatible Edition version 1 end of life](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/Aurora.MySQL56.EOL.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let clusterInstances = cluster_instance::create(
///         "clusterInstances",
///         ClusterInstanceArgs::builder()
///             .cluster_identifier("${default.id}")
///             .engine("${default.engine}")
///             .engine_version("${default.engineVersion}")
///             .identifier("aurora-cluster-demo-${range.value}")
///             .instance_class("db.r4.large")
///             .build_struct(),
///     );
///     let default = cluster::create(
///         "default",
///         ClusterArgs::builder()
///             .availability_zones(vec!["us-west-2a", "us-west-2b", "us-west-2c",])
///             .cluster_identifier("aurora-cluster-demo")
///             .database_name("mydb")
///             .master_password("barbut8chars")
///             .master_username("foo")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import RDS Cluster Instances using the `identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/clusterInstance:ClusterInstance prod_instance_1 aurora-cluster-instance-1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterInstanceArgs {
        /// Specifies whether any database modifications are applied immediately, or during the next maintenance window. Default is`false`.
        #[builder(into, default)]
        pub apply_immediately: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Indicates that minor engine upgrades will be applied automatically to the DB instance during the maintenance window. Default `true`.
        #[builder(into, default)]
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// EC2 Availability Zone that the DB instance is created in. See [docs](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_CreateDBInstance.html) about the details.
        #[builder(into, default)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the CA certificate for the DB instance.
        #[builder(into, default)]
        pub ca_cert_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the `aws.rds.Cluster` in which to launch this instance.
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicates whether to copy all of the user-defined tags from the DB instance to snapshots of the DB instance. Default `false`.
        #[builder(into, default)]
        pub copy_tags_to_snapshot: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Instance profile associated with the underlying Amazon EC2 instance of an RDS Custom DB instance.
        #[builder(into, default)]
        pub custom_iam_instance_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Name of the DB parameter group to associate with this instance.
        #[builder(into, default)]
        pub db_parameter_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the DB subnet group to associate with this DB instance. The default behavior varies depending on whether `db_subnet_group_name` is specified. Please refer to official [AWS documentation](https://docs.aws.amazon.com/cli/latest/reference/rds/create-db-instance.html) to understand how `db_subnet_group_name` and `publicly_accessible` parameters affect DB instance behaviour. **NOTE:** This must match the `db_subnet_group_name` of the attached `aws.rds.Cluster`.
        #[builder(into, default)]
        pub db_subnet_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the database engine to be used for the RDS cluster instance.
        /// Valid Values: `aurora-mysql`, `aurora-postgresql`, `mysql`, `postgres`.(Note that `mysql` and `postgres` are Multi-AZ RDS clusters).
        #[builder(into)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Database engine version. Please note that to upgrade the `engine_version` of the instance, it must be done on the `aws.rds.Cluster` `engine_version`. Trying to upgrade in `aws_cluster_instance` will not update the `engine_version`.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Forces an instance to be destroyed when a part of a read replica cluster. **Note:** will promote the read replica to a standalone cluster before instance deletion.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Identifier for the RDS instance, if omitted, Pulumi will assign a random, unique identifier.
        #[builder(into, default)]
        pub identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique identifier beginning with the specified prefix. Conflicts with `identifier`.
        #[builder(into, default)]
        pub identifier_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Instance class to use. For details on CPU and memory, see [Scaling Aurora DB Instances](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Aurora.Managing.html). Aurora uses `db.*` instance classes/types. Please see [AWS Documentation](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Concepts.DBInstanceClass.html) for currently available instance classes and complete details. For Aurora Serverless v2 use `db.serverless`.
        #[builder(into)]
        pub instance_class: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB instance. To disable collecting Enhanced Monitoring metrics, specify 0. The default is 0. Valid Values: 0, 1, 5, 10, 15, 30, 60.
        #[builder(into, default)]
        pub monitoring_interval: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// ARN for the IAM role that permits RDS to send enhanced monitoring metrics to CloudWatch Logs. You can find more information on the [AWS Documentation](http://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Monitoring.html) what IAM permissions are needed to allow Enhanced Monitoring for RDS Instances.
        #[builder(into, default)]
        pub monitoring_role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether Performance Insights is enabled or not.
        #[builder(into, default)]
        pub performance_insights_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// ARN for the KMS key to encrypt Performance Insights data. When specifying `performance_insights_kms_key_id`, `performance_insights_enabled` needs to be set to true.
        #[builder(into, default)]
        pub performance_insights_kms_key_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Amount of time in days to retain Performance Insights data. Valid values are `7`, `731` (2 years) or a multiple of `31`. When specifying `performance_insights_retention_period`, `performance_insights_enabled` needs to be set to true. Defaults to '7'.
        #[builder(into, default)]
        pub performance_insights_retention_period: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Daily time range during which automated backups are created if automated backups are enabled. Eg: "04:00-09:00". **NOTE:** If `preferred_backup_window` is set at the cluster level, this argument **must** be omitted.
        #[builder(into, default)]
        pub preferred_backup_window: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Window to perform maintenance in. Syntax: "ddd:hh24:mi-ddd:hh24:mi". Eg: "Mon:00:00-Mon:03:00".
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Default 0. Failover Priority setting on instance level. The reader who has lower tier has higher priority to get promoted to writer.
        #[builder(into, default)]
        pub promotion_tier: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Bool to control if instance is publicly accessible. Default `false`. See the documentation on [Creating DB Instances](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_CreateDBInstance.html) for more details on controlling this property.
        #[builder(into, default)]
        pub publicly_accessible: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Map of tags to assign to the instance. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterInstanceResult {
        /// Specifies whether any database modifications are applied immediately, or during the next maintenance window. Default is`false`.
        pub apply_immediately: pulumi_gestalt_rust::Output<bool>,
        /// Amazon Resource Name (ARN) of cluster instance
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Indicates that minor engine upgrades will be applied automatically to the DB instance during the maintenance window. Default `true`.
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::Output<Option<bool>>,
        /// EC2 Availability Zone that the DB instance is created in. See [docs](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_CreateDBInstance.html) about the details.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the CA certificate for the DB instance.
        pub ca_cert_identifier: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the `aws.rds.Cluster` in which to launch this instance.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether to copy all of the user-defined tags from the DB instance to snapshots of the DB instance. Default `false`.
        pub copy_tags_to_snapshot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Instance profile associated with the underlying Amazon EC2 instance of an RDS Custom DB instance.
        pub custom_iam_instance_profile: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the DB parameter group to associate with this instance.
        pub db_parameter_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the DB subnet group to associate with this DB instance. The default behavior varies depending on whether `db_subnet_group_name` is specified. Please refer to official [AWS documentation](https://docs.aws.amazon.com/cli/latest/reference/rds/create-db-instance.html) to understand how `db_subnet_group_name` and `publicly_accessible` parameters affect DB instance behaviour. **NOTE:** This must match the `db_subnet_group_name` of the attached `aws.rds.Cluster`.
        pub db_subnet_group_name: pulumi_gestalt_rust::Output<String>,
        /// Region-unique, immutable identifier for the DB instance.
        pub dbi_resource_id: pulumi_gestalt_rust::Output<String>,
        /// DNS address for this instance. May not be writable
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Name of the database engine to be used for the RDS cluster instance.
        /// Valid Values: `aurora-mysql`, `aurora-postgresql`, `mysql`, `postgres`.(Note that `mysql` and `postgres` are Multi-AZ RDS clusters).
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Database engine version. Please note that to upgrade the `engine_version` of the instance, it must be done on the `aws.rds.Cluster` `engine_version`. Trying to upgrade in `aws_cluster_instance` will not update the `engine_version`.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// Database engine version
        pub engine_version_actual: pulumi_gestalt_rust::Output<String>,
        /// Forces an instance to be destroyed when a part of a read replica cluster. **Note:** will promote the read replica to a standalone cluster before instance deletion.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Identifier for the RDS instance, if omitted, Pulumi will assign a random, unique identifier.
        pub identifier: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique identifier beginning with the specified prefix. Conflicts with `identifier`.
        pub identifier_prefix: pulumi_gestalt_rust::Output<String>,
        /// Instance class to use. For details on CPU and memory, see [Scaling Aurora DB Instances](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Aurora.Managing.html). Aurora uses `db.*` instance classes/types. Please see [AWS Documentation](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Concepts.DBInstanceClass.html) for currently available instance classes and complete details. For Aurora Serverless v2 use `db.serverless`.
        pub instance_class: pulumi_gestalt_rust::Output<String>,
        /// ARN for the KMS encryption key if one is set to the cluster.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB instance. To disable collecting Enhanced Monitoring metrics, specify 0. The default is 0. Valid Values: 0, 1, 5, 10, 15, 30, 60.
        pub monitoring_interval: pulumi_gestalt_rust::Output<Option<i32>>,
        /// ARN for the IAM role that permits RDS to send enhanced monitoring metrics to CloudWatch Logs. You can find more information on the [AWS Documentation](http://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Monitoring.html) what IAM permissions are needed to allow Enhanced Monitoring for RDS Instances.
        pub monitoring_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Network type of the DB instance.
        pub network_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether Performance Insights is enabled or not.
        pub performance_insights_enabled: pulumi_gestalt_rust::Output<bool>,
        /// ARN for the KMS key to encrypt Performance Insights data. When specifying `performance_insights_kms_key_id`, `performance_insights_enabled` needs to be set to true.
        pub performance_insights_kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Amount of time in days to retain Performance Insights data. Valid values are `7`, `731` (2 years) or a multiple of `31`. When specifying `performance_insights_retention_period`, `performance_insights_enabled` needs to be set to true. Defaults to '7'.
        pub performance_insights_retention_period: pulumi_gestalt_rust::Output<i32>,
        /// Database port
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// Daily time range during which automated backups are created if automated backups are enabled. Eg: "04:00-09:00". **NOTE:** If `preferred_backup_window` is set at the cluster level, this argument **must** be omitted.
        pub preferred_backup_window: pulumi_gestalt_rust::Output<String>,
        /// Window to perform maintenance in. Syntax: "ddd:hh24:mi-ddd:hh24:mi". Eg: "Mon:00:00-Mon:03:00".
        pub preferred_maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// Default 0. Failover Priority setting on instance level. The reader who has lower tier has higher priority to get promoted to writer.
        pub promotion_tier: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Bool to control if instance is publicly accessible. Default `false`. See the documentation on [Creating DB Instances](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_CreateDBInstance.html) for more details on controlling this property.
        pub publicly_accessible: pulumi_gestalt_rust::Output<bool>,
        /// Specifies whether the DB cluster is encrypted.
        pub storage_encrypted: pulumi_gestalt_rust::Output<bool>,
        /// Map of tags to assign to the instance. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Boolean indicating if this instance is writable. `False` indicates this instance is a read replica.
        pub writer: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ClusterInstanceArgs,
    ) -> ClusterInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let apply_immediately_binding_1 = args.apply_immediately.get_output(context);
        let apply_immediately_binding = apply_immediately_binding_1.get_inner();
        let auto_minor_version_upgrade_binding_1 = args
            .auto_minor_version_upgrade
            .get_output(context);
        let auto_minor_version_upgrade_binding = auto_minor_version_upgrade_binding_1
            .get_inner();
        let availability_zone_binding_1 = args.availability_zone.get_output(context);
        let availability_zone_binding = availability_zone_binding_1.get_inner();
        let ca_cert_identifier_binding_1 = args.ca_cert_identifier.get_output(context);
        let ca_cert_identifier_binding = ca_cert_identifier_binding_1.get_inner();
        let cluster_identifier_binding_1 = args.cluster_identifier.get_output(context);
        let cluster_identifier_binding = cluster_identifier_binding_1.get_inner();
        let copy_tags_to_snapshot_binding_1 = args
            .copy_tags_to_snapshot
            .get_output(context);
        let copy_tags_to_snapshot_binding = copy_tags_to_snapshot_binding_1.get_inner();
        let custom_iam_instance_profile_binding_1 = args
            .custom_iam_instance_profile
            .get_output(context);
        let custom_iam_instance_profile_binding = custom_iam_instance_profile_binding_1
            .get_inner();
        let db_parameter_group_name_binding_1 = args
            .db_parameter_group_name
            .get_output(context);
        let db_parameter_group_name_binding = db_parameter_group_name_binding_1
            .get_inner();
        let db_subnet_group_name_binding_1 = args
            .db_subnet_group_name
            .get_output(context);
        let db_subnet_group_name_binding = db_subnet_group_name_binding_1.get_inner();
        let engine_binding_1 = args.engine.get_output(context);
        let engine_binding = engine_binding_1.get_inner();
        let engine_version_binding_1 = args.engine_version.get_output(context);
        let engine_version_binding = engine_version_binding_1.get_inner();
        let force_destroy_binding_1 = args.force_destroy.get_output(context);
        let force_destroy_binding = force_destroy_binding_1.get_inner();
        let identifier_binding_1 = args.identifier.get_output(context);
        let identifier_binding = identifier_binding_1.get_inner();
        let identifier_prefix_binding_1 = args.identifier_prefix.get_output(context);
        let identifier_prefix_binding = identifier_prefix_binding_1.get_inner();
        let instance_class_binding_1 = args.instance_class.get_output(context);
        let instance_class_binding = instance_class_binding_1.get_inner();
        let monitoring_interval_binding_1 = args.monitoring_interval.get_output(context);
        let monitoring_interval_binding = monitoring_interval_binding_1.get_inner();
        let monitoring_role_arn_binding_1 = args.monitoring_role_arn.get_output(context);
        let monitoring_role_arn_binding = monitoring_role_arn_binding_1.get_inner();
        let performance_insights_enabled_binding_1 = args
            .performance_insights_enabled
            .get_output(context);
        let performance_insights_enabled_binding = performance_insights_enabled_binding_1
            .get_inner();
        let performance_insights_kms_key_id_binding_1 = args
            .performance_insights_kms_key_id
            .get_output(context);
        let performance_insights_kms_key_id_binding = performance_insights_kms_key_id_binding_1
            .get_inner();
        let performance_insights_retention_period_binding_1 = args
            .performance_insights_retention_period
            .get_output(context);
        let performance_insights_retention_period_binding = performance_insights_retention_period_binding_1
            .get_inner();
        let preferred_backup_window_binding_1 = args
            .preferred_backup_window
            .get_output(context);
        let preferred_backup_window_binding = preferred_backup_window_binding_1
            .get_inner();
        let preferred_maintenance_window_binding_1 = args
            .preferred_maintenance_window
            .get_output(context);
        let preferred_maintenance_window_binding = preferred_maintenance_window_binding_1
            .get_inner();
        let promotion_tier_binding_1 = args.promotion_tier.get_output(context);
        let promotion_tier_binding = promotion_tier_binding_1.get_inner();
        let publicly_accessible_binding_1 = args.publicly_accessible.get_output(context);
        let publicly_accessible_binding = publicly_accessible_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/clusterInstance:ClusterInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applyImmediately".into(),
                    value: &apply_immediately_binding,
                },
                register_interface::ObjectField {
                    name: "autoMinorVersionUpgrade".into(),
                    value: &auto_minor_version_upgrade_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "caCertIdentifier".into(),
                    value: &ca_cert_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "copyTagsToSnapshot".into(),
                    value: &copy_tags_to_snapshot_binding,
                },
                register_interface::ObjectField {
                    name: "customIamInstanceProfile".into(),
                    value: &custom_iam_instance_profile_binding,
                },
                register_interface::ObjectField {
                    name: "dbParameterGroupName".into(),
                    value: &db_parameter_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "dbSubnetGroupName".into(),
                    value: &db_subnet_group_name_binding,
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
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding,
                },
                register_interface::ObjectField {
                    name: "identifierPrefix".into(),
                    value: &identifier_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "instanceClass".into(),
                    value: &instance_class_binding,
                },
                register_interface::ObjectField {
                    name: "monitoringInterval".into(),
                    value: &monitoring_interval_binding,
                },
                register_interface::ObjectField {
                    name: "monitoringRoleArn".into(),
                    value: &monitoring_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "performanceInsightsEnabled".into(),
                    value: &performance_insights_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "performanceInsightsKmsKeyId".into(),
                    value: &performance_insights_kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "performanceInsightsRetentionPeriod".into(),
                    value: &performance_insights_retention_period_binding,
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
                    name: "promotionTier".into(),
                    value: &promotion_tier_binding,
                },
                register_interface::ObjectField {
                    name: "publiclyAccessible".into(),
                    value: &publicly_accessible_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterInstanceResult {
            apply_immediately: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applyImmediately"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auto_minor_version_upgrade: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoMinorVersionUpgrade"),
            ),
            availability_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            ca_cert_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("caCertIdentifier"),
            ),
            cluster_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterIdentifier"),
            ),
            copy_tags_to_snapshot: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("copyTagsToSnapshot"),
            ),
            custom_iam_instance_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customIamInstanceProfile"),
            ),
            db_parameter_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbParameterGroupName"),
            ),
            db_subnet_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbSubnetGroupName"),
            ),
            dbi_resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbiResourceId"),
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
            engine_version_actual: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineVersionActual"),
            ),
            force_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceDestroy"),
            ),
            identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identifier"),
            ),
            identifier_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identifierPrefix"),
            ),
            instance_class: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceClass"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            monitoring_interval: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitoringInterval"),
            ),
            monitoring_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitoringRoleArn"),
            ),
            network_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkType"),
            ),
            performance_insights_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("performanceInsightsEnabled"),
            ),
            performance_insights_kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("performanceInsightsKmsKeyId"),
            ),
            performance_insights_retention_period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("performanceInsightsRetentionPeriod"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            preferred_backup_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preferredBackupWindow"),
            ),
            preferred_maintenance_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preferredMaintenanceWindow"),
            ),
            promotion_tier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("promotionTier"),
            ),
            publicly_accessible: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publiclyAccessible"),
            ),
            storage_encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageEncrypted"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            writer: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("writer"),
            ),
        }
    }
}
