pub mod privatelink {
    include!("resources/privatelink/application_security_group_association.rs");
    include!("resources/privatelink/endpoint.rs");
}
pub mod functions {
    pub mod privatelink {
        include!("functions/privatelink/get_endpoint_connection.rs");
        include!("functions/privatelink/get_service.rs");
        include!("functions/privatelink/get_service_endpoint_connections.rs");
    }
}
pub mod types {
    pub mod privatelink {
        include!("types/privatelink/endpoint_custom_dns_config.rs");
        include!("types/privatelink/endpoint_ip_configuration.rs");
        include!("types/privatelink/endpoint_network_interface.rs");
        include!("types/privatelink/endpoint_private_dns_zone_config.rs");
        include!("types/privatelink/endpoint_private_dns_zone_config_record_set.rs");
        include!("types/privatelink/endpoint_private_dns_zone_group.rs");
        include!("types/privatelink/endpoint_private_service_connection.rs");
        include!("types/privatelink/get_endpoint_connection_network_interface.rs");
        include!(
            "types/privatelink/get_endpoint_connection_private_service_connection.rs"
        );
        include!(
            "types/privatelink/get_service_endpoint_connections_private_endpoint_connection.rs"
        );
        include!("types/privatelink/get_service_nat_ip_configuration.rs");
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
