/// Manages a Stream Analytics Job Schedule.
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: example
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: private
///   exampleBlob:
///     type: azure:storage:Blob
///     name: example
///     properties:
///       name: example
///       storageAccountName: ${exampleAccount.name}
///       storageContainerName: ${exampleContainer.name}
///       type: Block
///       source:
///         fn::FileAsset: example.csv
///   exampleJob:
///     type: azure:streamanalytics:Job
///     name: example
///     properties:
///       name: example-job
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       compatibilityLevel: '1.2'
///       dataLocale: en-GB
///       eventsLateArrivalMaxDelayInSeconds: 60
///       eventsOutOfOrderMaxDelayInSeconds: 50
///       eventsOutOfOrderPolicy: Adjust
///       outputErrorPolicy: Drop
///       streamingUnits: 3
///       tags:
///         environment: Example
///       transformationQuery: |2
///             SELECT *
///             INTO [exampleoutput]
///             FROM [exampleinput]
///   exampleStreamInputBlob:
///     type: azure:streamanalytics:StreamInputBlob
///     name: example
///     properties:
///       name: exampleinput
///       streamAnalyticsJobName: ${exampleJob.name}
///       resourceGroupName: ${exampleJob.resourceGroupName}
///       storageAccountName: ${exampleAccount.name}
///       storageAccountKey: ${exampleAccount.primaryAccessKey}
///       storageContainerName: ${exampleContainer.name}
///       pathPattern: ""
///       dateFormat: yyyy/MM/dd
///       timeFormat: HH
///       serialization:
///         type: Csv
///         encoding: UTF8
///         fieldDelimiter: ','
///   exampleOutputBlob:
///     type: azure:streamanalytics:OutputBlob
///     name: example
///     properties:
///       name: exampleoutput
///       streamAnalyticsJobName: ${exampleJob.name}
///       resourceGroupName: ${exampleJob.resourceGroupName}
///       storageAccountName: ${exampleAccount.name}
///       storageAccountKey: ${exampleAccount.primaryAccessKey}
///       storageContainerName: ${exampleContainer.name}
///       pathPattern: example-{date}-{time}
///       dateFormat: yyyy-MM-dd
///       timeFormat: HH
///       serialization:
///         type: Avro
///   exampleJobSchedule:
///     type: azure:streamanalytics:JobSchedule
///     name: example
///     properties:
///       streamAnalyticsJobId: ${exampleJob.id}
///       startMode: CustomTime
///       startTime: 2022-09-21T00:00:00Z
///     options:
///       dependsOn:
///         - ${exampleJob}
///         - ${exampleStreamInputBlob}
///         - ${exampleOutputBlob}
/// ```
///
/// ## Import
///
/// Stream Analytics Job's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/jobSchedule:JobSchedule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/schedule/default
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod job_schedule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobScheduleArgs {
        /// The starting mode of the Stream Analytics Job. Possible values are `JobStartTime`, `CustomTime` and `LastOutputEventTime`.
        ///
        /// > **Note:** Setting `start_mode` to `LastOutputEventTime` is only possible if the job had been previously started and produced output.
        #[builder(into)]
        pub start_mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The time in ISO8601 format at which the Stream Analytics Job should be started e.g. `2022-04-01T00:00:00Z`. This property can only be specified if `start_mode` is set to `CustomTime`
        #[builder(into, default)]
        pub start_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Stream Analytics Job that should be scheduled or started. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct JobScheduleResult {
        /// The time at which the Stream Analytics job last produced an output.
        pub last_output_time: pulumi_gestalt_rust::Output<String>,
        /// The starting mode of the Stream Analytics Job. Possible values are `JobStartTime`, `CustomTime` and `LastOutputEventTime`.
        ///
        /// > **Note:** Setting `start_mode` to `LastOutputEventTime` is only possible if the job had been previously started and produced output.
        pub start_mode: pulumi_gestalt_rust::Output<String>,
        /// The time in ISO8601 format at which the Stream Analytics Job should be started e.g. `2022-04-01T00:00:00Z`. This property can only be specified if `start_mode` is set to `CustomTime`
        pub start_time: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Stream Analytics Job that should be scheduled or started. Changing this forces a new resource to be created.
        pub stream_analytics_job_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: JobScheduleArgs,
    ) -> JobScheduleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let start_mode_binding_1 = args.start_mode.get_output(context);
        let start_mode_binding = start_mode_binding_1.get_inner();
        let start_time_binding_1 = args.start_time.get_output(context);
        let start_time_binding = start_time_binding_1.get_inner();
        let stream_analytics_job_id_binding_1 = args
            .stream_analytics_job_id
            .get_output(context);
        let stream_analytics_job_id_binding = stream_analytics_job_id_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/jobSchedule:JobSchedule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "startMode".into(),
                    value: &start_mode_binding,
                },
                register_interface::ObjectField {
                    name: "startTime".into(),
                    value: &start_time_binding,
                },
                register_interface::ObjectField {
                    name: "streamAnalyticsJobId".into(),
                    value: &stream_analytics_job_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        JobScheduleResult {
            last_output_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastOutputTime"),
            ),
            start_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("startMode"),
            ),
            start_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("startTime"),
            ),
            stream_analytics_job_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("streamAnalyticsJobId"),
            ),
        }
    }
}
