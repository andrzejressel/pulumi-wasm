/// Provides an Application AutoScaling Policy resource.
///
/// ## Example Usage
///
/// ### DynamoDB Table Autoscaling
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dynamodbTableReadPolicy = policy::create(
///         "dynamodbTableReadPolicy",
///         PolicyArgs::builder()
///             .name(
///                 "DynamoDBReadCapacityUtilization:${dynamodbTableReadTarget.resourceId}",
///             )
///             .policy_type("TargetTrackingScaling")
///             .resource_id("${dynamodbTableReadTarget.resourceId}")
///             .scalable_dimension("${dynamodbTableReadTarget.scalableDimension}")
///             .service_namespace("${dynamodbTableReadTarget.serviceNamespace}")
///             .target_tracking_scaling_policy_configuration(
///                 PolicyTargetTrackingScalingPolicyConfiguration::builder()
///                     .predefinedMetricSpecification(
///                         PolicyTargetTrackingScalingPolicyConfigurationPredefinedMetricSpecification::builder()
///                             .predefinedMetricType("DynamoDBReadCapacityUtilization")
///                             .build_struct(),
///                     )
///                     .targetValue(70)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let dynamodbTableReadTarget = target::create(
///         "dynamodbTableReadTarget",
///         TargetArgs::builder()
///             .max_capacity(100)
///             .min_capacity(5)
///             .resource_id("table/tableName")
///             .scalable_dimension("dynamodb:table:ReadCapacityUnits")
///             .service_namespace("dynamodb")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### ECS Service Autoscaling
///
/// ```yaml
/// resources:
///   ecsTarget:
///     type: aws:appautoscaling:Target
///     name: ecs_target
///     properties:
///       maxCapacity: 4
///       minCapacity: 1
///       resourceId: service/clusterName/serviceName
///       scalableDimension: ecs:service:DesiredCount
///       serviceNamespace: ecs
///   ecsPolicy:
///     type: aws:appautoscaling:Policy
///     name: ecs_policy
///     properties:
///       name: scale-down
///       policyType: StepScaling
///       resourceId: ${ecsTarget.resourceId}
///       scalableDimension: ${ecsTarget.scalableDimension}
///       serviceNamespace: ${ecsTarget.serviceNamespace}
///       stepScalingPolicyConfiguration:
///         adjustmentType: ChangeInCapacity
///         cooldown: 60
///         metricAggregationType: Maximum
///         stepAdjustments:
///           - metricIntervalUpperBound: 0
///             scalingAdjustment: -1
/// ```
///
/// ### Preserve desired count when updating an autoscaled ECS Service
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ecsService = service::create(
///         "ecsService",
///         ServiceArgs::builder()
///             .cluster("clusterName")
///             .desired_count(2)
///             .name("serviceName")
///             .task_definition("taskDefinitionFamily:1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Aurora Read Replica Autoscaling
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let replicas = target::create(
///         "replicas",
///         TargetArgs::builder()
///             .max_capacity(15)
///             .min_capacity(1)
///             .resource_id("cluster:${example.id}")
///             .scalable_dimension("rds:cluster:ReadReplicaCount")
///             .service_namespace("rds")
///             .build_struct(),
///     );
///     let replicasPolicy = policy::create(
///         "replicasPolicy",
///         PolicyArgs::builder()
///             .name("cpu-auto-scaling")
///             .policy_type("TargetTrackingScaling")
///             .resource_id("${replicas.resourceId}")
///             .scalable_dimension("${replicas.scalableDimension}")
///             .service_namespace("${replicas.serviceNamespace}")
///             .target_tracking_scaling_policy_configuration(
///                 PolicyTargetTrackingScalingPolicyConfiguration::builder()
///                     .predefinedMetricSpecification(
///                         PolicyTargetTrackingScalingPolicyConfigurationPredefinedMetricSpecification::builder()
///                             .predefinedMetricType("RDSReaderAverageCPUUtilization")
///                             .build_struct(),
///                     )
///                     .scaleInCooldown(300)
///                     .scaleOutCooldown(300)
///                     .targetValue(75)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Create target tracking scaling policy using metric math
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ecsTarget = target::create(
///         "ecsTarget",
///         TargetArgs::builder()
///             .max_capacity(4)
///             .min_capacity(1)
///             .resource_id("service/clusterName/serviceName")
///             .scalable_dimension("ecs:service:DesiredCount")
///             .service_namespace("ecs")
///             .build_struct(),
///     );
///     let example = policy::create(
///         "example",
///         PolicyArgs::builder()
///             .name("foo")
///             .policy_type("TargetTrackingScaling")
///             .resource_id("${ecsTarget.resourceId}")
///             .scalable_dimension("${ecsTarget.scalableDimension}")
///             .service_namespace("${ecsTarget.serviceNamespace}")
///             .target_tracking_scaling_policy_configuration(
///                 PolicyTargetTrackingScalingPolicyConfiguration::builder()
///                     .customizedMetricSpecification(
///                         PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecification::builder()
///                             .metrics(
///                                 vec![
///                                     PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecificationMetric::builder()
///                                     .id("m1")
///                                     .label("Get the queue size (the number of messages waiting to be processed)")
///                                     .metricStat(PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecificationMetricMetricStat::builder()
///                                     .metric(PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecificationMetricMetricStatMetric::builder()
///                                     .dimensions(vec![PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecificationMetricMetricStatMetricDimension::builder()
///                                     .name("QueueName").value("my-queue").build_struct(),])
///                                     .metricName("ApproximateNumberOfMessagesVisible")
///                                     .namespace("AWS/SQS").build_struct()).stat("Sum")
///                                     .build_struct()).returnData(false).build_struct(),
///                                     PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecificationMetric::builder()
///                                     .id("m2")
///                                     .label("Get the ECS running task count (the number of currently running tasks)")
///                                     .metricStat(PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecificationMetricMetricStat::builder()
///                                     .metric(PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecificationMetricMetricStatMetric::builder()
///                                     .dimensions(vec![PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecificationMetricMetricStatMetricDimension::builder()
///                                     .name("ClusterName").value("default").build_struct(),
///                                     PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecificationMetricMetricStatMetricDimension::builder()
///                                     .name("ServiceName").value("web-app").build_struct(),])
///                                     .metricName("RunningTaskCount")
///                                     .namespace("ECS/ContainerInsights").build_struct())
///                                     .stat("Average").build_struct()).returnData(false)
///                                     .build_struct(),
///                                     PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecificationMetric::builder()
///                                     .expression("m1 / m2").id("e1")
///                                     .label("Calculate the backlog per instance")
///                                     .returnData(true).build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .targetValue(100)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### MSK / Kafka Autoscaling
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let mskTarget = target::create(
///         "mskTarget",
///         TargetArgs::builder()
///             .max_capacity(8)
///             .min_capacity(1)
///             .resource_id("${example.arn}")
///             .scalable_dimension("kafka:broker-storage:VolumeSize")
///             .service_namespace("kafka")
///             .build_struct(),
///     );
///     let targets = policy::create(
///         "targets",
///         PolicyArgs::builder()
///             .name("storage-size-auto-scaling")
///             .policy_type("TargetTrackingScaling")
///             .resource_id("${mskTarget.resourceId}")
///             .scalable_dimension("${mskTarget.scalableDimension}")
///             .service_namespace("${mskTarget.serviceNamespace}")
///             .target_tracking_scaling_policy_configuration(
///                 PolicyTargetTrackingScalingPolicyConfiguration::builder()
///                     .predefinedMetricSpecification(
///                         PolicyTargetTrackingScalingPolicyConfigurationPredefinedMetricSpecification::builder()
///                             .predefinedMetricType("KafkaBrokerStorageUtilization")
///                             .build_struct(),
///                     )
///                     .targetValue(55)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Application AutoScaling Policy using the `service-namespace` , `resource-id`, `scalable-dimension` and `policy-name` separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:appautoscaling/policy:Policy test-policy service-namespace/resource-id/scalable-dimension/policy-name
/// ```
pub mod policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// Name of the policy. Must be between 1 and 255 characters in length.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Policy type. Valid values are `StepScaling` and `TargetTrackingScaling`. Defaults to `StepScaling`. Certain services only support only one policy type. For more information see the [Target Tracking Scaling Policies](https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-target-tracking.html) and [Step Scaling Policies](https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-step-scaling-policies.html) documentation.
        #[builder(into, default)]
        pub policy_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource type and unique identifier string for the resource associated with the scaling policy. Documentation can be found in the `ResourceId` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html)
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// Scalable dimension of the scalable target. Documentation can be found in the `ScalableDimension` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html)
        #[builder(into)]
        pub scalable_dimension: pulumi_wasm_rust::Output<String>,
        /// AWS service namespace of the scalable target. Documentation can be found in the `ServiceNamespace` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html)
        #[builder(into)]
        pub service_namespace: pulumi_wasm_rust::Output<String>,
        /// Step scaling policy configuration, requires `policy_type = "StepScaling"` (default). See supported fields below.
        #[builder(into, default)]
        pub step_scaling_policy_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::appautoscaling::PolicyStepScalingPolicyConfiguration,
            >,
        >,
        /// Target tracking policy, requires `policy_type = "TargetTrackingScaling"`. See supported fields below.
        #[builder(into, default)]
        pub target_tracking_scaling_policy_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::appautoscaling::PolicyTargetTrackingScalingPolicyConfiguration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// List of CloudWatch alarm ARNs associated with the scaling policy.
        pub alarm_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// ARN assigned by AWS to the scaling policy.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of the policy. Must be between 1 and 255 characters in length.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Policy type. Valid values are `StepScaling` and `TargetTrackingScaling`. Defaults to `StepScaling`. Certain services only support only one policy type. For more information see the [Target Tracking Scaling Policies](https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-target-tracking.html) and [Step Scaling Policies](https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-step-scaling-policies.html) documentation.
        pub policy_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource type and unique identifier string for the resource associated with the scaling policy. Documentation can be found in the `ResourceId` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html)
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// Scalable dimension of the scalable target. Documentation can be found in the `ScalableDimension` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html)
        pub scalable_dimension: pulumi_wasm_rust::Output<String>,
        /// AWS service namespace of the scalable target. Documentation can be found in the `ServiceNamespace` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html)
        pub service_namespace: pulumi_wasm_rust::Output<String>,
        /// Step scaling policy configuration, requires `policy_type = "StepScaling"` (default). See supported fields below.
        pub step_scaling_policy_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::appautoscaling::PolicyStepScalingPolicyConfiguration,
            >,
        >,
        /// Target tracking policy, requires `policy_type = "TargetTrackingScaling"`. See supported fields below.
        pub target_tracking_scaling_policy_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::appautoscaling::PolicyTargetTrackingScalingPolicyConfiguration,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PolicyArgs) -> PolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let policy_type_binding = args.policy_type.get_inner();
        let resource_id_binding = args.resource_id.get_inner();
        let scalable_dimension_binding = args.scalable_dimension.get_inner();
        let service_namespace_binding = args.service_namespace.get_inner();
        let step_scaling_policy_configuration_binding = args
            .step_scaling_policy_configuration
            .get_inner();
        let target_tracking_scaling_policy_configuration_binding = args
            .target_tracking_scaling_policy_configuration
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appautoscaling/policy:Policy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policyType".into(),
                    value: &policy_type_binding,
                },
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "scalableDimension".into(),
                    value: &scalable_dimension_binding,
                },
                register_interface::ObjectField {
                    name: "serviceNamespace".into(),
                    value: &service_namespace_binding,
                },
                register_interface::ObjectField {
                    name: "stepScalingPolicyConfiguration".into(),
                    value: &step_scaling_policy_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "targetTrackingScalingPolicyConfiguration".into(),
                    value: &target_tracking_scaling_policy_configuration_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alarmArns".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "policyType".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "scalableDimension".into(),
                },
                register_interface::ResultField {
                    name: "serviceNamespace".into(),
                },
                register_interface::ResultField {
                    name: "stepScalingPolicyConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "targetTrackingScalingPolicyConfiguration".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PolicyResult {
            alarm_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alarmArns").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            policy_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyType").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            scalable_dimension: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scalableDimension").unwrap(),
            ),
            service_namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceNamespace").unwrap(),
            ),
            step_scaling_policy_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stepScalingPolicyConfiguration").unwrap(),
            ),
            target_tracking_scaling_policy_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetTrackingScalingPolicyConfiguration").unwrap(),
            ),
        }
    }
}