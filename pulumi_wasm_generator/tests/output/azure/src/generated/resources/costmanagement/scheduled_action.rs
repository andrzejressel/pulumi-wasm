/// Manages an Azure Cost Management Scheduled Action.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = scheduled_action::create(
///         "example",
///         ScheduledActionArgs::builder()
///             .display_name("Report Last 6 Months")
///             .email_address_sender("platformteam@test.com")
///             .email_addresses(vec!["example@example.com",])
///             .email_subject("Cost Management Report")
///             .end_date("2023-02-02T00:00:00Z")
///             .frequency("Daily")
///             .message("Hi all, take a look at last 6 months spending!")
///             .name("examplescheduledaction")
///             .start_date("2023-01-02T00:00:00Z")
///             .view_id(
///                 "/subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.CostManagement/views/ms:CostByService",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure Cost Management Scheduled Actions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:costmanagement/scheduledAction:ScheduledAction example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.CostManagement/scheduledActions/scheduledaction1
/// ```
///
pub mod scheduled_action {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduledActionArgs {
        /// UTC day on which cost analysis data will be emailed. Must be between `1` and `31`. This property is applicable when `frequency` is `Monthly`.
        #[builder(into, default)]
        pub day_of_month: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies a list of day names on which cost analysis data will be emailed. This property is applicable when frequency is `Weekly` or `Monthly`. Possible values are `Friday`, `Monday`, `Saturday`, `Sunday`, `Thursday`, `Tuesday` and `Wednesday`.
        #[builder(into, default)]
        pub days_of_weeks: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// User visible input name of the Cost Management Scheduled Action.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Email address of the point of contact that should get the unsubscribe requests of Scheduled Action notification emails.
        #[builder(into)]
        pub email_address_sender: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of email addresses that will receive the Scheduled Action.
        #[builder(into)]
        pub email_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Subject of the email. Length is limited to 70 characters.
        #[builder(into)]
        pub email_subject: pulumi_wasm_rust::Output<String>,
        /// The end date and time of the Scheduled Action (UTC).
        #[builder(into)]
        pub end_date: pulumi_wasm_rust::Output<String>,
        /// Frequency of the schedule. Possible values are `Daily`, `Monthly` and `Weekly`. Value `Monthly` requires either `weeks_of_month` and `days_of_week` or `day_of_month` to be specified. Value `Weekly` requires `days_of_week` to be specified.
        #[builder(into)]
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// UTC time at which cost analysis data will be emailed. Must be between `0` and `23`.
        #[builder(into, default)]
        pub hour_of_day: pulumi_wasm_rust::Output<Option<i32>>,
        /// Message to be added in the email. Length is limited to 250 characters.
        #[builder(into, default)]
        pub message: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Azure Cost Management Scheduled Action. Changing this forces a new Azure Cost Management Scheduled Action to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The start date and time of the Scheduled Action (UTC).
        #[builder(into)]
        pub start_date: pulumi_wasm_rust::Output<String>,
        /// The ID of the Cost Management View that is used by the Scheduled Action. Changing this forces a new resource to be created.
        #[builder(into)]
        pub view_id: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of weeks in which cost analysis data will be emailed. This property is applicable when `frequency` is `Monthly` and used in combination with `days_of_week`. Possible values are `First`, `Fourth`, `Last`, `Second` and `Third`.
        #[builder(into, default)]
        pub weeks_of_months: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ScheduledActionResult {
        /// UTC day on which cost analysis data will be emailed. Must be between `1` and `31`. This property is applicable when `frequency` is `Monthly`.
        pub day_of_month: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies a list of day names on which cost analysis data will be emailed. This property is applicable when frequency is `Weekly` or `Monthly`. Possible values are `Friday`, `Monday`, `Saturday`, `Sunday`, `Thursday`, `Tuesday` and `Wednesday`.
        pub days_of_weeks: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// User visible input name of the Cost Management Scheduled Action.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Email address of the point of contact that should get the unsubscribe requests of Scheduled Action notification emails.
        pub email_address_sender: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of email addresses that will receive the Scheduled Action.
        pub email_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Subject of the email. Length is limited to 70 characters.
        pub email_subject: pulumi_wasm_rust::Output<String>,
        /// The end date and time of the Scheduled Action (UTC).
        pub end_date: pulumi_wasm_rust::Output<String>,
        /// Frequency of the schedule. Possible values are `Daily`, `Monthly` and `Weekly`. Value `Monthly` requires either `weeks_of_month` and `days_of_week` or `day_of_month` to be specified. Value `Weekly` requires `days_of_week` to be specified.
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// UTC time at which cost analysis data will be emailed. Must be between `0` and `23`.
        pub hour_of_day: pulumi_wasm_rust::Output<Option<i32>>,
        /// Message to be added in the email. Length is limited to 250 characters.
        pub message: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Azure Cost Management Scheduled Action. Changing this forces a new Azure Cost Management Scheduled Action to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The start date and time of the Scheduled Action (UTC).
        pub start_date: pulumi_wasm_rust::Output<String>,
        /// The ID of the Cost Management View that is used by the Scheduled Action. Changing this forces a new resource to be created.
        pub view_id: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of weeks in which cost analysis data will be emailed. This property is applicable when `frequency` is `Monthly` and used in combination with `days_of_week`. Possible values are `First`, `Fourth`, `Last`, `Second` and `Third`.
        pub weeks_of_months: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ScheduledActionArgs) -> ScheduledActionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let day_of_month_binding = args.day_of_month.get_inner();
        let days_of_weeks_binding = args.days_of_weeks.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let email_address_sender_binding = args.email_address_sender.get_inner();
        let email_addresses_binding = args.email_addresses.get_inner();
        let email_subject_binding = args.email_subject.get_inner();
        let end_date_binding = args.end_date.get_inner();
        let frequency_binding = args.frequency.get_inner();
        let hour_of_day_binding = args.hour_of_day.get_inner();
        let message_binding = args.message.get_inner();
        let name_binding = args.name.get_inner();
        let start_date_binding = args.start_date.get_inner();
        let view_id_binding = args.view_id.get_inner();
        let weeks_of_months_binding = args.weeks_of_months.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:costmanagement/scheduledAction:ScheduledAction".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dayOfMonth".into(),
                    value: &day_of_month_binding,
                },
                register_interface::ObjectField {
                    name: "daysOfWeeks".into(),
                    value: &days_of_weeks_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "emailAddressSender".into(),
                    value: &email_address_sender_binding,
                },
                register_interface::ObjectField {
                    name: "emailAddresses".into(),
                    value: &email_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "emailSubject".into(),
                    value: &email_subject_binding,
                },
                register_interface::ObjectField {
                    name: "endDate".into(),
                    value: &end_date_binding,
                },
                register_interface::ObjectField {
                    name: "frequency".into(),
                    value: &frequency_binding,
                },
                register_interface::ObjectField {
                    name: "hourOfDay".into(),
                    value: &hour_of_day_binding,
                },
                register_interface::ObjectField {
                    name: "message".into(),
                    value: &message_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "startDate".into(),
                    value: &start_date_binding,
                },
                register_interface::ObjectField {
                    name: "viewId".into(),
                    value: &view_id_binding,
                },
                register_interface::ObjectField {
                    name: "weeksOfMonths".into(),
                    value: &weeks_of_months_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dayOfMonth".into(),
                },
                register_interface::ResultField {
                    name: "daysOfWeeks".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "emailAddressSender".into(),
                },
                register_interface::ResultField {
                    name: "emailAddresses".into(),
                },
                register_interface::ResultField {
                    name: "emailSubject".into(),
                },
                register_interface::ResultField {
                    name: "endDate".into(),
                },
                register_interface::ResultField {
                    name: "frequency".into(),
                },
                register_interface::ResultField {
                    name: "hourOfDay".into(),
                },
                register_interface::ResultField {
                    name: "message".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "startDate".into(),
                },
                register_interface::ResultField {
                    name: "viewId".into(),
                },
                register_interface::ResultField {
                    name: "weeksOfMonths".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ScheduledActionResult {
            day_of_month: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dayOfMonth").unwrap(),
            ),
            days_of_weeks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("daysOfWeeks").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            email_address_sender: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailAddressSender").unwrap(),
            ),
            email_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailAddresses").unwrap(),
            ),
            email_subject: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailSubject").unwrap(),
            ),
            end_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endDate").unwrap(),
            ),
            frequency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frequency").unwrap(),
            ),
            hour_of_day: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hourOfDay").unwrap(),
            ),
            message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("message").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            start_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startDate").unwrap(),
            ),
            view_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("viewId").unwrap(),
            ),
            weeks_of_months: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("weeksOfMonths").unwrap(),
            ),
        }
    }
}