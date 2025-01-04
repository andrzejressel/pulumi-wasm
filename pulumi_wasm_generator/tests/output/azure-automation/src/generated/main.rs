pub mod automation {
    include!("resources/automation/account.rs");
    include!("resources/automation/bool_variable.rs");
    include!("resources/automation/certificate.rs");
    include!("resources/automation/connection.rs");
    include!("resources/automation/connection_certificate.rs");
    include!("resources/automation/connection_classic_certificate.rs");
    include!("resources/automation/connection_service_principal.rs");
    include!("resources/automation/connection_type.rs");
    include!("resources/automation/credential.rs");
    include!("resources/automation/date_time_variable.rs");
    include!("resources/automation/dsc_configuration.rs");
    include!("resources/automation/dsc_node_configuration.rs");
    include!("resources/automation/hybrid_runbook_worker.rs");
    include!("resources/automation/hybrid_runbook_worker_group.rs");
    include!("resources/automation/int_variable.rs");
    include!("resources/automation/job_schedule.rs");
    include!("resources/automation/module.rs");
    include!("resources/automation/powershell_72_module.rs");
    include!("resources/automation/python_3_package.rs");
    include!("resources/automation/run_book.rs");
    include!("resources/automation/schedule.rs");
    include!("resources/automation/software_update_configuration.rs");
    include!("resources/automation/source_control.rs");
    include!("resources/automation/string_variable.rs");
    include!("resources/automation/variable_object.rs");
    include!("resources/automation/watcher.rs");
    include!("resources/automation/webhook.rs");
}
pub mod functions {
    pub mod automation {
        include!("functions/automation/get_account.rs");
        include!("functions/automation/get_bool_variable.rs");
        include!("functions/automation/get_date_time_variable.rs");
        include!("functions/automation/get_int_variable.rs");
        include!("functions/automation/get_runbook.rs");
        include!("functions/automation/get_string_variable.rs");
        include!("functions/automation/get_variable_object.rs");
        include!("functions/automation/get_variables.rs");
    }
}
pub mod types {
    pub mod automation {
        include!("types/automation/account_encryption.rs");
        include!("types/automation/account_identity.rs");
        include!("types/automation/account_private_endpoint_connection.rs");
        include!("types/automation/connection_type_field.rs");
        include!("types/automation/module_module_link.rs");
        include!("types/automation/module_module_link_hash.rs");
        include!("types/automation/powershell_72_module_module_link.rs");
        include!("types/automation/powershell_72_module_module_link_hash.rs");
        include!("types/automation/run_book_draft.rs");
        include!("types/automation/run_book_draft_content_link.rs");
        include!("types/automation/run_book_draft_content_link_hash.rs");
        include!("types/automation/run_book_draft_parameter.rs");
        include!("types/automation/run_book_job_schedule.rs");
        include!("types/automation/run_book_publish_content_link.rs");
        include!("types/automation/run_book_publish_content_link_hash.rs");
        include!("types/automation/schedule_monthly_occurrence.rs");
        include!("types/automation/software_update_configuration_linux.rs");
        include!("types/automation/software_update_configuration_post_task.rs");
        include!("types/automation/software_update_configuration_pre_task.rs");
        include!("types/automation/software_update_configuration_schedule.rs");
        include!(
            "types/automation/software_update_configuration_schedule_monthly_occurrence.rs"
        );
        include!("types/automation/software_update_configuration_target.rs");
        include!("types/automation/software_update_configuration_target_azure_query.rs");
        include!(
            "types/automation/software_update_configuration_target_azure_query_tag.rs"
        );
        include!(
            "types/automation/software_update_configuration_target_non_azure_query.rs"
        );
        include!("types/automation/software_update_configuration_windows.rs");
        include!("types/automation/source_control_security.rs");
        include!("types/automation/get_account_identity.rs");
        include!("types/automation/get_account_private_endpoint_connection.rs");
        include!("types/automation/get_variables_bool.rs");
        include!("types/automation/get_variables_datetime.rs");
        include!("types/automation/get_variables_encrypted.rs");
        include!("types/automation/get_variables_int.rs");
        include!("types/automation/get_variables_null.rs");
        include!("types/automation/get_variables_object.rs");
        include!("types/automation/get_variables_string.rs");
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
