pub mod cloudhsmv2 {
    include!("resources/cloudhsmv2/cluster.rs");
    include!("resources/cloudhsmv2/hsm.rs");
}
pub mod cloudsearch {
    include!("resources/cloudsearch/domain.rs");
    include!("resources/cloudsearch/domain_service_access_policy.rs");
}
pub mod cloudtrail {
    include!("resources/cloudtrail/event_data_store.rs");
    include!("resources/cloudtrail/organization_delegated_admin_account.rs");
    include!("resources/cloudtrail/trail.rs");
}
pub mod cloudwatch {
    include!("resources/cloudwatch/composite_alarm.rs");
    include!("resources/cloudwatch/dashboard.rs");
    include!("resources/cloudwatch/event_api_destination.rs");
    include!("resources/cloudwatch/event_archive.rs");
    include!("resources/cloudwatch/event_bus.rs");
    include!("resources/cloudwatch/event_bus_policy.rs");
    include!("resources/cloudwatch/event_connection.rs");
    include!("resources/cloudwatch/event_endpoint.rs");
    include!("resources/cloudwatch/event_permission.rs");
    include!("resources/cloudwatch/event_rule.rs");
    include!("resources/cloudwatch/event_target.rs");
    include!("resources/cloudwatch/internet_monitor.rs");
    include!("resources/cloudwatch/log_account_policy.rs");
    include!("resources/cloudwatch/log_anomaly_detector.rs");
    include!("resources/cloudwatch/log_data_protection_policy.rs");
    include!("resources/cloudwatch/log_destination.rs");
    include!("resources/cloudwatch/log_destination_policy.rs");
    include!("resources/cloudwatch/log_group.rs");
    include!("resources/cloudwatch/log_metric_filter.rs");
    include!("resources/cloudwatch/log_resource_policy.rs");
    include!("resources/cloudwatch/log_stream.rs");
    include!("resources/cloudwatch/log_subscription_filter.rs");
    include!("resources/cloudwatch/metric_alarm.rs");
    include!("resources/cloudwatch/metric_stream.rs");
    include!("resources/cloudwatch/query_definition.rs");
}
pub mod codeartifact {
    include!("resources/codeartifact/domain.rs");
    include!("resources/codeartifact/domain_permissions.rs");
    include!("resources/codeartifact/repository.rs");
    include!("resources/codeartifact/repository_permissions_policy.rs");
}
pub mod codebuild {
    include!("resources/codebuild/fleet.rs");
    include!("resources/codebuild/project.rs");
    include!("resources/codebuild/report_group.rs");
    include!("resources/codebuild/resource_policy.rs");
    include!("resources/codebuild/source_credential.rs");
    include!("resources/codebuild/webhook.rs");
}
pub mod codecatalyst {
    include!("resources/codecatalyst/dev_environment.rs");
    include!("resources/codecatalyst/project.rs");
    include!("resources/codecatalyst/source_repository.rs");
}
pub mod codecommit {
    include!("resources/codecommit/approval_rule_template.rs");
    include!("resources/codecommit/approval_rule_template_association.rs");
    include!("resources/codecommit/repository.rs");
    include!("resources/codecommit/trigger.rs");
}
pub mod codeconnections {
    include!("resources/codeconnections/connection.rs");
    include!("resources/codeconnections/host.rs");
}
pub mod codedeploy {
    include!("resources/codedeploy/application.rs");
    include!("resources/codedeploy/deployment_config.rs");
    include!("resources/codedeploy/deployment_group.rs");
}
pub mod functions {
    pub mod cloudhsmv2 {
        include!("functions/cloudhsmv2/get_cluster.rs");
    }
    pub mod cloudtrail {
        include!("functions/cloudtrail/get_service_account.rs");
    }
    pub mod cloudwatch {
        include!("functions/cloudwatch/get_event_bus.rs");
        include!("functions/cloudwatch/get_event_connection.rs");
        include!("functions/cloudwatch/get_event_source.rs");
        include!("functions/cloudwatch/get_log_data_protection_policy_document.rs");
        include!("functions/cloudwatch/get_log_group.rs");
        include!("functions/cloudwatch/get_log_groups.rs");
    }
    pub mod codeartifact {
        include!("functions/codeartifact/get_authorization_token.rs");
        include!("functions/codeartifact/get_repository_endpoint.rs");
    }
    pub mod codebuild {
        include!("functions/codebuild/get_fleet.rs");
    }
    pub mod codecatalyst {
        include!("functions/codecatalyst/get_dev_environment.rs");
    }
    pub mod codecommit {
        include!("functions/codecommit/get_approval_rule_template.rs");
        include!("functions/codecommit/get_repository.rs");
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
    pub mod cloudhsmv2 {
        include!("types/cloudhsmv2/cluster_cluster_certificate.rs");
        include!("types/cloudhsmv2/get_cluster_cluster_certificate.rs");
    }
    pub mod cloudsearch {
        include!("types/cloudsearch/domain_endpoint_options.rs");
        include!("types/cloudsearch/domain_index_field.rs");
        include!("types/cloudsearch/domain_scaling_parameters.rs");
    }
    pub mod cloudtrail {
        include!("types/cloudtrail/event_data_store_advanced_event_selector.rs");
        include!(
            "types/cloudtrail/event_data_store_advanced_event_selector_field_selector.rs"
        );
        include!("types/cloudtrail/trail_advanced_event_selector.rs");
        include!("types/cloudtrail/trail_advanced_event_selector_field_selector.rs");
        include!("types/cloudtrail/trail_event_selector.rs");
        include!("types/cloudtrail/trail_event_selector_data_resource.rs");
        include!("types/cloudtrail/trail_insight_selector.rs");
    }
    pub mod cloudwatch {
        include!("types/cloudwatch/composite_alarm_actions_suppressor.rs");
        include!("types/cloudwatch/event_connection_auth_parameters.rs");
        include!("types/cloudwatch/event_connection_auth_parameters_api_key.rs");
        include!("types/cloudwatch/event_connection_auth_parameters_basic.rs");
        include!(
            "types/cloudwatch/event_connection_auth_parameters_invocation_http_parameters.rs"
        );
        include!(
            "types/cloudwatch/event_connection_auth_parameters_invocation_http_parameters_body.rs"
        );
        include!(
            "types/cloudwatch/event_connection_auth_parameters_invocation_http_parameters_header.rs"
        );
        include!(
            "types/cloudwatch/event_connection_auth_parameters_invocation_http_parameters_query_string.rs"
        );
        include!("types/cloudwatch/event_connection_auth_parameters_oauth.rs");
        include!(
            "types/cloudwatch/event_connection_auth_parameters_oauth_client_parameters.rs"
        );
        include!(
            "types/cloudwatch/event_connection_auth_parameters_oauth_oauth_http_parameters.rs"
        );
        include!(
            "types/cloudwatch/event_connection_auth_parameters_oauth_oauth_http_parameters_body.rs"
        );
        include!(
            "types/cloudwatch/event_connection_auth_parameters_oauth_oauth_http_parameters_header.rs"
        );
        include!(
            "types/cloudwatch/event_connection_auth_parameters_oauth_oauth_http_parameters_query_string.rs"
        );
        include!("types/cloudwatch/event_endpoint_event_bus.rs");
        include!("types/cloudwatch/event_endpoint_replication_config.rs");
        include!("types/cloudwatch/event_endpoint_routing_config.rs");
        include!("types/cloudwatch/event_endpoint_routing_config_failover_config.rs");
        include!(
            "types/cloudwatch/event_endpoint_routing_config_failover_config_primary.rs"
        );
        include!(
            "types/cloudwatch/event_endpoint_routing_config_failover_config_secondary.rs"
        );
        include!("types/cloudwatch/event_permission_condition.rs");
        include!("types/cloudwatch/event_target_appsync_target.rs");
        include!("types/cloudwatch/event_target_batch_target.rs");
        include!("types/cloudwatch/event_target_dead_letter_config.rs");
        include!("types/cloudwatch/event_target_ecs_target.rs");
        include!(
            "types/cloudwatch/event_target_ecs_target_capacity_provider_strategy.rs"
        );
        include!("types/cloudwatch/event_target_ecs_target_network_configuration.rs");
        include!(
            "types/cloudwatch/event_target_ecs_target_ordered_placement_strategy.rs"
        );
        include!("types/cloudwatch/event_target_ecs_target_placement_constraint.rs");
        include!("types/cloudwatch/event_target_http_target.rs");
        include!("types/cloudwatch/event_target_input_transformer.rs");
        include!("types/cloudwatch/event_target_kinesis_target.rs");
        include!("types/cloudwatch/event_target_redshift_target.rs");
        include!("types/cloudwatch/event_target_retry_policy.rs");
        include!("types/cloudwatch/event_target_run_command_target.rs");
        include!("types/cloudwatch/event_target_sagemaker_pipeline_target.rs");
        include!(
            "types/cloudwatch/event_target_sagemaker_pipeline_target_pipeline_parameter_list.rs"
        );
        include!("types/cloudwatch/event_target_sqs_target.rs");
        include!("types/cloudwatch/internet_monitor_health_events_config.rs");
        include!(
            "types/cloudwatch/internet_monitor_internet_measurements_log_delivery.rs"
        );
        include!(
            "types/cloudwatch/internet_monitor_internet_measurements_log_delivery_s_3_config.rs"
        );
        include!("types/cloudwatch/log_metric_filter_metric_transformation.rs");
        include!("types/cloudwatch/metric_alarm_metric_query.rs");
        include!("types/cloudwatch/metric_alarm_metric_query_metric.rs");
        include!("types/cloudwatch/metric_stream_exclude_filter.rs");
        include!("types/cloudwatch/metric_stream_include_filter.rs");
        include!("types/cloudwatch/metric_stream_statistics_configuration.rs");
        include!(
            "types/cloudwatch/metric_stream_statistics_configuration_include_metric.rs"
        );
        include!(
            "types/cloudwatch/get_log_data_protection_policy_document_statement.rs"
        );
        include!(
            "types/cloudwatch/get_log_data_protection_policy_document_statement_operation.rs"
        );
        include!(
            "types/cloudwatch/get_log_data_protection_policy_document_statement_operation_audit.rs"
        );
        include!(
            "types/cloudwatch/get_log_data_protection_policy_document_statement_operation_audit_findings_destination.rs"
        );
        include!(
            "types/cloudwatch/get_log_data_protection_policy_document_statement_operation_audit_findings_destination_cloudwatch_logs.rs"
        );
        include!(
            "types/cloudwatch/get_log_data_protection_policy_document_statement_operation_audit_findings_destination_firehose.rs"
        );
        include!(
            "types/cloudwatch/get_log_data_protection_policy_document_statement_operation_audit_findings_destination_s_3.rs"
        );
        include!(
            "types/cloudwatch/get_log_data_protection_policy_document_statement_operation_deidentify.rs"
        );
        include!(
            "types/cloudwatch/get_log_data_protection_policy_document_statement_operation_deidentify_mask_config.rs"
        );
    }
    pub mod codeartifact {
        include!("types/codeartifact/repository_external_connections.rs");
        include!("types/codeartifact/repository_upstream.rs");
    }
    pub mod codebuild {
        include!("types/codebuild/fleet_scaling_configuration.rs");
        include!(
            "types/codebuild/fleet_scaling_configuration_target_tracking_scaling_config.rs"
        );
        include!("types/codebuild/fleet_status.rs");
        include!("types/codebuild/fleet_vpc_config.rs");
        include!("types/codebuild/project_artifacts.rs");
        include!("types/codebuild/project_build_batch_config.rs");
        include!("types/codebuild/project_build_batch_config_restrictions.rs");
        include!("types/codebuild/project_cache.rs");
        include!("types/codebuild/project_environment.rs");
        include!("types/codebuild/project_environment_environment_variable.rs");
        include!("types/codebuild/project_environment_fleet.rs");
        include!("types/codebuild/project_environment_registry_credential.rs");
        include!("types/codebuild/project_file_system_location.rs");
        include!("types/codebuild/project_logs_config.rs");
        include!("types/codebuild/project_logs_config_cloudwatch_logs.rs");
        include!("types/codebuild/project_logs_config_s_3_logs.rs");
        include!("types/codebuild/project_secondary_artifact.rs");
        include!("types/codebuild/project_secondary_source.rs");
        include!("types/codebuild/project_secondary_source_build_status_config.rs");
        include!("types/codebuild/project_secondary_source_git_submodules_config.rs");
        include!("types/codebuild/project_secondary_source_version.rs");
        include!("types/codebuild/project_source.rs");
        include!("types/codebuild/project_source_build_status_config.rs");
        include!("types/codebuild/project_source_git_submodules_config.rs");
        include!("types/codebuild/project_vpc_config.rs");
        include!("types/codebuild/report_group_export_config.rs");
        include!("types/codebuild/report_group_export_config_s_3_destination.rs");
        include!("types/codebuild/webhook_filter_group.rs");
        include!("types/codebuild/webhook_filter_group_filter.rs");
        include!("types/codebuild/webhook_scope_configuration.rs");
        include!("types/codebuild/get_fleet_scaling_configuration.rs");
        include!(
            "types/codebuild/get_fleet_scaling_configuration_target_tracking_scaling_config.rs"
        );
        include!("types/codebuild/get_fleet_status.rs");
        include!("types/codebuild/get_fleet_vpc_config.rs");
    }
    pub mod codecatalyst {
        include!("types/codecatalyst/dev_environment_ides.rs");
        include!("types/codecatalyst/dev_environment_persistent_storage.rs");
        include!("types/codecatalyst/dev_environment_repository.rs");
        include!("types/codecatalyst/get_dev_environment_ide.rs");
        include!("types/codecatalyst/get_dev_environment_persistent_storage.rs");
        include!("types/codecatalyst/get_dev_environment_repository.rs");
    }
    pub mod codecommit {
        include!("types/codecommit/trigger_trigger.rs");
    }
    pub mod codeconnections {
        include!("types/codeconnections/connection_timeouts.rs");
        include!("types/codeconnections/host_timeouts.rs");
        include!("types/codeconnections/host_vpc_configuration.rs");
    }
    pub mod codedeploy {
        include!("types/codedeploy/deployment_config_minimum_healthy_hosts.rs");
        include!("types/codedeploy/deployment_config_traffic_routing_config.rs");
        include!(
            "types/codedeploy/deployment_config_traffic_routing_config_time_based_canary.rs"
        );
        include!(
            "types/codedeploy/deployment_config_traffic_routing_config_time_based_linear.rs"
        );
        include!("types/codedeploy/deployment_config_zonal_config.rs");
        include!(
            "types/codedeploy/deployment_config_zonal_config_minimum_healthy_hosts_per_zone.rs"
        );
        include!("types/codedeploy/deployment_group_alarm_configuration.rs");
        include!("types/codedeploy/deployment_group_auto_rollback_configuration.rs");
        include!("types/codedeploy/deployment_group_blue_green_deployment_config.rs");
        include!(
            "types/codedeploy/deployment_group_blue_green_deployment_config_deployment_ready_option.rs"
        );
        include!(
            "types/codedeploy/deployment_group_blue_green_deployment_config_green_fleet_provisioning_option.rs"
        );
        include!(
            "types/codedeploy/deployment_group_blue_green_deployment_config_terminate_blue_instances_on_deployment_success.rs"
        );
        include!("types/codedeploy/deployment_group_deployment_style.rs");
        include!("types/codedeploy/deployment_group_ec_2_tag_filter.rs");
        include!("types/codedeploy/deployment_group_ec_2_tag_set.rs");
        include!("types/codedeploy/deployment_group_ec_2_tag_set_ec_2_tag_filter.rs");
        include!("types/codedeploy/deployment_group_ecs_service.rs");
        include!("types/codedeploy/deployment_group_load_balancer_info.rs");
        include!("types/codedeploy/deployment_group_load_balancer_info_elb_info.rs");
        include!(
            "types/codedeploy/deployment_group_load_balancer_info_target_group_info.rs"
        );
        include!(
            "types/codedeploy/deployment_group_load_balancer_info_target_group_pair_info.rs"
        );
        include!(
            "types/codedeploy/deployment_group_load_balancer_info_target_group_pair_info_prod_traffic_route.rs"
        );
        include!(
            "types/codedeploy/deployment_group_load_balancer_info_target_group_pair_info_target_group.rs"
        );
        include!(
            "types/codedeploy/deployment_group_load_balancer_info_target_group_pair_info_test_traffic_route.rs"
        );
        include!("types/codedeploy/deployment_group_on_premises_instance_tag_filter.rs");
        include!("types/codedeploy/deployment_group_trigger_configuration.rs");
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
#[link_section = "pulumi_wasm_provider::aws"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub static PULUMI_WASM_PROVIDER_AWS: [u8; 45] = *b"{\"version\":\"6.66.2\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> &'static str {
    "6.66.2"
}
