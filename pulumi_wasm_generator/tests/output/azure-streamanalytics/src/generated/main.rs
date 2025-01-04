pub mod streamanalytics {
    include!("resources/streamanalytics/cluster.rs");
    include!("resources/streamanalytics/function_java_script_udf.rs");
    include!("resources/streamanalytics/function_javascript_uda.rs");
    include!("resources/streamanalytics/job.rs");
    include!("resources/streamanalytics/job_schedule.rs");
    include!("resources/streamanalytics/managed_private_endpoint.rs");
    include!("resources/streamanalytics/output_blob.rs");
    include!("resources/streamanalytics/output_cosmosdb.rs");
    include!("resources/streamanalytics/output_event_hub.rs");
    include!("resources/streamanalytics/output_function.rs");
    include!("resources/streamanalytics/output_mssql.rs");
    include!("resources/streamanalytics/output_powerbi.rs");
    include!("resources/streamanalytics/output_service_bus_queue.rs");
    include!("resources/streamanalytics/output_servicebus_topic.rs");
    include!("resources/streamanalytics/output_synapse.rs");
    include!("resources/streamanalytics/output_table.rs");
    include!("resources/streamanalytics/reference_input_blob.rs");
    include!("resources/streamanalytics/reference_input_mssql.rs");
    include!("resources/streamanalytics/stream_input_blob.rs");
    include!("resources/streamanalytics/stream_input_event_hub.rs");
    include!("resources/streamanalytics/stream_input_event_hub_v_2.rs");
    include!("resources/streamanalytics/stream_input_iot_hub.rs");
}
pub mod functions {
    pub mod streamanalytics {
        include!("functions/streamanalytics/get_job.rs");
    }
}
pub mod types {
    pub mod streamanalytics {
        include!("types/streamanalytics/function_java_script_udf_input.rs");
        include!("types/streamanalytics/function_java_script_udf_output.rs");
        include!("types/streamanalytics/function_javascript_uda_input.rs");
        include!("types/streamanalytics/function_javascript_uda_output.rs");
        include!("types/streamanalytics/job_identity.rs");
        include!("types/streamanalytics/job_job_storage_account.rs");
        include!("types/streamanalytics/output_blob_serialization.rs");
        include!("types/streamanalytics/output_event_hub_serialization.rs");
        include!("types/streamanalytics/output_service_bus_queue_serialization.rs");
        include!("types/streamanalytics/output_servicebus_topic_serialization.rs");
        include!("types/streamanalytics/reference_input_blob_serialization.rs");
        include!("types/streamanalytics/stream_input_blob_serialization.rs");
        include!("types/streamanalytics/stream_input_event_hub_serialization.rs");
        include!("types/streamanalytics/stream_input_event_hub_v_2_serialization.rs");
        include!("types/streamanalytics/stream_input_iot_hub_serialization.rs");
        include!("types/streamanalytics/get_job_identity.rs");
    }
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-azure {
    import output-interface;
}

interface output-interface {

    resource output {
        constructor(value: string);
        map: func(function-name: string) -> output;
    }
    combine: func(outputs: list<borrow<output>>) -> output;
}


interface register-interface {
    use output-interface.{output};

    record object-field {
        name: string,
        value: borrow<output>
    }

    record result-field {
        name: string
    }

    record register-resource-result-field {
        name: string,
        output: output
    }

    record register-resource-request {
        %type: string,
        name: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record register-resource-result {
        fields: list<register-resource-result-field>
    }

    register: func(request: register-resource-request) -> register-resource-result;

    record resource-invoke-result-field {
        name: string,
        output: output
    }

    record resource-invoke-request {
        token: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record resource-invoke-result {
        fields: list<resource-invoke-result-field>
    }

    invoke: func(request: resource-invoke-request) -> resource-invoke-result;
}",
        with : { "component:pulumi-wasm/output-interface@0.0.0-DEV" :
        pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
        } }
    );
}
