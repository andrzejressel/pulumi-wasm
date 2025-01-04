pub mod privatedns {
    include!("resources/privatedns/aaaa_record.rs");
    include!("resources/privatedns/a_record.rs");
    include!("resources/privatedns/cname_record.rs");
    include!("resources/privatedns/link_service.rs");
    include!("resources/privatedns/mx_record.rs");
    include!("resources/privatedns/ptr_record.rs");
    include!("resources/privatedns/resolver.rs");
    include!("resources/privatedns/resolver_dns_forwarding_ruleset.rs");
    include!("resources/privatedns/resolver_forwarding_rule.rs");
    include!("resources/privatedns/resolver_inbound_endpoint.rs");
    include!("resources/privatedns/resolver_outbound_endpoint.rs");
    include!("resources/privatedns/resolver_virtual_network_link.rs");
    include!("resources/privatedns/srv_record.rs");
    include!("resources/privatedns/txt_record.rs");
    include!("resources/privatedns/zone.rs");
    include!("resources/privatedns/zone_virtual_network_link.rs");
}
pub mod functions {
    pub mod privatedns {
        include!("functions/privatedns/get_aaaa_record.rs");
        include!("functions/privatedns/get_a_record.rs");
        include!("functions/privatedns/get_cname_record.rs");
        include!("functions/privatedns/get_dns_zone.rs");
        include!("functions/privatedns/get_mx_record.rs");
        include!("functions/privatedns/get_ptr_record.rs");
        include!("functions/privatedns/get_resolver.rs");
        include!("functions/privatedns/get_resolver_dns_forwarding_ruleset.rs");
        include!("functions/privatedns/get_resolver_forwarding_rule.rs");
        include!("functions/privatedns/get_resolver_inbound_endpoint.rs");
        include!("functions/privatedns/get_resolver_outbound_endpoint.rs");
        include!("functions/privatedns/get_resolver_virtual_network_link.rs");
        include!("functions/privatedns/get_soa_record.rs");
        include!("functions/privatedns/get_srv_record.rs");
        include!("functions/privatedns/get_txt_record.rs");
        include!("functions/privatedns/get_zone_virtual_network_link.rs");
    }
}
pub mod types {
    pub mod privatedns {
        include!("types/privatedns/link_service_nat_ip_configuration.rs");
        include!("types/privatedns/mx_record_record.rs");
        include!("types/privatedns/resolver_forwarding_rule_target_dns_server.rs");
        include!("types/privatedns/resolver_inbound_endpoint_ip_configurations.rs");
        include!("types/privatedns/srv_record_record.rs");
        include!("types/privatedns/txt_record_record.rs");
        include!("types/privatedns/zone_soa_record.rs");
        include!("types/privatedns/get_mx_record_record.rs");
        include!("types/privatedns/get_resolver_forwarding_rule_target_dns_server.rs");
        include!("types/privatedns/get_resolver_inbound_endpoint_ip_configuration.rs");
        include!("types/privatedns/get_srv_record_record.rs");
        include!("types/privatedns/get_txt_record_record.rs");
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
