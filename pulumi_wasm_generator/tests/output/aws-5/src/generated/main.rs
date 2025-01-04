pub mod codegurureviewer {
    include!("resources/codegurureviewer/repository_association.rs");
}
pub mod codepipeline {
    include!("resources/codepipeline/custom_action_type.rs");
    include!("resources/codepipeline/pipeline.rs");
    include!("resources/codepipeline/webhook.rs");
}
pub mod codestarconnections {
    include!("resources/codestarconnections/connection.rs");
    include!("resources/codestarconnections/host.rs");
}
pub mod codestarnotifications {
    include!("resources/codestarnotifications/notification_rule.rs");
}
pub mod cognito {
    include!("resources/cognito/identity_pool.rs");
    include!("resources/cognito/identity_pool_provider_principal_tag.rs");
    include!("resources/cognito/identity_pool_role_attachment.rs");
    include!("resources/cognito/identity_provider.rs");
    include!("resources/cognito/managed_user_pool_client.rs");
    include!("resources/cognito/resource_server.rs");
    include!("resources/cognito/risk_configuration.rs");
    include!("resources/cognito/user.rs");
    include!("resources/cognito/user_group.rs");
    include!("resources/cognito/user_in_group.rs");
    include!("resources/cognito/user_pool.rs");
    include!("resources/cognito/user_pool_client.rs");
    include!("resources/cognito/user_pool_domain.rs");
    include!("resources/cognito/user_pool_ui_customization.rs");
}
pub mod comprehend {
    include!("resources/comprehend/document_classifier.rs");
    include!("resources/comprehend/entity_recognizer.rs");
}
pub mod computeoptimizer {
    include!("resources/computeoptimizer/enrollment_status.rs");
    include!("resources/computeoptimizer/recommendation_preferences.rs");
}
pub mod connect {
    include!("resources/connect/bot_association.rs");
    include!("resources/connect/contact_flow.rs");
    include!("resources/connect/contact_flow_module.rs");
    include!("resources/connect/hours_of_operation.rs");
    include!("resources/connect/instance.rs");
    include!("resources/connect/instance_storage_config.rs");
    include!("resources/connect/lambda_function_association.rs");
    include!("resources/connect/phone_number.rs");
    include!("resources/connect/queue.rs");
    include!("resources/connect/quick_connect.rs");
    include!("resources/connect/routing_profile.rs");
    include!("resources/connect/security_profile.rs");
    include!("resources/connect/user.rs");
    include!("resources/connect/user_hierarchy_group.rs");
    include!("resources/connect/user_hierarchy_structure.rs");
    include!("resources/connect/vocabulary.rs");
}
pub mod controltower {
    include!("resources/controltower/control_tower_control.rs");
    include!("resources/controltower/landing_zone.rs");
}
pub mod costexplorer {
    include!("resources/costexplorer/anomaly_monitor.rs");
    include!("resources/costexplorer/anomaly_subscription.rs");
    include!("resources/costexplorer/cost_allocation_tag.rs");
    include!("resources/costexplorer/cost_category.rs");
}
pub mod functions {
    pub mod codestarconnections {
        include!("functions/codestarconnections/get_connection.rs");
    }
    pub mod cognito {
        include!("functions/cognito/get_identity_pool.rs");
        include!("functions/cognito/get_user_group.rs");
        include!("functions/cognito/get_user_groups.rs");
        include!("functions/cognito/get_user_pool.rs");
        include!("functions/cognito/get_user_pool_client.rs");
        include!("functions/cognito/get_user_pool_clients.rs");
        include!("functions/cognito/get_user_pool_signing_certificate.rs");
        include!("functions/cognito/get_user_pools.rs");
    }
    pub mod connect {
        include!("functions/connect/get_bot_association.rs");
        include!("functions/connect/get_contact_flow.rs");
        include!("functions/connect/get_contact_flow_module.rs");
        include!("functions/connect/get_hours_of_operation.rs");
        include!("functions/connect/get_instance.rs");
        include!("functions/connect/get_instance_storage_config.rs");
        include!("functions/connect/get_lambda_function_association.rs");
        include!("functions/connect/get_prompt.rs");
        include!("functions/connect/get_queue.rs");
        include!("functions/connect/get_quick_connect.rs");
        include!("functions/connect/get_routing_profile.rs");
        include!("functions/connect/get_security_profile.rs");
        include!("functions/connect/get_user.rs");
        include!("functions/connect/get_user_hierarchy_group.rs");
        include!("functions/connect/get_user_hierarchy_structure.rs");
        include!("functions/connect/get_vocabulary.rs");
    }
    pub mod controltower {
        include!("functions/controltower/get_controls.rs");
    }
    pub mod costexplorer {
        include!("functions/costexplorer/get_cost_category.rs");
        include!("functions/costexplorer/get_tags.rs");
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
    pub mod codegurureviewer {
        include!("types/codegurureviewer/repository_association_kms_key_details.rs");
        include!("types/codegurureviewer/repository_association_repository.rs");
        include!(
            "types/codegurureviewer/repository_association_repository_bitbucket.rs"
        );
        include!(
            "types/codegurureviewer/repository_association_repository_codecommit.rs"
        );
        include!(
            "types/codegurureviewer/repository_association_repository_github_enterprise_server.rs"
        );
        include!(
            "types/codegurureviewer/repository_association_repository_s_3_bucket.rs"
        );
        include!(
            "types/codegurureviewer/repository_association_s_3_repository_detail.rs"
        );
        include!(
            "types/codegurureviewer/repository_association_s_3_repository_detail_code_artifact.rs"
        );
    }
    pub mod codepipeline {
        include!("types/codepipeline/custom_action_type_configuration_property.rs");
        include!("types/codepipeline/custom_action_type_input_artifact_details.rs");
        include!("types/codepipeline/custom_action_type_output_artifact_details.rs");
        include!("types/codepipeline/custom_action_type_settings.rs");
        include!("types/codepipeline/pipeline_artifact_store.rs");
        include!("types/codepipeline/pipeline_artifact_store_encryption_key.rs");
        include!("types/codepipeline/pipeline_stage.rs");
        include!("types/codepipeline/pipeline_stage_action.rs");
        include!("types/codepipeline/pipeline_trigger.rs");
        include!("types/codepipeline/pipeline_trigger_git_configuration.rs");
        include!(
            "types/codepipeline/pipeline_trigger_git_configuration_pull_request.rs"
        );
        include!(
            "types/codepipeline/pipeline_trigger_git_configuration_pull_request_branches.rs"
        );
        include!(
            "types/codepipeline/pipeline_trigger_git_configuration_pull_request_file_paths.rs"
        );
        include!("types/codepipeline/pipeline_trigger_git_configuration_push.rs");
        include!(
            "types/codepipeline/pipeline_trigger_git_configuration_push_branches.rs"
        );
        include!(
            "types/codepipeline/pipeline_trigger_git_configuration_push_file_paths.rs"
        );
        include!("types/codepipeline/pipeline_trigger_git_configuration_push_tags.rs");
        include!("types/codepipeline/pipeline_variable.rs");
        include!("types/codepipeline/webhook_authentication_configuration.rs");
        include!("types/codepipeline/webhook_filter.rs");
    }
    pub mod codestarconnections {
        include!("types/codestarconnections/host_vpc_configuration.rs");
    }
    pub mod codestarnotifications {
        include!("types/codestarnotifications/notification_rule_target.rs");
    }
    pub mod cognito {
        include!("types/cognito/identity_pool_cognito_identity_provider.rs");
        include!("types/cognito/identity_pool_role_attachment_role_mapping.rs");
        include!(
            "types/cognito/identity_pool_role_attachment_role_mapping_mapping_rule.rs"
        );
        include!("types/cognito/managed_user_pool_client_analytics_configuration.rs");
        include!("types/cognito/managed_user_pool_client_token_validity_units.rs");
        include!("types/cognito/resource_server_scope.rs");
        include!(
            "types/cognito/risk_configuration_account_takeover_risk_configuration.rs"
        );
        include!(
            "types/cognito/risk_configuration_account_takeover_risk_configuration_actions.rs"
        );
        include!(
            "types/cognito/risk_configuration_account_takeover_risk_configuration_actions_high_action.rs"
        );
        include!(
            "types/cognito/risk_configuration_account_takeover_risk_configuration_actions_low_action.rs"
        );
        include!(
            "types/cognito/risk_configuration_account_takeover_risk_configuration_actions_medium_action.rs"
        );
        include!(
            "types/cognito/risk_configuration_account_takeover_risk_configuration_notify_configuration.rs"
        );
        include!(
            "types/cognito/risk_configuration_account_takeover_risk_configuration_notify_configuration_block_email.rs"
        );
        include!(
            "types/cognito/risk_configuration_account_takeover_risk_configuration_notify_configuration_mfa_email.rs"
        );
        include!(
            "types/cognito/risk_configuration_account_takeover_risk_configuration_notify_configuration_no_action_email.rs"
        );
        include!(
            "types/cognito/risk_configuration_compromised_credentials_risk_configuration.rs"
        );
        include!(
            "types/cognito/risk_configuration_compromised_credentials_risk_configuration_actions.rs"
        );
        include!("types/cognito/risk_configuration_risk_exception_configuration.rs");
        include!("types/cognito/user_pool_account_recovery_setting.rs");
        include!(
            "types/cognito/user_pool_account_recovery_setting_recovery_mechanism.rs"
        );
        include!("types/cognito/user_pool_admin_create_user_config.rs");
        include!(
            "types/cognito/user_pool_admin_create_user_config_invite_message_template.rs"
        );
        include!("types/cognito/user_pool_client_analytics_configuration.rs");
        include!("types/cognito/user_pool_client_token_validity_units.rs");
        include!("types/cognito/user_pool_device_configuration.rs");
        include!("types/cognito/user_pool_email_configuration.rs");
        include!("types/cognito/user_pool_lambda_config.rs");
        include!("types/cognito/user_pool_lambda_config_custom_email_sender.rs");
        include!("types/cognito/user_pool_lambda_config_custom_sms_sender.rs");
        include!("types/cognito/user_pool_lambda_config_pre_token_generation_config.rs");
        include!("types/cognito/user_pool_password_policy.rs");
        include!("types/cognito/user_pool_schema.rs");
        include!("types/cognito/user_pool_schema_number_attribute_constraints.rs");
        include!("types/cognito/user_pool_schema_string_attribute_constraints.rs");
        include!("types/cognito/user_pool_sms_configuration.rs");
        include!("types/cognito/user_pool_software_token_mfa_configuration.rs");
        include!("types/cognito/user_pool_user_attribute_update_settings.rs");
        include!("types/cognito/user_pool_user_pool_add_ons.rs");
        include!("types/cognito/user_pool_username_configuration.rs");
        include!("types/cognito/user_pool_verification_message_template.rs");
        include!("types/cognito/get_identity_pool_cognito_identity_provider.rs");
        include!("types/cognito/get_user_groups_group.rs");
        include!("types/cognito/get_user_pool_account_recovery_setting.rs");
        include!(
            "types/cognito/get_user_pool_account_recovery_setting_recovery_mechanism.rs"
        );
        include!("types/cognito/get_user_pool_admin_create_user_config.rs");
        include!(
            "types/cognito/get_user_pool_admin_create_user_config_invite_message_template.rs"
        );
        include!("types/cognito/get_user_pool_client_analytics_configuration.rs");
        include!("types/cognito/get_user_pool_client_token_validity_unit.rs");
        include!("types/cognito/get_user_pool_device_configuration.rs");
        include!("types/cognito/get_user_pool_email_configuration.rs");
        include!("types/cognito/get_user_pool_lambda_config.rs");
        include!("types/cognito/get_user_pool_lambda_config_custom_email_sender.rs");
        include!("types/cognito/get_user_pool_lambda_config_custom_sms_sender.rs");
        include!(
            "types/cognito/get_user_pool_lambda_config_pre_token_generation_config.rs"
        );
        include!("types/cognito/get_user_pool_schema_attribute.rs");
        include!(
            "types/cognito/get_user_pool_schema_attribute_number_attribute_constraint.rs"
        );
        include!(
            "types/cognito/get_user_pool_schema_attribute_string_attribute_constraint.rs"
        );
    }
    pub mod comprehend {
        include!("types/comprehend/document_classifier_input_data_config.rs");
        include!(
            "types/comprehend/document_classifier_input_data_config_augmented_manifest.rs"
        );
        include!("types/comprehend/document_classifier_output_data_config.rs");
        include!("types/comprehend/document_classifier_vpc_config.rs");
        include!("types/comprehend/entity_recognizer_input_data_config.rs");
        include!("types/comprehend/entity_recognizer_input_data_config_annotations.rs");
        include!(
            "types/comprehend/entity_recognizer_input_data_config_augmented_manifest.rs"
        );
        include!("types/comprehend/entity_recognizer_input_data_config_documents.rs");
        include!("types/comprehend/entity_recognizer_input_data_config_entity_list.rs");
        include!("types/comprehend/entity_recognizer_input_data_config_entity_type.rs");
        include!("types/comprehend/entity_recognizer_vpc_config.rs");
    }
    pub mod computeoptimizer {
        include!("types/computeoptimizer/enrollment_status_timeouts.rs");
        include!(
            "types/computeoptimizer/recommendation_preferences_external_metrics_preference.rs"
        );
        include!(
            "types/computeoptimizer/recommendation_preferences_preferred_resource.rs"
        );
        include!("types/computeoptimizer/recommendation_preferences_scope.rs");
        include!(
            "types/computeoptimizer/recommendation_preferences_utilization_preference.rs"
        );
        include!(
            "types/computeoptimizer/recommendation_preferences_utilization_preference_metric_parameters.rs"
        );
    }
    pub mod connect {
        include!("types/connect/bot_association_lex_bot.rs");
        include!("types/connect/hours_of_operation_config.rs");
        include!("types/connect/hours_of_operation_config_end_time.rs");
        include!("types/connect/hours_of_operation_config_start_time.rs");
        include!("types/connect/instance_storage_config_storage_config.rs");
        include!(
            "types/connect/instance_storage_config_storage_config_kinesis_firehose_config.rs"
        );
        include!(
            "types/connect/instance_storage_config_storage_config_kinesis_stream_config.rs"
        );
        include!(
            "types/connect/instance_storage_config_storage_config_kinesis_video_stream_config.rs"
        );
        include!(
            "types/connect/instance_storage_config_storage_config_kinesis_video_stream_config_encryption_config.rs"
        );
        include!("types/connect/instance_storage_config_storage_config_s_3_config.rs");
        include!(
            "types/connect/instance_storage_config_storage_config_s_3_config_encryption_config.rs"
        );
        include!("types/connect/phone_number_status.rs");
        include!("types/connect/queue_outbound_caller_config.rs");
        include!("types/connect/quick_connect_quick_connect_config.rs");
        include!("types/connect/quick_connect_quick_connect_config_phone_config.rs");
        include!("types/connect/quick_connect_quick_connect_config_queue_config.rs");
        include!("types/connect/quick_connect_quick_connect_config_user_config.rs");
        include!("types/connect/routing_profile_media_concurrency.rs");
        include!("types/connect/routing_profile_queue_config.rs");
        include!("types/connect/user_hierarchy_group_hierarchy_path.rs");
        include!("types/connect/user_hierarchy_group_hierarchy_path_level_fife.rs");
        include!("types/connect/user_hierarchy_group_hierarchy_path_level_four.rs");
        include!("types/connect/user_hierarchy_group_hierarchy_path_level_one.rs");
        include!("types/connect/user_hierarchy_group_hierarchy_path_level_three.rs");
        include!("types/connect/user_hierarchy_group_hierarchy_path_level_two.rs");
        include!("types/connect/user_hierarchy_structure_hierarchy_structure.rs");
        include!(
            "types/connect/user_hierarchy_structure_hierarchy_structure_level_five.rs"
        );
        include!(
            "types/connect/user_hierarchy_structure_hierarchy_structure_level_four.rs"
        );
        include!(
            "types/connect/user_hierarchy_structure_hierarchy_structure_level_one.rs"
        );
        include!(
            "types/connect/user_hierarchy_structure_hierarchy_structure_level_three.rs"
        );
        include!(
            "types/connect/user_hierarchy_structure_hierarchy_structure_level_two.rs"
        );
        include!("types/connect/user_identity_info.rs");
        include!("types/connect/user_phone_config.rs");
        include!("types/connect/get_bot_association_lex_bot.rs");
        include!("types/connect/get_hours_of_operation_config.rs");
        include!("types/connect/get_hours_of_operation_config_end_time.rs");
        include!("types/connect/get_hours_of_operation_config_start_time.rs");
        include!("types/connect/get_instance_storage_config_storage_config.rs");
        include!(
            "types/connect/get_instance_storage_config_storage_config_kinesis_firehose_config.rs"
        );
        include!(
            "types/connect/get_instance_storage_config_storage_config_kinesis_stream_config.rs"
        );
        include!(
            "types/connect/get_instance_storage_config_storage_config_kinesis_video_stream_config.rs"
        );
        include!(
            "types/connect/get_instance_storage_config_storage_config_kinesis_video_stream_config_encryption_config.rs"
        );
        include!(
            "types/connect/get_instance_storage_config_storage_config_s_3_config.rs"
        );
        include!(
            "types/connect/get_instance_storage_config_storage_config_s_3_config_encryption_config.rs"
        );
        include!("types/connect/get_queue_outbound_caller_config.rs");
        include!("types/connect/get_quick_connect_quick_connect_config.rs");
        include!("types/connect/get_quick_connect_quick_connect_config_phone_config.rs");
        include!("types/connect/get_quick_connect_quick_connect_config_queue_config.rs");
        include!("types/connect/get_quick_connect_quick_connect_config_user_config.rs");
        include!("types/connect/get_routing_profile_media_concurrency.rs");
        include!("types/connect/get_routing_profile_queue_config.rs");
        include!("types/connect/get_user_hierarchy_group_hierarchy_path.rs");
        include!("types/connect/get_user_hierarchy_group_hierarchy_path_level_fife.rs");
        include!("types/connect/get_user_hierarchy_group_hierarchy_path_level_four.rs");
        include!("types/connect/get_user_hierarchy_group_hierarchy_path_level_one.rs");
        include!("types/connect/get_user_hierarchy_group_hierarchy_path_level_three.rs");
        include!("types/connect/get_user_hierarchy_group_hierarchy_path_level_two.rs");
        include!("types/connect/get_user_hierarchy_structure_hierarchy_structure.rs");
        include!(
            "types/connect/get_user_hierarchy_structure_hierarchy_structure_level_fife.rs"
        );
        include!(
            "types/connect/get_user_hierarchy_structure_hierarchy_structure_level_four.rs"
        );
        include!(
            "types/connect/get_user_hierarchy_structure_hierarchy_structure_level_one.rs"
        );
        include!(
            "types/connect/get_user_hierarchy_structure_hierarchy_structure_level_three.rs"
        );
        include!(
            "types/connect/get_user_hierarchy_structure_hierarchy_structure_level_two.rs"
        );
        include!("types/connect/get_user_identity_info.rs");
        include!("types/connect/get_user_phone_config.rs");
    }
    pub mod controltower {
        include!("types/controltower/control_tower_control_parameter.rs");
        include!("types/controltower/landing_zone_drift_status.rs");
    }
    pub mod costexplorer {
        include!("types/costexplorer/anomaly_subscription_subscriber.rs");
        include!("types/costexplorer/anomaly_subscription_threshold_expression.rs");
        include!("types/costexplorer/anomaly_subscription_threshold_expression_and.rs");
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_and_cost_category.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_and_dimension.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_and_tags.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_cost_category.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_dimension.rs"
        );
        include!("types/costexplorer/anomaly_subscription_threshold_expression_not.rs");
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_not_cost_category.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_not_dimension.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_not_tags.rs"
        );
        include!("types/costexplorer/anomaly_subscription_threshold_expression_or.rs");
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_or_cost_category.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_or_dimension.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_or_tags.rs"
        );
        include!("types/costexplorer/anomaly_subscription_threshold_expression_tags.rs");
        include!("types/costexplorer/cost_category_rule.rs");
        include!("types/costexplorer/cost_category_rule_inherited_value.rs");
        include!("types/costexplorer/cost_category_rule_rule.rs");
        include!("types/costexplorer/cost_category_rule_rule_and.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_and.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_and_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_and_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_and_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_not.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_not_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_not_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_not_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_or.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_or_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_or_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_or_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_not.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_and.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_and_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_and_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_and_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_not.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_not_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_not_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_not_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_or.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_or_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_or_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_or_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_or.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_and.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_and_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_and_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_and_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_not.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_not_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_not_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_not_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_or.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_or_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_or_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_or_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_tags.rs");
        include!("types/costexplorer/cost_category_split_charge_rule.rs");
        include!("types/costexplorer/cost_category_split_charge_rule_parameter.rs");
        include!("types/costexplorer/get_cost_category_rule.rs");
        include!("types/costexplorer/get_cost_category_rule_inherited_value.rs");
        include!("types/costexplorer/get_cost_category_rule_rule.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_and.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_and_and_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_and_and_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_and_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_cost_category.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_not.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_and_not_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_and_not_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_not_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_or.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_and_or_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_and_or_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_or_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_cost_category.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_and.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_not_and_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_not_and_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_and_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_cost_category.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_not.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_not_not_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_not_not_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_not_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_or.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_not_or_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_not_or_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_or_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_and.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_or_and_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_or_and_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_and_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_cost_category.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_not.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_or_not_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_or_not_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_not_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_or.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_or_or_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_or_or_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_or_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_tag.rs");
        include!("types/costexplorer/get_cost_category_split_charge_rule.rs");
        include!("types/costexplorer/get_cost_category_split_charge_rule_parameter.rs");
        include!("types/costexplorer/get_tags_filter.rs");
        include!("types/costexplorer/get_tags_filter_and.rs");
        include!("types/costexplorer/get_tags_filter_and_cost_category.rs");
        include!("types/costexplorer/get_tags_filter_and_dimension.rs");
        include!("types/costexplorer/get_tags_filter_and_tags.rs");
        include!("types/costexplorer/get_tags_filter_cost_category.rs");
        include!("types/costexplorer/get_tags_filter_dimension.rs");
        include!("types/costexplorer/get_tags_filter_not.rs");
        include!("types/costexplorer/get_tags_filter_not_cost_category.rs");
        include!("types/costexplorer/get_tags_filter_not_dimension.rs");
        include!("types/costexplorer/get_tags_filter_not_tags.rs");
        include!("types/costexplorer/get_tags_filter_or.rs");
        include!("types/costexplorer/get_tags_filter_or_cost_category.rs");
        include!("types/costexplorer/get_tags_filter_or_dimension.rs");
        include!("types/costexplorer/get_tags_filter_or_tags.rs");
        include!("types/costexplorer/get_tags_filter_tags.rs");
        include!("types/costexplorer/get_tags_sort_by.rs");
        include!("types/costexplorer/get_tags_time_period.rs");
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
