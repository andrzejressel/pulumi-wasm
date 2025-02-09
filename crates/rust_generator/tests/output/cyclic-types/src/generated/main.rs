pub mod functions {}
pub mod types {
    include!("types/acyclic_referent.rs");
    include!("types/acyclic_s.rs");
    include!("types/acyclic_t.rs");
    include!("types/direct_cycle.rs");
    include!("types/indirect_cycle_s.rs");
    include!("types/indirect_cycle_t.rs");
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_gestalt_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-gestalt@0.0.0-DEV;

world world-example {
    import output-interface;
}

interface pulumi-engine {
    resource engine {
        constructor(in-preview: bool);
    }
}

interface output-interface {
    use pulumi-engine.{engine};

    resource output {
        constructor(engine: borrow<engine>, value: string, secret: bool);
        map: func(function-name: string) -> output;
        clone: func() -> output;
    }
    combine: func(outputs: list<borrow<output>>) -> output;

    resource register-output {
        extract-field: func(field-name: string) -> output;
    }
}

interface register-interface {
    use pulumi-engine.{engine};
    use output-interface.{output, register-output};

    record object-field {
        name: string,
        value: borrow<output>
    }

    record register-resource-request {
        %type: string,
        name: string,
        version: string,
        object: list<object-field>,
    }

    register: func(engine: borrow<engine>, request: register-resource-request) -> register-output;

    record resource-invoke-request {
        token: string,
        version: string,
        object: list<object-field>,
    }

    invoke: func(engine: borrow<engine>, request: resource-invoke-request) -> register-output;
}",
        with : { "component:pulumi-gestalt/output-interface@0.0.0-DEV" :
        pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::output_interface
        } }
    );
}
#[link_section = "pulumi_gestalt_provider::example"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_EXAMPLE: [u8; 44] = *b"{\"version\":\"0.0.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "0.0.1".to_string()
}
