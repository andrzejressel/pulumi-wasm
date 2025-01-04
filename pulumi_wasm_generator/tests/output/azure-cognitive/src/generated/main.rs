pub mod cognitive {
    include!("resources/cognitive/ai_services.rs");
    include!("resources/cognitive/account.rs");
    include!("resources/cognitive/account_customer_managed_key.rs");
    include!("resources/cognitive/account_rai_blocklist.rs");
    include!("resources/cognitive/deployment.rs");
}
pub mod functions {
    pub mod cognitive {
        include!("functions/cognitive/get_account.rs");
    }
}
pub mod types {
    pub mod cognitive {
        include!("types/cognitive/ai_services_customer_managed_key.rs");
        include!("types/cognitive/ai_services_identity.rs");
        include!("types/cognitive/ai_services_network_acls.rs");
        include!("types/cognitive/ai_services_network_acls_virtual_network_rule.rs");
        include!("types/cognitive/ai_services_storage.rs");
        include!("types/cognitive/account_customer_managed_key.rs");
        include!("types/cognitive/account_identity.rs");
        include!("types/cognitive/account_network_acls.rs");
        include!("types/cognitive/account_network_acls_virtual_network_rule.rs");
        include!("types/cognitive/account_storage.rs");
        include!("types/cognitive/deployment_model.rs");
        include!("types/cognitive/deployment_sku.rs");
        include!("types/cognitive/get_account_identity.rs");
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
