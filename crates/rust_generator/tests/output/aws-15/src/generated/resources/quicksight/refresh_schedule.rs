/// Resource for managing a QuickSight Refresh Schedule.
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
///     let example = refresh_schedule::create(
///         "example",
///         RefreshScheduleArgs::builder()
///             .data_set_id("dataset-id")
///             .schedule(
///                 RefreshScheduleSchedule::builder()
///                     .refreshType("FULL_REFRESH")
///                     .scheduleFrequency(
///                         RefreshScheduleScheduleScheduleFrequency::builder()
///                             .interval("HOURLY")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .schedule_id("schedule-id")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Weekly Refresh
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = refresh_schedule::create(
///         "example",
///         RefreshScheduleArgs::builder()
///             .data_set_id("dataset-id")
///             .schedule(
///                 RefreshScheduleSchedule::builder()
///                     .refreshType("INCREMENTAL_REFRESH")
///                     .scheduleFrequency(
///                         RefreshScheduleScheduleScheduleFrequency::builder()
///                             .interval("WEEKLY")
///                             .refreshOnDay(
///                                 RefreshScheduleScheduleScheduleFrequencyRefreshOnDay::builder()
///                                     .dayOfWeek("MONDAY")
///                                     .build_struct(),
///                             )
///                             .timeOfTheDay("01:00")
///                             .timezone("Europe/London")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .schedule_id("schedule-id")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Monthly Refresh
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = refresh_schedule::create(
///         "example",
///         RefreshScheduleArgs::builder()
///             .data_set_id("dataset-id")
///             .schedule(
///                 RefreshScheduleSchedule::builder()
///                     .refreshType("INCREMENTAL_REFRESH")
///                     .scheduleFrequency(
///                         RefreshScheduleScheduleScheduleFrequency::builder()
///                             .interval("MONTHLY")
///                             .refreshOnDay(
///                                 RefreshScheduleScheduleScheduleFrequencyRefreshOnDay::builder()
///                                     .dayOfMonth("1")
///                                     .build_struct(),
///                             )
///                             .timeOfTheDay("01:00")
///                             .timezone("Europe/London")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .schedule_id("schedule-id")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a QuickSight Refresh Schedule using the AWS account ID, data set ID and schedule ID separated by commas (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/refreshSchedule:RefreshSchedule example 123456789012,dataset-id,schedule-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod refresh_schedule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RefreshScheduleArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the dataset.
        #[builder(into)]
        pub data_set_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The [refresh schedule](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_RefreshSchedule.html). See schedule
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::quicksight::RefreshScheduleSchedule>,
        >,
        /// The ID of the refresh schedule.
        #[builder(into)]
        pub schedule_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RefreshScheduleResult {
        /// Amazon Resource Name (ARN) of the refresh schedule.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the dataset.
        pub data_set_id: pulumi_gestalt_rust::Output<String>,
        /// The [refresh schedule](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_RefreshSchedule.html). See schedule
        ///
        /// The following arguments are optional:
        pub schedule: pulumi_gestalt_rust::Output<
            Option<super::super::types::quicksight::RefreshScheduleSchedule>,
        >,
        /// The ID of the refresh schedule.
        pub schedule_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RefreshScheduleArgs,
    ) -> RefreshScheduleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let aws_account_id_binding = args.aws_account_id.get_output(context);
        let data_set_id_binding = args.data_set_id.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let schedule_id_binding = args.schedule_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:quicksight/refreshSchedule:RefreshSchedule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsAccountId".into(),
                    value: aws_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataSetId".into(),
                    value: data_set_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: schedule_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduleId".into(),
                    value: schedule_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RefreshScheduleResult {
            arn: o.get_field("arn"),
            aws_account_id: o.get_field("awsAccountId"),
            data_set_id: o.get_field("dataSetId"),
            schedule: o.get_field("schedule"),
            schedule_id: o.get_field("scheduleId"),
        }
    }
}
