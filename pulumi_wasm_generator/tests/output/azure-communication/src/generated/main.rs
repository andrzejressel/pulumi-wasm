pub mod communication {
    include!("resources/communication/email_service.rs");
    include!("resources/communication/email_service_domain.rs");
    include!("resources/communication/service.rs");
    include!("resources/communication/service_email_domain_association.rs");
}
pub mod functions {
    pub mod communication {
        include!("functions/communication/get_service.rs");
    }
}
pub mod types {
    pub mod communication {
        include!("types/communication/email_service_domain_verification_record.rs");
        include!("types/communication/email_service_domain_verification_record_dkim.rs");
        include!(
            "types/communication/email_service_domain_verification_record_dkim_2.rs"
        );
        include!(
            "types/communication/email_service_domain_verification_record_dmarc.rs"
        );
        include!(
            "types/communication/email_service_domain_verification_record_domain.rs"
        );
        include!("types/communication/email_service_domain_verification_record_spf.rs");
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
