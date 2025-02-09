/// Manages a Automation Schedule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("tfex-automation-account")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .location("${example.location}")
///             .name("tfex-automation-account")
///             .resource_group_name("${example.name}")
///             .sku_name("Basic")
///             .build_struct(),
///     );
///     let exampleSchedule = schedule::create(
///         "exampleSchedule",
///         ScheduleArgs::builder()
///             .automation_account_name("${exampleAccount.name}")
///             .description("This is an example schedule")
///             .frequency("Week")
///             .interval(1)
///             .name("tfex-automation-schedule")
///             .resource_group_name("${example.name}")
///             .start_time("2014-04-15T18:00:15+02:00")
///             .timezone("Australia/Perth")
///             .week_days(vec!["Friday",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Automation Schedule can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/schedule:Schedule schedule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/schedules/schedule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod schedule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduleArgs {
        /// The name of the automation account in which the Schedule is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description for this Schedule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The end time of the schedule.
        #[builder(into, default)]
        pub expiry_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The frequency of the schedule. - can be either `OneTime`, `Day`, `Hour`, `Week`, or `Month`.
        #[builder(into)]
        pub frequency: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of `frequency`s between runs. Only valid when frequency is `Day`, `Hour`, `Week`, or `Month` and defaults to `1`.
        #[builder(into, default)]
        pub interval: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// List of days of the month that the job should execute on. Must be between `1` and `31`. `-1` for last day of the month. Only valid when frequency is `Month`.
        #[builder(into, default)]
        pub month_days: pulumi_gestalt_rust::InputOrOutput<Option<Vec<i32>>>,
        /// One `monthly_occurrence` blocks as defined below to specifies occurrences of days within a month. Only valid when frequency is `Month`. The `monthly_occurrence` block supports fields documented below.
        #[builder(into, default)]
        pub monthly_occurrence: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::automation::ScheduleMonthlyOccurrence>,
        >,
        /// Specifies the name of the Schedule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Schedule is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Start time of the schedule. Must be at least five minutes in the future. Defaults to seven minutes in the future from the time the resource is created.
        #[builder(into, default)]
        pub start_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The timezone of the start time. Defaults to `Etc/UTC`. For possible values see: <https://docs.microsoft.com/en-us/rest/api/maps/timezone/gettimezoneenumwindows>
        #[builder(into, default)]
        pub timezone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of days of the week that the job should execute on. Only valid when frequency is `Week`. Possible values are `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday` and `Sunday`.
        #[builder(into, default)]
        pub week_days: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ScheduleResult {
        /// The name of the automation account in which the Schedule is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        /// A description for this Schedule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The end time of the schedule.
        pub expiry_time: pulumi_gestalt_rust::Output<String>,
        /// The frequency of the schedule. - can be either `OneTime`, `Day`, `Hour`, `Week`, or `Month`.
        pub frequency: pulumi_gestalt_rust::Output<String>,
        /// The number of `frequency`s between runs. Only valid when frequency is `Day`, `Hour`, `Week`, or `Month` and defaults to `1`.
        pub interval: pulumi_gestalt_rust::Output<i32>,
        /// List of days of the month that the job should execute on. Must be between `1` and `31`. `-1` for last day of the month. Only valid when frequency is `Month`.
        pub month_days: pulumi_gestalt_rust::Output<Option<Vec<i32>>>,
        /// One `monthly_occurrence` blocks as defined below to specifies occurrences of days within a month. Only valid when frequency is `Month`. The `monthly_occurrence` block supports fields documented below.
        pub monthly_occurrence: pulumi_gestalt_rust::Output<
            Option<super::super::types::automation::ScheduleMonthlyOccurrence>,
        >,
        /// Specifies the name of the Schedule. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the Schedule is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Start time of the schedule. Must be at least five minutes in the future. Defaults to seven minutes in the future from the time the resource is created.
        pub start_time: pulumi_gestalt_rust::Output<String>,
        /// The timezone of the start time. Defaults to `Etc/UTC`. For possible values see: <https://docs.microsoft.com/en-us/rest/api/maps/timezone/gettimezoneenumwindows>
        pub timezone: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of days of the week that the job should execute on. Only valid when frequency is `Week`. Possible values are `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday` and `Sunday`.
        pub week_days: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ScheduleArgs,
    ) -> ScheduleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding_1 = args
            .automation_account_name
            .get_output(context);
        let automation_account_name_binding = automation_account_name_binding_1
            .get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let expiry_time_binding_1 = args.expiry_time.get_output(context);
        let expiry_time_binding = expiry_time_binding_1.get_inner();
        let frequency_binding_1 = args.frequency.get_output(context);
        let frequency_binding = frequency_binding_1.get_inner();
        let interval_binding_1 = args.interval.get_output(context);
        let interval_binding = interval_binding_1.get_inner();
        let month_days_binding_1 = args.month_days.get_output(context);
        let month_days_binding = month_days_binding_1.get_inner();
        let monthly_occurrence_binding_1 = args.monthly_occurrence.get_output(context);
        let monthly_occurrence_binding = monthly_occurrence_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let start_time_binding_1 = args.start_time.get_output(context);
        let start_time_binding = start_time_binding_1.get_inner();
        let timezone_binding_1 = args.timezone.get_output(context);
        let timezone_binding = timezone_binding_1.get_inner();
        let week_days_binding_1 = args.week_days.get_output(context);
        let week_days_binding = week_days_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/schedule:Schedule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "expiryTime".into(),
                    value: &expiry_time_binding,
                },
                register_interface::ObjectField {
                    name: "frequency".into(),
                    value: &frequency_binding,
                },
                register_interface::ObjectField {
                    name: "interval".into(),
                    value: &interval_binding,
                },
                register_interface::ObjectField {
                    name: "monthDays".into(),
                    value: &month_days_binding,
                },
                register_interface::ObjectField {
                    name: "monthlyOccurrence".into(),
                    value: &monthly_occurrence_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "startTime".into(),
                    value: &start_time_binding,
                },
                register_interface::ObjectField {
                    name: "timezone".into(),
                    value: &timezone_binding,
                },
                register_interface::ObjectField {
                    name: "weekDays".into(),
                    value: &week_days_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ScheduleResult {
            automation_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automationAccountName"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            expiry_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expiryTime"),
            ),
            frequency: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frequency"),
            ),
            interval: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("interval"),
            ),
            month_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monthDays"),
            ),
            monthly_occurrence: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monthlyOccurrence"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            start_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("startTime"),
            ),
            timezone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timezone"),
            ),
            week_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("weekDays"),
            ),
        }
    }
}
