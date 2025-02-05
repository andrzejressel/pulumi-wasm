/// Manages automated startup and shutdown schedules for Azure Dev Test Lab.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleLab:
///     type: azure:devtest:Lab
///     name: example
///     properties:
///       name: YourDevTestLab
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSchedule:
///     type: azure:devtest:Schedule
///     name: example
///     properties:
///       name: LabVmAutoStart
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       labName: ${exampleLab.name}
///       status: Enabled
///       weeklyRecurrence:
///         time: '1100'
///         weekDays:
///           - Monday
///           - Tuesday
///       timeZoneId: Pacific Standard Time
///       taskType: LabVmsStartupTask
///       notificationSettings: {}
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// DevTest Schedule's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devtest/schedule:Schedule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.DevTestLab/labs/myDevTestLab/schedules/labvmautostart
/// ```
///
pub mod schedule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduleArgs {
        /// The properties of a daily schedule. If the schedule occurs once each day of the week, specify the daily recurrence. A `daily_recurrence` block as defined below.
        #[builder(into, default)]
        pub daily_recurrence: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::devtest::ScheduleDailyRecurrence>,
        >,
        /// The properties of an hourly schedule. If the schedule occurs multiple times a day, specify the hourly recurrence. A `hourly_recurrence` block as defined below.
        #[builder(into, default)]
        pub hourly_recurrence: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::devtest::ScheduleHourlyRecurrence>,
        >,
        /// The name of the dev test lab. Changing this forces a new resource to be created.
        #[builder(into)]
        pub lab_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The location where the schedule is created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the dev test lab schedule. Valid value for name depends on the `task_type`. For instance for task_type `LabVmsStartupTask` the name needs to be `LabVmAutoStart`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The notification setting of a schedule. A `notification_settings` block as defined below.
        #[builder(into)]
        pub notification_settings: pulumi_wasm_rust::InputOrOutput<
            super::super::types::devtest::ScheduleNotificationSettings,
        >,
        /// The name of the resource group in which to create the dev test lab schedule. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The status of this schedule. Possible values are `Enabled` and `Disabled`. Defaults to `Disabled`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The task type of the schedule. Possible values include `LabVmsShutdownTask` and `LabVmAutoStart`.
        #[builder(into)]
        pub task_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The time zone ID (e.g. Pacific Standard time).
        #[builder(into)]
        pub time_zone_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The properties of a weekly schedule. If the schedule occurs only some days of the week, specify the weekly recurrence. A `weekly_recurrence` block as defined below.
        #[builder(into, default)]
        pub weekly_recurrence: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::devtest::ScheduleWeeklyRecurrence>,
        >,
    }
    #[allow(dead_code)]
    pub struct ScheduleResult {
        /// The properties of a daily schedule. If the schedule occurs once each day of the week, specify the daily recurrence. A `daily_recurrence` block as defined below.
        pub daily_recurrence: pulumi_wasm_rust::Output<
            Option<super::super::types::devtest::ScheduleDailyRecurrence>,
        >,
        /// The properties of an hourly schedule. If the schedule occurs multiple times a day, specify the hourly recurrence. A `hourly_recurrence` block as defined below.
        pub hourly_recurrence: pulumi_wasm_rust::Output<
            Option<super::super::types::devtest::ScheduleHourlyRecurrence>,
        >,
        /// The name of the dev test lab. Changing this forces a new resource to be created.
        pub lab_name: pulumi_wasm_rust::Output<String>,
        /// The location where the schedule is created. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the dev test lab schedule. Valid value for name depends on the `task_type`. For instance for task_type `LabVmsStartupTask` the name needs to be `LabVmAutoStart`. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The notification setting of a schedule. A `notification_settings` block as defined below.
        pub notification_settings: pulumi_wasm_rust::Output<
            super::super::types::devtest::ScheduleNotificationSettings,
        >,
        /// The name of the resource group in which to create the dev test lab schedule. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The status of this schedule. Possible values are `Enabled` and `Disabled`. Defaults to `Disabled`.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The task type of the schedule. Possible values include `LabVmsShutdownTask` and `LabVmAutoStart`.
        pub task_type: pulumi_wasm_rust::Output<String>,
        /// The time zone ID (e.g. Pacific Standard time).
        pub time_zone_id: pulumi_wasm_rust::Output<String>,
        /// The properties of a weekly schedule. If the schedule occurs only some days of the week, specify the weekly recurrence. A `weekly_recurrence` block as defined below.
        pub weekly_recurrence: pulumi_wasm_rust::Output<
            Option<super::super::types::devtest::ScheduleWeeklyRecurrence>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ScheduleArgs,
    ) -> ScheduleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let daily_recurrence_binding = args
            .daily_recurrence
            .get_output(context)
            .get_inner();
        let hourly_recurrence_binding = args
            .hourly_recurrence
            .get_output(context)
            .get_inner();
        let lab_name_binding = args.lab_name.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let notification_settings_binding = args
            .notification_settings
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let status_binding = args.status.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let task_type_binding = args.task_type.get_output(context).get_inner();
        let time_zone_id_binding = args.time_zone_id.get_output(context).get_inner();
        let weekly_recurrence_binding = args
            .weekly_recurrence
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:devtest/schedule:Schedule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dailyRecurrence".into(),
                    value: &daily_recurrence_binding,
                },
                register_interface::ObjectField {
                    name: "hourlyRecurrence".into(),
                    value: &hourly_recurrence_binding,
                },
                register_interface::ObjectField {
                    name: "labName".into(),
                    value: &lab_name_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notificationSettings".into(),
                    value: &notification_settings_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "taskType".into(),
                    value: &task_type_binding,
                },
                register_interface::ObjectField {
                    name: "timeZoneId".into(),
                    value: &time_zone_id_binding,
                },
                register_interface::ObjectField {
                    name: "weeklyRecurrence".into(),
                    value: &weekly_recurrence_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ScheduleResult {
            daily_recurrence: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dailyRecurrence"),
            ),
            hourly_recurrence: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hourlyRecurrence"),
            ),
            lab_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("labName"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            notification_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("notificationSettings"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            task_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("taskType"),
            ),
            time_zone_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeZoneId"),
            ),
            weekly_recurrence: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("weeklyRecurrence"),
            ),
        }
    }
}
