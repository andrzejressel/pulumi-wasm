/// Manages a Automation Schedule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod schedule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduleArgs {
        /// The name of the automation account in which the Schedule is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// A description for this Schedule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The end time of the schedule.
        #[builder(into, default)]
        pub expiry_time: pulumi_wasm_rust::Output<Option<String>>,
        /// The frequency of the schedule. - can be either `OneTime`, `Day`, `Hour`, `Week`, or `Month`.
        #[builder(into)]
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// The number of `frequency`s between runs. Only valid when frequency is `Day`, `Hour`, `Week`, or `Month` and defaults to `1`.
        #[builder(into, default)]
        pub interval: pulumi_wasm_rust::Output<Option<i32>>,
        /// List of days of the month that the job should execute on. Must be between `1` and `31`. `-1` for last day of the month. Only valid when frequency is `Month`.
        #[builder(into, default)]
        pub month_days: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
        /// One `monthly_occurrence` blocks as defined below to specifies occurrences of days within a month. Only valid when frequency is `Month`. The `monthly_occurrence` block supports fields documented below.
        #[builder(into, default)]
        pub monthly_occurrence: pulumi_wasm_rust::Output<
            Option<super::super::types::automation::ScheduleMonthlyOccurrence>,
        >,
        /// Specifies the name of the Schedule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the Schedule is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Start time of the schedule. Must be at least five minutes in the future. Defaults to seven minutes in the future from the time the resource is created.
        #[builder(into, default)]
        pub start_time: pulumi_wasm_rust::Output<Option<String>>,
        /// The timezone of the start time. Defaults to `Etc/UTC`. For possible values see: <https://docs.microsoft.com/en-us/rest/api/maps/timezone/gettimezoneenumwindows>
        #[builder(into, default)]
        pub timezone: pulumi_wasm_rust::Output<Option<String>>,
        /// List of days of the week that the job should execute on. Only valid when frequency is `Week`. Possible values are `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday` and `Sunday`.
        #[builder(into, default)]
        pub week_days: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ScheduleResult {
        /// The name of the automation account in which the Schedule is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// A description for this Schedule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The end time of the schedule.
        pub expiry_time: pulumi_wasm_rust::Output<String>,
        /// The frequency of the schedule. - can be either `OneTime`, `Day`, `Hour`, `Week`, or `Month`.
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// The number of `frequency`s between runs. Only valid when frequency is `Day`, `Hour`, `Week`, or `Month` and defaults to `1`.
        pub interval: pulumi_wasm_rust::Output<i32>,
        /// List of days of the month that the job should execute on. Must be between `1` and `31`. `-1` for last day of the month. Only valid when frequency is `Month`.
        pub month_days: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
        /// One `monthly_occurrence` blocks as defined below to specifies occurrences of days within a month. Only valid when frequency is `Month`. The `monthly_occurrence` block supports fields documented below.
        pub monthly_occurrence: pulumi_wasm_rust::Output<
            Option<super::super::types::automation::ScheduleMonthlyOccurrence>,
        >,
        /// Specifies the name of the Schedule. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Schedule is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Start time of the schedule. Must be at least five minutes in the future. Defaults to seven minutes in the future from the time the resource is created.
        pub start_time: pulumi_wasm_rust::Output<String>,
        /// The timezone of the start time. Defaults to `Etc/UTC`. For possible values see: <https://docs.microsoft.com/en-us/rest/api/maps/timezone/gettimezoneenumwindows>
        pub timezone: pulumi_wasm_rust::Output<Option<String>>,
        /// List of days of the week that the job should execute on. Only valid when frequency is `Week`. Possible values are `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday` and `Sunday`.
        pub week_days: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ScheduleArgs) -> ScheduleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding = args.automation_account_name.get_inner();
        let description_binding = args.description.get_inner();
        let expiry_time_binding = args.expiry_time.get_inner();
        let frequency_binding = args.frequency.get_inner();
        let interval_binding = args.interval.get_inner();
        let month_days_binding = args.month_days.get_inner();
        let monthly_occurrence_binding = args.monthly_occurrence.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let start_time_binding = args.start_time.get_inner();
        let timezone_binding = args.timezone.get_inner();
        let week_days_binding = args.week_days.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/schedule:Schedule".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "automationAccountName".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "expiryTime".into(),
                },
                register_interface::ResultField {
                    name: "frequency".into(),
                },
                register_interface::ResultField {
                    name: "interval".into(),
                },
                register_interface::ResultField {
                    name: "monthDays".into(),
                },
                register_interface::ResultField {
                    name: "monthlyOccurrence".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "startTime".into(),
                },
                register_interface::ResultField {
                    name: "timezone".into(),
                },
                register_interface::ResultField {
                    name: "weekDays".into(),
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
            automation_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automationAccountName").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            expiry_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiryTime").unwrap(),
            ),
            frequency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frequency").unwrap(),
            ),
            interval: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interval").unwrap(),
            ),
            month_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monthDays").unwrap(),
            ),
            monthly_occurrence: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monthlyOccurrence").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startTime").unwrap(),
            ),
            timezone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timezone").unwrap(),
            ),
            week_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("weekDays").unwrap(),
            ),
        }
    }
}
