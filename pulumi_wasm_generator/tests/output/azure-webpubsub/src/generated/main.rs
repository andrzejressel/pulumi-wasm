pub mod webpubsub {
    include!("resources/webpubsub/custom_certificate.rs");
    include!("resources/webpubsub/custom_domain.rs");
    include!("resources/webpubsub/hub.rs");
    include!("resources/webpubsub/network_acl.rs");
    include!("resources/webpubsub/service.rs");
    include!("resources/webpubsub/shared_private_link_resource.rs");
}
pub mod functions {
    pub mod webpubsub {
        include!("functions/webpubsub/get_private_link_resource.rs");
        include!("functions/webpubsub/get_service.rs");
    }
}
pub mod types {
    pub mod webpubsub {
        include!("types/webpubsub/hub_event_handler.rs");
        include!("types/webpubsub/hub_event_handler_auth.rs");
        include!("types/webpubsub/hub_event_listener.rs");
        include!("types/webpubsub/network_acl_private_endpoint.rs");
        include!("types/webpubsub/network_acl_public_network.rs");
        include!("types/webpubsub/service_identity.rs");
        include!("types/webpubsub/service_live_trace.rs");
        include!(
            "types/webpubsub/get_private_link_resource_shared_private_link_resource_type.rs"
        );
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
