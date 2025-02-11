#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_trigger_schedule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTriggerScheduleArgs {
        /// The ID of the Azure Data Factory to fetch trigger schedule from.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the trigger schedule.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTriggerScheduleResult {
        /// Specifies if the Data Factory Schedule Trigger is activated.
        pub activated: pulumi_gestalt_rust::Output<bool>,
        /// List of tags that can be used for describing the Data Factory Schedule Trigger.
        pub annotations: pulumi_gestalt_rust::Output<Vec<String>>,
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The Schedule Trigger's description.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The time the Schedule Trigger should end. The time will be represented in UTC.
        pub end_time: pulumi_gestalt_rust::Output<String>,
        /// The trigger frequency.
        pub frequency: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The interval for how often the trigger occurs.
        pub interval: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Data Factory Pipeline name that the trigger will act on.
        pub pipeline_name: pulumi_gestalt_rust::Output<String>,
        /// A `schedule` block as described below, which further specifies the recurrence schedule for the trigger.
        pub schedules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::datafactory::GetTriggerScheduleSchedule>,
        >,
        /// The time the Schedule Trigger will start. The time will be represented in UTC.
        pub start_time: pulumi_gestalt_rust::Output<String>,
        /// The timezone of the start/end time.
        pub time_zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTriggerScheduleArgs,
    ) -> GetTriggerScheduleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:datafactory/getTriggerSchedule:getTriggerSchedule".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTriggerScheduleResult {
            activated: o.get_field("activated"),
            annotations: o.get_field("annotations"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            end_time: o.get_field("endTime"),
            frequency: o.get_field("frequency"),
            id: o.get_field("id"),
            interval: o.get_field("interval"),
            name: o.get_field("name"),
            pipeline_name: o.get_field("pipelineName"),
            schedules: o.get_field("schedules"),
            start_time: o.get_field("startTime"),
            time_zone: o.get_field("timeZone"),
        }
    }
}
