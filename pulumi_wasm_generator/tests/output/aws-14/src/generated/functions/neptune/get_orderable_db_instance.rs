pub mod get_orderable_db_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrderableDbInstanceArgs {
        /// DB engine. (Default: `neptune`)
        #[builder(into, default)]
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        /// Version of the DB engine. For example, `1.0.1.0`, `1.0.1.2`, `1.0.2.2`, and `1.0.3.0`.
        #[builder(into, default)]
        pub engine_version: pulumi_wasm_rust::Output<Option<String>>,
        /// DB instance class. Examples of classes are `db.r5.large`, `db.r5.xlarge`, `db.r4.large`, `db.r5.4xlarge`, `db.r5.12xlarge`, `db.r4.xlarge`, and `db.t3.medium`.
        #[builder(into, default)]
        pub instance_class: pulumi_wasm_rust::Output<Option<String>>,
        /// License model. (Default: `amazon-license`)
        #[builder(into, default)]
        pub license_model: pulumi_wasm_rust::Output<Option<String>>,
        /// Ordered list of preferred Neptune DB instance classes. The first match in this list will be returned. If no preferred matches are found and the original search returned more than one result, an error is returned.
        #[builder(into, default)]
        pub preferred_instance_classes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Enable to show only VPC offerings.
        #[builder(into, default)]
        pub vpc: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetOrderableDbInstanceResult {
        /// Availability zones where the instance is available.
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_class: pulumi_wasm_rust::Output<String>,
        pub license_model: pulumi_wasm_rust::Output<Option<String>>,
        /// Maximum total provisioned IOPS for a DB instance.
        pub max_iops_per_db_instance: pulumi_wasm_rust::Output<i32>,
        /// Maximum provisioned IOPS per GiB for a DB instance.
        pub max_iops_per_gib: pulumi_wasm_rust::Output<f64>,
        /// Maximum storage size for a DB instance.
        pub max_storage_size: pulumi_wasm_rust::Output<i32>,
        /// Minimum total provisioned IOPS for a DB instance.
        pub min_iops_per_db_instance: pulumi_wasm_rust::Output<i32>,
        /// Minimum provisioned IOPS per GiB for a DB instance.
        pub min_iops_per_gib: pulumi_wasm_rust::Output<f64>,
        /// Minimum storage size for a DB instance.
        pub min_storage_size: pulumi_wasm_rust::Output<i32>,
        /// Whether a DB instance is Multi-AZ capable.
        pub multi_az_capable: pulumi_wasm_rust::Output<bool>,
        pub preferred_instance_classes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Whether a DB instance can have a read replica.
        pub read_replica_capable: pulumi_wasm_rust::Output<bool>,
        /// Storage type for a DB instance.
        pub storage_type: pulumi_wasm_rust::Output<String>,
        /// Whether a DB instance supports Enhanced Monitoring at intervals from 1 to 60 seconds.
        pub supports_enhanced_monitoring: pulumi_wasm_rust::Output<bool>,
        /// Whether a DB instance supports IAM database authentication.
        pub supports_iam_database_authentication: pulumi_wasm_rust::Output<bool>,
        /// Whether a DB instance supports provisioned IOPS.
        pub supports_iops: pulumi_wasm_rust::Output<bool>,
        /// Whether a DB instance supports Performance Insights.
        pub supports_performance_insights: pulumi_wasm_rust::Output<bool>,
        /// Whether a DB instance supports encrypted storage.
        pub supports_storage_encryption: pulumi_wasm_rust::Output<bool>,
        pub vpc: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetOrderableDbInstanceArgs) -> GetOrderableDbInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let engine_binding = args.engine.get_inner();
        let engine_version_binding = args.engine_version.get_inner();
        let instance_class_binding = args.instance_class.get_inner();
        let license_model_binding = args.license_model.get_inner();
        let preferred_instance_classes_binding = args
            .preferred_instance_classes
            .get_inner();
        let vpc_binding = args.vpc.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:neptune/getOrderableDbInstance:getOrderableDbInstance".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
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
                    name: "preferredInstanceClasses".into(),
                    value: &preferred_instance_classes_binding,
                },
                register_interface::ObjectField {
                    name: "vpc".into(),
                    value: &vpc_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "availabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceClass".into(),
                },
                register_interface::ResultField {
                    name: "licenseModel".into(),
                },
                register_interface::ResultField {
                    name: "maxIopsPerDbInstance".into(),
                },
                register_interface::ResultField {
                    name: "maxIopsPerGib".into(),
                },
                register_interface::ResultField {
                    name: "maxStorageSize".into(),
                },
                register_interface::ResultField {
                    name: "minIopsPerDbInstance".into(),
                },
                register_interface::ResultField {
                    name: "minIopsPerGib".into(),
                },
                register_interface::ResultField {
                    name: "minStorageSize".into(),
                },
                register_interface::ResultField {
                    name: "multiAzCapable".into(),
                },
                register_interface::ResultField {
                    name: "preferredInstanceClasses".into(),
                },
                register_interface::ResultField {
                    name: "readReplicaCapable".into(),
                },
                register_interface::ResultField {
                    name: "storageType".into(),
                },
                register_interface::ResultField {
                    name: "supportsEnhancedMonitoring".into(),
                },
                register_interface::ResultField {
                    name: "supportsIamDatabaseAuthentication".into(),
                },
                register_interface::ResultField {
                    name: "supportsIops".into(),
                },
                register_interface::ResultField {
                    name: "supportsPerformanceInsights".into(),
                },
                register_interface::ResultField {
                    name: "supportsStorageEncryption".into(),
                },
                register_interface::ResultField {
                    name: "vpc".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOrderableDbInstanceResult {
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZones").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceClass").unwrap(),
            ),
            license_model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseModel").unwrap(),
            ),
            max_iops_per_db_instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxIopsPerDbInstance").unwrap(),
            ),
            max_iops_per_gib: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxIopsPerGib").unwrap(),
            ),
            max_storage_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxStorageSize").unwrap(),
            ),
            min_iops_per_db_instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minIopsPerDbInstance").unwrap(),
            ),
            min_iops_per_gib: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minIopsPerGib").unwrap(),
            ),
            min_storage_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minStorageSize").unwrap(),
            ),
            multi_az_capable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiAzCapable").unwrap(),
            ),
            preferred_instance_classes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredInstanceClasses").unwrap(),
            ),
            read_replica_capable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readReplicaCapable").unwrap(),
            ),
            storage_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageType").unwrap(),
            ),
            supports_enhanced_monitoring: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportsEnhancedMonitoring").unwrap(),
            ),
            supports_iam_database_authentication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportsIamDatabaseAuthentication").unwrap(),
            ),
            supports_iops: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportsIops").unwrap(),
            ),
            supports_performance_insights: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportsPerformanceInsights").unwrap(),
            ),
            supports_storage_encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportsStorageEncryption").unwrap(),
            ),
            vpc: pulumi_wasm_rust::__private::into_domain(hashmap.remove("vpc").unwrap()),
        }
    }
}
