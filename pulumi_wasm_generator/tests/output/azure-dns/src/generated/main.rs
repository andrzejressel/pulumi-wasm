pub mod dns {
    include!("resources/dns/a_record.rs");
    include!("resources/dns/aaaa_record.rs");
    include!("resources/dns/c_name_record.rs");
    include!("resources/dns/caa_record.rs");
    include!("resources/dns/mx_record.rs");
    include!("resources/dns/ns_record.rs");
    include!("resources/dns/ptr_record.rs");
    include!("resources/dns/srv_record.rs");
    include!("resources/dns/txt_record.rs");
    include!("resources/dns/zone.rs");
}
pub mod functions {
    pub mod dns {
        include!("functions/dns/get_aaaa_record.rs");
        include!("functions/dns/get_a_record.rs");
        include!("functions/dns/get_caa_record.rs");
        include!("functions/dns/get_cname_record.rs");
        include!("functions/dns/get_mx_record.rs");
        include!("functions/dns/get_ns_record.rs");
        include!("functions/dns/get_ptr_record.rs");
        include!("functions/dns/get_soa_record.rs");
        include!("functions/dns/get_srv_record.rs");
        include!("functions/dns/get_txt_record.rs");
        include!("functions/dns/get_zone.rs");
    }
}
pub mod types {
    pub mod dns {
        include!("types/dns/caa_record_record.rs");
        include!("types/dns/mx_record_record.rs");
        include!("types/dns/srv_record_record.rs");
        include!("types/dns/txt_record_record.rs");
        include!("types/dns/zone_soa_record.rs");
        include!("types/dns/get_caa_record_record.rs");
        include!("types/dns/get_mx_record_record.rs");
        include!("types/dns/get_srv_record_record.rs");
        include!("types/dns/get_txt_record_record.rs");
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
