/// Provides an DocumentDB Cluster Resource Instance. A Cluster Instance Resource defines
/// attributes that are specific to a single instance in a [DocumentDB Cluster][1].
///
/// You do not designate a primary and subsequent replicas. Instead, you simply add DocumentDB
/// Instances and DocumentDB manages the replication. You can use the [count][3]
/// meta-parameter to make multiple instances and join them all to the same DocumentDB
/// Cluster, or you may specify different Cluster Instance resources with various
/// `instance_class` sizes.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let clusterInstances = cluster_instance::create(
///         "clusterInstances",
///         ClusterInstanceArgs::builder()
///             .cluster_identifier("${default.id}")
///             .identifier("docdb-cluster-demo-${range.value}")
///             .instance_class("db.r5.large")
///             .build_struct(),
///     );
///     let default = cluster::create(
///         "default",
///         ClusterArgs::builder()
///             .availability_zones(vec!["us-west-2a", "us-west-2b", "us-west-2c",])
///             .cluster_identifier("docdb-cluster-demo")
///             .master_password("barbut8chars")
///             .master_username("foo")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DocumentDB Cluster Instances using the `identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:docdb/clusterInstance:ClusterInstance prod_instance_1 aurora-cluster-instance-1
/// ```
pub mod cluster_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterInstanceArgs {
        /// Specifies whether any database modifications
        /// are applied immediately, or during the next maintenance window. Default is`false`.
        #[builder(into, default)]
        pub apply_immediately: pulumi_wasm_rust::Output<Option<bool>>,
        /// This parameter does not apply to Amazon DocumentDB. Amazon DocumentDB does not perform minor version upgrades regardless of the value set (see [docs](https://docs.aws.amazon.com/documentdb/latest/developerguide/API_DBInstance.html)). Default `true`.
        #[builder(into, default)]
        pub auto_minor_version_upgrade: pulumi_wasm_rust::Output<Option<bool>>,
        /// The EC2 Availability Zone that the DB instance is created in. See [docs](https://docs.aws.amazon.com/documentdb/latest/developerguide/API_CreateDBInstance.html) about the details.
        #[builder(into, default)]
        pub availability_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// The identifier of the certificate authority (CA) certificate for the DB instance.
        #[builder(into, default)]
        pub ca_cert_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// The identifier of the `aws.docdb.Cluster` in which to launch this instance.
        #[builder(into)]
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// Copy all DB instance `tags` to snapshots. Default is `false`.
        #[builder(into, default)]
        pub copy_tags_to_snapshot: pulumi_wasm_rust::Output<Option<bool>>,
        /// A value that indicates whether to enable Performance Insights for the DB Instance. Default `false`. See [docs] (https://docs.aws.amazon.com/documentdb/latest/developerguide/performance-insights.html) about the details.
        #[builder(into, default)]
        pub enable_performance_insights: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the database engine to be used for the DocumentDB instance. Defaults to `docdb`. Valid Values: `docdb`.
        #[builder(into, default)]
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        /// The identifier for the DocumentDB instance, if omitted, the provider will assign a random, unique identifier.
        #[builder(into, default)]
        pub identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique identifier beginning with the specified prefix. Conflicts with `identifier`.
        #[builder(into, default)]
        pub identifier_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The instance class to use. For details on CPU and memory, see [Scaling for DocumentDB Instances](https://docs.aws.amazon.com/documentdb/latest/developerguide/db-cluster-manage-performance.html#db-cluster-manage-scaling-instance).
        /// DocumentDB currently supports the below instance classes.
        /// Please see [AWS Documentation](https://docs.aws.amazon.com/documentdb/latest/developerguide/db-instance-classes.html#db-instance-class-specs) for complete details.
        /// - db.r6g.large
        /// - db.r6g.xlarge
        /// - db.r6g.2xlarge
        /// - db.r6g.4xlarge
        /// - db.r6g.8xlarge
        /// - db.r6g.12xlarge
        /// - db.r6g.16xlarge
        /// - db.r5.large
        /// - db.r5.xlarge
        /// - db.r5.2xlarge
        /// - db.r5.4xlarge
        /// - db.r5.12xlarge
        /// - db.r5.24xlarge
        /// - db.r4.large
        /// - db.r4.xlarge
        /// - db.r4.2xlarge
        /// - db.r4.4xlarge
        /// - db.r4.8xlarge
        /// - db.r4.16xlarge
        /// - db.t4g.medium
        /// - db.t3.medium
        #[builder(into)]
        pub instance_class: pulumi_wasm_rust::Output<String>,
        /// The KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key. If you do not specify a value for PerformanceInsightsKMSKeyId, then Amazon DocumentDB uses your default KMS key.
        #[builder(into, default)]
        pub performance_insights_kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The window to perform maintenance in.
        /// Syntax: "ddd:hh24:mi-ddd:hh24:mi". Eg: "Mon:00:00-Mon:03:00".
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_wasm_rust::Output<Option<String>>,
        /// Default 0. Failover Priority setting on instance level. The reader who has lower tier has higher priority to get promoter to writer.
        #[builder(into, default)]
        pub promotion_tier: pulumi_wasm_rust::Output<Option<i32>>,
        /// A map of tags to assign to the instance. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterInstanceResult {
        /// Specifies whether any database modifications
        /// are applied immediately, or during the next maintenance window. Default is`false`.
        pub apply_immediately: pulumi_wasm_rust::Output<Option<bool>>,
        /// Amazon Resource Name (ARN) of cluster instance
        pub arn: pulumi_wasm_rust::Output<String>,
        /// This parameter does not apply to Amazon DocumentDB. Amazon DocumentDB does not perform minor version upgrades regardless of the value set (see [docs](https://docs.aws.amazon.com/documentdb/latest/developerguide/API_DBInstance.html)). Default `true`.
        pub auto_minor_version_upgrade: pulumi_wasm_rust::Output<Option<bool>>,
        /// The EC2 Availability Zone that the DB instance is created in. See [docs](https://docs.aws.amazon.com/documentdb/latest/developerguide/API_CreateDBInstance.html) about the details.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// The identifier of the certificate authority (CA) certificate for the DB instance.
        pub ca_cert_identifier: pulumi_wasm_rust::Output<String>,
        /// The identifier of the `aws.docdb.Cluster` in which to launch this instance.
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// Copy all DB instance `tags` to snapshots. Default is `false`.
        pub copy_tags_to_snapshot: pulumi_wasm_rust::Output<Option<bool>>,
        /// The DB subnet group to associate with this DB instance.
        pub db_subnet_group_name: pulumi_wasm_rust::Output<String>,
        /// The region-unique, immutable identifier for the DB instance.
        pub dbi_resource_id: pulumi_wasm_rust::Output<String>,
        /// A value that indicates whether to enable Performance Insights for the DB Instance. Default `false`. See [docs] (https://docs.aws.amazon.com/documentdb/latest/developerguide/performance-insights.html) about the details.
        pub enable_performance_insights: pulumi_wasm_rust::Output<Option<bool>>,
        /// The DNS address for this instance. May not be writable
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// The name of the database engine to be used for the DocumentDB instance. Defaults to `docdb`. Valid Values: `docdb`.
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        /// The database engine version
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The identifier for the DocumentDB instance, if omitted, the provider will assign a random, unique identifier.
        pub identifier: pulumi_wasm_rust::Output<String>,
        /// Creates a unique identifier beginning with the specified prefix. Conflicts with `identifier`.
        pub identifier_prefix: pulumi_wasm_rust::Output<String>,
        /// The instance class to use. For details on CPU and memory, see [Scaling for DocumentDB Instances](https://docs.aws.amazon.com/documentdb/latest/developerguide/db-cluster-manage-performance.html#db-cluster-manage-scaling-instance).
        /// DocumentDB currently supports the below instance classes.
        /// Please see [AWS Documentation](https://docs.aws.amazon.com/documentdb/latest/developerguide/db-instance-classes.html#db-instance-class-specs) for complete details.
        /// - db.r6g.large
        /// - db.r6g.xlarge
        /// - db.r6g.2xlarge
        /// - db.r6g.4xlarge
        /// - db.r6g.8xlarge
        /// - db.r6g.12xlarge
        /// - db.r6g.16xlarge
        /// - db.r5.large
        /// - db.r5.xlarge
        /// - db.r5.2xlarge
        /// - db.r5.4xlarge
        /// - db.r5.12xlarge
        /// - db.r5.24xlarge
        /// - db.r4.large
        /// - db.r4.xlarge
        /// - db.r4.2xlarge
        /// - db.r4.4xlarge
        /// - db.r4.8xlarge
        /// - db.r4.16xlarge
        /// - db.t4g.medium
        /// - db.t3.medium
        pub instance_class: pulumi_wasm_rust::Output<String>,
        /// The ARN for the KMS encryption key if one is set to the cluster.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// The KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key. If you do not specify a value for PerformanceInsightsKMSKeyId, then Amazon DocumentDB uses your default KMS key.
        pub performance_insights_kms_key_id: pulumi_wasm_rust::Output<String>,
        /// The database port
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The daily time range during which automated backups are created if automated backups are enabled.
        pub preferred_backup_window: pulumi_wasm_rust::Output<String>,
        /// The window to perform maintenance in.
        /// Syntax: "ddd:hh24:mi-ddd:hh24:mi". Eg: "Mon:00:00-Mon:03:00".
        pub preferred_maintenance_window: pulumi_wasm_rust::Output<String>,
        /// Default 0. Failover Priority setting on instance level. The reader who has lower tier has higher priority to get promoter to writer.
        pub promotion_tier: pulumi_wasm_rust::Output<Option<i32>>,
        pub publicly_accessible: pulumi_wasm_rust::Output<bool>,
        /// Specifies whether the DB cluster is encrypted.
        pub storage_encrypted: pulumi_wasm_rust::Output<bool>,
        /// A map of tags to assign to the instance. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Boolean indicating if this instance is writable. `False` indicates this instance is a read replica.
        pub writer: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ClusterInstanceArgs) -> ClusterInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let apply_immediately_binding = args.apply_immediately.get_inner();
        let auto_minor_version_upgrade_binding = args
            .auto_minor_version_upgrade
            .get_inner();
        let availability_zone_binding = args.availability_zone.get_inner();
        let ca_cert_identifier_binding = args.ca_cert_identifier.get_inner();
        let cluster_identifier_binding = args.cluster_identifier.get_inner();
        let copy_tags_to_snapshot_binding = args.copy_tags_to_snapshot.get_inner();
        let enable_performance_insights_binding = args
            .enable_performance_insights
            .get_inner();
        let engine_binding = args.engine.get_inner();
        let identifier_binding = args.identifier.get_inner();
        let identifier_prefix_binding = args.identifier_prefix.get_inner();
        let instance_class_binding = args.instance_class.get_inner();
        let performance_insights_kms_key_id_binding = args
            .performance_insights_kms_key_id
            .get_inner();
        let preferred_maintenance_window_binding = args
            .preferred_maintenance_window
            .get_inner();
        let promotion_tier_binding = args.promotion_tier.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:docdb/clusterInstance:ClusterInstance".into(),
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
                    name: "enablePerformanceInsights".into(),
                    value: &enable_performance_insights_binding,
                },
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
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
                    name: "performanceInsightsKmsKeyId".into(),
                    value: &performance_insights_kms_key_id_binding,
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applyImmediately".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoMinorVersionUpgrade".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "caCertIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "clusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "copyTagsToSnapshot".into(),
                },
                register_interface::ResultField {
                    name: "dbSubnetGroupName".into(),
                },
                register_interface::ResultField {
                    name: "dbiResourceId".into(),
                },
                register_interface::ResultField {
                    name: "enablePerformanceInsights".into(),
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
                    name: "identifier".into(),
                },
                register_interface::ResultField {
                    name: "identifierPrefix".into(),
                },
                register_interface::ResultField {
                    name: "instanceClass".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "performanceInsightsKmsKeyId".into(),
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
                    name: "promotionTier".into(),
                },
                register_interface::ResultField {
                    name: "publiclyAccessible".into(),
                },
                register_interface::ResultField {
                    name: "storageEncrypted".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "writer".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterInstanceResult {
            apply_immediately: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applyImmediately").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_minor_version_upgrade: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoMinorVersionUpgrade").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            ca_cert_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("caCertIdentifier").unwrap(),
            ),
            cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterIdentifier").unwrap(),
            ),
            copy_tags_to_snapshot: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("copyTagsToSnapshot").unwrap(),
            ),
            db_subnet_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbSubnetGroupName").unwrap(),
            ),
            dbi_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbiResourceId").unwrap(),
            ),
            enable_performance_insights: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enablePerformanceInsights").unwrap(),
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
            identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identifier").unwrap(),
            ),
            identifier_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identifierPrefix").unwrap(),
            ),
            instance_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceClass").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            performance_insights_kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("performanceInsightsKmsKeyId").unwrap(),
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
            promotion_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("promotionTier").unwrap(),
            ),
            publicly_accessible: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publiclyAccessible").unwrap(),
            ),
            storage_encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageEncrypted").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            writer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("writer").unwrap(),
            ),
        }
    }
}
