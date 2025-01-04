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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// }
/// ```
///
/// ### Suppressing `tags_all` Differences For Older Resources
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
pub mod target {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetArgs {
        /// Max capacity of the scalable target.
        #[builder(into)]
        pub max_capacity: pulumi_wasm_rust::Output<i32>,
        /// Min capacity of the scalable target.
        #[builder(into)]
        pub min_capacity: pulumi_wasm_rust::Output<i32>,
        /// Resource type and unique identifier string for the resource associated with the scaling policy. Documentation can be found in the `ResourceId` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html#API_RegisterScalableTarget_RequestParameters)
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role that allows Application AutoScaling to modify your scalable target on your behalf. This defaults to an IAM Service-Linked Role for most services and custom IAM Roles are ignored by the API for those namespaces. See the [AWS Application Auto Scaling documentation](https://docs.aws.amazon.com/autoscaling/application/userguide/security_iam_service-with-iam.html#security_iam_service-with-iam-roles) for more information about how this service interacts with IAM.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Scalable dimension of the scalable target. Documentation can be found in the `ScalableDimension` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html#API_RegisterScalableTarget_RequestParameters)
        #[builder(into)]
        pub scalable_dimension: pulumi_wasm_rust::Output<String>,
        /// AWS service namespace of the scalable target. Documentation can be found in the `ServiceNamespace` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html#API_RegisterScalableTarget_RequestParameters)
        #[builder(into)]
        pub service_namespace: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the scaling activities for a scalable target are in a suspended state.
        #[builder(into, default)]
        pub suspended_state: pulumi_wasm_rust::Output<
            Option<super::super::types::appautoscaling::TargetSuspendedState>,
        >,
        /// Map of tags to assign to the scalable target. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TargetResult {
        /// The ARN of the scalable target.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Max capacity of the scalable target.
        pub max_capacity: pulumi_wasm_rust::Output<i32>,
        /// Min capacity of the scalable target.
        pub min_capacity: pulumi_wasm_rust::Output<i32>,
        /// Resource type and unique identifier string for the resource associated with the scaling policy. Documentation can be found in the `ResourceId` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html#API_RegisterScalableTarget_RequestParameters)
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role that allows Application AutoScaling to modify your scalable target on your behalf. This defaults to an IAM Service-Linked Role for most services and custom IAM Roles are ignored by the API for those namespaces. See the [AWS Application Auto Scaling documentation](https://docs.aws.amazon.com/autoscaling/application/userguide/security_iam_service-with-iam.html#security_iam_service-with-iam-roles) for more information about how this service interacts with IAM.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Scalable dimension of the scalable target. Documentation can be found in the `ScalableDimension` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html#API_RegisterScalableTarget_RequestParameters)
        pub scalable_dimension: pulumi_wasm_rust::Output<String>,
        /// AWS service namespace of the scalable target. Documentation can be found in the `ServiceNamespace` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html#API_RegisterScalableTarget_RequestParameters)
        pub service_namespace: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the scaling activities for a scalable target are in a suspended state.
        pub suspended_state: pulumi_wasm_rust::Output<
            super::super::types::appautoscaling::TargetSuspendedState,
        >,
        /// Map of tags to assign to the scalable target. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TargetArgs) -> TargetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let max_capacity_binding = args.max_capacity.get_inner();
        let min_capacity_binding = args.min_capacity.get_inner();
        let resource_id_binding = args.resource_id.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let scalable_dimension_binding = args.scalable_dimension.get_inner();
        let service_namespace_binding = args.service_namespace.get_inner();
        let suspended_state_binding = args.suspended_state.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appautoscaling/target:Target".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "maxCapacity".into(),
                    value: &max_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "minCapacity".into(),
                    value: &min_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
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
                    name: "suspendedState".into(),
                    value: &suspended_state_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "maxCapacity".into(),
                },
                register_interface::ResultField {
                    name: "minCapacity".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "scalableDimension".into(),
                },
                register_interface::ResultField {
                    name: "serviceNamespace".into(),
                },
                register_interface::ResultField {
                    name: "suspendedState".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TargetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            max_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxCapacity").unwrap(),
            ),
            min_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minCapacity").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            scalable_dimension: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scalableDimension").unwrap(),
            ),
            service_namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceNamespace").unwrap(),
            ),
            suspended_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("suspendedState").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
