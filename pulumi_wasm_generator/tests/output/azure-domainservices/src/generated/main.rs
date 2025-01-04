pub mod domainservices {
    include!("resources/domainservices/replica_set.rs");
    include!("resources/domainservices/service.rs");
    include!("resources/domainservices/service_trust.rs");
}
pub mod functions {
    pub mod domainservices {
        include!("functions/domainservices/get_service.rs");
    }
}
pub mod types {
    pub mod domainservices {
        include!("types/domainservices/service_initial_replica_set.rs");
        include!("types/domainservices/service_notifications.rs");
        include!("types/domainservices/service_secure_ldap.rs");
        include!("types/domainservices/service_security.rs");
        include!("types/domainservices/get_service_notification.rs");
        include!("types/domainservices/get_service_replica_set.rs");
        include!("types/domainservices/get_service_secure_ldap.rs");
        include!("types/domainservices/get_service_security.rs");
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
