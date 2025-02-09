/// Provides an EventBridge Scheduler Schedule resource.
///
/// You can find out more about EventBridge Scheduler in the [User Guide](https://docs.aws.amazon.com/scheduler/latest/UserGuide/what-is-scheduler.html).
///
/// > **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = schedule::create(
///         "example",
///         ScheduleArgs::builder()
///             .flexible_time_window(
///                 ScheduleFlexibleTimeWindow::builder().mode("OFF").build_struct(),
///             )
///             .group_name("default")
///             .name("my-schedule")
///             .schedule_expression("rate(1 hours)")
///             .target(
///                 ScheduleTarget::builder()
///                     .arn("${exampleAwsSqsQueue.arn}")
///                     .roleArn("${exampleAwsIamRole.arn}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Universal Target
///
/// ```yaml
/// resources:
///   example:
///     type: aws:sqs:Queue
///   exampleSchedule:
///     type: aws:scheduler:Schedule
///     name: example
///     properties:
///       name: my-schedule
///       flexibleTimeWindow:
///         mode: OFF
///       scheduleExpression: rate(1 hours)
///       target:
///         arn: arn:aws:scheduler:::aws-sdk:sqs:sendMessage
///         roleArn: ${exampleAwsIamRole.arn}
///         input:
///           fn::toJSON:
///             MessageBody: Greetings, programs!
///             QueueUrl: ${example.url}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import schedules using the combination `group_name/name`. For example:
///
/// ```sh
/// $ pulumi import aws:scheduler/schedule:Schedule example my-schedule-group/my-schedule
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod schedule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduleArgs {
        /// Brief description of the schedule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The date, in UTC, before which the schedule can invoke its target. Depending on the schedule's recurrence expression, invocations might stop on, or before, the end date you specify. EventBridge Scheduler ignores the end date for one-time schedules. Example: `2030-01-01T01:00:00Z`.
        #[builder(into, default)]
        pub end_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configures a time window during which EventBridge Scheduler invokes the schedule. Detailed below.
        #[builder(into)]
        pub flexible_time_window: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::scheduler::ScheduleFlexibleTimeWindow,
        >,
        /// Name of the schedule group to associate with this schedule. When omitted, the `default` schedule group is used.
        #[builder(into, default)]
        pub group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN for the customer managed KMS key that EventBridge Scheduler will use to encrypt and decrypt your data.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the schedule. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Defines when the schedule runs. Read more in [Schedule types on EventBridge Scheduler](https://docs.aws.amazon.com/scheduler/latest/UserGuide/schedule-types.html).
        #[builder(into)]
        pub schedule_expression: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Timezone in which the scheduling expression is evaluated. Defaults to `UTC`. Example: `Australia/Sydney`.
        #[builder(into, default)]
        pub schedule_expression_timezone: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The date, in UTC, after which the schedule can begin invoking its target. Depending on the schedule's recurrence expression, invocations might occur on, or after, the start date you specify. EventBridge Scheduler ignores the start date for one-time schedules. Example: `2030-01-01T01:00:00Z`.
        #[builder(into, default)]
        pub start_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the schedule is enabled or disabled. One of: `ENABLED` (default), `DISABLED`.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configures the target of the schedule. Detailed below.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::scheduler::ScheduleTarget,
        >,
    }
    #[allow(dead_code)]
    pub struct ScheduleResult {
        /// ARN of the schedule.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Brief description of the schedule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The date, in UTC, before which the schedule can invoke its target. Depending on the schedule's recurrence expression, invocations might stop on, or before, the end date you specify. EventBridge Scheduler ignores the end date for one-time schedules. Example: `2030-01-01T01:00:00Z`.
        pub end_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configures a time window during which EventBridge Scheduler invokes the schedule. Detailed below.
        pub flexible_time_window: pulumi_gestalt_rust::Output<
            super::super::types::scheduler::ScheduleFlexibleTimeWindow,
        >,
        /// Name of the schedule group to associate with this schedule. When omitted, the `default` schedule group is used.
        pub group_name: pulumi_gestalt_rust::Output<String>,
        /// ARN for the customer managed KMS key that EventBridge Scheduler will use to encrypt and decrypt your data.
        pub kms_key_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the schedule. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// Defines when the schedule runs. Read more in [Schedule types on EventBridge Scheduler](https://docs.aws.amazon.com/scheduler/latest/UserGuide/schedule-types.html).
        pub schedule_expression: pulumi_gestalt_rust::Output<String>,
        /// Timezone in which the scheduling expression is evaluated. Defaults to `UTC`. Example: `Australia/Sydney`.
        pub schedule_expression_timezone: pulumi_gestalt_rust::Output<Option<String>>,
        /// The date, in UTC, after which the schedule can begin invoking its target. Depending on the schedule's recurrence expression, invocations might occur on, or after, the start date you specify. EventBridge Scheduler ignores the start date for one-time schedules. Example: `2030-01-01T01:00:00Z`.
        pub start_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether the schedule is enabled or disabled. One of: `ENABLED` (default), `DISABLED`.
        pub state: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configures the target of the schedule. Detailed below.
        ///
        /// The following arguments are optional:
        pub target: pulumi_gestalt_rust::Output<
            super::super::types::scheduler::ScheduleTarget,
        >,
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
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let end_date_binding = args.end_date.get_output(context);
        let flexible_time_window_binding = args.flexible_time_window.get_output(context);
        let group_name_binding = args.group_name.get_output(context);
        let kms_key_arn_binding = args.kms_key_arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let schedule_expression_binding = args.schedule_expression.get_output(context);
        let schedule_expression_timezone_binding = args
            .schedule_expression_timezone
            .get_output(context);
        let start_date_binding = args.start_date.get_output(context);
        let state_binding = args.state.get_output(context);
        let target_binding = args.target.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:scheduler/schedule:Schedule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endDate".into(),
                    value: end_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "flexibleTimeWindow".into(),
                    value: flexible_time_window_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupName".into(),
                    value: group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyArn".into(),
                    value: kms_key_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduleExpression".into(),
                    value: schedule_expression_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduleExpressionTimezone".into(),
                    value: schedule_expression_timezone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startDate".into(),
                    value: start_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "target".into(),
                    value: target_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ScheduleResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            end_date: o.get_field("endDate"),
            flexible_time_window: o.get_field("flexibleTimeWindow"),
            group_name: o.get_field("groupName"),
            kms_key_arn: o.get_field("kmsKeyArn"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            schedule_expression: o.get_field("scheduleExpression"),
            schedule_expression_timezone: o.get_field("scheduleExpressionTimezone"),
            start_date: o.get_field("startDate"),
            state: o.get_field("state"),
            target: o.get_field("target"),
        }
    }
}
