pub mod get_orderable_db_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrderableDbInstanceArgs {
        /// DB engine. (Default: `neptune`)
        #[builder(into, default)]
        pub engine: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Version of the DB engine. For example, `1.0.1.0`, `1.0.1.2`, `1.0.2.2`, and `1.0.3.0`.
        #[builder(into, default)]
        pub engine_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// DB instance class. Examples of classes are `db.r5.large`, `db.r5.xlarge`, `db.r4.large`, `db.r5.4xlarge`, `db.r5.12xlarge`, `db.r4.xlarge`, and `db.t3.medium`.
        #[builder(into, default)]
        pub instance_class: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// License model. (Default: `amazon-license`)
        #[builder(into, default)]
        pub license_model: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Ordered list of preferred Neptune DB instance classes. The first match in this list will be returned. If no preferred matches are found and the original search returned more than one result, an error is returned.
        #[builder(into, default)]
        pub preferred_instance_classes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Enable to show only VPC offerings.
        #[builder(into, default)]
        pub vpc: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetOrderableDbInstanceArgs,
    ) -> GetOrderableDbInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let engine_binding = args.engine.get_output(context).get_inner();
        let engine_version_binding = args.engine_version.get_output(context).get_inner();
        let instance_class_binding = args.instance_class.get_output(context).get_inner();
        let license_model_binding = args.license_model.get_output(context).get_inner();
        let preferred_instance_classes_binding = args
            .preferred_instance_classes
            .get_output(context)
            .get_inner();
        let vpc_binding = args.vpc.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:neptune/getOrderableDbInstance:getOrderableDbInstance".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOrderableDbInstanceResult {
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZones"),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(o.extract_field("engine")),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("engineVersion"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            instance_class: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceClass"),
            ),
            license_model: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("licenseModel"),
            ),
            max_iops_per_db_instance: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maxIopsPerDbInstance"),
            ),
            max_iops_per_gib: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maxIopsPerGib"),
            ),
            max_storage_size: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maxStorageSize"),
            ),
            min_iops_per_db_instance: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("minIopsPerDbInstance"),
            ),
            min_iops_per_gib: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("minIopsPerGib"),
            ),
            min_storage_size: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("minStorageSize"),
            ),
            multi_az_capable: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("multiAzCapable"),
            ),
            preferred_instance_classes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("preferredInstanceClasses"),
            ),
            read_replica_capable: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("readReplicaCapable"),
            ),
            storage_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageType"),
            ),
            supports_enhanced_monitoring: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("supportsEnhancedMonitoring"),
            ),
            supports_iam_database_authentication: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("supportsIamDatabaseAuthentication"),
            ),
            supports_iops: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("supportsIops"),
            ),
            supports_performance_insights: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("supportsPerformanceInsights"),
            ),
            supports_storage_encryption: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("supportsStorageEncryption"),
            ),
            vpc: pulumi_wasm_rust::__private::into_domain(o.extract_field("vpc")),
        }
    }
}
