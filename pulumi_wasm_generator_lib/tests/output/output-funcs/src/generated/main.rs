pub mod functions {
    /// Check codegen of functions with all optional inputs.
    pub mod func_with_all_optional_inputs {
        include!("functions/func_with_all_optional_inputs.rs");
    }
    /// Codegen demo with const inputs
    pub mod func_with_const_input {
        include!("functions/func_with_const_input.rs");
    }
    /// Check codegen of functions with default values.
    pub mod func_with_default_value {
        include!("functions/func_with_default_value.rs");
    }
    /// Check codegen of functions with a Dict<str,str> parameter.
    pub mod func_with_dict_param {
        include!("functions/func_with_dict_param.rs");
    }
    /// n/a
    pub mod func_with_empty_outputs {
        include!("functions/func_with_empty_outputs.rs");
    }
    /// Check codegen of functions with a List parameter.
    pub mod func_with_list_param {
        include!("functions/func_with_list_param.rs");
    }
    /// Response for all the Bastion Shareable Link endpoints.
    /// API Version: 2020-11-01.
    pub mod get_bastion_shareable_link {
        include!("functions/get_bastion_shareable_link.rs");
    }
    /// Failing example taken from azure-native. Original doc: Use this function to access the current configuration of the native Azure provider.
    pub mod get_client_config {
        include!("functions/get_client_config.rs");
    }
    /// Another failing example. A list of SSIS object metadata.
    /// API Version: 2018-06-01.
    pub mod get_integration_runtime_object_metadatum {
        include!("functions/get_integration_runtime_object_metadatum.rs");
    }
    /// The response from the ListKeys operation.
    /// API Version: 2021-02-01.
    pub mod list_storage_account_keys {
        include!("functions/list_storage_account_keys.rs");
    }
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
    pulumi_wasm_provider_common::generate_string_const!(
        ConstStringEnvironment, "Environment"
    );
    pulumi_wasm_provider_common::generate_string_const!(ConstStringFixed, "Fixed");
    pulumi_wasm_provider_common::generate_string_const!(ConstStringFolder, "Folder");
    pulumi_wasm_provider_common::generate_string_const!(ConstStringPackage, "Package");
    pulumi_wasm_provider_common::generate_string_const!(ConstStringProject, "Project");
}
mod bindings {
    wit_bindgen::generate!(
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
        pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface } }
    );
}
