pub mod eventhub {
    include!("resources/eventhub/authorization_rule.rs");
    include!("resources/eventhub/cluster.rs");
    include!("resources/eventhub/consumer_group.rs");
    include!("resources/eventhub/domain.rs");
    include!("resources/eventhub/event_grid_topic.rs");
    include!("resources/eventhub/event_hub.rs");
    include!("resources/eventhub/event_hub_authorization_rule.rs");
    include!("resources/eventhub/event_hub_consumer_group.rs");
    include!("resources/eventhub/event_hub_namespace.rs");
    include!("resources/eventhub/event_hub_namespace_authorization_rule.rs");
    include!("resources/eventhub/event_subscription.rs");
    include!("resources/eventhub/eventhub_namespace_disaster_recovery_config.rs");
    include!("resources/eventhub/namespace.rs");
    include!("resources/eventhub/namespace_authorization_rule.rs");
    include!("resources/eventhub/namespace_customer_managed_key.rs");
    include!("resources/eventhub/namespace_schema_group.rs");
    include!("resources/eventhub/queue.rs");
    include!("resources/eventhub/queue_authorization_rule.rs");
    include!("resources/eventhub/subscription.rs");
    include!("resources/eventhub/subscription_rule.rs");
    include!("resources/eventhub/topic.rs");
    include!("resources/eventhub/topic_authorization_rule.rs");
}
pub mod functions {
    pub mod eventhub {
        include!("functions/eventhub/get_authorization_rule.rs");
        include!("functions/eventhub/get_cluster.rs");
        include!("functions/eventhub/get_consume_group.rs");
        include!("functions/eventhub/get_event_hub.rs");
        include!("functions/eventhub/get_eventhub_namespace.rs");
        include!("functions/eventhub/get_namespace.rs");
        include!("functions/eventhub/get_namespace_authorization_rule.rs");
        include!("functions/eventhub/get_sas.rs");
        include!("functions/eventhub/get_service_bus_namespace.rs");
    }
}
pub mod types {
    pub mod eventhub {
        include!("types/eventhub/domain_identity.rs");
        include!("types/eventhub/domain_inbound_ip_rule.rs");
        include!("types/eventhub/domain_input_mapping_default_values.rs");
        include!("types/eventhub/domain_input_mapping_fields.rs");
        include!("types/eventhub/event_grid_topic_identity.rs");
        include!("types/eventhub/event_grid_topic_inbound_ip_rule.rs");
        include!("types/eventhub/event_grid_topic_input_mapping_default_values.rs");
        include!("types/eventhub/event_grid_topic_input_mapping_fields.rs");
        include!("types/eventhub/event_hub_capture_description.rs");
        include!("types/eventhub/event_hub_capture_description_destination.rs");
        include!("types/eventhub/event_hub_namespace_identity.rs");
        include!("types/eventhub/event_hub_namespace_network_rulesets.rs");
        include!("types/eventhub/event_hub_namespace_network_rulesets_ip_rule.rs");
        include!(
            "types/eventhub/event_hub_namespace_network_rulesets_virtual_network_rule.rs"
        );
        include!("types/eventhub/event_subscription_advanced_filter.rs");
        include!("types/eventhub/event_subscription_advanced_filter_bool_equal.rs");
        include!("types/eventhub/event_subscription_advanced_filter_is_not_null.rs");
        include!(
            "types/eventhub/event_subscription_advanced_filter_is_null_or_undefined.rs"
        );
        include!(
            "types/eventhub/event_subscription_advanced_filter_number_greater_than.rs"
        );
        include!(
            "types/eventhub/event_subscription_advanced_filter_number_greater_than_or_equal.rs"
        );
        include!("types/eventhub/event_subscription_advanced_filter_number_in.rs");
        include!("types/eventhub/event_subscription_advanced_filter_number_in_range.rs");
        include!(
            "types/eventhub/event_subscription_advanced_filter_number_less_than.rs"
        );
        include!(
            "types/eventhub/event_subscription_advanced_filter_number_less_than_or_equal.rs"
        );
        include!("types/eventhub/event_subscription_advanced_filter_number_not_in.rs");
        include!(
            "types/eventhub/event_subscription_advanced_filter_number_not_in_range.rs"
        );
        include!(
            "types/eventhub/event_subscription_advanced_filter_string_begins_with.rs"
        );
        include!("types/eventhub/event_subscription_advanced_filter_string_contain.rs");
        include!(
            "types/eventhub/event_subscription_advanced_filter_string_ends_with.rs"
        );
        include!("types/eventhub/event_subscription_advanced_filter_string_in.rs");
        include!(
            "types/eventhub/event_subscription_advanced_filter_string_not_begins_with.rs"
        );
        include!(
            "types/eventhub/event_subscription_advanced_filter_string_not_contain.rs"
        );
        include!(
            "types/eventhub/event_subscription_advanced_filter_string_not_ends_with.rs"
        );
        include!("types/eventhub/event_subscription_advanced_filter_string_not_in.rs");
        include!("types/eventhub/event_subscription_azure_function_endpoint.rs");
        include!("types/eventhub/event_subscription_dead_letter_identity.rs");
        include!("types/eventhub/event_subscription_delivery_identity.rs");
        include!("types/eventhub/event_subscription_delivery_property.rs");
        include!("types/eventhub/event_subscription_retry_policy.rs");
        include!(
            "types/eventhub/event_subscription_storage_blob_dead_letter_destination.rs"
        );
        include!("types/eventhub/event_subscription_storage_queue_endpoint.rs");
        include!("types/eventhub/event_subscription_subject_filter.rs");
        include!("types/eventhub/event_subscription_webhook_endpoint.rs");
        include!("types/eventhub/namespace_customer_managed_key.rs");
        include!("types/eventhub/namespace_identity.rs");
        include!("types/eventhub/namespace_network_rule_set.rs");
        include!("types/eventhub/namespace_network_rule_set_network_rule.rs");
        include!("types/eventhub/subscription_client_scoped_subscription.rs");
        include!("types/eventhub/subscription_rule_correlation_filter.rs");
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
