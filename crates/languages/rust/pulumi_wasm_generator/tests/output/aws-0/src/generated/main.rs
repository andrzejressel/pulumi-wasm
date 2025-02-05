pub mod accessanalyzer {
    include!("resources/accessanalyzer/analyzer.rs");
    include!("resources/accessanalyzer/archive_rule.rs");
}
pub mod account {
    include!("resources/account/alternative_contact.rs");
    include!("resources/account/primary_contact.rs");
    include!("resources/account/region.rs");
}
pub mod acm {
    include!("resources/acm/certificate.rs");
    include!("resources/acm/certificate_validation.rs");
}
pub mod acmpca {
    include!("resources/acmpca/certificate.rs");
    include!("resources/acmpca/certificate_authority.rs");
    include!("resources/acmpca/certificate_authority_certificate.rs");
    include!("resources/acmpca/permission.rs");
    include!("resources/acmpca/policy.rs");
}
pub mod alb {
    include!("resources/alb/listener.rs");
    include!("resources/alb/listener_certificate.rs");
    include!("resources/alb/listener_rule.rs");
    include!("resources/alb/load_balancer.rs");
    include!("resources/alb/target_group.rs");
    include!("resources/alb/target_group_attachment.rs");
}
pub mod amp {
    include!("resources/amp/alert_manager_definition.rs");
    include!("resources/amp/rule_group_namespace.rs");
    include!("resources/amp/scraper.rs");
    include!("resources/amp/workspace.rs");
}
pub mod amplify {
    include!("resources/amplify/app.rs");
    include!("resources/amplify/backend_environment.rs");
    include!("resources/amplify/branch.rs");
    include!("resources/amplify/domain_association.rs");
    include!("resources/amplify/webhook.rs");
}
pub mod apigateway {
    include!("resources/apigateway/account.rs");
    include!("resources/apigateway/api_key.rs");
    include!("resources/apigateway/authorizer.rs");
    include!("resources/apigateway/base_path_mapping.rs");
    include!("resources/apigateway/client_certificate.rs");
    include!("resources/apigateway/deployment.rs");
    include!("resources/apigateway/documentation_part.rs");
    include!("resources/apigateway/documentation_version.rs");
    include!("resources/apigateway/domain_name.rs");
    include!("resources/apigateway/domain_name_access_association.rs");
    include!("resources/apigateway/integration.rs");
    include!("resources/apigateway/integration_response.rs");
    include!("resources/apigateway/method.rs");
    include!("resources/apigateway/method_response.rs");
    include!("resources/apigateway/method_settings.rs");
    include!("resources/apigateway/model.rs");
    include!("resources/apigateway/request_validator.rs");
    include!("resources/apigateway/resource.rs");
    include!("resources/apigateway/response.rs");
    include!("resources/apigateway/rest_api.rs");
    include!("resources/apigateway/rest_api_policy.rs");
    include!("resources/apigateway/stage.rs");
    include!("resources/apigateway/usage_plan.rs");
    include!("resources/apigateway/usage_plan_key.rs");
    include!("resources/apigateway/vpc_link.rs");
}
pub mod apigatewayv2 {
    include!("resources/apigatewayv2/api.rs");
    include!("resources/apigatewayv2/api_mapping.rs");
    include!("resources/apigatewayv2/authorizer.rs");
    include!("resources/apigatewayv2/deployment.rs");
    include!("resources/apigatewayv2/domain_name.rs");
    include!("resources/apigatewayv2/integration.rs");
    include!("resources/apigatewayv2/integration_response.rs");
    include!("resources/apigatewayv2/model.rs");
    include!("resources/apigatewayv2/route.rs");
    include!("resources/apigatewayv2/route_response.rs");
    include!("resources/apigatewayv2/stage.rs");
    include!("resources/apigatewayv2/vpc_link.rs");
}
pub mod appautoscaling {
    include!("resources/appautoscaling/policy.rs");
    include!("resources/appautoscaling/scheduled_action.rs");
    include!("resources/appautoscaling/target.rs");
}
pub mod functions {
    pub mod acm {
        include!("functions/acm/get_certificate.rs");
    }
    pub mod acmpca {
        include!("functions/acmpca/get_certificate.rs");
        include!("functions/acmpca/get_certificate_authority.rs");
    }
    pub mod alb {
        include!("functions/alb/get_listener.rs");
        include!("functions/alb/get_load_balancer.rs");
        include!("functions/alb/get_target_group.rs");
    }
    pub mod amp {
        include!("functions/amp/get_default_scraper_configuration.rs");
        include!("functions/amp/get_workspace.rs");
        include!("functions/amp/get_workspaces.rs");
    }
    pub mod apigateway {
        include!("functions/apigateway/get_authorizer.rs");
        include!("functions/apigateway/get_authorizers.rs");
        include!("functions/apigateway/get_domain_name.rs");
        include!("functions/apigateway/get_export.rs");
        include!("functions/apigateway/get_key.rs");
        include!("functions/apigateway/get_resource.rs");
        include!("functions/apigateway/get_rest_api.rs");
        include!("functions/apigateway/get_sdk.rs");
        include!("functions/apigateway/get_vpc_link.rs");
    }
    pub mod apigatewayv2 {
        include!("functions/apigatewayv2/get_api.rs");
        include!("functions/apigatewayv2/get_apis.rs");
        include!("functions/apigatewayv2/get_export.rs");
        include!("functions/apigatewayv2/get_vpc_link.rs");
    }
    include!("functions/get_arn.rs");
    include!("functions/get_availability_zone.rs");
    include!("functions/get_availability_zones.rs");
    include!("functions/get_billing_service_account.rs");
    include!("functions/get_caller_identity.rs");
    include!("functions/get_default_tags.rs");
    include!("functions/get_ip_ranges.rs");
    include!("functions/get_partition.rs");
    include!("functions/get_region.rs");
    include!("functions/get_regions.rs");
    include!("functions/get_service.rs");
    include!("functions/get_service_principal.rs");
}
pub mod types {
    pub mod accessanalyzer {
        include!("types/accessanalyzer/analyzer_configuration.rs");
        include!("types/accessanalyzer/analyzer_configuration_unused_access.rs");
        include!("types/accessanalyzer/archive_rule_filter.rs");
    }
    pub mod acm {
        include!("types/acm/certificate_domain_validation_option.rs");
        include!("types/acm/certificate_options.rs");
        include!("types/acm/certificate_renewal_summary.rs");
        include!("types/acm/certificate_validation_option.rs");
    }
    pub mod acmpca {
        include!(
            "types/acmpca/certificate_authority_certificate_authority_configuration.rs"
        );
        include!(
            "types/acmpca/certificate_authority_certificate_authority_configuration_subject.rs"
        );
        include!("types/acmpca/certificate_authority_revocation_configuration.rs");
        include!(
            "types/acmpca/certificate_authority_revocation_configuration_crl_configuration.rs"
        );
        include!(
            "types/acmpca/certificate_authority_revocation_configuration_ocsp_configuration.rs"
        );
        include!("types/acmpca/certificate_validity.rs");
        include!("types/acmpca/get_certificate_authority_revocation_configuration.rs");
        include!(
            "types/acmpca/get_certificate_authority_revocation_configuration_crl_configuration.rs"
        );
        include!(
            "types/acmpca/get_certificate_authority_revocation_configuration_ocsp_configuration.rs"
        );
    }
    pub mod alb {
        include!("types/alb/listener_default_action.rs");
        include!("types/alb/listener_default_action_authenticate_cognito.rs");
        include!("types/alb/listener_default_action_authenticate_oidc.rs");
        include!("types/alb/listener_default_action_fixed_response.rs");
        include!("types/alb/listener_default_action_forward.rs");
        include!("types/alb/listener_default_action_forward_stickiness.rs");
        include!("types/alb/listener_default_action_forward_target_group.rs");
        include!("types/alb/listener_default_action_redirect.rs");
        include!("types/alb/listener_mutual_authentication.rs");
        include!("types/alb/listener_rule_action.rs");
        include!("types/alb/listener_rule_action_authenticate_cognito.rs");
        include!("types/alb/listener_rule_action_authenticate_oidc.rs");
        include!("types/alb/listener_rule_action_fixed_response.rs");
        include!("types/alb/listener_rule_action_forward.rs");
        include!("types/alb/listener_rule_action_forward_stickiness.rs");
        include!("types/alb/listener_rule_action_forward_target_group.rs");
        include!("types/alb/listener_rule_action_redirect.rs");
        include!("types/alb/listener_rule_condition.rs");
        include!("types/alb/listener_rule_condition_host_header.rs");
        include!("types/alb/listener_rule_condition_http_header.rs");
        include!("types/alb/listener_rule_condition_http_request_method.rs");
        include!("types/alb/listener_rule_condition_path_pattern.rs");
        include!("types/alb/listener_rule_condition_query_string.rs");
        include!("types/alb/listener_rule_condition_source_ip.rs");
        include!("types/alb/load_balancer_access_logs.rs");
        include!("types/alb/load_balancer_connection_logs.rs");
        include!("types/alb/load_balancer_subnet_mapping.rs");
        include!("types/alb/target_group_health_check.rs");
        include!("types/alb/target_group_stickiness.rs");
        include!("types/alb/target_group_target_failover.rs");
        include!("types/alb/target_group_target_group_health.rs");
        include!("types/alb/target_group_target_group_health_dns_failover.rs");
        include!(
            "types/alb/target_group_target_group_health_unhealthy_state_routing.rs"
        );
        include!("types/alb/target_group_target_health_state.rs");
        include!("types/alb/get_listener_default_action.rs");
        include!("types/alb/get_listener_default_action_authenticate_cognito.rs");
        include!("types/alb/get_listener_default_action_authenticate_oidc.rs");
        include!("types/alb/get_listener_default_action_fixed_response.rs");
        include!("types/alb/get_listener_default_action_forward.rs");
        include!("types/alb/get_listener_default_action_forward_stickiness.rs");
        include!("types/alb/get_listener_default_action_forward_target_group.rs");
        include!("types/alb/get_listener_default_action_redirect.rs");
        include!("types/alb/get_listener_mutual_authentication.rs");
        include!("types/alb/get_load_balancer_access_logs.rs");
        include!("types/alb/get_load_balancer_connection_log.rs");
        include!("types/alb/get_load_balancer_subnet_mapping.rs");
        include!("types/alb/get_target_group_health_check.rs");
        include!("types/alb/get_target_group_stickiness.rs");
    }
    pub mod amp {
        include!("types/amp/scraper_destination.rs");
        include!("types/amp/scraper_destination_amp.rs");
        include!("types/amp/scraper_source.rs");
        include!("types/amp/scraper_source_eks.rs");
        include!("types/amp/scraper_timeouts.rs");
        include!("types/amp/workspace_logging_configuration.rs");
    }
    pub mod amplify {
        include!("types/amplify/app_auto_branch_creation_config.rs");
        include!("types/amplify/app_cache_config.rs");
        include!("types/amplify/app_custom_rule.rs");
        include!("types/amplify/app_production_branch.rs");
        include!("types/amplify/domain_association_certificate_settings.rs");
        include!("types/amplify/domain_association_sub_domain.rs");
    }
    pub mod apigateway {
        include!("types/apigateway/account_throttle_setting.rs");
        include!("types/apigateway/deployment_canary_settings.rs");
        include!("types/apigateway/documentation_part_location.rs");
        include!("types/apigateway/domain_name_endpoint_configuration.rs");
        include!("types/apigateway/domain_name_mutual_tls_authentication.rs");
        include!("types/apigateway/integration_tls_config.rs");
        include!("types/apigateway/method_settings_settings.rs");
        include!("types/apigateway/rest_api_endpoint_configuration.rs");
        include!("types/apigateway/stage_access_log_settings.rs");
        include!("types/apigateway/stage_canary_settings.rs");
        include!("types/apigateway/usage_plan_api_stage.rs");
        include!("types/apigateway/usage_plan_api_stage_throttle.rs");
        include!("types/apigateway/usage_plan_quota_settings.rs");
        include!("types/apigateway/usage_plan_throttle_settings.rs");
        include!("types/apigateway/get_domain_name_endpoint_configuration.rs");
        include!("types/apigateway/get_rest_api_endpoint_configuration.rs");
    }
    pub mod apigatewayv2 {
        include!("types/apigatewayv2/api_cors_configuration.rs");
        include!("types/apigatewayv2/authorizer_jwt_configuration.rs");
        include!("types/apigatewayv2/domain_name_domain_name_configuration.rs");
        include!("types/apigatewayv2/domain_name_mutual_tls_authentication.rs");
        include!("types/apigatewayv2/integration_response_parameter.rs");
        include!("types/apigatewayv2/integration_tls_config.rs");
        include!("types/apigatewayv2/route_request_parameter.rs");
        include!("types/apigatewayv2/stage_access_log_settings.rs");
        include!("types/apigatewayv2/stage_default_route_settings.rs");
        include!("types/apigatewayv2/stage_route_setting.rs");
        include!("types/apigatewayv2/get_api_cors_configuration.rs");
    }
    pub mod appautoscaling {
        include!("types/appautoscaling/policy_step_scaling_policy_configuration.rs");
        include!(
            "types/appautoscaling/policy_step_scaling_policy_configuration_step_adjustment.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration_customized_metric_specification.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration_customized_metric_specification_dimension.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration_customized_metric_specification_metric.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration_customized_metric_specification_metric_metric_stat.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration_customized_metric_specification_metric_metric_stat_metric.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration_customized_metric_specification_metric_metric_stat_metric_dimension.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration_predefined_metric_specification.rs"
        );
        include!("types/appautoscaling/scheduled_action_scalable_target_action.rs");
        include!("types/appautoscaling/target_suspended_state.rs");
    }
    include!("types/get_availability_zone_filter.rs");
    include!("types/get_availability_zones_filter.rs");
    include!("types/get_regions_filter.rs");
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-aws {
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
        pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
        } }
    );
}
#[link_section = "pulumi_wasm_provider::aws"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_AWS: [u8; 45] = *b"{\"version\":\"6.66.2\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "6.66.2".to_string()
}
