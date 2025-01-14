pub mod functions {
    include!("functions/func_with_all_optional_inputs.rs");
    include!("functions/func_with_const_input.rs");
    include!("functions/func_with_default_value.rs");
    include!("functions/func_with_dict_param.rs");
    include!("functions/func_with_empty_outputs.rs");
    include!("functions/func_with_list_param.rs");
    include!("functions/get_bastion_shareable_link.rs");
    include!("functions/get_client_config.rs");
    include!("functions/get_integration_runtime_object_metadatum.rs");
    include!("functions/list_storage_account_keys.rs");
}
pub mod types {
    include!("types/bastion_shareable_link.rs");
    include!("types/ssis_environment_reference_response.rs");
    include!("types/ssis_environment_response.rs");
    include!("types/ssis_folder_response.rs");
    include!("types/ssis_package_response.rs");
    include!("types/ssis_parameter_response.rs");
    include!("types/ssis_project_response.rs");
    include!("types/ssis_variable_response.rs");
    include!("types/storage_account_key_response.rs");
}
#[doc(hidden)]
pub mod constants {
    pulumi_wasm_rust::__private::constant::generate_string_const!(
        ConstStringEnvironment, "Environment"
    );
    pulumi_wasm_rust::__private::constant::generate_string_const!(
        ConstStringFixed, "Fixed"
    );
    pulumi_wasm_rust::__private::constant::generate_string_const!(
        ConstStringFolder, "Folder"
    );
    pulumi_wasm_rust::__private::constant::generate_string_const!(
        ConstStringPackage, "Package"
    );
    pulumi_wasm_rust::__private::constant::generate_string_const!(
        ConstStringProject, "Project"
    );
}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-mypkg {
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
#[link_section = "pulumi_wasm_provider::mypkg"]
#[no_mangle]
pub static PULUMI_WASM_PROVIDER_mypkg: [u8; 5] = *b"0.0.1";
