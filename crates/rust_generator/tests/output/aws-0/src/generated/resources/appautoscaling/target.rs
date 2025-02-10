/// Provides an Application AutoScaling ScalableTarget resource. To manage policies which get attached to the target, see the `aws.appautoscaling.Policy` resource.
///
/// > **NOTE:** Scalable targets created before 2023-03-20 may not have an assigned `arn`. These resource cannot use `tags` or participate in `default_tags`. To prevent `pulumi preview` showing differences that can never be reconciled, use the `lifecycle.ignore_changes` meta-argument. See the example below.
///
/// > **NOTE:** The [Application Auto Scaling service automatically attempts to manage IAM Service-Linked Roles](https://docs.aws.amazon.com/autoscaling/application/userguide/security_iam_service-with-iam.html#security_iam_service-with-iam-roles) when registering certain service namespaces for the first time. To manually manage this role, see the `aws.iam.ServiceLinkedRole` resource.
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
///     let dynamodbTableReadTarget = target::create(
///         "dynamodbTableReadTarget",
///         TargetArgs::builder()
///             .max_capacity(100)
///             .min_capacity(5)
///             .resource_id("table/${example.name}")
///             .scalable_dimension("dynamodb:table:ReadCapacityUnits")
///             .service_namespace("dynamodb")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### DynamoDB Index Autoscaling
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dynamodbIndexReadTarget = target::create(
///         "dynamodbIndexReadTarget",
///         TargetArgs::builder()
///             .max_capacity(100)
///             .min_capacity(5)
///             .resource_id("table/${example.name}/index/${indexName}")
///             .scalable_dimension("dynamodb:index:ReadCapacityUnits")
///             .service_namespace("dynamodb")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### ECS Service Autoscaling
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
///             .resource_id("service/${example.name}/${exampleAwsEcsService.name}")
///             .scalable_dimension("ecs:service:DesiredCount")
///             .service_namespace("ecs")
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
/// }
/// ```
///
/// ### Suppressing `tags_all` Differences For Older Resources
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
///             .resource_id("service/${example.name}/${exampleAwsEcsService.name}")
///             .scalable_dimension("ecs:service:DesiredCount")
///             .service_namespace("ecs")
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
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Application AutoScaling Target using the `service-namespace` , `resource-id` and `scalable-dimension` separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:appautoscaling/target:Target test-target service-namespace/resource-id/scalable-dimension
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetArgs {
        /// Max capacity of the scalable target.
        #[builder(into)]
        pub max_capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Min capacity of the scalable target.
        #[builder(into)]
        pub min_capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Resource type and unique identifier string for the resource associated with the scaling policy. Documentation can be found in the `ResourceId` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html#API_RegisterScalableTarget_RequestParameters)
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the IAM role that allows Application AutoScaling to modify your scalable target on your behalf. This defaults to an IAM Service-Linked Role for most services and custom IAM Roles are ignored by the API for those namespaces. See the [AWS Application Auto Scaling documentation](https://docs.aws.amazon.com/autoscaling/application/userguide/security_iam_service-with-iam.html#security_iam_service-with-iam-roles) for more information about how this service interacts with IAM.
        #[builder(into, default)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Scalable dimension of the scalable target. Documentation can be found in the `ScalableDimension` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html#API_RegisterScalableTarget_RequestParameters)
        #[builder(into)]
        pub scalable_dimension: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS service namespace of the scalable target. Documentation can be found in the `ServiceNamespace` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html#API_RegisterScalableTarget_RequestParameters)
        #[builder(into)]
        pub service_namespace: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies whether the scaling activities for a scalable target are in a suspended state.
        #[builder(into, default)]
        pub suspended_state: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appautoscaling::TargetSuspendedState>,
        >,
        /// Map of tags to assign to the scalable target. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TargetResult {
        /// The ARN of the scalable target.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Max capacity of the scalable target.
        pub max_capacity: pulumi_gestalt_rust::Output<i32>,
        /// Min capacity of the scalable target.
        pub min_capacity: pulumi_gestalt_rust::Output<i32>,
        /// Resource type and unique identifier string for the resource associated with the scaling policy. Documentation can be found in the `ResourceId` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html#API_RegisterScalableTarget_RequestParameters)
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM role that allows Application AutoScaling to modify your scalable target on your behalf. This defaults to an IAM Service-Linked Role for most services and custom IAM Roles are ignored by the API for those namespaces. See the [AWS Application Auto Scaling documentation](https://docs.aws.amazon.com/autoscaling/application/userguide/security_iam_service-with-iam.html#security_iam_service-with-iam-roles) for more information about how this service interacts with IAM.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Scalable dimension of the scalable target. Documentation can be found in the `ScalableDimension` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html#API_RegisterScalableTarget_RequestParameters)
        pub scalable_dimension: pulumi_gestalt_rust::Output<String>,
        /// AWS service namespace of the scalable target. Documentation can be found in the `ServiceNamespace` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html#API_RegisterScalableTarget_RequestParameters)
        pub service_namespace: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the scaling activities for a scalable target are in a suspended state.
        pub suspended_state: pulumi_gestalt_rust::Output<
            super::super::types::appautoscaling::TargetSuspendedState,
        >,
        /// Map of tags to assign to the scalable target. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TargetArgs,
    ) -> TargetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let max_capacity_binding = args.max_capacity.get_output(context);
        let min_capacity_binding = args.min_capacity.get_output(context);
        let resource_id_binding = args.resource_id.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let scalable_dimension_binding = args.scalable_dimension.get_output(context);
        let service_namespace_binding = args.service_namespace.get_output(context);
        let suspended_state_binding = args.suspended_state.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appautoscaling/target:Target".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxCapacity".into(),
                    value: max_capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minCapacity".into(),
                    value: min_capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceId".into(),
                    value: resource_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scalableDimension".into(),
                    value: scalable_dimension_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceNamespace".into(),
                    value: service_namespace_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "suspendedState".into(),
                    value: suspended_state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TargetResult {
            arn: o.get_field("arn"),
            max_capacity: o.get_field("maxCapacity"),
            min_capacity: o.get_field("minCapacity"),
            resource_id: o.get_field("resourceId"),
            role_arn: o.get_field("roleArn"),
            scalable_dimension: o.get_field("scalableDimension"),
            service_namespace: o.get_field("serviceNamespace"),
            suspended_state: o.get_field("suspendedState"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
