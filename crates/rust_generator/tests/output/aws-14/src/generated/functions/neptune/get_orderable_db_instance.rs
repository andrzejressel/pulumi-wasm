#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_orderable_db_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrderableDbInstanceArgs {
        /// DB engine. (Default: `neptune`)
        #[builder(into, default)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Version of the DB engine. For example, `1.0.1.0`, `1.0.1.2`, `1.0.2.2`, and `1.0.3.0`.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// DB instance class. Examples of classes are `db.r5.large`, `db.r5.xlarge`, `db.r4.large`, `db.r5.4xlarge`, `db.r5.12xlarge`, `db.r4.xlarge`, and `db.t3.medium`.
        #[builder(into, default)]
        pub instance_class: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// License model. (Default: `amazon-license`)
        #[builder(into, default)]
        pub license_model: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Ordered list of preferred Neptune DB instance classes. The first match in this list will be returned. If no preferred matches are found and the original search returned more than one result, an error is returned.
        #[builder(into, default)]
        pub preferred_instance_classes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Enable to show only VPC offerings.
        #[builder(into, default)]
        pub vpc: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetOrderableDbInstanceResult {
        /// Availability zones where the instance is available.
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        pub engine: pulumi_gestalt_rust::Output<Option<String>>,
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_class: pulumi_gestalt_rust::Output<String>,
        pub license_model: pulumi_gestalt_rust::Output<Option<String>>,
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
        pub preferred_instance_classes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Whether a DB instance can have a read replica.
        pub read_replica_capable: pulumi_gestalt_rust::Output<bool>,
        /// Storage type for a DB instance.
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        /// Whether a DB instance supports Enhanced Monitoring at intervals from 1 to 60 seconds.
        pub supports_enhanced_monitoring: pulumi_gestalt_rust::Output<bool>,
        /// Whether a DB instance supports IAM database authentication.
        pub supports_iam_database_authentication: pulumi_gestalt_rust::Output<bool>,
        /// Whether a DB instance supports provisioned IOPS.
        pub supports_iops: pulumi_gestalt_rust::Output<bool>,
        /// Whether a DB instance supports Performance Insights.
        pub supports_performance_insights: pulumi_gestalt_rust::Output<bool>,
        /// Whether a DB instance supports encrypted storage.
        pub supports_storage_encryption: pulumi_gestalt_rust::Output<bool>,
        pub vpc: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOrderableDbInstanceArgs,
    ) -> GetOrderableDbInstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let engine_binding = args.engine.get_output(context);
        let engine_version_binding = args.engine_version.get_output(context);
        let instance_class_binding = args.instance_class.get_output(context);
        let license_model_binding = args.license_model.get_output(context);
        let preferred_instance_classes_binding = args
            .preferred_instance_classes
            .get_output(context);
        let vpc_binding = args.vpc.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:neptune/getOrderableDbInstance:getOrderableDbInstance".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engine".into(),
                    value: &engine_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceClass".into(),
                    value: &instance_class_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseModel".into(),
                    value: &license_model_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredInstanceClasses".into(),
                    value: &preferred_instance_classes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpc".into(),
                    value: &vpc_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOrderableDbInstanceResult {
            availability_zones: o.get_field("availabilityZones"),
            engine: o.get_field("engine"),
            engine_version: o.get_field("engineVersion"),
            id: o.get_field("id"),
            instance_class: o.get_field("instanceClass"),
            license_model: o.get_field("licenseModel"),
            max_iops_per_db_instance: o.get_field("maxIopsPerDbInstance"),
            max_iops_per_gib: o.get_field("maxIopsPerGib"),
            max_storage_size: o.get_field("maxStorageSize"),
            min_iops_per_db_instance: o.get_field("minIopsPerDbInstance"),
            min_iops_per_gib: o.get_field("minIopsPerGib"),
            min_storage_size: o.get_field("minStorageSize"),
            multi_az_capable: o.get_field("multiAzCapable"),
            preferred_instance_classes: o.get_field("preferredInstanceClasses"),
            read_replica_capable: o.get_field("readReplicaCapable"),
            storage_type: o.get_field("storageType"),
            supports_enhanced_monitoring: o.get_field("supportsEnhancedMonitoring"),
            supports_iam_database_authentication: o
                .get_field("supportsIamDatabaseAuthentication"),
            supports_iops: o.get_field("supportsIops"),
            supports_performance_insights: o.get_field("supportsPerformanceInsights"),
            supports_storage_encryption: o.get_field("supportsStorageEncryption"),
            vpc: o.get_field("vpc"),
        }
    }
}
