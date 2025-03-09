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
    pulumi_gestalt_rust::__private::constant::generate_string_const!(
        ConstStringEnvironment, "Environment"
    );
    pulumi_gestalt_rust::__private::constant::generate_string_const!(
        ConstStringFixed, "Fixed"
    );
    pulumi_gestalt_rust::__private::constant::generate_string_const!(
        ConstStringFolder, "Folder"
    );
    pulumi_gestalt_rust::__private::constant::generate_string_const!(
        ConstStringPackage, "Package"
    );
    pulumi_gestalt_rust::__private::constant::generate_string_const!(
        ConstStringProject, "Project"
    );
}
#[unsafe(link_section = "pulumi_gestalt_provider::mypkg")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_MYPKG: [u8; 44] = *b"{\"version\":\"0.0.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "0.0.1".to_string()
}
