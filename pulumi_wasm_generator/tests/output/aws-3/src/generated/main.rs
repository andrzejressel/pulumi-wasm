pub mod cfg {
    include!("resources/cfg/aggregate_authorization.rs");
    include!("resources/cfg/configuration_aggregator.rs");
    include!("resources/cfg/conformance_pack.rs");
    include!("resources/cfg/delivery_channel.rs");
    include!("resources/cfg/organization_conformance_pack.rs");
    include!("resources/cfg/organization_custom_policy_rule.rs");
    include!("resources/cfg/organization_custom_rule.rs");
    include!("resources/cfg/organization_managed_rule.rs");
    include!("resources/cfg/recorder.rs");
    include!("resources/cfg/recorder_status.rs");
    include!("resources/cfg/remediation_configuration.rs");
    include!("resources/cfg/retention_configuration.rs");
    include!("resources/cfg/rule.rs");
}
pub mod chatbot {
    include!("resources/chatbot/slack_channel_configuration.rs");
    include!("resources/chatbot/teams_channel_configuration.rs");
}
pub mod chime {
    include!("resources/chime/sdkvoice_global_settings.rs");
    include!("resources/chime/sdkvoice_sip_media_application.rs");
    include!("resources/chime/sdkvoice_sip_rule.rs");
    include!("resources/chime/sdkvoice_voice_profile_domain.rs");
    include!("resources/chime/voice_connector.rs");
    include!("resources/chime/voice_connector_group.rs");
    include!("resources/chime/voice_connector_logging.rs");
    include!("resources/chime/voice_connector_organization.rs");
    include!("resources/chime/voice_connector_streaming.rs");
    include!("resources/chime/voice_connector_termination.rs");
    include!("resources/chime/voice_connector_termination_credentials.rs");
}
pub mod chimesdkmediapipelines {
    include!(
        "resources/chimesdkmediapipelines/media_insights_pipeline_configuration.rs"
    );
}
pub mod cleanrooms {
    include!("resources/cleanrooms/collaboration.rs");
    include!("resources/cleanrooms/configured_table.rs");
}
pub mod cloud9 {
    include!("resources/cloud9/environment_ec_2.rs");
    include!("resources/cloud9/environment_membership.rs");
}
pub mod cloudcontrol {
    include!("resources/cloudcontrol/resource.rs");
}
pub mod cloudformation {
    include!("resources/cloudformation/cloud_formation_type.rs");
    include!("resources/cloudformation/stack.rs");
    include!("resources/cloudformation/stack_instances.rs");
    include!("resources/cloudformation/stack_set.rs");
    include!("resources/cloudformation/stack_set_instance.rs");
}
pub mod cloudfront {
    include!("resources/cloudfront/cache_policy.rs");
    include!("resources/cloudfront/continuous_deployment_policy.rs");
    include!("resources/cloudfront/distribution.rs");
    include!("resources/cloudfront/field_level_encryption_config.rs");
    include!("resources/cloudfront/field_level_encryption_profile.rs");
    include!("resources/cloudfront/function.rs");
    include!("resources/cloudfront/key_group.rs");
    include!("resources/cloudfront/key_value_store.rs");
    include!("resources/cloudfront/keyvaluestore_key.rs");
    include!("resources/cloudfront/monitoring_subscription.rs");
    include!("resources/cloudfront/origin_access_control.rs");
    include!("resources/cloudfront/origin_access_identity.rs");
    include!("resources/cloudfront/origin_request_policy.rs");
    include!("resources/cloudfront/public_key.rs");
    include!("resources/cloudfront/realtime_log_config.rs");
    include!("resources/cloudfront/response_headers_policy.rs");
    include!("resources/cloudfront/vpc_origin.rs");
}
pub mod cloudhsmv2 {
    include!("resources/cloudhsmv2/cluster.rs");
    include!("resources/cloudhsmv2/hsm.rs");
}
pub mod functions {
    pub mod chatbot {
        include!("functions/chatbot/get_slack_workspace.rs");
    }
    pub mod cloudcontrol {
        include!("functions/cloudcontrol/get_resource.rs");
    }
    pub mod cloudformation {
        include!("functions/cloudformation/get_cloud_formation_type.rs");
        include!("functions/cloudformation/get_export.rs");
        include!("functions/cloudformation/get_stack.rs");
    }
    pub mod cloudfront {
        include!("functions/cloudfront/get_cache_policy.rs");
        include!("functions/cloudfront/get_distribution.rs");
        include!("functions/cloudfront/get_function.rs");
        include!("functions/cloudfront/get_log_delivery_canonical_user_id.rs");
        include!("functions/cloudfront/get_origin_access_control.rs");
        include!("functions/cloudfront/get_origin_access_identities.rs");
        include!("functions/cloudfront/get_origin_access_identity.rs");
        include!("functions/cloudfront/get_origin_request_policy.rs");
        include!("functions/cloudfront/get_realtime_log_config.rs");
        include!("functions/cloudfront/get_response_headers_policy.rs");
    }
    pub mod cloudhsmv2 {
        include!("functions/cloudhsmv2/get_cluster.rs");
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
    pub mod cfg {
        include!("types/cfg/configuration_aggregator_account_aggregation_source.rs");
        include!(
            "types/cfg/configuration_aggregator_organization_aggregation_source.rs"
        );
        include!("types/cfg/conformance_pack_input_parameter.rs");
        include!("types/cfg/delivery_channel_snapshot_delivery_properties.rs");
        include!("types/cfg/organization_conformance_pack_input_parameter.rs");
        include!("types/cfg/recorder_recording_group.rs");
        include!("types/cfg/recorder_recording_group_exclusion_by_resource_type.rs");
        include!("types/cfg/recorder_recording_group_recording_strategy.rs");
        include!("types/cfg/recorder_recording_mode.rs");
        include!("types/cfg/recorder_recording_mode_recording_mode_override.rs");
        include!("types/cfg/remediation_configuration_execution_controls.rs");
        include!(
            "types/cfg/remediation_configuration_execution_controls_ssm_controls.rs"
        );
        include!("types/cfg/remediation_configuration_parameter.rs");
        include!("types/cfg/rule_evaluation_mode.rs");
        include!("types/cfg/rule_scope.rs");
        include!("types/cfg/rule_source.rs");
        include!("types/cfg/rule_source_custom_policy_details.rs");
        include!("types/cfg/rule_source_source_detail.rs");
    }
    pub mod chatbot {
        include!("types/chatbot/slack_channel_configuration_timeouts.rs");
        include!("types/chatbot/teams_channel_configuration_timeouts.rs");
    }
    pub mod chime {
        include!("types/chime/sdkvoice_global_settings_voice_connector.rs");
        include!("types/chime/sdkvoice_sip_media_application_endpoints.rs");
        include!("types/chime/sdkvoice_sip_rule_target_application.rs");
        include!(
            "types/chime/sdkvoice_voice_profile_domain_server_side_encryption_configuration.rs"
        );
        include!("types/chime/voice_connector_group_connector.rs");
        include!("types/chime/voice_connector_organization_route.rs");
        include!(
            "types/chime/voice_connector_streaming_media_insights_configuration.rs"
        );
        include!("types/chime/voice_connector_termination_credentials_credential.rs");
    }
    pub mod chimesdkmediapipelines {
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_amazon_transcribe_call_analytics_processor_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_amazon_transcribe_call_analytics_processor_configuration_post_call_analytics_settings.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_amazon_transcribe_processor_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_kinesis_data_stream_sink_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_lambda_function_sink_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_s_3_recording_sink_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_sns_topic_sink_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_sqs_queue_sink_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_voice_analytics_processor_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_real_time_alert_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_real_time_alert_configuration_rule.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_real_time_alert_configuration_rule_issue_detection_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_real_time_alert_configuration_rule_keyword_match_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_real_time_alert_configuration_rule_sentiment_configuration.rs"
        );
    }
    pub mod cleanrooms {
        include!("types/cleanrooms/collaboration_data_encryption_metadata.rs");
        include!("types/cleanrooms/collaboration_member.rs");
        include!("types/cleanrooms/configured_table_table_reference.rs");
    }
    pub mod cloudformation {
        include!("types/cloudformation/cloud_formation_type_logging_config.rs");
        include!("types/cloudformation/stack_instances_deployment_targets.rs");
        include!("types/cloudformation/stack_instances_operation_preferences.rs");
        include!("types/cloudformation/stack_instances_stack_instance_summary.rs");
        include!("types/cloudformation/stack_set_auto_deployment.rs");
        include!("types/cloudformation/stack_set_instance_deployment_targets.rs");
        include!("types/cloudformation/stack_set_instance_operation_preferences.rs");
        include!("types/cloudformation/stack_set_instance_stack_instance_summary.rs");
        include!("types/cloudformation/stack_set_managed_execution.rs");
        include!("types/cloudformation/stack_set_operation_preferences.rs");
        include!("types/cloudformation/get_cloud_formation_type_logging_config.rs");
    }
    pub mod cloudfront {
        include!(
            "types/cloudfront/cache_policy_parameters_in_cache_key_and_forwarded_to_origin.rs"
        );
        include!(
            "types/cloudfront/cache_policy_parameters_in_cache_key_and_forwarded_to_origin_cookies_config.rs"
        );
        include!(
            "types/cloudfront/cache_policy_parameters_in_cache_key_and_forwarded_to_origin_cookies_config_cookies.rs"
        );
        include!(
            "types/cloudfront/cache_policy_parameters_in_cache_key_and_forwarded_to_origin_headers_config.rs"
        );
        include!(
            "types/cloudfront/cache_policy_parameters_in_cache_key_and_forwarded_to_origin_headers_config_headers.rs"
        );
        include!(
            "types/cloudfront/cache_policy_parameters_in_cache_key_and_forwarded_to_origin_query_strings_config.rs"
        );
        include!(
            "types/cloudfront/cache_policy_parameters_in_cache_key_and_forwarded_to_origin_query_strings_config_query_strings.rs"
        );
        include!(
            "types/cloudfront/continuous_deployment_policy_staging_distribution_dns_names.rs"
        );
        include!("types/cloudfront/continuous_deployment_policy_traffic_config.rs");
        include!(
            "types/cloudfront/continuous_deployment_policy_traffic_config_single_header_config.rs"
        );
        include!(
            "types/cloudfront/continuous_deployment_policy_traffic_config_single_weight_config.rs"
        );
        include!(
            "types/cloudfront/continuous_deployment_policy_traffic_config_single_weight_config_session_stickiness_config.rs"
        );
        include!("types/cloudfront/distribution_custom_error_response.rs");
        include!("types/cloudfront/distribution_default_cache_behavior.rs");
        include!(
            "types/cloudfront/distribution_default_cache_behavior_forwarded_values.rs"
        );
        include!(
            "types/cloudfront/distribution_default_cache_behavior_forwarded_values_cookies.rs"
        );
        include!(
            "types/cloudfront/distribution_default_cache_behavior_function_association.rs"
        );
        include!(
            "types/cloudfront/distribution_default_cache_behavior_lambda_function_association.rs"
        );
        include!("types/cloudfront/distribution_logging_config.rs");
        include!("types/cloudfront/distribution_ordered_cache_behavior.rs");
        include!(
            "types/cloudfront/distribution_ordered_cache_behavior_forwarded_values.rs"
        );
        include!(
            "types/cloudfront/distribution_ordered_cache_behavior_forwarded_values_cookies.rs"
        );
        include!(
            "types/cloudfront/distribution_ordered_cache_behavior_function_association.rs"
        );
        include!(
            "types/cloudfront/distribution_ordered_cache_behavior_lambda_function_association.rs"
        );
        include!("types/cloudfront/distribution_origin.rs");
        include!("types/cloudfront/distribution_origin_custom_header.rs");
        include!("types/cloudfront/distribution_origin_custom_origin_config.rs");
        include!("types/cloudfront/distribution_origin_group.rs");
        include!("types/cloudfront/distribution_origin_group_failover_criteria.rs");
        include!("types/cloudfront/distribution_origin_group_member.rs");
        include!("types/cloudfront/distribution_origin_origin_shield.rs");
        include!("types/cloudfront/distribution_origin_s_3_origin_config.rs");
        include!("types/cloudfront/distribution_origin_vpc_origin_config.rs");
        include!("types/cloudfront/distribution_restrictions.rs");
        include!("types/cloudfront/distribution_restrictions_geo_restriction.rs");
        include!("types/cloudfront/distribution_trusted_key_group.rs");
        include!("types/cloudfront/distribution_trusted_key_group_item.rs");
        include!("types/cloudfront/distribution_trusted_signer.rs");
        include!("types/cloudfront/distribution_trusted_signer_item.rs");
        include!("types/cloudfront/distribution_viewer_certificate.rs");
        include!(
            "types/cloudfront/field_level_encryption_config_content_type_profile_config.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_config_content_type_profile_config_content_type_profiles.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_config_content_type_profile_config_content_type_profiles_item.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_config_query_arg_profile_config.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_config_query_arg_profile_config_query_arg_profiles.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_config_query_arg_profile_config_query_arg_profiles_item.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_profile_encryption_entities.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_profile_encryption_entities_item.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_profile_encryption_entities_item_field_patterns.rs"
        );
        include!("types/cloudfront/key_value_store_timeouts.rs");
        include!("types/cloudfront/monitoring_subscription_monitoring_subscription.rs");
        include!(
            "types/cloudfront/monitoring_subscription_monitoring_subscription_realtime_metrics_subscription_config.rs"
        );
        include!("types/cloudfront/origin_request_policy_cookies_config.rs");
        include!("types/cloudfront/origin_request_policy_cookies_config_cookies.rs");
        include!("types/cloudfront/origin_request_policy_headers_config.rs");
        include!("types/cloudfront/origin_request_policy_headers_config_headers.rs");
        include!("types/cloudfront/origin_request_policy_query_strings_config.rs");
        include!(
            "types/cloudfront/origin_request_policy_query_strings_config_query_strings.rs"
        );
        include!("types/cloudfront/realtime_log_config_endpoint.rs");
        include!(
            "types/cloudfront/realtime_log_config_endpoint_kinesis_stream_config.rs"
        );
        include!("types/cloudfront/response_headers_policy_cors_config.rs");
        include!(
            "types/cloudfront/response_headers_policy_cors_config_access_control_allow_headers.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_cors_config_access_control_allow_methods.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_cors_config_access_control_allow_origins.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_cors_config_access_control_expose_headers.rs"
        );
        include!("types/cloudfront/response_headers_policy_custom_headers_config.rs");
        include!(
            "types/cloudfront/response_headers_policy_custom_headers_config_item.rs"
        );
        include!("types/cloudfront/response_headers_policy_remove_headers_config.rs");
        include!(
            "types/cloudfront/response_headers_policy_remove_headers_config_item.rs"
        );
        include!("types/cloudfront/response_headers_policy_security_headers_config.rs");
        include!(
            "types/cloudfront/response_headers_policy_security_headers_config_content_security_policy.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_security_headers_config_content_type_options.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_security_headers_config_frame_options.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_security_headers_config_referrer_policy.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_security_headers_config_strict_transport_security.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_security_headers_config_xss_protection.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_server_timing_headers_config.rs"
        );
        include!("types/cloudfront/vpc_origin_timeouts.rs");
        include!("types/cloudfront/vpc_origin_vpc_origin_endpoint_config.rs");
        include!(
            "types/cloudfront/vpc_origin_vpc_origin_endpoint_config_origin_ssl_protocols.rs"
        );
        include!(
            "types/cloudfront/get_cache_policy_parameters_in_cache_key_and_forwarded_to_origin.rs"
        );
        include!(
            "types/cloudfront/get_cache_policy_parameters_in_cache_key_and_forwarded_to_origin_cookies_config.rs"
        );
        include!(
            "types/cloudfront/get_cache_policy_parameters_in_cache_key_and_forwarded_to_origin_cookies_config_cookie.rs"
        );
        include!(
            "types/cloudfront/get_cache_policy_parameters_in_cache_key_and_forwarded_to_origin_headers_config.rs"
        );
        include!(
            "types/cloudfront/get_cache_policy_parameters_in_cache_key_and_forwarded_to_origin_headers_config_header.rs"
        );
        include!(
            "types/cloudfront/get_cache_policy_parameters_in_cache_key_and_forwarded_to_origin_query_strings_config.rs"
        );
        include!(
            "types/cloudfront/get_cache_policy_parameters_in_cache_key_and_forwarded_to_origin_query_strings_config_query_string.rs"
        );
        include!("types/cloudfront/get_origin_request_policy_cookies_config.rs");
        include!("types/cloudfront/get_origin_request_policy_cookies_config_cookie.rs");
        include!("types/cloudfront/get_origin_request_policy_headers_config.rs");
        include!("types/cloudfront/get_origin_request_policy_headers_config_header.rs");
        include!("types/cloudfront/get_origin_request_policy_query_strings_config.rs");
        include!(
            "types/cloudfront/get_origin_request_policy_query_strings_config_query_string.rs"
        );
        include!("types/cloudfront/get_realtime_log_config_endpoint.rs");
        include!(
            "types/cloudfront/get_realtime_log_config_endpoint_kinesis_stream_config.rs"
        );
        include!("types/cloudfront/get_response_headers_policy_cors_config.rs");
        include!(
            "types/cloudfront/get_response_headers_policy_cors_config_access_control_allow_header.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_cors_config_access_control_allow_method.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_cors_config_access_control_allow_origin.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_cors_config_access_control_expose_header.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_custom_headers_config.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_custom_headers_config_item.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_remove_headers_config.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_remove_headers_config_item.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_security_headers_config.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_security_headers_config_content_security_policy.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_security_headers_config_content_type_option.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_security_headers_config_frame_option.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_security_headers_config_referrer_policy.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_security_headers_config_strict_transport_security.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_security_headers_config_xss_protection.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_server_timing_headers_config.rs"
        );
    }
    pub mod cloudhsmv2 {
        include!("types/cloudhsmv2/cluster_cluster_certificate.rs");
        include!("types/cloudhsmv2/get_cluster_cluster_certificate.rs");
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
