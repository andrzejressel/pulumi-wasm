pub mod tree {
    pub mod v1 {
        include!("resources/tree/v1/nursery.rs");
        include!("resources/tree/v1/rubber_tree.rs");
    }
}
pub mod functions {}
pub mod types {
    pub mod tree {
        pub mod v1 {
            include!("types/tree/v1/diameter.rs");
            include!("types/tree/v1/farm.rs");
            include!("types/tree/v1/rubber_tree_variety.rs");
            include!("types/tree/v1/tree_size.rs");
        }
    }
    include!("types/cloud_audit_options_log_name.rs");
    include!("types/container.rs");
    include!("types/container_brightness.rs");
    include!("types/container_color.rs");
    include!("types/container_size.rs");
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-plant {
    import output-interface;
}

interface output-interface {

    resource output {
        constructor(value: string, secret: bool);
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
#[link_section = "pulumi_wasm_provider::plant"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_PLANT: [u8; 44] = *b"{\"version\":\"0.0.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "0.0.1".to_string()
}
