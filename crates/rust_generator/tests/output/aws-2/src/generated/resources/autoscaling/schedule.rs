/// Provides an AutoScaling Schedule resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foobar = group::create(
///         "foobar",
///         GroupArgs::builder()
///             .availability_zones(vec!["us-west-2a",])
///             .force_delete(true)
///             .health_check_grace_period(300)
///             .health_check_type("ELB")
///             .max_size(1)
///             .min_size(1)
///             .name("test-foobar5")
///             .termination_policies(vec!["OldestInstance",])
///             .build_struct(),
///     );
///     let foobarSchedule = schedule::create(
///         "foobarSchedule",
///         ScheduleArgs::builder()
///             .autoscaling_group_name("${foobar.name}")
///             .desired_capacity(0)
///             .end_time("2016-12-12T06:00:00Z")
///             .max_size(1)
///             .min_size(0)
///             .scheduled_action_name("foobar")
///             .start_time("2016-12-11T18:00:00Z")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AutoScaling ScheduledAction using the `auto-scaling-group-name` and `scheduled-action-name`. For example:
///
/// ```sh
/// $ pulumi import aws:autoscaling/schedule:Schedule resource-name auto-scaling-group-name/scheduled-action-name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod schedule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduleArgs {
        /// The name of the Auto Scaling group.
        #[builder(into)]
        pub autoscaling_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The initial capacity of the Auto Scaling group after the scheduled action runs and the capacity it attempts to maintain. Set to `-1` if you don't want to change the desired capacity at the scheduled time. Defaults to `0`.
        #[builder(into, default)]
        pub desired_capacity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The date and time for the recurring schedule to end, in UTC with the format `"YYYY-MM-DDThh:mm:ssZ"` (e.g. `"2021-06-01T00:00:00Z"`).
        #[builder(into, default)]
        pub end_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum size of the Auto Scaling group. Set to `-1` if you don't want to change the maximum size at the scheduled time. Defaults to `0`.
        #[builder(into, default)]
        pub max_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The minimum size of the Auto Scaling group. Set to `-1` if you don't want to change the minimum size at the scheduled time. Defaults to `0`.
        #[builder(into, default)]
        pub min_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The recurring schedule for this action specified using the Unix cron syntax format.
        #[builder(into, default)]
        pub recurrence: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of this scaling action.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub scheduled_action_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The date and time for the recurring schedule to start, in UTC with the format `"YYYY-MM-DDThh:mm:ssZ"` (e.g. `"2021-06-01T00:00:00Z"`).
        #[builder(into, default)]
        pub start_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the time zone for a cron expression. Valid values are the canonical names of the IANA time zones (such as `Etc/GMT+9` or `Pacific/Tahiti`).
        ///
        /// > **NOTE:** When `start_time` and `end_time` are specified with `recurrence` , they form the boundaries of when the recurring action will start and stop.
        #[builder(into, default)]
        pub time_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ScheduleResult {
        /// ARN assigned by AWS to the autoscaling schedule.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the Auto Scaling group.
        pub autoscaling_group_name: pulumi_gestalt_rust::Output<String>,
        /// The initial capacity of the Auto Scaling group after the scheduled action runs and the capacity it attempts to maintain. Set to `-1` if you don't want to change the desired capacity at the scheduled time. Defaults to `0`.
        pub desired_capacity: pulumi_gestalt_rust::Output<i32>,
        /// The date and time for the recurring schedule to end, in UTC with the format `"YYYY-MM-DDThh:mm:ssZ"` (e.g. `"2021-06-01T00:00:00Z"`).
        pub end_time: pulumi_gestalt_rust::Output<String>,
        /// The maximum size of the Auto Scaling group. Set to `-1` if you don't want to change the maximum size at the scheduled time. Defaults to `0`.
        pub max_size: pulumi_gestalt_rust::Output<i32>,
        /// The minimum size of the Auto Scaling group. Set to `-1` if you don't want to change the minimum size at the scheduled time. Defaults to `0`.
        pub min_size: pulumi_gestalt_rust::Output<i32>,
        /// The recurring schedule for this action specified using the Unix cron syntax format.
        pub recurrence: pulumi_gestalt_rust::Output<String>,
        /// The name of this scaling action.
        ///
        /// The following arguments are optional:
        pub scheduled_action_name: pulumi_gestalt_rust::Output<String>,
        /// The date and time for the recurring schedule to start, in UTC with the format `"YYYY-MM-DDThh:mm:ssZ"` (e.g. `"2021-06-01T00:00:00Z"`).
        pub start_time: pulumi_gestalt_rust::Output<String>,
        /// Specifies the time zone for a cron expression. Valid values are the canonical names of the IANA time zones (such as `Etc/GMT+9` or `Pacific/Tahiti`).
        ///
        /// > **NOTE:** When `start_time` and `end_time` are specified with `recurrence` , they form the boundaries of when the recurring action will start and stop.
        pub time_zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScheduleArgs,
    ) -> ScheduleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let autoscaling_group_name_binding = args
            .autoscaling_group_name
            .get_output(context);
        let desired_capacity_binding = args.desired_capacity.get_output(context);
        let end_time_binding = args.end_time.get_output(context);
        let max_size_binding = args.max_size.get_output(context);
        let min_size_binding = args.min_size.get_output(context);
        let recurrence_binding = args.recurrence.get_output(context);
        let scheduled_action_name_binding = args
            .scheduled_action_name
            .get_output(context);
        let start_time_binding = args.start_time.get_output(context);
        let time_zone_binding = args.time_zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:autoscaling/schedule:Schedule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscalingGroupName".into(),
                    value: &autoscaling_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "desiredCapacity".into(),
                    value: &desired_capacity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endTime".into(),
                    value: &end_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxSize".into(),
                    value: &max_size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minSize".into(),
                    value: &min_size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recurrence".into(),
                    value: &recurrence_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduledActionName".into(),
                    value: &scheduled_action_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startTime".into(),
                    value: &start_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ScheduleResult {
            arn: o.get_field("arn"),
            autoscaling_group_name: o.get_field("autoscalingGroupName"),
            desired_capacity: o.get_field("desiredCapacity"),
            end_time: o.get_field("endTime"),
            max_size: o.get_field("maxSize"),
            min_size: o.get_field("minSize"),
            recurrence: o.get_field("recurrence"),
            scheduled_action_name: o.get_field("scheduledActionName"),
            start_time: o.get_field("startTime"),
            time_zone: o.get_field("timeZone"),
        }
    }
}
