/// Manages an Azure Cost Management Scheduled Action.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod scheduled_action {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduledActionArgs {
        /// UTC day on which cost analysis data will be emailed. Must be between `1` and `31`. This property is applicable when `frequency` is `Monthly`.
        #[builder(into, default)]
        pub day_of_month: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies a list of day names on which cost analysis data will be emailed. This property is applicable when frequency is `Weekly` or `Monthly`. Possible values are `Friday`, `Monday`, `Saturday`, `Sunday`, `Thursday`, `Tuesday` and `Wednesday`.
        #[builder(into, default)]
        pub days_of_weeks: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// User visible input name of the Cost Management Scheduled Action.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Email address of the point of contact that should get the unsubscribe requests of Scheduled Action notification emails.
        #[builder(into)]
        pub email_address_sender: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies a list of email addresses that will receive the Scheduled Action.
        #[builder(into)]
        pub email_addresses: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Subject of the email. Length is limited to 70 characters.
        #[builder(into)]
        pub email_subject: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The end date and time of the Scheduled Action (UTC).
        #[builder(into)]
        pub end_date: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Frequency of the schedule. Possible values are `Daily`, `Monthly` and `Weekly`. Value `Monthly` requires either `weeks_of_month` and `days_of_week` or `day_of_month` to be specified. Value `Weekly` requires `days_of_week` to be specified.
        #[builder(into)]
        pub frequency: pulumi_gestalt_rust::InputOrOutput<String>,
        /// UTC time at which cost analysis data will be emailed. Must be between `0` and `23`.
        #[builder(into, default)]
        pub hour_of_day: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Message to be added in the email. Length is limited to 250 characters.
        #[builder(into, default)]
        pub message: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Azure Cost Management Scheduled Action. Changing this forces a new Azure Cost Management Scheduled Action to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The start date and time of the Scheduled Action (UTC).
        #[builder(into)]
        pub start_date: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Cost Management View that is used by the Scheduled Action. Changing this forces a new resource to be created.
        #[builder(into)]
        pub view_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies a list of weeks in which cost analysis data will be emailed. This property is applicable when `frequency` is `Monthly` and used in combination with `days_of_week`. Possible values are `First`, `Fourth`, `Last`, `Second` and `Third`.
        #[builder(into, default)]
        pub weeks_of_months: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ScheduledActionResult {
        /// UTC day on which cost analysis data will be emailed. Must be between `1` and `31`. This property is applicable when `frequency` is `Monthly`.
        pub day_of_month: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies a list of day names on which cost analysis data will be emailed. This property is applicable when frequency is `Weekly` or `Monthly`. Possible values are `Friday`, `Monday`, `Saturday`, `Sunday`, `Thursday`, `Tuesday` and `Wednesday`.
        pub days_of_weeks: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// User visible input name of the Cost Management Scheduled Action.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Email address of the point of contact that should get the unsubscribe requests of Scheduled Action notification emails.
        pub email_address_sender: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of email addresses that will receive the Scheduled Action.
        pub email_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Subject of the email. Length is limited to 70 characters.
        pub email_subject: pulumi_gestalt_rust::Output<String>,
        /// The end date and time of the Scheduled Action (UTC).
        pub end_date: pulumi_gestalt_rust::Output<String>,
        /// Frequency of the schedule. Possible values are `Daily`, `Monthly` and `Weekly`. Value `Monthly` requires either `weeks_of_month` and `days_of_week` or `day_of_month` to be specified. Value `Weekly` requires `days_of_week` to be specified.
        pub frequency: pulumi_gestalt_rust::Output<String>,
        /// UTC time at which cost analysis data will be emailed. Must be between `0` and `23`.
        pub hour_of_day: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Message to be added in the email. Length is limited to 250 characters.
        pub message: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Azure Cost Management Scheduled Action. Changing this forces a new Azure Cost Management Scheduled Action to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The start date and time of the Scheduled Action (UTC).
        pub start_date: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Cost Management View that is used by the Scheduled Action. Changing this forces a new resource to be created.
        pub view_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of weeks in which cost analysis data will be emailed. This property is applicable when `frequency` is `Monthly` and used in combination with `days_of_week`. Possible values are `First`, `Fourth`, `Last`, `Second` and `Third`.
        pub weeks_of_months: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScheduledActionArgs,
    ) -> ScheduledActionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let day_of_month_binding = args.day_of_month.get_output(context);
        let days_of_weeks_binding = args.days_of_weeks.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let email_address_sender_binding = args.email_address_sender.get_output(context);
        let email_addresses_binding = args.email_addresses.get_output(context);
        let email_subject_binding = args.email_subject.get_output(context);
        let end_date_binding = args.end_date.get_output(context);
        let frequency_binding = args.frequency.get_output(context);
        let hour_of_day_binding = args.hour_of_day.get_output(context);
        let message_binding = args.message.get_output(context);
        let name_binding = args.name.get_output(context);
        let start_date_binding = args.start_date.get_output(context);
        let view_id_binding = args.view_id.get_output(context);
        let weeks_of_months_binding = args.weeks_of_months.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:costmanagement/scheduledAction:ScheduledAction".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dayOfMonth".into(),
                    value: day_of_month_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "daysOfWeeks".into(),
                    value: days_of_weeks_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailAddressSender".into(),
                    value: email_address_sender_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailAddresses".into(),
                    value: email_addresses_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailSubject".into(),
                    value: email_subject_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endDate".into(),
                    value: end_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frequency".into(),
                    value: frequency_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hourOfDay".into(),
                    value: hour_of_day_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "message".into(),
                    value: message_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startDate".into(),
                    value: start_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "viewId".into(),
                    value: view_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "weeksOfMonths".into(),
                    value: weeks_of_months_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ScheduledActionResult {
            day_of_month: o.get_field("dayOfMonth"),
            days_of_weeks: o.get_field("daysOfWeeks"),
            display_name: o.get_field("displayName"),
            email_address_sender: o.get_field("emailAddressSender"),
            email_addresses: o.get_field("emailAddresses"),
            email_subject: o.get_field("emailSubject"),
            end_date: o.get_field("endDate"),
            frequency: o.get_field("frequency"),
            hour_of_day: o.get_field("hourOfDay"),
            message: o.get_field("message"),
            name: o.get_field("name"),
            start_date: o.get_field("startDate"),
            view_id: o.get_field("viewId"),
            weeks_of_months: o.get_field("weeksOfMonths"),
        }
    }
}
