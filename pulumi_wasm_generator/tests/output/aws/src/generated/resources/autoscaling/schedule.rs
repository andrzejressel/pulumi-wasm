/// Provides an AutoScaling Schedule resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod schedule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduleArgs {
        /// The name of the Auto Scaling group.
        #[builder(into)]
        pub autoscaling_group_name: pulumi_wasm_rust::Output<String>,
        /// The initial capacity of the Auto Scaling group after the scheduled action runs and the capacity it attempts to maintain. Set to `-1` if you don't want to change the desired capacity at the scheduled time. Defaults to `0`.
        #[builder(into, default)]
        pub desired_capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// The date and time for the recurring schedule to end, in UTC with the format `"YYYY-MM-DDThh:mm:ssZ"` (e.g. `"2021-06-01T00:00:00Z"`).
        #[builder(into, default)]
        pub end_time: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum size of the Auto Scaling group. Set to `-1` if you don't want to change the maximum size at the scheduled time. Defaults to `0`.
        #[builder(into, default)]
        pub max_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// The minimum size of the Auto Scaling group. Set to `-1` if you don't want to change the minimum size at the scheduled time. Defaults to `0`.
        #[builder(into, default)]
        pub min_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// The recurring schedule for this action specified using the Unix cron syntax format.
        #[builder(into, default)]
        pub recurrence: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of this scaling action.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub scheduled_action_name: pulumi_wasm_rust::Output<String>,
        /// The date and time for the recurring schedule to start, in UTC with the format `"YYYY-MM-DDThh:mm:ssZ"` (e.g. `"2021-06-01T00:00:00Z"`).
        #[builder(into, default)]
        pub start_time: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the time zone for a cron expression. Valid values are the canonical names of the IANA time zones (such as `Etc/GMT+9` or `Pacific/Tahiti`).
        ///
        /// > **NOTE:** When `start_time` and `end_time` are specified with `recurrence` , they form the boundaries of when the recurring action will start and stop.
        #[builder(into, default)]
        pub time_zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ScheduleResult {
        /// ARN assigned by AWS to the autoscaling schedule.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the Auto Scaling group.
        pub autoscaling_group_name: pulumi_wasm_rust::Output<String>,
        /// The initial capacity of the Auto Scaling group after the scheduled action runs and the capacity it attempts to maintain. Set to `-1` if you don't want to change the desired capacity at the scheduled time. Defaults to `0`.
        pub desired_capacity: pulumi_wasm_rust::Output<i32>,
        /// The date and time for the recurring schedule to end, in UTC with the format `"YYYY-MM-DDThh:mm:ssZ"` (e.g. `"2021-06-01T00:00:00Z"`).
        pub end_time: pulumi_wasm_rust::Output<String>,
        /// The maximum size of the Auto Scaling group. Set to `-1` if you don't want to change the maximum size at the scheduled time. Defaults to `0`.
        pub max_size: pulumi_wasm_rust::Output<i32>,
        /// The minimum size of the Auto Scaling group. Set to `-1` if you don't want to change the minimum size at the scheduled time. Defaults to `0`.
        pub min_size: pulumi_wasm_rust::Output<i32>,
        /// The recurring schedule for this action specified using the Unix cron syntax format.
        pub recurrence: pulumi_wasm_rust::Output<String>,
        /// The name of this scaling action.
        ///
        /// The following arguments are optional:
        pub scheduled_action_name: pulumi_wasm_rust::Output<String>,
        /// The date and time for the recurring schedule to start, in UTC with the format `"YYYY-MM-DDThh:mm:ssZ"` (e.g. `"2021-06-01T00:00:00Z"`).
        pub start_time: pulumi_wasm_rust::Output<String>,
        /// Specifies the time zone for a cron expression. Valid values are the canonical names of the IANA time zones (such as `Etc/GMT+9` or `Pacific/Tahiti`).
        ///
        /// > **NOTE:** When `start_time` and `end_time` are specified with `recurrence` , they form the boundaries of when the recurring action will start and stop.
        pub time_zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ScheduleArgs) -> ScheduleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let autoscaling_group_name_binding = args.autoscaling_group_name.get_inner();
        let desired_capacity_binding = args.desired_capacity.get_inner();
        let end_time_binding = args.end_time.get_inner();
        let max_size_binding = args.max_size.get_inner();
        let min_size_binding = args.min_size.get_inner();
        let recurrence_binding = args.recurrence.get_inner();
        let scheduled_action_name_binding = args.scheduled_action_name.get_inner();
        let start_time_binding = args.start_time.get_inner();
        let time_zone_binding = args.time_zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:autoscaling/schedule:Schedule".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoscalingGroupName".into(),
                    value: &autoscaling_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "desiredCapacity".into(),
                    value: &desired_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "endTime".into(),
                    value: &end_time_binding,
                },
                register_interface::ObjectField {
                    name: "maxSize".into(),
                    value: &max_size_binding,
                },
                register_interface::ObjectField {
                    name: "minSize".into(),
                    value: &min_size_binding,
                },
                register_interface::ObjectField {
                    name: "recurrence".into(),
                    value: &recurrence_binding,
                },
                register_interface::ObjectField {
                    name: "scheduledActionName".into(),
                    value: &scheduled_action_name_binding,
                },
                register_interface::ObjectField {
                    name: "startTime".into(),
                    value: &start_time_binding,
                },
                register_interface::ObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoscalingGroupName".into(),
                },
                register_interface::ResultField {
                    name: "desiredCapacity".into(),
                },
                register_interface::ResultField {
                    name: "endTime".into(),
                },
                register_interface::ResultField {
                    name: "maxSize".into(),
                },
                register_interface::ResultField {
                    name: "minSize".into(),
                },
                register_interface::ResultField {
                    name: "recurrence".into(),
                },
                register_interface::ResultField {
                    name: "scheduledActionName".into(),
                },
                register_interface::ResultField {
                    name: "startTime".into(),
                },
                register_interface::ResultField {
                    name: "timeZone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ScheduleResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            autoscaling_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscalingGroupName").unwrap(),
            ),
            desired_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desiredCapacity").unwrap(),
            ),
            end_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endTime").unwrap(),
            ),
            max_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxSize").unwrap(),
            ),
            min_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minSize").unwrap(),
            ),
            recurrence: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recurrence").unwrap(),
            ),
            scheduled_action_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scheduledActionName").unwrap(),
            ),
            start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startTime").unwrap(),
            ),
            time_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeZone").unwrap(),
            ),
        }
    }
}