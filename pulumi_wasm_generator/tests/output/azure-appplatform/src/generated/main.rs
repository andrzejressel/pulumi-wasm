pub mod appplatform {
    include!("resources/appplatform/spring_cloud_accelerator.rs");
    include!("resources/appplatform/spring_cloud_active_deployment.rs");
    include!("resources/appplatform/spring_cloud_api_portal.rs");
    include!("resources/appplatform/spring_cloud_api_portal_custom_domain.rs");
    include!("resources/appplatform/spring_cloud_app.rs");
    include!("resources/appplatform/spring_cloud_app_cosmos_db_association.rs");
    include!(
        "resources/appplatform/spring_cloud_app_dynamics_application_performance_monitoring.rs"
    );
    include!("resources/appplatform/spring_cloud_app_mysql_association.rs");
    include!("resources/appplatform/spring_cloud_app_redis_association.rs");
    include!(
        "resources/appplatform/spring_cloud_application_insights_application_performance_monitoring.rs"
    );
    include!("resources/appplatform/spring_cloud_application_live_view.rs");
    include!("resources/appplatform/spring_cloud_build_deployment.rs");
    include!("resources/appplatform/spring_cloud_build_pack_binding.rs");
    include!("resources/appplatform/spring_cloud_builder.rs");
    include!("resources/appplatform/spring_cloud_certificate.rs");
    include!("resources/appplatform/spring_cloud_configuration_service.rs");
    include!("resources/appplatform/spring_cloud_connection.rs");
    include!("resources/appplatform/spring_cloud_container_deployment.rs");
    include!("resources/appplatform/spring_cloud_custom_domain.rs");
    include!("resources/appplatform/spring_cloud_customized_accelerator.rs");
    include!("resources/appplatform/spring_cloud_dev_tool_portal.rs");
    include!(
        "resources/appplatform/spring_cloud_dynatrace_application_performance_monitoring.rs"
    );
    include!(
        "resources/appplatform/spring_cloud_elastic_application_performance_monitoring.rs"
    );
    include!("resources/appplatform/spring_cloud_gateway.rs");
    include!("resources/appplatform/spring_cloud_gateway_custom_domain.rs");
    include!("resources/appplatform/spring_cloud_gateway_route_config.rs");
    include!("resources/appplatform/spring_cloud_java_deployment.rs");
    include!(
        "resources/appplatform/spring_cloud_new_relic_application_performance_monitoring.rs"
    );
    include!("resources/appplatform/spring_cloud_service.rs");
    include!("resources/appplatform/spring_cloud_storage.rs");
}
pub mod functions {
    pub mod appplatform {
        include!("functions/appplatform/get_spring_cloud_app.rs");
        include!("functions/appplatform/get_spring_cloud_service.rs");
    }
}
pub mod types {
    pub mod appplatform {
        include!("types/appplatform/spring_cloud_api_portal_sso.rs");
        include!("types/appplatform/spring_cloud_app_custom_persistent_disk.rs");
        include!("types/appplatform/spring_cloud_app_identity.rs");
        include!("types/appplatform/spring_cloud_app_ingress_settings.rs");
        include!("types/appplatform/spring_cloud_app_persistent_disk.rs");
        include!("types/appplatform/spring_cloud_build_deployment_quota.rs");
        include!("types/appplatform/spring_cloud_build_pack_binding_launch.rs");
        include!("types/appplatform/spring_cloud_builder_build_pack_group.rs");
        include!("types/appplatform/spring_cloud_builder_stack.rs");
        include!("types/appplatform/spring_cloud_configuration_service_repository.rs");
        include!("types/appplatform/spring_cloud_connection_authentication.rs");
        include!("types/appplatform/spring_cloud_connection_secret_store.rs");
        include!("types/appplatform/spring_cloud_container_deployment_quota.rs");
        include!(
            "types/appplatform/spring_cloud_customized_accelerator_git_repository.rs"
        );
        include!(
            "types/appplatform/spring_cloud_customized_accelerator_git_repository_basic_auth.rs"
        );
        include!(
            "types/appplatform/spring_cloud_customized_accelerator_git_repository_ssh_auth.rs"
        );
        include!("types/appplatform/spring_cloud_dev_tool_portal_sso.rs");
        include!("types/appplatform/spring_cloud_gateway_api_metadata.rs");
        include!("types/appplatform/spring_cloud_gateway_client_authorization.rs");
        include!("types/appplatform/spring_cloud_gateway_cors.rs");
        include!(
            "types/appplatform/spring_cloud_gateway_local_response_cache_per_instance.rs"
        );
        include!(
            "types/appplatform/spring_cloud_gateway_local_response_cache_per_route.rs"
        );
        include!("types/appplatform/spring_cloud_gateway_quota.rs");
        include!("types/appplatform/spring_cloud_gateway_route_config_open_api.rs");
        include!("types/appplatform/spring_cloud_gateway_route_config_route.rs");
        include!("types/appplatform/spring_cloud_gateway_sso.rs");
        include!("types/appplatform/spring_cloud_java_deployment_quota.rs");
        include!("types/appplatform/spring_cloud_service_config_server_git_setting.rs");
        include!(
            "types/appplatform/spring_cloud_service_config_server_git_setting_http_basic_auth.rs"
        );
        include!(
            "types/appplatform/spring_cloud_service_config_server_git_setting_repository.rs"
        );
        include!(
            "types/appplatform/spring_cloud_service_config_server_git_setting_repository_http_basic_auth.rs"
        );
        include!(
            "types/appplatform/spring_cloud_service_config_server_git_setting_repository_ssh_auth.rs"
        );
        include!(
            "types/appplatform/spring_cloud_service_config_server_git_setting_ssh_auth.rs"
        );
        include!("types/appplatform/spring_cloud_service_container_registry.rs");
        include!("types/appplatform/spring_cloud_service_default_build_service.rs");
        include!("types/appplatform/spring_cloud_service_marketplace.rs");
        include!("types/appplatform/spring_cloud_service_network.rs");
        include!(
            "types/appplatform/spring_cloud_service_required_network_traffic_rule.rs"
        );
        include!("types/appplatform/spring_cloud_service_trace.rs");
        include!("types/appplatform/get_spring_cloud_app_identity.rs");
        include!("types/appplatform/get_spring_cloud_app_persistent_disk.rs");
        include!(
            "types/appplatform/get_spring_cloud_service_config_server_git_setting.rs"
        );
        include!(
            "types/appplatform/get_spring_cloud_service_config_server_git_setting_http_basic_auth.rs"
        );
        include!(
            "types/appplatform/get_spring_cloud_service_config_server_git_setting_repository.rs"
        );
        include!(
            "types/appplatform/get_spring_cloud_service_config_server_git_setting_repository_http_basic_auth.rs"
        );
        include!(
            "types/appplatform/get_spring_cloud_service_config_server_git_setting_repository_ssh_auth.rs"
        );
        include!(
            "types/appplatform/get_spring_cloud_service_config_server_git_setting_ssh_auth.rs"
        );
        include!(
            "types/appplatform/get_spring_cloud_service_required_network_traffic_rule.rs"
        );
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
