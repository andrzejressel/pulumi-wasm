pub mod cdn {
    include!("resources/cdn/endpoint.rs");
    include!("resources/cdn/endpoint_custom_domain.rs");
    include!("resources/cdn/frontdoor_custom_domain.rs");
    include!("resources/cdn/frontdoor_custom_domain_association.rs");
    include!("resources/cdn/frontdoor_endpoint.rs");
    include!("resources/cdn/frontdoor_firewall_policy.rs");
    include!("resources/cdn/frontdoor_origin.rs");
    include!("resources/cdn/frontdoor_origin_group.rs");
    include!("resources/cdn/frontdoor_profile.rs");
    include!("resources/cdn/frontdoor_route.rs");
    include!("resources/cdn/frontdoor_rule.rs");
    include!("resources/cdn/frontdoor_rule_set.rs");
    include!("resources/cdn/frontdoor_secret.rs");
    include!("resources/cdn/frontdoor_security_policy.rs");
    include!("resources/cdn/profile.rs");
}
pub mod functions {
    pub mod cdn {
        include!("functions/cdn/get_frontdoor_custom_domain.rs");
        include!("functions/cdn/get_frontdoor_endpoint.rs");
        include!("functions/cdn/get_frontdoor_firewall_policy.rs");
        include!("functions/cdn/get_frontdoor_origin_group.rs");
        include!("functions/cdn/get_frontdoor_profile.rs");
        include!("functions/cdn/get_frontdoor_rule_set.rs");
        include!("functions/cdn/get_frontdoor_secret.rs");
        include!("functions/cdn/get_profile.rs");
    }
}
pub mod types {
    pub mod cdn {
        include!("types/cdn/endpoint_custom_domain_cdn_managed_https.rs");
        include!("types/cdn/endpoint_custom_domain_user_managed_https.rs");
        include!("types/cdn/endpoint_delivery_rule.rs");
        include!("types/cdn/endpoint_delivery_rule_cache_expiration_action.rs");
        include!("types/cdn/endpoint_delivery_rule_cache_key_query_string_action.rs");
        include!("types/cdn/endpoint_delivery_rule_cookies_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_device_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_http_version_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_modify_request_header_action.rs");
        include!("types/cdn/endpoint_delivery_rule_modify_response_header_action.rs");
        include!("types/cdn/endpoint_delivery_rule_post_arg_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_query_string_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_remote_address_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_request_body_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_request_header_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_request_method_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_request_scheme_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_request_uri_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_url_file_extension_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_url_file_name_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_url_path_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_url_redirect_action.rs");
        include!("types/cdn/endpoint_delivery_rule_url_rewrite_action.rs");
        include!("types/cdn/endpoint_geo_filter.rs");
        include!("types/cdn/endpoint_global_delivery_rule.rs");
        include!("types/cdn/endpoint_global_delivery_rule_cache_expiration_action.rs");
        include!(
            "types/cdn/endpoint_global_delivery_rule_cache_key_query_string_action.rs"
        );
        include!(
            "types/cdn/endpoint_global_delivery_rule_modify_request_header_action.rs"
        );
        include!(
            "types/cdn/endpoint_global_delivery_rule_modify_response_header_action.rs"
        );
        include!("types/cdn/endpoint_global_delivery_rule_url_redirect_action.rs");
        include!("types/cdn/endpoint_global_delivery_rule_url_rewrite_action.rs");
        include!("types/cdn/endpoint_origin.rs");
        include!("types/cdn/frontdoor_custom_domain_tls.rs");
        include!("types/cdn/frontdoor_firewall_policy_custom_rule.rs");
        include!("types/cdn/frontdoor_firewall_policy_custom_rule_match_condition.rs");
        include!("types/cdn/frontdoor_firewall_policy_managed_rule.rs");
        include!("types/cdn/frontdoor_firewall_policy_managed_rule_exclusion.rs");
        include!("types/cdn/frontdoor_firewall_policy_managed_rule_override.rs");
        include!(
            "types/cdn/frontdoor_firewall_policy_managed_rule_override_exclusion.rs"
        );
        include!("types/cdn/frontdoor_firewall_policy_managed_rule_override_rule.rs");
        include!(
            "types/cdn/frontdoor_firewall_policy_managed_rule_override_rule_exclusion.rs"
        );
        include!("types/cdn/frontdoor_origin_group_health_probe.rs");
        include!("types/cdn/frontdoor_origin_group_load_balancing.rs");
        include!("types/cdn/frontdoor_origin_private_link.rs");
        include!("types/cdn/frontdoor_route_cache.rs");
        include!("types/cdn/frontdoor_rule_actions.rs");
        include!("types/cdn/frontdoor_rule_actions_request_header_action.rs");
        include!("types/cdn/frontdoor_rule_actions_response_header_action.rs");
        include!(
            "types/cdn/frontdoor_rule_actions_route_configuration_override_action.rs"
        );
        include!("types/cdn/frontdoor_rule_actions_url_redirect_action.rs");
        include!("types/cdn/frontdoor_rule_actions_url_rewrite_action.rs");
        include!("types/cdn/frontdoor_rule_conditions.rs");
        include!("types/cdn/frontdoor_rule_conditions_client_port_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_cookies_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_host_name_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_http_version_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_is_device_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_post_args_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_query_string_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_remote_address_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_request_body_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_request_header_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_request_method_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_request_scheme_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_request_uri_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_server_port_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_socket_address_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_ssl_protocol_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_url_file_extension_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_url_filename_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_url_path_condition.rs");
        include!("types/cdn/frontdoor_secret_secret.rs");
        include!("types/cdn/frontdoor_secret_secret_customer_certificate.rs");
        include!("types/cdn/frontdoor_security_policy_security_policies.rs");
        include!("types/cdn/frontdoor_security_policy_security_policies_firewall.rs");
        include!(
            "types/cdn/frontdoor_security_policy_security_policies_firewall_association.rs"
        );
        include!(
            "types/cdn/frontdoor_security_policy_security_policies_firewall_association_domain.rs"
        );
        include!("types/cdn/get_frontdoor_custom_domain_tl.rs");
        include!("types/cdn/get_frontdoor_origin_group_health_probe.rs");
        include!("types/cdn/get_frontdoor_origin_group_load_balancing.rs");
        include!("types/cdn/get_frontdoor_secret_secret.rs");
        include!("types/cdn/get_frontdoor_secret_secret_customer_certificate.rs");
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
