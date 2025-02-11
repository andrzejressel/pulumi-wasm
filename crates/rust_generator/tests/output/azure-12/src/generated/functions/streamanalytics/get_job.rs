#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_job {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetJobArgs {
        /// Specifies the name of the Stream Analytics Job.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Stream Analytics Job is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetJobResult {
        /// The compatibility level for this job.
        pub compatibility_level: pulumi_gestalt_rust::Output<String>,
        /// The Data Locale of the Job.
        pub data_locale: pulumi_gestalt_rust::Output<String>,
        /// The maximum tolerable delay in seconds where events arriving late could be included.
        pub events_late_arrival_max_delay_in_seconds: pulumi_gestalt_rust::Output<i32>,
        /// The maximum tolerable delay in seconds where out-of-order events can be adjusted to be back in order.
        pub events_out_of_order_max_delay_in_seconds: pulumi_gestalt_rust::Output<i32>,
        /// The policy which should be applied to events which arrive out of order in the input event stream.
        pub events_out_of_order_policy: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::streamanalytics::GetJobIdentity>,
        >,
        /// The Job ID assigned by the Stream Analytics Job.
        pub job_id: pulumi_gestalt_rust::Output<String>,
        /// The time at which the Stream Analytics job last produced an output.
        pub last_output_time: pulumi_gestalt_rust::Output<String>,
        /// The Azure location where the Stream Analytics Job exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The policy which should be applied to events which arrive at the output and cannot be written to the external storage due to being malformed (such as missing column values, column values of wrong type or size).
        pub output_error_policy: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU Name to use for the Stream Analytics Job.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// The starting mode set for this Stream Analytics Job.
        pub start_mode: pulumi_gestalt_rust::Output<String>,
        /// The time at which this Stream Analytics Job was scheduled to start.
        pub start_time: pulumi_gestalt_rust::Output<String>,
        /// The number of streaming units that this Stream Analytics Job uses.
        pub streaming_units: pulumi_gestalt_rust::Output<i32>,
        /// The query that will be run in this Stream Analytics Job, [written in Stream Analytics Query Language (SAQL)](https://msdn.microsoft.com/library/azure/dn834998).
        pub transformation_query: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetJobArgs,
    ) -> GetJobResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:streamanalytics/getJob:getJob".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetJobResult {
            compatibility_level: o.get_field("compatibilityLevel"),
            data_locale: o.get_field("dataLocale"),
            events_late_arrival_max_delay_in_seconds: o
                .get_field("eventsLateArrivalMaxDelayInSeconds"),
            events_out_of_order_max_delay_in_seconds: o
                .get_field("eventsOutOfOrderMaxDelayInSeconds"),
            events_out_of_order_policy: o.get_field("eventsOutOfOrderPolicy"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            job_id: o.get_field("jobId"),
            last_output_time: o.get_field("lastOutputTime"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            output_error_policy: o.get_field("outputErrorPolicy"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            start_mode: o.get_field("startMode"),
            start_time: o.get_field("startTime"),
            streaming_units: o.get_field("streamingUnits"),
            transformation_query: o.get_field("transformationQuery"),
        }
    }
}
