pub mod get_job {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetJobArgs {
        /// Specifies the name of the Stream Analytics Job.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the resource group the Stream Analytics Job is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetJobResult {
        /// The compatibility level for this job.
        pub compatibility_level: pulumi_wasm_rust::Output<String>,
        /// The Data Locale of the Job.
        pub data_locale: pulumi_wasm_rust::Output<String>,
        /// The maximum tolerable delay in seconds where events arriving late could be included.
        pub events_late_arrival_max_delay_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// The maximum tolerable delay in seconds where out-of-order events can be adjusted to be back in order.
        pub events_out_of_order_max_delay_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// The policy which should be applied to events which arrive out of order in the input event stream.
        pub events_out_of_order_policy: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::streamanalytics::GetJobIdentity>,
        >,
        /// The Job ID assigned by the Stream Analytics Job.
        pub job_id: pulumi_wasm_rust::Output<String>,
        /// The time at which the Stream Analytics job last produced an output.
        pub last_output_time: pulumi_wasm_rust::Output<String>,
        /// The Azure location where the Stream Analytics Job exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The policy which should be applied to events which arrive at the output and cannot be written to the external storage due to being malformed (such as missing column values, column values of wrong type or size).
        pub output_error_policy: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU Name to use for the Stream Analytics Job.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The starting mode set for this Stream Analytics Job.
        pub start_mode: pulumi_wasm_rust::Output<String>,
        /// The time at which this Stream Analytics Job was scheduled to start.
        pub start_time: pulumi_wasm_rust::Output<String>,
        /// The number of streaming units that this Stream Analytics Job uses.
        pub streaming_units: pulumi_wasm_rust::Output<i32>,
        /// The query that will be run in this Stream Analytics Job, [written in Stream Analytics Query Language (SAQL)](https://msdn.microsoft.com/library/azure/dn834998).
        pub transformation_query: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetJobArgs) -> GetJobResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:streamanalytics/getJob:getJob".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "compatibilityLevel".into(),
                },
                register_interface::ResultField {
                    name: "dataLocale".into(),
                },
                register_interface::ResultField {
                    name: "eventsLateArrivalMaxDelayInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "eventsOutOfOrderMaxDelayInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "eventsOutOfOrderPolicy".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "jobId".into(),
                },
                register_interface::ResultField {
                    name: "lastOutputTime".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outputErrorPolicy".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "startMode".into(),
                },
                register_interface::ResultField {
                    name: "startTime".into(),
                },
                register_interface::ResultField {
                    name: "streamingUnits".into(),
                },
                register_interface::ResultField {
                    name: "transformationQuery".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetJobResult {
            compatibility_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("compatibilityLevel").unwrap(),
            ),
            data_locale: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataLocale").unwrap(),
            ),
            events_late_arrival_max_delay_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventsLateArrivalMaxDelayInSeconds").unwrap(),
            ),
            events_out_of_order_max_delay_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventsOutOfOrderMaxDelayInSeconds").unwrap(),
            ),
            events_out_of_order_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventsOutOfOrderPolicy").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            job_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jobId").unwrap(),
            ),
            last_output_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastOutputTime").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            output_error_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputErrorPolicy").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            start_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startMode").unwrap(),
            ),
            start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startTime").unwrap(),
            ),
            streaming_units: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamingUnits").unwrap(),
            ),
            transformation_query: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transformationQuery").unwrap(),
            ),
        }
    }
}
