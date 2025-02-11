/// Provides an Application AutoScaling Policy resource.
///
/// ## Example Usage
///
/// ### DynamoDB Table Autoscaling
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// Name of the policy. Must be between 1 and 255 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Policy type. Valid values are `StepScaling` and `TargetTrackingScaling`. Defaults to `StepScaling`. Certain services only support only one policy type. For more information see the [Target Tracking Scaling Policies](https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-target-tracking.html) and [Step Scaling Policies](https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-step-scaling-policies.html) documentation.
        #[builder(into, default)]
        pub policy_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Resource type and unique identifier string for the resource associated with the scaling policy. Documentation can be found in the `ResourceId` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html)
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Scalable dimension of the scalable target. Documentation can be found in the `ScalableDimension` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html)
        #[builder(into)]
        pub scalable_dimension: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS service namespace of the scalable target. Documentation can be found in the `ServiceNamespace` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html)
        #[builder(into)]
        pub service_namespace: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Step scaling policy configuration, requires `policy_type = "StepScaling"` (default). See supported fields below.
        #[builder(into, default)]
        pub step_scaling_policy_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::appautoscaling::PolicyStepScalingPolicyConfiguration,
            >,
        >,
        /// Target tracking policy, requires `policy_type = "TargetTrackingScaling"`. See supported fields below.
        #[builder(into, default)]
        pub target_tracking_scaling_policy_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::appautoscaling::PolicyTargetTrackingScalingPolicyConfiguration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// List of CloudWatch alarm ARNs associated with the scaling policy.
        pub alarm_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ARN assigned by AWS to the scaling policy.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the policy. Must be between 1 and 255 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Policy type. Valid values are `StepScaling` and `TargetTrackingScaling`. Defaults to `StepScaling`. Certain services only support only one policy type. For more information see the [Target Tracking Scaling Policies](https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-target-tracking.html) and [Step Scaling Policies](https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-step-scaling-policies.html) documentation.
        pub policy_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Resource type and unique identifier string for the resource associated with the scaling policy. Documentation can be found in the `ResourceId` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html)
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// Scalable dimension of the scalable target. Documentation can be found in the `ScalableDimension` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html)
        pub scalable_dimension: pulumi_gestalt_rust::Output<String>,
        /// AWS service namespace of the scalable target. Documentation can be found in the `ServiceNamespace` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html)
        pub service_namespace: pulumi_gestalt_rust::Output<String>,
        /// Step scaling policy configuration, requires `policy_type = "StepScaling"` (default). See supported fields below.
        pub step_scaling_policy_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::appautoscaling::PolicyStepScalingPolicyConfiguration,
            >,
        >,
        /// Target tracking policy, requires `policy_type = "TargetTrackingScaling"`. See supported fields below.
        pub target_tracking_scaling_policy_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::appautoscaling::PolicyTargetTrackingScalingPolicyConfiguration,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyArgs,
    ) -> PolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let policy_type_binding = args.policy_type.get_output(context);
        let resource_id_binding = args.resource_id.get_output(context);
        let scalable_dimension_binding = args.scalable_dimension.get_output(context);
        let service_namespace_binding = args.service_namespace.get_output(context);
        let step_scaling_policy_configuration_binding = args
            .step_scaling_policy_configuration
            .get_output(context);
        let target_tracking_scaling_policy_configuration_binding = args
            .target_tracking_scaling_policy_configuration
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appautoscaling/policy:Policy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyType".into(),
                    value: &policy_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scalableDimension".into(),
                    value: &scalable_dimension_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceNamespace".into(),
                    value: &service_namespace_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stepScalingPolicyConfiguration".into(),
                    value: &step_scaling_policy_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetTrackingScalingPolicyConfiguration".into(),
                    value: &target_tracking_scaling_policy_configuration_binding
                        .drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyResult {
            alarm_arns: o.get_field("alarmArns"),
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            policy_type: o.get_field("policyType"),
            resource_id: o.get_field("resourceId"),
            scalable_dimension: o.get_field("scalableDimension"),
            service_namespace: o.get_field("serviceNamespace"),
            step_scaling_policy_configuration: o
                .get_field("stepScalingPolicyConfiguration"),
            target_tracking_scaling_policy_configuration: o
                .get_field("targetTrackingScalingPolicyConfiguration"),
        }
    }
}
