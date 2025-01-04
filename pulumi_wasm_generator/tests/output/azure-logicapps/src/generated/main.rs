pub mod logicapps {
    include!("resources/logicapps/action_custom.rs");
    include!("resources/logicapps/action_http.rs");
    include!("resources/logicapps/integration_account.rs");
    include!("resources/logicapps/integration_account_agreement.rs");
    include!("resources/logicapps/integration_account_assembly.rs");
    include!("resources/logicapps/integration_account_batch_configuration.rs");
    include!("resources/logicapps/integration_account_certificate.rs");
    include!("resources/logicapps/integration_account_map.rs");
    include!("resources/logicapps/integration_account_partner.rs");
    include!("resources/logicapps/integration_account_schema.rs");
    include!("resources/logicapps/integration_account_session.rs");
    include!("resources/logicapps/standard.rs");
    include!("resources/logicapps/trigger_custom.rs");
    include!("resources/logicapps/trigger_http_request.rs");
    include!("resources/logicapps/trigger_recurrence.rs");
    include!("resources/logicapps/workflow.rs");
}
pub mod functions {
    pub mod logicapps {
        include!("functions/logicapps/get_integration_account.rs");
        include!("functions/logicapps/get_standard.rs");
        include!("functions/logicapps/get_workflow.rs");
    }
}
pub mod types {
    pub mod logicapps {
        include!("types/logicapps/action_http_run_after.rs");
        include!("types/logicapps/integration_account_agreement_guest_identity.rs");
        include!("types/logicapps/integration_account_agreement_host_identity.rs");
        include!(
            "types/logicapps/integration_account_batch_configuration_release_criteria.rs"
        );
        include!(
            "types/logicapps/integration_account_batch_configuration_release_criteria_recurrence.rs"
        );
        include!(
            "types/logicapps/integration_account_batch_configuration_release_criteria_recurrence_schedule.rs"
        );
        include!(
            "types/logicapps/integration_account_batch_configuration_release_criteria_recurrence_schedule_monthly.rs"
        );
        include!("types/logicapps/integration_account_certificate_key_vault_key.rs");
        include!("types/logicapps/integration_account_partner_business_identity.rs");
        include!("types/logicapps/standard_connection_string.rs");
        include!("types/logicapps/standard_identity.rs");
        include!("types/logicapps/standard_site_config.rs");
        include!("types/logicapps/standard_site_config_cors.rs");
        include!("types/logicapps/standard_site_config_ip_restriction.rs");
        include!("types/logicapps/standard_site_config_ip_restriction_headers.rs");
        include!("types/logicapps/standard_site_config_scm_ip_restriction.rs");
        include!("types/logicapps/standard_site_config_scm_ip_restriction_headers.rs");
        include!("types/logicapps/standard_site_credential.rs");
        include!("types/logicapps/trigger_recurrence_schedule.rs");
        include!("types/logicapps/workflow_access_control.rs");
        include!("types/logicapps/workflow_access_control_action.rs");
        include!("types/logicapps/workflow_access_control_content.rs");
        include!("types/logicapps/workflow_access_control_trigger.rs");
        include!(
            "types/logicapps/workflow_access_control_trigger_open_authentication_policy.rs"
        );
        include!(
            "types/logicapps/workflow_access_control_trigger_open_authentication_policy_claim.rs"
        );
        include!("types/logicapps/workflow_access_control_workflow_management.rs");
        include!("types/logicapps/workflow_identity.rs");
        include!("types/logicapps/get_standard_connection_string.rs");
        include!("types/logicapps/get_standard_identity.rs");
        include!("types/logicapps/get_standard_site_config.rs");
        include!("types/logicapps/get_standard_site_config_cors.rs");
        include!("types/logicapps/get_standard_site_config_ip_restriction.rs");
        include!("types/logicapps/get_standard_site_config_ip_restriction_headers.rs");
        include!("types/logicapps/get_standard_site_config_scm_ip_restriction.rs");
        include!(
            "types/logicapps/get_standard_site_config_scm_ip_restriction_headers.rs"
        );
        include!("types/logicapps/get_standard_site_credential.rs");
        include!("types/logicapps/get_workflow_identity.rs");
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
