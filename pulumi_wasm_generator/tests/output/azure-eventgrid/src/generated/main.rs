pub mod eventgrid {
    include!("resources/eventgrid/domain.rs");
    include!("resources/eventgrid/domain_topic.rs");
    include!("resources/eventgrid/event_subscription.rs");
    include!("resources/eventgrid/namespace.rs");
    include!("resources/eventgrid/system_topic.rs");
    include!("resources/eventgrid/system_topic_event_subscription.rs");
    include!("resources/eventgrid/topic.rs");
}
pub mod functions {
    pub mod eventgrid {
        include!("functions/eventgrid/get_domain.rs");
        include!("functions/eventgrid/get_domain_topic.rs");
        include!("functions/eventgrid/get_system_topic.rs");
        include!("functions/eventgrid/get_topic.rs");
    }
}
pub mod types {
    pub mod eventgrid {
        include!("types/eventgrid/domain_identity.rs");
        include!("types/eventgrid/domain_inbound_ip_rule.rs");
        include!("types/eventgrid/domain_input_mapping_default_values.rs");
        include!("types/eventgrid/domain_input_mapping_fields.rs");
        include!("types/eventgrid/event_subscription_advanced_filter.rs");
        include!("types/eventgrid/event_subscription_advanced_filter_bool_equal.rs");
        include!("types/eventgrid/event_subscription_advanced_filter_is_not_null.rs");
        include!(
            "types/eventgrid/event_subscription_advanced_filter_is_null_or_undefined.rs"
        );
        include!(
            "types/eventgrid/event_subscription_advanced_filter_number_greater_than.rs"
        );
        include!(
            "types/eventgrid/event_subscription_advanced_filter_number_greater_than_or_equal.rs"
        );
        include!("types/eventgrid/event_subscription_advanced_filter_number_in.rs");
        include!(
            "types/eventgrid/event_subscription_advanced_filter_number_in_range.rs"
        );
        include!(
            "types/eventgrid/event_subscription_advanced_filter_number_less_than.rs"
        );
        include!(
            "types/eventgrid/event_subscription_advanced_filter_number_less_than_or_equal.rs"
        );
        include!("types/eventgrid/event_subscription_advanced_filter_number_not_in.rs");
        include!(
            "types/eventgrid/event_subscription_advanced_filter_number_not_in_range.rs"
        );
        include!(
            "types/eventgrid/event_subscription_advanced_filter_string_begins_with.rs"
        );
        include!("types/eventgrid/event_subscription_advanced_filter_string_contain.rs");
        include!(
            "types/eventgrid/event_subscription_advanced_filter_string_ends_with.rs"
        );
        include!("types/eventgrid/event_subscription_advanced_filter_string_in.rs");
        include!(
            "types/eventgrid/event_subscription_advanced_filter_string_not_begins_with.rs"
        );
        include!(
            "types/eventgrid/event_subscription_advanced_filter_string_not_contain.rs"
        );
        include!(
            "types/eventgrid/event_subscription_advanced_filter_string_not_ends_with.rs"
        );
        include!("types/eventgrid/event_subscription_advanced_filter_string_not_in.rs");
        include!("types/eventgrid/event_subscription_azure_function_endpoint.rs");
        include!("types/eventgrid/event_subscription_dead_letter_identity.rs");
        include!("types/eventgrid/event_subscription_delivery_identity.rs");
        include!("types/eventgrid/event_subscription_delivery_property.rs");
        include!("types/eventgrid/event_subscription_retry_policy.rs");
        include!(
            "types/eventgrid/event_subscription_storage_blob_dead_letter_destination.rs"
        );
        include!("types/eventgrid/event_subscription_storage_queue_endpoint.rs");
        include!("types/eventgrid/event_subscription_subject_filter.rs");
        include!("types/eventgrid/event_subscription_webhook_endpoint.rs");
        include!("types/eventgrid/namespace_identity.rs");
        include!("types/eventgrid/namespace_inbound_ip_rule.rs");
        include!("types/eventgrid/namespace_topic_spaces_configuration.rs");
        include!(
            "types/eventgrid/namespace_topic_spaces_configuration_dynamic_routing_enrichment.rs"
        );
        include!(
            "types/eventgrid/namespace_topic_spaces_configuration_static_routing_enrichment.rs"
        );
        include!("types/eventgrid/system_topic_event_subscription_advanced_filter.rs");
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_bool_equal.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_is_not_null.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_is_null_or_undefined.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_number_greater_than.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_number_greater_than_or_equal.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_number_in.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_number_in_range.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_number_less_than.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_number_less_than_or_equal.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_number_not_in.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_number_not_in_range.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_string_begins_with.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_string_contain.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_string_ends_with.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_string_in.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_string_not_begins_with.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_string_not_contain.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_string_not_ends_with.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_advanced_filter_string_not_in.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_azure_function_endpoint.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_dead_letter_identity.rs"
        );
        include!("types/eventgrid/system_topic_event_subscription_delivery_identity.rs");
        include!("types/eventgrid/system_topic_event_subscription_delivery_property.rs");
        include!("types/eventgrid/system_topic_event_subscription_retry_policy.rs");
        include!(
            "types/eventgrid/system_topic_event_subscription_storage_blob_dead_letter_destination.rs"
        );
        include!(
            "types/eventgrid/system_topic_event_subscription_storage_queue_endpoint.rs"
        );
        include!("types/eventgrid/system_topic_event_subscription_subject_filter.rs");
        include!("types/eventgrid/system_topic_event_subscription_webhook_endpoint.rs");
        include!("types/eventgrid/system_topic_identity.rs");
        include!("types/eventgrid/topic_identity.rs");
        include!("types/eventgrid/topic_inbound_ip_rule.rs");
        include!("types/eventgrid/topic_input_mapping_default_values.rs");
        include!("types/eventgrid/topic_input_mapping_fields.rs");
        include!("types/eventgrid/get_domain_identity.rs");
        include!("types/eventgrid/get_domain_inbound_ip_rule.rs");
        include!("types/eventgrid/get_domain_input_mapping_default_value.rs");
        include!("types/eventgrid/get_domain_input_mapping_field.rs");
        include!("types/eventgrid/get_system_topic_identity.rs");
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
