pub mod dynatrace {
    include!("resources/dynatrace/monitor.rs");
}
pub mod elasticcloud {
    include!("resources/elasticcloud/elasticsearch.rs");
}
pub mod elasticsan {
    include!("resources/elasticsan/elastic_san.rs");
    include!("resources/elasticsan/volume.rs");
    include!("resources/elasticsan/volume_group.rs");
}
pub mod eventgrid {
    include!("resources/eventgrid/domain.rs");
    include!("resources/eventgrid/domain_topic.rs");
    include!("resources/eventgrid/event_subscription.rs");
    include!("resources/eventgrid/namespace.rs");
    include!("resources/eventgrid/system_topic.rs");
    include!("resources/eventgrid/system_topic_event_subscription.rs");
    include!("resources/eventgrid/topic.rs");
}
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
pub mod extendedlocation {
    include!("resources/extendedlocation/custom_location.rs");
}
pub mod fluidrelay {
    include!("resources/fluidrelay/server.rs");
}
pub mod frontdoor {
    include!("resources/frontdoor/custom_https_configuration.rs");
    include!("resources/frontdoor/firewall_policy.rs");
    include!("resources/frontdoor/frontdoor.rs");
    include!("resources/frontdoor/rules_engine.rs");
}
pub mod graph {
    include!("resources/graph/services_account.rs");
}
pub mod functions {
    pub mod elasticcloud {
        include!("functions/elasticcloud/get_elasticsearch.rs");
    }
    pub mod elasticsan {
        include!("functions/elasticsan/get.rs");
        include!("functions/elasticsan/get_volume_group.rs");
        include!("functions/elasticsan/get_volume_snapshot.rs");
    }
    pub mod eventgrid {
        include!("functions/eventgrid/get_domain.rs");
        include!("functions/eventgrid/get_domain_topic.rs");
        include!("functions/eventgrid/get_system_topic.rs");
        include!("functions/eventgrid/get_topic.rs");
    }
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
    pub mod expressroute {
        include!("functions/expressroute/get_circuit_peering.rs");
    }
}
pub mod types {
    pub mod dynatrace {
        include!("types/dynatrace/monitor_identity.rs");
        include!("types/dynatrace/monitor_plan.rs");
        include!("types/dynatrace/monitor_user.rs");
    }
    pub mod elasticcloud {
        include!("types/elasticcloud/elasticsearch_logs.rs");
        include!("types/elasticcloud/elasticsearch_logs_filtering_tag.rs");
        include!("types/elasticcloud/get_elasticsearch_log.rs");
        include!("types/elasticcloud/get_elasticsearch_log_filtering_tag.rs");
    }
    pub mod elasticsan {
        include!("types/elasticsan/elastic_san_sku.rs");
        include!("types/elasticsan/volume_create_source.rs");
        include!("types/elasticsan/volume_group_encryption.rs");
        include!("types/elasticsan/volume_group_identity.rs");
        include!("types/elasticsan/volume_group_network_rule.rs");
        include!("types/elasticsan/get_skus.rs");
        include!("types/elasticsan/get_volume_group_encryption.rs");
        include!("types/elasticsan/get_volume_group_identity.rs");
        include!("types/elasticsan/get_volume_group_network_rule.rs");
    }
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
    pub mod extendedlocation {
        include!("types/extendedlocation/custom_location_authentication.rs");
    }
    pub mod fluidrelay {
        include!("types/fluidrelay/server_customer_managed_key.rs");
        include!("types/fluidrelay/server_identity.rs");
    }
    pub mod frontdoor {
        include!(
            "types/frontdoor/custom_https_configuration_custom_https_configuration.rs"
        );
        include!("types/frontdoor/firewall_policy_custom_rule.rs");
        include!("types/frontdoor/firewall_policy_custom_rule_match_condition.rs");
        include!("types/frontdoor/firewall_policy_managed_rule.rs");
        include!("types/frontdoor/firewall_policy_managed_rule_exclusion.rs");
        include!("types/frontdoor/firewall_policy_managed_rule_override.rs");
        include!("types/frontdoor/firewall_policy_managed_rule_override_exclusion.rs");
        include!("types/frontdoor/firewall_policy_managed_rule_override_rule.rs");
        include!(
            "types/frontdoor/firewall_policy_managed_rule_override_rule_exclusion.rs"
        );
        include!("types/frontdoor/frontdoor_backend_pool.rs");
        include!("types/frontdoor/frontdoor_backend_pool_backend.rs");
        include!("types/frontdoor/frontdoor_backend_pool_health_probe.rs");
        include!("types/frontdoor/frontdoor_backend_pool_load_balancing.rs");
        include!("types/frontdoor/frontdoor_backend_pool_setting.rs");
        include!("types/frontdoor/frontdoor_explicit_resource_order.rs");
        include!("types/frontdoor/frontdoor_frontend_endpoint.rs");
        include!("types/frontdoor/frontdoor_routing_rule.rs");
        include!("types/frontdoor/frontdoor_routing_rule_forwarding_configuration.rs");
        include!("types/frontdoor/frontdoor_routing_rule_redirect_configuration.rs");
        include!("types/frontdoor/rules_engine_rule.rs");
        include!("types/frontdoor/rules_engine_rule_action.rs");
        include!("types/frontdoor/rules_engine_rule_action_request_header.rs");
        include!("types/frontdoor/rules_engine_rule_action_response_header.rs");
        include!("types/frontdoor/rules_engine_rule_match_condition.rs");
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
