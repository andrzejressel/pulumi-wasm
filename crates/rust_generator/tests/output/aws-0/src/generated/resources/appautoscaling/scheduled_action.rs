/// Provides an Application AutoScaling ScheduledAction resource.
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
///     let dynamodb = target::create(
///         "dynamodb",
///         TargetArgs::builder()
///             .max_capacity(100)
///             .min_capacity(5)
///             .resource_id("table/tableName")
///             .scalable_dimension("dynamodb:table:ReadCapacityUnits")
///             .service_namespace("dynamodb")
///             .build_struct(),
///     );
///     let dynamodbScheduledAction = scheduled_action::create(
///         "dynamodbScheduledAction",
///         ScheduledActionArgs::builder()
///             .name("dynamodb")
///             .resource_id("${dynamodb.resourceId}")
///             .scalable_dimension("${dynamodb.scalableDimension}")
///             .scalable_target_action(
///                 ScheduledActionScalableTargetAction::builder()
///                     .maxCapacity(200)
///                     .minCapacity(1)
///                     .build_struct(),
///             )
///             .schedule("at(2006-01-02T15:04:05)")
///             .service_namespace("${dynamodb.serviceNamespace}")
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
///     let ecs = target::create(
///         "ecs",
///         TargetArgs::builder()
///             .max_capacity(4)
///             .min_capacity(1)
///             .resource_id("service/clusterName/serviceName")
///             .scalable_dimension("ecs:service:DesiredCount")
///             .service_namespace("ecs")
///             .build_struct(),
///     );
///     let ecsScheduledAction = scheduled_action::create(
///         "ecsScheduledAction",
///         ScheduledActionArgs::builder()
///             .name("ecs")
///             .resource_id("${ecs.resourceId}")
///             .scalable_dimension("${ecs.scalableDimension}")
///             .scalable_target_action(
///                 ScheduledActionScalableTargetAction::builder()
///                     .maxCapacity(10)
///                     .minCapacity(1)
///                     .build_struct(),
///             )
///             .schedule("at(2006-01-02T15:04:05)")
///             .service_namespace("${ecs.serviceNamespace}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod scheduled_action {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduledActionArgs {
        /// Date and time for the scheduled action to end in RFC 3339 format. The timezone is not affected by the setting of `timezone`.
        #[builder(into, default)]
        pub end_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the scheduled action.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the resource associated with the scheduled action. Documentation can be found in the `ResourceId` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html)
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Scalable dimension. Documentation can be found in the `ScalableDimension` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html) Example: ecs:service:DesiredCount
        #[builder(into)]
        pub scalable_dimension: pulumi_gestalt_rust::InputOrOutput<String>,
        /// New minimum and maximum capacity. You can set both values or just one. See below
        #[builder(into)]
        pub scalable_target_action: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appautoscaling::ScheduledActionScalableTargetAction,
        >,
        /// Schedule for this action. The following formats are supported: At expressions - at(yyyy-mm-ddThh:mm:ss), Rate expressions - rate(valueunit), Cron expressions - cron(fields). Times for at expressions and cron expressions are evaluated using the time zone configured in `timezone`. Documentation can be found in the `Timezone` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html)
        #[builder(into)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Namespace of the AWS service. Documentation can be found in the `ServiceNamespace` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html) Example: ecs
        #[builder(into)]
        pub service_namespace: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Date and time for the scheduled action to start in RFC 3339 format. The timezone is not affected by the setting of `timezone`.
        #[builder(into, default)]
        pub start_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Time zone used when setting a scheduled action by using an at or cron expression. Does not affect timezone for `start_time` and `end_time`. Valid values are the [canonical names of the IANA time zones supported by Joda-Time](https://www.joda.org/joda-time/timezones.html), such as `Etc/GMT+9` or `Pacific/Tahiti`. Default is `UTC`.
        #[builder(into, default)]
        pub timezone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ScheduledActionResult {
        /// ARN of the scheduled action.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date and time for the scheduled action to end in RFC 3339 format. The timezone is not affected by the setting of `timezone`.
        pub end_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the scheduled action.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the resource associated with the scheduled action. Documentation can be found in the `ResourceId` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html)
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// Scalable dimension. Documentation can be found in the `ScalableDimension` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html) Example: ecs:service:DesiredCount
        pub scalable_dimension: pulumi_gestalt_rust::Output<String>,
        /// New minimum and maximum capacity. You can set both values or just one. See below
        pub scalable_target_action: pulumi_gestalt_rust::Output<
            super::super::types::appautoscaling::ScheduledActionScalableTargetAction,
        >,
        /// Schedule for this action. The following formats are supported: At expressions - at(yyyy-mm-ddThh:mm:ss), Rate expressions - rate(valueunit), Cron expressions - cron(fields). Times for at expressions and cron expressions are evaluated using the time zone configured in `timezone`. Documentation can be found in the `Timezone` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html)
        pub schedule: pulumi_gestalt_rust::Output<String>,
        /// Namespace of the AWS service. Documentation can be found in the `ServiceNamespace` parameter at: [AWS Application Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/application/APIReference/API_PutScheduledAction.html) Example: ecs
        pub service_namespace: pulumi_gestalt_rust::Output<String>,
        /// Date and time for the scheduled action to start in RFC 3339 format. The timezone is not affected by the setting of `timezone`.
        pub start_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// Time zone used when setting a scheduled action by using an at or cron expression. Does not affect timezone for `start_time` and `end_time`. Valid values are the [canonical names of the IANA time zones supported by Joda-Time](https://www.joda.org/joda-time/timezones.html), such as `Etc/GMT+9` or `Pacific/Tahiti`. Default is `UTC`.
        pub timezone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ScheduledActionArgs,
    ) -> ScheduledActionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let end_time_binding = args.end_time.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_id_binding = args.resource_id.get_output(context).get_inner();
        let scalable_dimension_binding = args
            .scalable_dimension
            .get_output(context)
            .get_inner();
        let scalable_target_action_binding = args
            .scalable_target_action
            .get_output(context)
            .get_inner();
        let schedule_binding = args.schedule.get_output(context).get_inner();
        let service_namespace_binding = args
            .service_namespace
            .get_output(context)
            .get_inner();
        let start_time_binding = args.start_time.get_output(context).get_inner();
        let timezone_binding = args.timezone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appautoscaling/scheduledAction:ScheduledAction".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "endTime".into(),
                    value: &end_time_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "scalableTargetAction".into(),
                    value: &scalable_target_action_binding,
                },
                register_interface::ObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "serviceNamespace".into(),
                    value: &service_namespace_binding,
                },
                register_interface::ObjectField {
                    name: "startTime".into(),
                    value: &start_time_binding,
                },
                register_interface::ObjectField {
                    name: "timezone".into(),
                    value: &timezone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ScheduledActionResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            end_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endTime"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
            scalable_dimension: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scalableDimension"),
            ),
            scalable_target_action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scalableTargetAction"),
            ),
            schedule: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schedule"),
            ),
            service_namespace: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceNamespace"),
            ),
            start_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("startTime"),
            ),
            timezone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timezone"),
            ),
        }
    }
}
