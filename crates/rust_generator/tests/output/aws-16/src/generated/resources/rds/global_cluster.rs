/// Manages an RDS Global Cluster, which is an Aurora global database spread across multiple regions. The global database contains a single primary cluster with read-write capability, and a read-only secondary cluster that receives data from the primary cluster through high-speed replication performed by the Aurora storage subsystem.
///
/// More information about Aurora global databases can be found in the [Aurora User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-global-database.html#aurora-global-database-creating).
///
/// ## Example Usage
///
/// ### New MySQL Global Cluster
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = global_cluster::create(
///         "example",
///         GlobalClusterArgs::builder()
///             .database_name("example_db")
///             .engine("aurora")
///             .engine_version("5.6.mysql_aurora.1.22.2")
///             .global_cluster_identifier("global-test")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .cluster_identifier("test-primary-cluster")
///             .database_name("example_db")
///             .db_subnet_group_name("default")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .global_cluster_identifier("${example.id}")
///             .master_password("somepass123")
///             .master_username("username")
///             .build_struct(),
///     );
///     let primaryClusterInstance = cluster_instance::create(
///         "primaryClusterInstance",
///         ClusterInstanceArgs::builder()
///             .cluster_identifier("${primary.id}")
///             .db_subnet_group_name("default")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .identifier("test-primary-cluster-instance")
///             .instance_class("db.r4.large")
///             .build_struct(),
///     );
///     let secondary = cluster::create(
///         "secondary",
///         ClusterArgs::builder()
///             .cluster_identifier("test-secondary-cluster")
///             .db_subnet_group_name("default")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .global_cluster_identifier("${example.id}")
///             .build_struct(),
///     );
///     let secondaryClusterInstance = cluster_instance::create(
///         "secondaryClusterInstance",
///         ClusterInstanceArgs::builder()
///             .cluster_identifier("${secondary.id}")
///             .db_subnet_group_name("default")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .identifier("test-secondary-cluster-instance")
///             .instance_class("db.r4.large")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### New PostgreSQL Global Cluster
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = global_cluster::create(
///         "example",
///         GlobalClusterArgs::builder()
///             .database_name("example_db")
///             .engine("aurora-postgresql")
///             .engine_version("11.9")
///             .global_cluster_identifier("global-test")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .cluster_identifier("test-primary-cluster")
///             .database_name("example_db")
///             .db_subnet_group_name("default")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .global_cluster_identifier("${example.id}")
///             .master_password("somepass123")
///             .master_username("username")
///             .build_struct(),
///     );
///     let primaryClusterInstance = cluster_instance::create(
///         "primaryClusterInstance",
///         ClusterInstanceArgs::builder()
///             .cluster_identifier("${primary.id}")
///             .db_subnet_group_name("default")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .identifier("test-primary-cluster-instance")
///             .instance_class("db.r4.large")
///             .build_struct(),
///     );
///     let secondary = cluster::create(
///         "secondary",
///         ClusterArgs::builder()
///             .cluster_identifier("test-secondary-cluster")
///             .db_subnet_group_name("default")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .global_cluster_identifier("${example.id}")
///             .skip_final_snapshot(true)
///             .build_struct(),
///     );
///     let secondaryClusterInstance = cluster_instance::create(
///         "secondaryClusterInstance",
///         ClusterInstanceArgs::builder()
///             .cluster_identifier("${secondary.id}")
///             .db_subnet_group_name("default")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .identifier("test-secondary-cluster-instance")
///             .instance_class("db.r4.large")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### New Global Cluster From Existing DB Cluster
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create("example", ClusterArgs::builder().build_struct());
///     let exampleGlobalCluster = global_cluster::create(
///         "exampleGlobalCluster",
///         GlobalClusterArgs::builder()
///             .force_destroy(true)
///             .global_cluster_identifier("example")
///             .source_db_cluster_identifier("${example.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Upgrading Engine Versions
///
/// When you upgrade the version of an `aws.rds.GlobalCluster`, the provider will attempt to in-place upgrade the engine versions of all associated clusters. Since the `aws.rds.Cluster` resource is being updated through the `aws.rds.GlobalCluster`, you are likely to get an error (`Provider produced inconsistent final plan`). To avoid this, use the `lifecycle` `ignore_changes` meta argument as shown below on the `aws.rds.Cluster`.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = global_cluster::create(
///         "example",
///         GlobalClusterArgs::builder()
///             .engine("aurora-mysql")
///             .engine_version("5.7.mysql_aurora.2.07.5")
///             .global_cluster_identifier("kyivkharkiv")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .allow_major_version_upgrade(true)
///             .apply_immediately(true)
///             .cluster_identifier("odessadnipro")
///             .database_name("totoro")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .global_cluster_identifier("${example.id}")
///             .master_password("satsukimae")
///             .master_username("maesatsuki")
///             .skip_final_snapshot(true)
///             .build_struct(),
///     );
///     let primaryClusterInstance = cluster_instance::create(
///         "primaryClusterInstance",
///         ClusterInstanceArgs::builder()
///             .apply_immediately(true)
///             .cluster_identifier("${primary.id}")
///             .engine("${primary.engine}")
///             .engine_version("${primary.engineVersion}")
///             .identifier("donetsklviv")
///             .instance_class("db.r4.large")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_rds_global_cluster` using the RDS Global Cluster identifier. For example:
///
/// ```sh
/// $ pulumi import aws:rds/globalCluster:GlobalCluster example example
/// ```
/// Certain resource arguments, like `force_destroy`, only exist within this provider. If the argument is set in the the provider configuration on an imported resource, This provider will show a difference on the first plan after import to update the state value. This change is safe to apply immediately so the state matches the desired configuration.
///
/// Certain resource arguments, like `source_db_cluster_identifier`, do not have an API method for reading the information after creation. If the argument is set in the Pulumi program on an imported resource, Pulumi will always show a difference. To workaround this behavior, either omit the argument from the Pulumi program or use `ignore_changes` to hide the difference. For example:
///
#[allow(clippy::doc_lazy_continuation)]
pub mod global_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlobalClusterArgs {
        /// Name for an automatically created database on cluster creation. Pulumi will only perform drift detection if a configuration value is provided.
        #[builder(into, default)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If the Global Cluster should have deletion protection enabled. The database can't be deleted when this value is set to `true`. The default is `false`.
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the database engine to be used for this DB cluster. The provider will only perform drift detection if a configuration value is provided. Valid values: `aurora`, `aurora-mysql`, `aurora-postgresql`. Defaults to `aurora`. Conflicts with `source_db_cluster_identifier`.
        #[builder(into, default)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The life cycle type for this DB instance. This setting applies only to Aurora PostgreSQL-based global databases. Valid values are `open-source-rds-extended-support`, `open-source-rds-extended-support-disabled`. Default value is `open-source-rds-extended-support`. [Using Amazon RDS Extended Support]: https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/extended-support.html
        #[builder(into, default)]
        pub engine_lifecycle_support: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Engine version of the Aurora global database. The `engine`, `engine_version`, and `instance_class` (on the `aws.rds.ClusterInstance`) must together support global databases. See [Using Amazon Aurora global databases](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-global-database.html) for more information. By upgrading the engine version, the provider will upgrade cluster members. **NOTE:** To avoid an `inconsistent final plan` error while upgrading, use the `lifecycle` `ignore_changes` for `engine_version` meta argument on the associated `aws.rds.Cluster` resource as shown above in Upgrading Engine Versions example.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enable to remove DB Cluster members from Global Cluster on destroy. Required with `source_db_cluster_identifier`.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Global cluster identifier.
        #[builder(into)]
        pub global_cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Amazon Resource Name (ARN) to use as the primary DB Cluster of the Global Cluster on creation. The provider cannot perform drift detection of this value.
        #[builder(into, default)]
        pub source_db_cluster_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies whether the DB cluster is encrypted. The default is `false` unless `source_db_cluster_identifier` is specified and encrypted. The provider will only perform drift detection if a configuration value is provided.
        #[builder(into, default)]
        pub storage_encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A map of tags to assign to the DB cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GlobalClusterResult {
        /// RDS Global Cluster Amazon Resource Name (ARN).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name for an automatically created database on cluster creation. Pulumi will only perform drift detection if a configuration value is provided.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// If the Global Cluster should have deletion protection enabled. The database can't be deleted when this value is set to `true`. The default is `false`.
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Writer endpoint for the new global database cluster. This endpoint always points to the writer DB instance in the current primary cluster.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Name of the database engine to be used for this DB cluster. The provider will only perform drift detection if a configuration value is provided. Valid values: `aurora`, `aurora-mysql`, `aurora-postgresql`. Defaults to `aurora`. Conflicts with `source_db_cluster_identifier`.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// The life cycle type for this DB instance. This setting applies only to Aurora PostgreSQL-based global databases. Valid values are `open-source-rds-extended-support`, `open-source-rds-extended-support-disabled`. Default value is `open-source-rds-extended-support`. [Using Amazon RDS Extended Support]: https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/extended-support.html
        pub engine_lifecycle_support: pulumi_gestalt_rust::Output<String>,
        /// Engine version of the Aurora global database. The `engine`, `engine_version`, and `instance_class` (on the `aws.rds.ClusterInstance`) must together support global databases. See [Using Amazon Aurora global databases](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-global-database.html) for more information. By upgrading the engine version, the provider will upgrade cluster members. **NOTE:** To avoid an `inconsistent final plan` error while upgrading, use the `lifecycle` `ignore_changes` for `engine_version` meta argument on the associated `aws.rds.Cluster` resource as shown above in Upgrading Engine Versions example.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        pub engine_version_actual: pulumi_gestalt_rust::Output<String>,
        /// Enable to remove DB Cluster members from Global Cluster on destroy. Required with `source_db_cluster_identifier`.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Global cluster identifier.
        pub global_cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// Set of objects containing Global Cluster members.
        pub global_cluster_members: pulumi_gestalt_rust::Output<
            Vec<super::super::types::rds::GlobalClusterGlobalClusterMember>,
        >,
        /// AWS Region-unique, immutable identifier for the global database cluster. This identifier is found in AWS CloudTrail log entries whenever the AWS KMS key for the DB cluster is accessed.
        pub global_cluster_resource_id: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) to use as the primary DB Cluster of the Global Cluster on creation. The provider cannot perform drift detection of this value.
        pub source_db_cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the DB cluster is encrypted. The default is `false` unless `source_db_cluster_identifier` is specified and encrypted. The provider will only perform drift detection if a configuration value is provided.
        pub storage_encrypted: pulumi_gestalt_rust::Output<bool>,
        /// A map of tags to assign to the DB cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GlobalClusterArgs,
    ) -> GlobalClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let database_name_binding = args.database_name.get_output(context).get_inner();
        let deletion_protection_binding = args
            .deletion_protection
            .get_output(context)
            .get_inner();
        let engine_binding = args.engine.get_output(context).get_inner();
        let engine_lifecycle_support_binding = args
            .engine_lifecycle_support
            .get_output(context)
            .get_inner();
        let engine_version_binding = args.engine_version.get_output(context).get_inner();
        let force_destroy_binding = args.force_destroy.get_output(context).get_inner();
        let global_cluster_identifier_binding = args
            .global_cluster_identifier
            .get_output(context)
            .get_inner();
        let source_db_cluster_identifier_binding = args
            .source_db_cluster_identifier
            .get_output(context)
            .get_inner();
        let storage_encrypted_binding = args
            .storage_encrypted
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/globalCluster:GlobalCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
                },
                register_interface::ObjectField {
                    name: "engineLifecycleSupport".into(),
                    value: &engine_lifecycle_support_binding,
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
                    name: "globalClusterIdentifier".into(),
                    value: &global_cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "sourceDbClusterIdentifier".into(),
                    value: &source_db_cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "storageEncrypted".into(),
                    value: &storage_encrypted_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GlobalClusterResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            database_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseName"),
            ),
            deletion_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            engine: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engine"),
            ),
            engine_lifecycle_support: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineLifecycleSupport"),
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
            global_cluster_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("globalClusterIdentifier"),
            ),
            global_cluster_members: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("globalClusterMembers"),
            ),
            global_cluster_resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("globalClusterResourceId"),
            ),
            source_db_cluster_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceDbClusterIdentifier"),
            ),
            storage_encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageEncrypted"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
