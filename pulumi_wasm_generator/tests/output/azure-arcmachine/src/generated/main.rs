pub mod arcmachine {
    include!("resources/arcmachine/arc_machine.rs");
    include!("resources/arcmachine/automanage_configuration_assignment.rs");
    include!("resources/arcmachine/extension.rs");
}
pub mod functions {
    pub mod arcmachine {
        include!("functions/arcmachine/get.rs");
    }
}
pub mod types {
    pub mod arcmachine {
        include!("types/arcmachine/arc_machine_identity.rs");
        include!("types/arcmachine/get_agent.rs");
        include!("types/arcmachine/get_agent_extensions_allow_list.rs");
        include!("types/arcmachine/get_agent_extensions_block_list.rs");
        include!("types/arcmachine/get_cloud_metadata.rs");
        include!("types/arcmachine/get_identity.rs");
        include!("types/arcmachine/get_location_data.rs");
        include!("types/arcmachine/get_os_profile.rs");
        include!("types/arcmachine/get_os_profile_linux.rs");
        include!("types/arcmachine/get_os_profile_linux_patch.rs");
        include!("types/arcmachine/get_os_profile_window.rs");
        include!("types/arcmachine/get_os_profile_window_patch.rs");
        include!("types/arcmachine/get_service_status.rs");
        include!("types/arcmachine/get_service_status_extension_service.rs");
        include!("types/arcmachine/get_service_status_guest_configuration_service.rs");
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
