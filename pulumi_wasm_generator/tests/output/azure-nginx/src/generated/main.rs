pub mod nginx {
    include!("resources/nginx/certificate.rs");
    include!("resources/nginx/configuration.rs");
    include!("resources/nginx/deployment.rs");
}
pub mod functions {
    pub mod nginx {
        include!("functions/nginx/get_certificate.rs");
        include!("functions/nginx/get_configuration.rs");
        include!("functions/nginx/get_deployment.rs");
    }
}
pub mod types {
    pub mod nginx {
        include!("types/nginx/configuration_config_file.rs");
        include!("types/nginx/configuration_protected_file.rs");
        include!("types/nginx/deployment_auto_scale_profile.rs");
        include!("types/nginx/deployment_frontend_private.rs");
        include!("types/nginx/deployment_frontend_public.rs");
        include!("types/nginx/deployment_identity.rs");
        include!("types/nginx/deployment_logging_storage_account.rs");
        include!("types/nginx/deployment_network_interface.rs");
        include!("types/nginx/get_configuration_config_file.rs");
        include!("types/nginx/get_configuration_protected_file.rs");
        include!("types/nginx/get_deployment_auto_scale_profile.rs");
        include!("types/nginx/get_deployment_frontend_private.rs");
        include!("types/nginx/get_deployment_frontend_public.rs");
        include!("types/nginx/get_deployment_identity.rs");
        include!("types/nginx/get_deployment_logging_storage_account.rs");
        include!("types/nginx/get_deployment_network_interface.rs");
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
