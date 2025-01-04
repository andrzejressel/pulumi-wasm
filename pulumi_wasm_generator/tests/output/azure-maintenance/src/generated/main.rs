pub mod maintenance {
    include!("resources/maintenance/assignment_dedicated_host.rs");
    include!("resources/maintenance/assignment_dynamic_scope.rs");
    include!("resources/maintenance/assignment_virtual_machine.rs");
    include!("resources/maintenance/assignment_virtual_machine_scale_set.rs");
    include!("resources/maintenance/configuration.rs");
}
pub mod functions {
    pub mod maintenance {
        include!("functions/maintenance/get_configuration.rs");
        include!("functions/maintenance/get_public_configurations.rs");
    }
}
pub mod types {
    pub mod maintenance {
        include!("types/maintenance/assignment_dynamic_scope_filter.rs");
        include!("types/maintenance/assignment_dynamic_scope_filter_tag.rs");
        include!("types/maintenance/configuration_install_patches.rs");
        include!("types/maintenance/configuration_install_patches_linux.rs");
        include!("types/maintenance/configuration_install_patches_window.rs");
        include!("types/maintenance/configuration_window.rs");
        include!("types/maintenance/get_configuration_install_patch.rs");
        include!("types/maintenance/get_configuration_install_patch_linux.rs");
        include!("types/maintenance/get_configuration_install_patch_window.rs");
        include!("types/maintenance/get_configuration_window.rs");
        include!("types/maintenance/get_public_configurations_config.rs");
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
