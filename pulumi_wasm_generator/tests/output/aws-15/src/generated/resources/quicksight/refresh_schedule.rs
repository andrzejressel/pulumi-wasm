/// Resource for managing a QuickSight Refresh Schedule.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod refresh_schedule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RefreshScheduleArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the dataset.
        #[builder(into)]
        pub data_set_id: pulumi_wasm_rust::Output<String>,
        /// The [refresh schedule](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_RefreshSchedule.html). See schedule
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::RefreshScheduleSchedule>,
        >,
        /// The ID of the refresh schedule.
        #[builder(into)]
        pub schedule_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RefreshScheduleResult {
        /// Amazon Resource Name (ARN) of the refresh schedule.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the dataset.
        pub data_set_id: pulumi_wasm_rust::Output<String>,
        /// The [refresh schedule](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_RefreshSchedule.html). See schedule
        ///
        /// The following arguments are optional:
        pub schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::RefreshScheduleSchedule>,
        >,
        /// The ID of the refresh schedule.
        pub schedule_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RefreshScheduleArgs) -> RefreshScheduleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding = args.aws_account_id.get_inner();
        let data_set_id_binding = args.data_set_id.get_inner();
        let schedule_binding = args.schedule.get_inner();
        let schedule_id_binding = args.schedule_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/refreshSchedule:RefreshSchedule".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "dataSetId".into(),
                    value: &data_set_id_binding,
                },
                register_interface::ObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "scheduleId".into(),
                    value: &schedule_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "awsAccountId".into(),
                },
                register_interface::ResultField {
                    name: "dataSetId".into(),
                },
                register_interface::ResultField {
                    name: "schedule".into(),
                },
                register_interface::ResultField {
                    name: "scheduleId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RefreshScheduleResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsAccountId").unwrap(),
            ),
            data_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSetId").unwrap(),
            ),
            schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedule").unwrap(),
            ),
            schedule_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scheduleId").unwrap(),
            ),
        }
    }
}
