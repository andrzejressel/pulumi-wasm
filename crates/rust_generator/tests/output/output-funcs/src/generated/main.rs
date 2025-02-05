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
        with : { "component:pulumi-wasm/output-interface@0.0.0-DEV" :
        pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
        } }
    );
}
#[link_section = "pulumi_wasm_provider::mypkg"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_MYPKG: [u8; 44] = *b"{\"version\":\"0.0.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "0.0.1".to_string()
}
