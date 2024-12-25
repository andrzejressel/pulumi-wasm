/// test new feature with resoruces
pub mod foo {
    include!("resources/foo.rs");
}
pub mod module_test {
    include!("resources/module_test.rs");
}
pub mod functions {
    /// Check codegen of functions with all optional inputs.
    pub mod func_with_all_optional_inputs {
        include!("functions/func_with_all_optional_inputs.rs");
    }
}
pub mod types {
    pub mod mod1 {
        include!("types/mod1/typ.rs");
    }
    pub mod mod2 {
        include!("types/mod2/typ.rs");
    }
    include!("types/helm_release_settings.rs");
    include!("types/kube_client_settings.rs");
    include!("types/layered_type.rs");
    include!("types/typ.rs");
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-example {
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
        pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface } }
    );
}
