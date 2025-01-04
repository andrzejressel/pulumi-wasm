pub mod postgresql {
    include!("resources/postgresql/active_directory_administrator.rs");
    include!("resources/postgresql/configuration.rs");
    include!("resources/postgresql/database.rs");
    include!("resources/postgresql/firewall_rule.rs");
    include!("resources/postgresql/flexible_server.rs");
    include!("resources/postgresql/flexible_server_active_directory_administrator.rs");
    include!("resources/postgresql/flexible_server_configuration.rs");
    include!("resources/postgresql/flexible_server_database.rs");
    include!("resources/postgresql/flexible_server_firewall_rule.rs");
    include!("resources/postgresql/flexible_server_virtual_endpoint.rs");
    include!("resources/postgresql/server.rs");
    include!("resources/postgresql/server_key.rs");
    include!("resources/postgresql/virtual_network_rule.rs");
}
pub mod functions {
    pub mod postgresql {
        include!("functions/postgresql/get_flexible_server.rs");
        include!("functions/postgresql/get_server.rs");
    }
}
pub mod types {
    pub mod postgresql {
        include!("types/postgresql/flexible_server_authentication.rs");
        include!("types/postgresql/flexible_server_customer_managed_key.rs");
        include!("types/postgresql/flexible_server_high_availability.rs");
        include!("types/postgresql/flexible_server_identity.rs");
        include!("types/postgresql/flexible_server_maintenance_window.rs");
        include!("types/postgresql/server_identity.rs");
        include!("types/postgresql/server_threat_detection_policy.rs");
        include!("types/postgresql/get_server_identity.rs");
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
