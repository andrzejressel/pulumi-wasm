pub mod synthetics {
    include!("resources/synthetics/canary.rs");
    include!("resources/synthetics/group.rs");
    include!("resources/synthetics/group_association.rs");
}
pub mod timestreaminfluxdb {
    include!("resources/timestreaminfluxdb/db_instance.rs");
}
pub mod timestreamwrite {
    include!("resources/timestreamwrite/database.rs");
    include!("resources/timestreamwrite/table.rs");
}
pub mod transcribe {
    include!("resources/transcribe/language_model.rs");
    include!("resources/transcribe/medical_vocabulary.rs");
    include!("resources/transcribe/vocabulary.rs");
    include!("resources/transcribe/vocabulary_filter.rs");
}
pub mod transfer {
    include!("resources/transfer/access.rs");
    include!("resources/transfer/agreement.rs");
    include!("resources/transfer/certificate.rs");
    include!("resources/transfer/connector.rs");
    include!("resources/transfer/profile.rs");
    include!("resources/transfer/server.rs");
    include!("resources/transfer/ssh_key.rs");
    include!("resources/transfer/tag.rs");
    include!("resources/transfer/user.rs");
    include!("resources/transfer/workflow.rs");
}
pub mod verifiedaccess {
    include!("resources/verifiedaccess/endpoint.rs");
    include!("resources/verifiedaccess/group.rs");
    include!("resources/verifiedaccess/instance.rs");
    include!("resources/verifiedaccess/instance_logging_configuration.rs");
    include!("resources/verifiedaccess/instance_trust_provider_attachment.rs");
    include!("resources/verifiedaccess/trust_provider.rs");
}
pub mod verifiedpermissions {
    include!("resources/verifiedpermissions/identity_source.rs");
    include!("resources/verifiedpermissions/policy.rs");
    include!("resources/verifiedpermissions/policy_store.rs");
    include!("resources/verifiedpermissions/policy_template.rs");
    include!("resources/verifiedpermissions/schema.rs");
}
pub mod vpc {
    include!("resources/vpc/endpoint_private_dns.rs");
    include!("resources/vpc/endpoint_service_private_dns_verification.rs");
    include!("resources/vpc/security_group_egress_rule.rs");
    include!("resources/vpc/security_group_ingress_rule.rs");
    include!("resources/vpc/security_group_vpc_association.rs");
}
pub mod vpclattice {
    include!("resources/vpclattice/access_log_subscription.rs");
    include!("resources/vpclattice/auth_policy.rs");
    include!("resources/vpclattice/listener.rs");
    include!("resources/vpclattice/listener_rule.rs");
    include!("resources/vpclattice/resource_policy.rs");
    include!("resources/vpclattice/service.rs");
    include!("resources/vpclattice/service_network.rs");
    include!("resources/vpclattice/service_network_service_association.rs");
    include!("resources/vpclattice/service_network_vpc_association.rs");
    include!("resources/vpclattice/target_group.rs");
    include!("resources/vpclattice/target_group_attachment.rs");
}
pub mod waf {
    include!("resources/waf/byte_match_set.rs");
    include!("resources/waf/geo_match_set.rs");
    include!("resources/waf/ip_set.rs");
    include!("resources/waf/rate_based_rule.rs");
    include!("resources/waf/regex_match_set.rs");
    include!("resources/waf/regex_pattern_set.rs");
    include!("resources/waf/rule.rs");
    include!("resources/waf/rule_group.rs");
    include!("resources/waf/size_constraint_set.rs");
    include!("resources/waf/sql_injection_match_set.rs");
    include!("resources/waf/web_acl.rs");
    include!("resources/waf/xss_match_set.rs");
}
pub mod functions {
    pub mod synthetics {
        include!("functions/synthetics/get_runtime_version.rs");
        include!("functions/synthetics/get_runtime_versions.rs");
    }
    pub mod timestreamwrite {
        include!("functions/timestreamwrite/get_database.rs");
        include!("functions/timestreamwrite/get_table.rs");
    }
    pub mod transfer {
        include!("functions/transfer/get_connector.rs");
        include!("functions/transfer/get_server.rs");
    }
    pub mod verifiedpermissions {
        include!("functions/verifiedpermissions/get_policy_store.rs");
    }
    pub mod vpc {
        include!("functions/vpc/get_security_group_rule.rs");
        include!("functions/vpc/get_security_group_rules.rs");
    }
    pub mod vpclattice {
        include!("functions/vpclattice/get_auth_policy.rs");
        include!("functions/vpclattice/get_listener.rs");
        include!("functions/vpclattice/get_resource_policy.rs");
        include!("functions/vpclattice/get_service.rs");
        include!("functions/vpclattice/get_service_network.rs");
    }
    pub mod waf {
        include!("functions/waf/get_ipset.rs");
        include!("functions/waf/get_rate_based_rule.rs");
        include!("functions/waf/get_rule.rs");
        include!("functions/waf/get_subscribed_rule_group.rs");
        include!("functions/waf/get_web_acl.rs");
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
    pub mod synthetics {
        include!("types/synthetics/canary_artifact_config.rs");
        include!("types/synthetics/canary_artifact_config_s_3_encryption.rs");
        include!("types/synthetics/canary_run_config.rs");
        include!("types/synthetics/canary_schedule.rs");
        include!("types/synthetics/canary_timeline.rs");
        include!("types/synthetics/canary_vpc_config.rs");
        include!("types/synthetics/get_runtime_versions_runtime_version.rs");
    }
    pub mod timestreaminfluxdb {
        include!("types/timestreaminfluxdb/db_instance_log_delivery_configuration.rs");
        include!(
            "types/timestreaminfluxdb/db_instance_log_delivery_configuration_s_3_configuration.rs"
        );
        include!("types/timestreaminfluxdb/db_instance_timeouts.rs");
    }
    pub mod timestreamwrite {
        include!("types/timestreamwrite/table_magnetic_store_write_properties.rs");
        include!(
            "types/timestreamwrite/table_magnetic_store_write_properties_magnetic_store_rejected_data_location.rs"
        );
        include!(
            "types/timestreamwrite/table_magnetic_store_write_properties_magnetic_store_rejected_data_location_s_3_configuration.rs"
        );
        include!("types/timestreamwrite/table_retention_properties.rs");
        include!("types/timestreamwrite/table_schema.rs");
        include!("types/timestreamwrite/table_schema_composite_partition_key.rs");
        include!("types/timestreamwrite/get_table_magnetic_store_write_property.rs");
        include!(
            "types/timestreamwrite/get_table_magnetic_store_write_property_magnetic_store_rejected_data_location.rs"
        );
        include!(
            "types/timestreamwrite/get_table_magnetic_store_write_property_magnetic_store_rejected_data_location_s_3_configuration.rs"
        );
        include!("types/timestreamwrite/get_table_retention_property.rs");
        include!("types/timestreamwrite/get_table_schema.rs");
        include!("types/timestreamwrite/get_table_schema_composite_partition_key.rs");
    }
    pub mod transcribe {
        include!("types/transcribe/language_model_input_data_config.rs");
    }
    pub mod transfer {
        include!("types/transfer/access_home_directory_mapping.rs");
        include!("types/transfer/access_posix_profile.rs");
        include!("types/transfer/connector_as_2_config.rs");
        include!("types/transfer/connector_sftp_config.rs");
        include!("types/transfer/server_endpoint_details.rs");
        include!("types/transfer/server_protocol_details.rs");
        include!("types/transfer/server_s_3_storage_options.rs");
        include!("types/transfer/server_workflow_details.rs");
        include!("types/transfer/server_workflow_details_on_partial_upload.rs");
        include!("types/transfer/server_workflow_details_on_upload.rs");
        include!("types/transfer/user_home_directory_mapping.rs");
        include!("types/transfer/user_posix_profile.rs");
        include!("types/transfer/workflow_on_exception_step.rs");
        include!("types/transfer/workflow_on_exception_step_copy_step_details.rs");
        include!(
            "types/transfer/workflow_on_exception_step_copy_step_details_destination_file_location.rs"
        );
        include!(
            "types/transfer/workflow_on_exception_step_copy_step_details_destination_file_location_efs_file_location.rs"
        );
        include!(
            "types/transfer/workflow_on_exception_step_copy_step_details_destination_file_location_s_3_file_location.rs"
        );
        include!("types/transfer/workflow_on_exception_step_custom_step_details.rs");
        include!("types/transfer/workflow_on_exception_step_decrypt_step_details.rs");
        include!(
            "types/transfer/workflow_on_exception_step_decrypt_step_details_destination_file_location.rs"
        );
        include!(
            "types/transfer/workflow_on_exception_step_decrypt_step_details_destination_file_location_efs_file_location.rs"
        );
        include!(
            "types/transfer/workflow_on_exception_step_decrypt_step_details_destination_file_location_s_3_file_location.rs"
        );
        include!("types/transfer/workflow_on_exception_step_delete_step_details.rs");
        include!("types/transfer/workflow_on_exception_step_tag_step_details.rs");
        include!("types/transfer/workflow_on_exception_step_tag_step_details_tag.rs");
        include!("types/transfer/workflow_step.rs");
        include!("types/transfer/workflow_step_copy_step_details.rs");
        include!(
            "types/transfer/workflow_step_copy_step_details_destination_file_location.rs"
        );
        include!(
            "types/transfer/workflow_step_copy_step_details_destination_file_location_efs_file_location.rs"
        );
        include!(
            "types/transfer/workflow_step_copy_step_details_destination_file_location_s_3_file_location.rs"
        );
        include!("types/transfer/workflow_step_custom_step_details.rs");
        include!("types/transfer/workflow_step_decrypt_step_details.rs");
        include!(
            "types/transfer/workflow_step_decrypt_step_details_destination_file_location.rs"
        );
        include!(
            "types/transfer/workflow_step_decrypt_step_details_destination_file_location_efs_file_location.rs"
        );
        include!(
            "types/transfer/workflow_step_decrypt_step_details_destination_file_location_s_3_file_location.rs"
        );
        include!("types/transfer/workflow_step_delete_step_details.rs");
        include!("types/transfer/workflow_step_tag_step_details.rs");
        include!("types/transfer/workflow_step_tag_step_details_tag.rs");
        include!("types/transfer/get_connector_as_2_config.rs");
        include!("types/transfer/get_connector_sftp_config.rs");
    }
    pub mod verifiedaccess {
        include!("types/verifiedaccess/endpoint_load_balancer_options.rs");
        include!("types/verifiedaccess/endpoint_network_interface_options.rs");
        include!("types/verifiedaccess/endpoint_sse_specification.rs");
        include!("types/verifiedaccess/group_sse_configuration.rs");
        include!("types/verifiedaccess/instance_logging_configuration_access_logs.rs");
        include!(
            "types/verifiedaccess/instance_logging_configuration_access_logs_cloudwatch_logs.rs"
        );
        include!(
            "types/verifiedaccess/instance_logging_configuration_access_logs_kinesis_data_firehose.rs"
        );
        include!(
            "types/verifiedaccess/instance_logging_configuration_access_logs_s_3.rs"
        );
        include!("types/verifiedaccess/instance_verified_access_trust_provider.rs");
        include!("types/verifiedaccess/trust_provider_device_options.rs");
        include!("types/verifiedaccess/trust_provider_oidc_options.rs");
    }
    pub mod verifiedpermissions {
        include!("types/verifiedpermissions/identity_source_configuration.rs");
        include!(
            "types/verifiedpermissions/identity_source_configuration_cognito_user_pool_configuration.rs"
        );
        include!(
            "types/verifiedpermissions/identity_source_configuration_cognito_user_pool_configuration_group_configuration.rs"
        );
        include!(
            "types/verifiedpermissions/identity_source_configuration_open_id_connect_configuration.rs"
        );
        include!(
            "types/verifiedpermissions/identity_source_configuration_open_id_connect_configuration_group_configuration.rs"
        );
        include!(
            "types/verifiedpermissions/identity_source_configuration_open_id_connect_configuration_token_selection.rs"
        );
        include!(
            "types/verifiedpermissions/identity_source_configuration_open_id_connect_configuration_token_selection_access_token_only.rs"
        );
        include!(
            "types/verifiedpermissions/identity_source_configuration_open_id_connect_configuration_token_selection_identity_token_only.rs"
        );
        include!("types/verifiedpermissions/policy_definition.rs");
        include!("types/verifiedpermissions/policy_definition_static.rs");
        include!("types/verifiedpermissions/policy_definition_template_linked.rs");
        include!(
            "types/verifiedpermissions/policy_definition_template_linked_principal.rs"
        );
        include!(
            "types/verifiedpermissions/policy_definition_template_linked_resource.rs"
        );
        include!("types/verifiedpermissions/policy_store_validation_settings.rs");
        include!("types/verifiedpermissions/schema_definition.rs");
        include!("types/verifiedpermissions/get_policy_store_validation_setting.rs");
    }
    pub mod vpc {
        include!("types/vpc/endpoint_service_private_dns_verification_timeouts.rs");
        include!("types/vpc/security_group_vpc_association_timeouts.rs");
        include!("types/vpc/get_security_group_rule_filter.rs");
        include!("types/vpc/get_security_group_rules_filter.rs");
    }
    pub mod vpclattice {
        include!("types/vpclattice/listener_default_action.rs");
        include!("types/vpclattice/listener_default_action_fixed_response.rs");
        include!("types/vpclattice/listener_default_action_forward.rs");
        include!("types/vpclattice/listener_default_action_forward_target_group.rs");
        include!("types/vpclattice/listener_rule_action.rs");
        include!("types/vpclattice/listener_rule_action_fixed_response.rs");
        include!("types/vpclattice/listener_rule_action_forward.rs");
        include!("types/vpclattice/listener_rule_action_forward_target_group.rs");
        include!("types/vpclattice/listener_rule_match.rs");
        include!("types/vpclattice/listener_rule_match_http_match.rs");
        include!("types/vpclattice/listener_rule_match_http_match_header_match.rs");
        include!(
            "types/vpclattice/listener_rule_match_http_match_header_match_match.rs"
        );
        include!("types/vpclattice/listener_rule_match_http_match_path_match.rs");
        include!("types/vpclattice/listener_rule_match_http_match_path_match_match.rs");
        include!("types/vpclattice/service_dns_entry.rs");
        include!("types/vpclattice/service_network_service_association_dns_entry.rs");
        include!("types/vpclattice/target_group_attachment_target.rs");
        include!("types/vpclattice/target_group_config.rs");
        include!("types/vpclattice/target_group_config_health_check.rs");
        include!("types/vpclattice/target_group_config_health_check_matcher.rs");
        include!("types/vpclattice/get_listener_default_action.rs");
        include!("types/vpclattice/get_listener_default_action_fixed_response.rs");
        include!("types/vpclattice/get_listener_default_action_forward.rs");
        include!("types/vpclattice/get_listener_default_action_forward_target_group.rs");
        include!("types/vpclattice/get_service_dns_entry.rs");
    }
    pub mod waf {
        include!("types/waf/byte_match_set_byte_match_tuple.rs");
        include!("types/waf/byte_match_set_byte_match_tuple_field_to_match.rs");
        include!("types/waf/geo_match_set_geo_match_constraint.rs");
        include!("types/waf/ip_set_ip_set_descriptor.rs");
        include!("types/waf/rate_based_rule_predicate.rs");
        include!("types/waf/regex_match_set_regex_match_tuple.rs");
        include!("types/waf/regex_match_set_regex_match_tuple_field_to_match.rs");
        include!("types/waf/rule_group_activated_rule.rs");
        include!("types/waf/rule_group_activated_rule_action.rs");
        include!("types/waf/rule_predicate.rs");
        include!("types/waf/size_constraint_set_size_constraint.rs");
        include!("types/waf/size_constraint_set_size_constraint_field_to_match.rs");
        include!("types/waf/sql_injection_match_set_sql_injection_match_tuple.rs");
        include!(
            "types/waf/sql_injection_match_set_sql_injection_match_tuple_field_to_match.rs"
        );
        include!("types/waf/web_acl_default_action.rs");
        include!("types/waf/web_acl_logging_configuration.rs");
        include!("types/waf/web_acl_logging_configuration_redacted_fields.rs");
        include!(
            "types/waf/web_acl_logging_configuration_redacted_fields_field_to_match.rs"
        );
        include!("types/waf/web_acl_rule.rs");
        include!("types/waf/web_acl_rule_action.rs");
        include!("types/waf/web_acl_rule_override_action.rs");
        include!("types/waf/xss_match_set_xss_match_tuple.rs");
        include!("types/waf/xss_match_set_xss_match_tuple_field_to_match.rs");
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
