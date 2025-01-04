pub mod apimanagement {
    include!("resources/apimanagement/api.rs");
    include!("resources/apimanagement/api_diagnostic.rs");
    include!("resources/apimanagement/api_operation.rs");
    include!("resources/apimanagement/api_operation_policy.rs");
    include!("resources/apimanagement/api_operation_tag.rs");
    include!("resources/apimanagement/api_policy.rs");
    include!("resources/apimanagement/api_release.rs");
    include!("resources/apimanagement/api_schema.rs");
    include!("resources/apimanagement/api_tag.rs");
    include!("resources/apimanagement/api_tag_description.rs");
    include!("resources/apimanagement/api_version_set.rs");
    include!("resources/apimanagement/authorization_server.rs");
    include!("resources/apimanagement/backend.rs");
    include!("resources/apimanagement/certificate.rs");
    include!("resources/apimanagement/custom_domain.rs");
    include!("resources/apimanagement/diagnostic.rs");
    include!("resources/apimanagement/email_template.rs");
    include!("resources/apimanagement/gateway.rs");
    include!("resources/apimanagement/gateway_api.rs");
    include!("resources/apimanagement/gateway_certificate_authority.rs");
    include!("resources/apimanagement/gateway_host_name_configuration.rs");
    include!("resources/apimanagement/global_schema.rs");
    include!("resources/apimanagement/group.rs");
    include!("resources/apimanagement/group_user.rs");
    include!("resources/apimanagement/identity_provider_aad.rs");
    include!("resources/apimanagement/identity_provider_aadb_2_c.rs");
    include!("resources/apimanagement/identity_provider_facebook.rs");
    include!("resources/apimanagement/identity_provider_google.rs");
    include!("resources/apimanagement/identity_provider_microsoft.rs");
    include!("resources/apimanagement/identity_provider_twitter.rs");
    include!("resources/apimanagement/logger.rs");
    include!("resources/apimanagement/named_value.rs");
    include!("resources/apimanagement/notification_recipient_email.rs");
    include!("resources/apimanagement/notification_recipient_user.rs");
    include!("resources/apimanagement/open_id_connect_provider.rs");
    include!("resources/apimanagement/policy.rs");
    include!("resources/apimanagement/policy_fragment.rs");
    include!("resources/apimanagement/product.rs");
    include!("resources/apimanagement/product_api.rs");
    include!("resources/apimanagement/product_group.rs");
    include!("resources/apimanagement/product_policy.rs");
    include!("resources/apimanagement/product_tag.rs");
    include!("resources/apimanagement/redis_cache.rs");
    include!("resources/apimanagement/service.rs");
    include!("resources/apimanagement/subscription.rs");
    include!("resources/apimanagement/tag.rs");
    include!("resources/apimanagement/user.rs");
}
pub mod functions {
    pub mod apimanagement {
        include!("functions/apimanagement/get_api.rs");
        include!("functions/apimanagement/get_api_version_set.rs");
        include!("functions/apimanagement/get_gateway.rs");
        include!("functions/apimanagement/get_gateway_host_name_configuration.rs");
        include!("functions/apimanagement/get_group.rs");
        include!("functions/apimanagement/get_product.rs");
        include!("functions/apimanagement/get_service.rs");
        include!("functions/apimanagement/get_user.rs");
    }
}
pub mod types {
    pub mod apimanagement {
        include!("types/apimanagement/api_contact.rs");
        include!("types/apimanagement/api_diagnostic_backend_request.rs");
        include!("types/apimanagement/api_diagnostic_backend_request_data_masking.rs");
        include!(
            "types/apimanagement/api_diagnostic_backend_request_data_masking_header.rs"
        );
        include!(
            "types/apimanagement/api_diagnostic_backend_request_data_masking_query_param.rs"
        );
        include!("types/apimanagement/api_diagnostic_backend_response.rs");
        include!("types/apimanagement/api_diagnostic_backend_response_data_masking.rs");
        include!(
            "types/apimanagement/api_diagnostic_backend_response_data_masking_header.rs"
        );
        include!(
            "types/apimanagement/api_diagnostic_backend_response_data_masking_query_param.rs"
        );
        include!("types/apimanagement/api_diagnostic_frontend_request.rs");
        include!("types/apimanagement/api_diagnostic_frontend_request_data_masking.rs");
        include!(
            "types/apimanagement/api_diagnostic_frontend_request_data_masking_header.rs"
        );
        include!(
            "types/apimanagement/api_diagnostic_frontend_request_data_masking_query_param.rs"
        );
        include!("types/apimanagement/api_diagnostic_frontend_response.rs");
        include!("types/apimanagement/api_diagnostic_frontend_response_data_masking.rs");
        include!(
            "types/apimanagement/api_diagnostic_frontend_response_data_masking_header.rs"
        );
        include!(
            "types/apimanagement/api_diagnostic_frontend_response_data_masking_query_param.rs"
        );
        include!("types/apimanagement/api_import.rs");
        include!("types/apimanagement/api_import_wsdl_selector.rs");
        include!("types/apimanagement/api_license.rs");
        include!("types/apimanagement/api_oauth_2_authorization.rs");
        include!("types/apimanagement/api_openid_authentication.rs");
        include!("types/apimanagement/api_operation_request.rs");
        include!("types/apimanagement/api_operation_request_header.rs");
        include!("types/apimanagement/api_operation_request_header_example.rs");
        include!("types/apimanagement/api_operation_request_query_parameter.rs");
        include!("types/apimanagement/api_operation_request_query_parameter_example.rs");
        include!("types/apimanagement/api_operation_request_representation.rs");
        include!("types/apimanagement/api_operation_request_representation_example.rs");
        include!(
            "types/apimanagement/api_operation_request_representation_form_parameter.rs"
        );
        include!(
            "types/apimanagement/api_operation_request_representation_form_parameter_example.rs"
        );
        include!("types/apimanagement/api_operation_response.rs");
        include!("types/apimanagement/api_operation_response_header.rs");
        include!("types/apimanagement/api_operation_response_header_example.rs");
        include!("types/apimanagement/api_operation_response_representation.rs");
        include!("types/apimanagement/api_operation_response_representation_example.rs");
        include!(
            "types/apimanagement/api_operation_response_representation_form_parameter.rs"
        );
        include!(
            "types/apimanagement/api_operation_response_representation_form_parameter_example.rs"
        );
        include!("types/apimanagement/api_operation_template_parameter.rs");
        include!("types/apimanagement/api_operation_template_parameter_example.rs");
        include!("types/apimanagement/api_subscription_key_parameter_names.rs");
        include!("types/apimanagement/authorization_server_token_body_parameter.rs");
        include!("types/apimanagement/backend_credentials.rs");
        include!("types/apimanagement/backend_credentials_authorization.rs");
        include!("types/apimanagement/backend_proxy.rs");
        include!("types/apimanagement/backend_service_fabric_cluster.rs");
        include!(
            "types/apimanagement/backend_service_fabric_cluster_server_x_509_name.rs"
        );
        include!("types/apimanagement/backend_tls.rs");
        include!("types/apimanagement/custom_domain_developer_portal.rs");
        include!("types/apimanagement/custom_domain_gateway.rs");
        include!("types/apimanagement/custom_domain_management.rs");
        include!("types/apimanagement/custom_domain_portal.rs");
        include!("types/apimanagement/custom_domain_scm.rs");
        include!("types/apimanagement/diagnostic_backend_request.rs");
        include!("types/apimanagement/diagnostic_backend_request_data_masking.rs");
        include!(
            "types/apimanagement/diagnostic_backend_request_data_masking_header.rs"
        );
        include!(
            "types/apimanagement/diagnostic_backend_request_data_masking_query_param.rs"
        );
        include!("types/apimanagement/diagnostic_backend_response.rs");
        include!("types/apimanagement/diagnostic_backend_response_data_masking.rs");
        include!(
            "types/apimanagement/diagnostic_backend_response_data_masking_header.rs"
        );
        include!(
            "types/apimanagement/diagnostic_backend_response_data_masking_query_param.rs"
        );
        include!("types/apimanagement/diagnostic_frontend_request.rs");
        include!("types/apimanagement/diagnostic_frontend_request_data_masking.rs");
        include!(
            "types/apimanagement/diagnostic_frontend_request_data_masking_header.rs"
        );
        include!(
            "types/apimanagement/diagnostic_frontend_request_data_masking_query_param.rs"
        );
        include!("types/apimanagement/diagnostic_frontend_response.rs");
        include!("types/apimanagement/diagnostic_frontend_response_data_masking.rs");
        include!(
            "types/apimanagement/diagnostic_frontend_response_data_masking_header.rs"
        );
        include!(
            "types/apimanagement/diagnostic_frontend_response_data_masking_query_param.rs"
        );
        include!("types/apimanagement/gateway_location_data.rs");
        include!("types/apimanagement/logger_application_insights.rs");
        include!("types/apimanagement/logger_eventhub.rs");
        include!("types/apimanagement/named_value_value_from_key_vault.rs");
        include!("types/apimanagement/service_additional_location.rs");
        include!(
            "types/apimanagement/service_additional_location_virtual_network_configuration.rs"
        );
        include!("types/apimanagement/service_certificate.rs");
        include!("types/apimanagement/service_delegation.rs");
        include!("types/apimanagement/service_hostname_configuration.rs");
        include!(
            "types/apimanagement/service_hostname_configuration_developer_portal.rs"
        );
        include!("types/apimanagement/service_hostname_configuration_management.rs");
        include!("types/apimanagement/service_hostname_configuration_portal.rs");
        include!("types/apimanagement/service_hostname_configuration_proxy.rs");
        include!("types/apimanagement/service_hostname_configuration_scm.rs");
        include!("types/apimanagement/service_identity.rs");
        include!("types/apimanagement/service_protocols.rs");
        include!("types/apimanagement/service_security.rs");
        include!("types/apimanagement/service_sign_in.rs");
        include!("types/apimanagement/service_sign_up.rs");
        include!("types/apimanagement/service_sign_up_terms_of_service.rs");
        include!("types/apimanagement/service_tenant_access.rs");
        include!("types/apimanagement/service_virtual_network_configuration.rs");
        include!("types/apimanagement/get_api_subscription_key_parameter_name.rs");
        include!("types/apimanagement/get_gateway_location_data.rs");
        include!("types/apimanagement/get_service_additional_location.rs");
        include!("types/apimanagement/get_service_hostname_configuration.rs");
        include!(
            "types/apimanagement/get_service_hostname_configuration_developer_portal.rs"
        );
        include!("types/apimanagement/get_service_hostname_configuration_management.rs");
        include!("types/apimanagement/get_service_hostname_configuration_portal.rs");
        include!("types/apimanagement/get_service_hostname_configuration_proxy.rs");
        include!("types/apimanagement/get_service_hostname_configuration_scm.rs");
        include!("types/apimanagement/get_service_identity.rs");
        include!("types/apimanagement/get_service_tenant_access.rs");
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
