pub mod lb {
    include!("resources/lb/backend_address_pool.rs");
    include!("resources/lb/backend_address_pool_address.rs");
    include!("resources/lb/load_balancer.rs");
    include!("resources/lb/nat_pool.rs");
    include!("resources/lb/nat_rule.rs");
    include!("resources/lb/outbound_rule.rs");
    include!("resources/lb/probe.rs");
    include!("resources/lb/rule.rs");
}
pub mod functions {
    pub mod lb {
        include!("functions/lb/get_backend_address_pool.rs");
        include!("functions/lb/get_lb.rs");
        include!("functions/lb/get_lb_outbound_rule.rs");
        include!("functions/lb/get_lb_rule.rs");
    }
}
pub mod types {
    pub mod lb {
        include!(
            "types/lb/backend_address_pool_address_inbound_nat_rule_port_mapping.rs"
        );
        include!("types/lb/backend_address_pool_tunnel_interface.rs");
        include!("types/lb/load_balancer_frontend_ip_configuration.rs");
        include!("types/lb/outbound_rule_frontend_ip_configuration.rs");
        include!("types/lb/get_backend_address_pool_backend_address.rs");
        include!(
            "types/lb/get_backend_address_pool_backend_address_inbound_nat_rule_port_mapping.rs"
        );
        include!("types/lb/get_backend_address_pool_backend_ip_configuration.rs");
        include!("types/lb/get_lb_frontend_ip_configuration.rs");
        include!("types/lb/get_lb_outbound_rule_frontend_ip_configuration.rs");
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
