pub mod devcenter {
    include!("resources/devcenter/attached_network.rs");
    include!("resources/devcenter/catalog.rs");
    include!("resources/devcenter/dev_box_definition.rs");
    include!("resources/devcenter/dev_center.rs");
    include!("resources/devcenter/environment_type.rs");
    include!("resources/devcenter/gallery.rs");
    include!("resources/devcenter/network_connection.rs");
    include!("resources/devcenter/project.rs");
    include!("resources/devcenter/project_environment_type.rs");
    include!("resources/devcenter/project_pool.rs");
}
pub mod functions {}
pub mod types {
    pub mod devcenter {
        include!("types/devcenter/catalog_catalog_adogit.rs");
        include!("types/devcenter/catalog_catalog_github.rs");
        include!("types/devcenter/dev_center_identity.rs");
        include!("types/devcenter/project_environment_type_identity.rs");
        include!("types/devcenter/project_environment_type_user_role_assignment.rs");
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
