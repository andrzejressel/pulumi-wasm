pub mod frontdoor {
    include!("resources/frontdoor/custom_https_configuration.rs");
    include!("resources/frontdoor/firewall_policy.rs");
    include!("resources/frontdoor/frontdoor.rs");
    include!("resources/frontdoor/rules_engine.rs");
}
pub mod functions {}
pub mod types {
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
