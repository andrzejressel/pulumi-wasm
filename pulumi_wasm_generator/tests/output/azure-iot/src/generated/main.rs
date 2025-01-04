pub mod iot {
    include!("resources/iot/certificate.rs");
    include!("resources/iot/consumer_group.rs");
    include!("resources/iot/dps_shared_access_policy.rs");
    include!("resources/iot/endpoint_cosmosdb_account.rs");
    include!("resources/iot/endpoint_eventhub.rs");
    include!("resources/iot/endpoint_servicebus_queue.rs");
    include!("resources/iot/endpoint_servicebus_topic.rs");
    include!("resources/iot/endpoint_storage_container.rs");
    include!("resources/iot/enrichment.rs");
    include!("resources/iot/fallback_route.rs");
    include!("resources/iot/file_upload.rs");
    include!("resources/iot/io_t_hub.rs");
    include!("resources/iot/iot_hub_certificate.rs");
    include!("resources/iot/iot_hub_device_update_account.rs");
    include!("resources/iot/iot_hub_device_update_instance.rs");
    include!("resources/iot/iot_hub_dps.rs");
    include!("resources/iot/route.rs");
    include!("resources/iot/security_device_group.rs");
    include!("resources/iot/security_solution.rs");
    include!("resources/iot/shared_access_policy.rs");
}
pub mod functions {
    pub mod iot {
        include!("functions/iot/get_dps.rs");
        include!("functions/iot/get_dps_shared_access_policy.rs");
        include!("functions/iot/get_iot_hub.rs");
        include!("functions/iot/get_shared_access_policy.rs");
    }
}
pub mod types {
    pub mod iot {
        include!("types/iot/io_t_hub_cloud_to_device.rs");
        include!("types/iot/io_t_hub_cloud_to_device_feedback.rs");
        include!("types/iot/io_t_hub_endpoint.rs");
        include!("types/iot/io_t_hub_enrichment.rs");
        include!("types/iot/io_t_hub_fallback_route.rs");
        include!("types/iot/io_t_hub_file_upload.rs");
        include!("types/iot/io_t_hub_identity.rs");
        include!("types/iot/io_t_hub_network_rule_set.rs");
        include!("types/iot/io_t_hub_network_rule_set_ip_rule.rs");
        include!("types/iot/io_t_hub_route.rs");
        include!("types/iot/io_t_hub_shared_access_policy.rs");
        include!("types/iot/io_t_hub_sku.rs");
        include!("types/iot/iot_hub_device_update_account_identity.rs");
        include!(
            "types/iot/iot_hub_device_update_instance_diagnostic_storage_account.rs"
        );
        include!("types/iot/iot_hub_dps_ip_filter_rule.rs");
        include!("types/iot/iot_hub_dps_linked_hub.rs");
        include!("types/iot/iot_hub_dps_sku.rs");
        include!("types/iot/security_device_group_allow_rule.rs");
        include!("types/iot/security_device_group_range_rule.rs");
        include!("types/iot/security_solution_additional_workspace.rs");
        include!("types/iot/security_solution_recommendations_enabled.rs");
        include!("types/iot/get_iot_hub_identity.rs");
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
