pub mod mysql {
    include!("resources/mysql/flexible_database.rs");
    include!("resources/mysql/flexible_server.rs");
    include!("resources/mysql/flexible_server_active_directory_administratory.rs");
    include!("resources/mysql/flexible_server_configuration.rs");
    include!("resources/mysql/flexible_server_firewall_rule.rs");
}
pub mod functions {
    pub mod mysql {
        include!("functions/mysql/get_flexible_server.rs");
    }
}
pub mod types {
    pub mod mysql {
        include!("types/mysql/flexible_server_customer_managed_key.rs");
        include!("types/mysql/flexible_server_high_availability.rs");
        include!("types/mysql/flexible_server_identity.rs");
        include!("types/mysql/flexible_server_maintenance_window.rs");
        include!("types/mysql/flexible_server_storage.rs");
        include!("types/mysql/get_flexible_server_high_availability.rs");
        include!("types/mysql/get_flexible_server_maintenance_window.rs");
        include!("types/mysql/get_flexible_server_storage.rs");
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
