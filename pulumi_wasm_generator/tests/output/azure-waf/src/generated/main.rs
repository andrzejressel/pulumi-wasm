pub mod waf {
    include!("resources/waf/policy.rs");
}
pub mod functions {
    pub mod waf {
        include!("functions/waf/get_firewall_policy.rs");
    }
}
pub mod types {
    pub mod waf {
        include!("types/waf/policy_custom_rule.rs");
        include!("types/waf/policy_custom_rule_match_condition.rs");
        include!("types/waf/policy_custom_rule_match_condition_match_variable.rs");
        include!("types/waf/policy_managed_rules.rs");
        include!("types/waf/policy_managed_rules_exclusion.rs");
        include!("types/waf/policy_managed_rules_exclusion_excluded_rule_set.rs");
        include!(
            "types/waf/policy_managed_rules_exclusion_excluded_rule_set_rule_group.rs"
        );
        include!("types/waf/policy_managed_rules_managed_rule_set.rs");
        include!(
            "types/waf/policy_managed_rules_managed_rule_set_rule_group_override.rs"
        );
        include!(
            "types/waf/policy_managed_rules_managed_rule_set_rule_group_override_rule.rs"
        );
        include!("types/waf/policy_policy_settings.rs");
        include!("types/waf/policy_policy_settings_log_scrubbing.rs");
        include!("types/waf/policy_policy_settings_log_scrubbing_rule.rs");
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
