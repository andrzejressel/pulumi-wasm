#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_orderable_db_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrderableDbInstanceArgs {
        /// Availability zone group.
        #[builder(into, default)]
        pub availability_zone_group: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// DB engine. Engine values include `aurora`, `aurora-mysql`, `aurora-postgresql`, `docdb`, `mariadb`, `mysql`, `neptune`, `oracle-ee`, `oracle-se`, `oracle-se1`, `oracle-se2`, `postgres`, `sqlserver-ee`, `sqlserver-ex`, `sqlserver-se`, and `sqlserver-web`.
        #[builder(into)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<String>,
        /// When set to `true`, the data source attempts to return the most recent version matching the other criteria you provide. You must use `engine_latest_version` with `preferred_instance_classes` and/or `preferred_engine_versions`. Using `engine_latest_version` will avoid `multiple RDS DB Instance Classes` errors. If you use `engine_latest_version` with `preferred_instance_classes`, the data source returns the latest version for the _first_ matching instance class (instance class priority). **Note:** The data source uses a best-effort approach at selecting the latest version but due to the complexity of version identifiers across engines, using `engine_latest_version` may _not_ return the latest version in every situation.
        #[builder(into, default)]
        pub engine_latest_version: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Version of the DB engine. If none is provided, the data source tries to use the AWS-defined default version that matches any other criteria.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// DB instance class. Examples of classes are `db.m3.2xlarge`, `db.t2.small`, and `db.m3.medium`.
        #[builder(into, default)]
        pub instance_class: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// License model. Examples of license models are `general-public-license`, `bring-your-own-license`, and `amazon-license`.
        #[builder(into, default)]
        pub license_model: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Ordered list of preferred RDS DB instance engine versions. When `engine_latest_version` is not set, the data source will return the first match in this list that matches any other criteria. If the data source finds no preferred matches or multiple matches without `engine_latest_version`, it returns an error. **CAUTION:** We don't recommend using `preferred_engine_versions` without `preferred_instance_classes` since the data source returns an arbitrary `instance_class` based on the first one AWS returns that matches the engine version and any other criteria.
        #[builder(into, default)]
        pub preferred_engine_versions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Ordered list of preferred RDS DB instance classes. The data source will return the first match in this list that matches any other criteria. If the data source finds no preferred matches or multiple matches without `engine_latest_version`, it returns an error. If you use `preferred_instance_classes` without `preferred_engine_versions` or `engine_latest_version`, the data source returns an arbitrary `engine_version` based on the first one AWS returns matching the instance class and any other criteria.
        #[builder(into, default)]
        pub preferred_instance_classes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Whether a DB instance can have a read replica.
        #[builder(into, default)]
        pub read_replica_capable: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Storage types. Examples of storage types are `standard`, `io1`, `gp2`, and `aurora`.
        #[builder(into, default)]
        pub storage_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Use to limit results to engine modes such as `provisioned`.
        #[builder(into, default)]
        pub supported_engine_modes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Use to limit results to network types `IPV4` or `DUAL`.
        #[builder(into, default)]
        pub supported_network_types: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Whether to limit results to instances that support clusters.
        #[builder(into, default)]
        pub supports_clusters: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enable this to ensure a DB instance supports Enhanced Monitoring at intervals from 1 to 60 seconds.
        #[builder(into, default)]
        pub supports_enhanced_monitoring: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Enable this to ensure a DB instance supports Aurora global databases with a specific combination of other DB engine attributes.
        #[builder(into, default)]
        pub supports_global_databases: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enable this to ensure a DB instance supports IAM database authentication.
        #[builder(into, default)]
        pub supports_iam_database_authentication: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Enable this to ensure a DB instance supports provisioned IOPS.
        #[builder(into, default)]
        pub supports_iops: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enable this to ensure a DB instance supports Kerberos Authentication.
        #[builder(into, default)]
        pub supports_kerberos_authentication: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether to limit results to instances that are multi-AZ capable.
        #[builder(into, default)]
        pub supports_multi_az: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enable this to ensure a DB instance supports Performance Insights.
        #[builder(into, default)]
        pub supports_performance_insights: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Enable this to ensure Amazon RDS can automatically scale storage for DB instances that use the specified DB instance class.
        #[builder(into, default)]
        pub supports_storage_autoscaling: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Enable this to ensure a DB instance supports encrypted storage.
        #[builder(into, default)]
        pub supports_storage_encryption: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Boolean that indicates whether to show only VPC or non-VPC offerings.
        #[builder(into, default)]
        pub vpc: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetOrderableDbInstanceResult {
        pub availability_zone_group: pulumi_gestalt_rust::Output<String>,
        /// Availability zones where the instance is available.
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        pub engine: pulumi_gestalt_rust::Output<String>,
        pub engine_latest_version: pulumi_gestalt_rust::Output<Option<bool>>,
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_class: pulumi_gestalt_rust::Output<String>,
        pub license_model: pulumi_gestalt_rust::Output<String>,
        /// Maximum total provisioned IOPS for a DB instance.
        pub max_iops_per_db_instance: pulumi_gestalt_rust::Output<i32>,
        /// Maximum provisioned IOPS per GiB for a DB instance.
        pub max_iops_per_gib: pulumi_gestalt_rust::Output<f64>,
        /// Maximum storage size for a DB instance.
        pub max_storage_size: pulumi_gestalt_rust::Output<i32>,
        /// Minimum total provisioned IOPS for a DB instance.
        pub min_iops_per_db_instance: pulumi_gestalt_rust::Output<i32>,
        /// Minimum provisioned IOPS per GiB for a DB instance.
        pub min_iops_per_gib: pulumi_gestalt_rust::Output<f64>,
        /// Minimum storage size for a DB instance.
        pub min_storage_size: pulumi_gestalt_rust::Output<i32>,
        /// Whether a DB instance is Multi-AZ capable.
        pub multi_az_capable: pulumi_gestalt_rust::Output<bool>,
        /// Whether a DB instance supports RDS on Outposts.
        pub outpost_capable: pulumi_gestalt_rust::Output<bool>,
        pub preferred_engine_versions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub preferred_instance_classes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub read_replica_capable: pulumi_gestalt_rust::Output<bool>,
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        pub supported_engine_modes: pulumi_gestalt_rust::Output<Vec<String>>,
        pub supported_network_types: pulumi_gestalt_rust::Output<Vec<String>>,
        pub supports_clusters: pulumi_gestalt_rust::Output<bool>,
        pub supports_enhanced_monitoring: pulumi_gestalt_rust::Output<bool>,
        pub supports_global_databases: pulumi_gestalt_rust::Output<bool>,
        pub supports_iam_database_authentication: pulumi_gestalt_rust::Output<bool>,
        pub supports_iops: pulumi_gestalt_rust::Output<bool>,
        pub supports_kerberos_authentication: pulumi_gestalt_rust::Output<bool>,
        pub supports_multi_az: pulumi_gestalt_rust::Output<bool>,
        pub supports_performance_insights: pulumi_gestalt_rust::Output<bool>,
        pub supports_storage_autoscaling: pulumi_gestalt_rust::Output<bool>,
        pub supports_storage_encryption: pulumi_gestalt_rust::Output<bool>,
        pub vpc: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetOrderableDbInstanceArgs,
    ) -> GetOrderableDbInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let availability_zone_group_binding = args
            .availability_zone_group
            .get_output(context)
            .get_inner();
        let engine_binding = args.engine.get_output(context).get_inner();
        let engine_latest_version_binding = args
            .engine_latest_version
            .get_output(context)
            .get_inner();
        let engine_version_binding = args.engine_version.get_output(context).get_inner();
        let instance_class_binding = args.instance_class.get_output(context).get_inner();
        let license_model_binding = args.license_model.get_output(context).get_inner();
        let preferred_engine_versions_binding = args
            .preferred_engine_versions
            .get_output(context)
            .get_inner();
        let preferred_instance_classes_binding = args
            .preferred_instance_classes
            .get_output(context)
            .get_inner();
        let read_replica_capable_binding = args
            .read_replica_capable
            .get_output(context)
            .get_inner();
        let storage_type_binding = args.storage_type.get_output(context).get_inner();
        let supported_engine_modes_binding = args
            .supported_engine_modes
            .get_output(context)
            .get_inner();
        let supported_network_types_binding = args
            .supported_network_types
            .get_output(context)
            .get_inner();
        let supports_clusters_binding = args
            .supports_clusters
            .get_output(context)
            .get_inner();
        let supports_enhanced_monitoring_binding = args
            .supports_enhanced_monitoring
            .get_output(context)
            .get_inner();
        let supports_global_databases_binding = args
            .supports_global_databases
            .get_output(context)
            .get_inner();
        let supports_iam_database_authentication_binding = args
            .supports_iam_database_authentication
            .get_output(context)
            .get_inner();
        let supports_iops_binding = args.supports_iops.get_output(context).get_inner();
        let supports_kerberos_authentication_binding = args
            .supports_kerberos_authentication
            .get_output(context)
            .get_inner();
        let supports_multi_az_binding = args
            .supports_multi_az
            .get_output(context)
            .get_inner();
        let supports_performance_insights_binding = args
            .supports_performance_insights
            .get_output(context)
            .get_inner();
        let supports_storage_autoscaling_binding = args
            .supports_storage_autoscaling
            .get_output(context)
            .get_inner();
        let supports_storage_encryption_binding = args
            .supports_storage_encryption
            .get_output(context)
            .get_inner();
        let vpc_binding = args.vpc.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:rds/getOrderableDbInstance:getOrderableDbInstance".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "availabilityZoneGroup".into(),
                    value: &availability_zone_group_binding,
                },
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
                },
                register_interface::ObjectField {
                    name: "engineLatestVersion".into(),
                    value: &engine_latest_version_binding,
                },
                register_interface::ObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding,
                },
                register_interface::ObjectField {
                    name: "instanceClass".into(),
                    value: &instance_class_binding,
                },
                register_interface::ObjectField {
                    name: "licenseModel".into(),
                    value: &license_model_binding,
                },
                register_interface::ObjectField {
                    name: "preferredEngineVersions".into(),
                    value: &preferred_engine_versions_binding,
                },
                register_interface::ObjectField {
                    name: "preferredInstanceClasses".into(),
                    value: &preferred_instance_classes_binding,
                },
                register_interface::ObjectField {
                    name: "readReplicaCapable".into(),
                    value: &read_replica_capable_binding,
                },
                register_interface::ObjectField {
                    name: "storageType".into(),
                    value: &storage_type_binding,
                },
                register_interface::ObjectField {
                    name: "supportedEngineModes".into(),
                    value: &supported_engine_modes_binding,
                },
                register_interface::ObjectField {
                    name: "supportedNetworkTypes".into(),
                    value: &supported_network_types_binding,
                },
                register_interface::ObjectField {
                    name: "supportsClusters".into(),
                    value: &supports_clusters_binding,
                },
                register_interface::ObjectField {
                    name: "supportsEnhancedMonitoring".into(),
                    value: &supports_enhanced_monitoring_binding,
                },
                register_interface::ObjectField {
                    name: "supportsGlobalDatabases".into(),
                    value: &supports_global_databases_binding,
                },
                register_interface::ObjectField {
                    name: "supportsIamDatabaseAuthentication".into(),
                    value: &supports_iam_database_authentication_binding,
                },
                register_interface::ObjectField {
                    name: "supportsIops".into(),
                    value: &supports_iops_binding,
                },
                register_interface::ObjectField {
                    name: "supportsKerberosAuthentication".into(),
                    value: &supports_kerberos_authentication_binding,
                },
                register_interface::ObjectField {
                    name: "supportsMultiAz".into(),
                    value: &supports_multi_az_binding,
                },
                register_interface::ObjectField {
                    name: "supportsPerformanceInsights".into(),
                    value: &supports_performance_insights_binding,
                },
                register_interface::ObjectField {
                    name: "supportsStorageAutoscaling".into(),
                    value: &supports_storage_autoscaling_binding,
                },
                register_interface::ObjectField {
                    name: "supportsStorageEncryption".into(),
                    value: &supports_storage_encryption_binding,
                },
                register_interface::ObjectField {
                    name: "vpc".into(),
                    value: &vpc_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOrderableDbInstanceResult {
            availability_zone_group: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZoneGroup"),
            ),
            availability_zones: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZones"),
            ),
            engine: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engine"),
            ),
            engine_latest_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineLatestVersion"),
            ),
            engine_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineVersion"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance_class: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceClass"),
            ),
            license_model: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseModel"),
            ),
            max_iops_per_db_instance: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxIopsPerDbInstance"),
            ),
            max_iops_per_gib: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxIopsPerGib"),
            ),
            max_storage_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxStorageSize"),
            ),
            min_iops_per_db_instance: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minIopsPerDbInstance"),
            ),
            min_iops_per_gib: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minIopsPerGib"),
            ),
            min_storage_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minStorageSize"),
            ),
            multi_az_capable: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("multiAzCapable"),
            ),
            outpost_capable: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outpostCapable"),
            ),
            preferred_engine_versions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preferredEngineVersions"),
            ),
            preferred_instance_classes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preferredInstanceClasses"),
            ),
            read_replica_capable: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readReplicaCapable"),
            ),
            storage_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageType"),
            ),
            supported_engine_modes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportedEngineModes"),
            ),
            supported_network_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportedNetworkTypes"),
            ),
            supports_clusters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportsClusters"),
            ),
            supports_enhanced_monitoring: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportsEnhancedMonitoring"),
            ),
            supports_global_databases: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportsGlobalDatabases"),
            ),
            supports_iam_database_authentication: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportsIamDatabaseAuthentication"),
            ),
            supports_iops: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportsIops"),
            ),
            supports_kerberos_authentication: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportsKerberosAuthentication"),
            ),
            supports_multi_az: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportsMultiAz"),
            ),
            supports_performance_insights: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportsPerformanceInsights"),
            ),
            supports_storage_autoscaling: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportsStorageAutoscaling"),
            ),
            supports_storage_encryption: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportsStorageEncryption"),
            ),
            vpc: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpc")),
        }
    }
}
