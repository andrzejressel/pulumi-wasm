pub mod imagebuilder {
    include!("resources/imagebuilder/component.rs");
    include!("resources/imagebuilder/container_recipe.rs");
    include!("resources/imagebuilder/distribution_configuration.rs");
    include!("resources/imagebuilder/image.rs");
    include!("resources/imagebuilder/image_pipeline.rs");
    include!("resources/imagebuilder/image_recipe.rs");
    include!("resources/imagebuilder/infrastructure_configuration.rs");
    include!("resources/imagebuilder/lifecycle_policy.rs");
    include!("resources/imagebuilder/workflow.rs");
}
pub mod inspector {
    include!("resources/inspector/assessment_target.rs");
    include!("resources/inspector/assessment_template.rs");
    include!("resources/inspector/resource_group.rs");
}
pub mod inspector2 {
    include!("resources/inspector2/delegated_admin_account.rs");
    include!("resources/inspector2/enabler.rs");
    include!("resources/inspector2/member_association.rs");
    include!("resources/inspector2/organization_configuration.rs");
}
pub mod iot {
    include!("resources/iot/authorizer.rs");
    include!("resources/iot/billing_group.rs");
    include!("resources/iot/ca_certificate.rs");
    include!("resources/iot/certificate.rs");
    include!("resources/iot/domain_configuration.rs");
    include!("resources/iot/event_configurations.rs");
    include!("resources/iot/indexing_configuration.rs");
    include!("resources/iot/logging_options.rs");
    include!("resources/iot/policy.rs");
    include!("resources/iot/policy_attachment.rs");
    include!("resources/iot/provisioning_template.rs");
    include!("resources/iot/role_alias.rs");
    include!("resources/iot/thing.rs");
    include!("resources/iot/thing_group.rs");
    include!("resources/iot/thing_group_membership.rs");
    include!("resources/iot/thing_principal_attachment.rs");
    include!("resources/iot/thing_type.rs");
    include!("resources/iot/topic_rule.rs");
    include!("resources/iot/topic_rule_destination.rs");
}
pub mod ivs {
    include!("resources/ivs/channel.rs");
    include!("resources/ivs/playback_key_pair.rs");
    include!("resources/ivs/recording_configuration.rs");
}
pub mod ivschat {
    include!("resources/ivschat/logging_configuration.rs");
    include!("resources/ivschat/room.rs");
}
pub mod kendra {
    include!("resources/kendra/data_source.rs");
    include!("resources/kendra/experience.rs");
    include!("resources/kendra/faq.rs");
    include!("resources/kendra/index.rs");
    include!("resources/kendra/query_suggestions_block_list.rs");
    include!("resources/kendra/thesaurus.rs");
}
pub mod keyspaces {
    include!("resources/keyspaces/keyspace.rs");
    include!("resources/keyspaces/table.rs");
}
pub mod kinesis {
    include!("resources/kinesis/analytics_application.rs");
    include!("resources/kinesis/firehose_delivery_stream.rs");
    include!("resources/kinesis/resource_policy.rs");
    include!("resources/kinesis/stream.rs");
    include!("resources/kinesis/stream_consumer.rs");
    include!("resources/kinesis/video_stream.rs");
}
pub mod functions {
    pub mod imagebuilder {
        include!("functions/imagebuilder/get_component.rs");
        include!("functions/imagebuilder/get_components.rs");
        include!("functions/imagebuilder/get_container_recipe.rs");
        include!("functions/imagebuilder/get_container_recipes.rs");
        include!("functions/imagebuilder/get_distribution_configuration.rs");
        include!("functions/imagebuilder/get_distribution_configurations.rs");
        include!("functions/imagebuilder/get_image.rs");
        include!("functions/imagebuilder/get_image_pipeline.rs");
        include!("functions/imagebuilder/get_image_pipelines.rs");
        include!("functions/imagebuilder/get_image_recipe.rs");
        include!("functions/imagebuilder/get_image_recipes.rs");
        include!("functions/imagebuilder/get_infrastructure_configuration.rs");
        include!("functions/imagebuilder/get_infrastructure_configurations.rs");
    }
    pub mod inspector {
        include!("functions/inspector/get_rules_packages.rs");
    }
    pub mod iot {
        include!("functions/iot/get_endpoint.rs");
        include!("functions/iot/get_registration_code.rs");
    }
    pub mod ivs {
        include!("functions/ivs/get_stream_key.rs");
    }
    pub mod kendra {
        include!("functions/kendra/get_experience.rs");
        include!("functions/kendra/get_faq.rs");
        include!("functions/kendra/get_index.rs");
        include!("functions/kendra/get_query_suggestions_block_list.rs");
        include!("functions/kendra/get_thesaurus.rs");
    }
    pub mod kinesis {
        include!("functions/kinesis/get_firehose_delivery_stream.rs");
        include!("functions/kinesis/get_stream.rs");
        include!("functions/kinesis/get_stream_consumer.rs");
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
    pub mod imagebuilder {
        include!("types/imagebuilder/container_recipe_component.rs");
        include!("types/imagebuilder/container_recipe_component_parameter.rs");
        include!("types/imagebuilder/container_recipe_instance_configuration.rs");
        include!(
            "types/imagebuilder/container_recipe_instance_configuration_block_device_mapping.rs"
        );
        include!(
            "types/imagebuilder/container_recipe_instance_configuration_block_device_mapping_ebs.rs"
        );
        include!("types/imagebuilder/container_recipe_target_repository.rs");
        include!("types/imagebuilder/distribution_configuration_distribution.rs");
        include!(
            "types/imagebuilder/distribution_configuration_distribution_ami_distribution_configuration.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_ami_distribution_configuration_launch_permission.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_container_distribution_configuration.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_container_distribution_configuration_target_repository.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_fast_launch_configuration.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_fast_launch_configuration_launch_template.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_fast_launch_configuration_snapshot_configuration.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_launch_template_configuration.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_s_3_export_configuration.rs"
        );
        include!("types/imagebuilder/image_image_scanning_configuration.rs");
        include!(
            "types/imagebuilder/image_image_scanning_configuration_ecr_configuration.rs"
        );
        include!("types/imagebuilder/image_image_tests_configuration.rs");
        include!("types/imagebuilder/image_output_resource.rs");
        include!("types/imagebuilder/image_output_resource_ami.rs");
        include!("types/imagebuilder/image_output_resource_container.rs");
        include!("types/imagebuilder/image_pipeline_image_scanning_configuration.rs");
        include!(
            "types/imagebuilder/image_pipeline_image_scanning_configuration_ecr_configuration.rs"
        );
        include!("types/imagebuilder/image_pipeline_image_tests_configuration.rs");
        include!("types/imagebuilder/image_pipeline_schedule.rs");
        include!("types/imagebuilder/image_pipeline_workflow.rs");
        include!("types/imagebuilder/image_pipeline_workflow_parameter.rs");
        include!("types/imagebuilder/image_recipe_block_device_mapping.rs");
        include!("types/imagebuilder/image_recipe_block_device_mapping_ebs.rs");
        include!("types/imagebuilder/image_recipe_component.rs");
        include!("types/imagebuilder/image_recipe_component_parameter.rs");
        include!("types/imagebuilder/image_recipe_systems_manager_agent.rs");
        include!("types/imagebuilder/image_workflow.rs");
        include!("types/imagebuilder/image_workflow_parameter.rs");
        include!(
            "types/imagebuilder/infrastructure_configuration_instance_metadata_options.rs"
        );
        include!("types/imagebuilder/infrastructure_configuration_logging.rs");
        include!("types/imagebuilder/infrastructure_configuration_logging_s_3_logs.rs");
        include!("types/imagebuilder/lifecycle_policy_policy_detail.rs");
        include!("types/imagebuilder/lifecycle_policy_policy_detail_action.rs");
        include!(
            "types/imagebuilder/lifecycle_policy_policy_detail_action_include_resources.rs"
        );
        include!("types/imagebuilder/lifecycle_policy_policy_detail_exclusion_rules.rs");
        include!(
            "types/imagebuilder/lifecycle_policy_policy_detail_exclusion_rules_amis.rs"
        );
        include!(
            "types/imagebuilder/lifecycle_policy_policy_detail_exclusion_rules_amis_last_launched.rs"
        );
        include!("types/imagebuilder/lifecycle_policy_policy_detail_filter.rs");
        include!("types/imagebuilder/lifecycle_policy_resource_selection.rs");
        include!("types/imagebuilder/lifecycle_policy_resource_selection_recipe.rs");
        include!("types/imagebuilder/get_components_filter.rs");
        include!("types/imagebuilder/get_container_recipe_component.rs");
        include!("types/imagebuilder/get_container_recipe_component_parameter.rs");
        include!("types/imagebuilder/get_container_recipe_instance_configuration.rs");
        include!(
            "types/imagebuilder/get_container_recipe_instance_configuration_block_device_mapping.rs"
        );
        include!(
            "types/imagebuilder/get_container_recipe_instance_configuration_block_device_mapping_eb.rs"
        );
        include!("types/imagebuilder/get_container_recipe_target_repository.rs");
        include!("types/imagebuilder/get_container_recipes_filter.rs");
        include!("types/imagebuilder/get_distribution_configuration_distribution.rs");
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_ami_distribution_configuration.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_ami_distribution_configuration_launch_permission.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_container_distribution_configuration.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_container_distribution_configuration_target_repository.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_fast_launch_configuration.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_fast_launch_configuration_launch_template.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_fast_launch_configuration_snapshot_configuration.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_launch_template_configuration.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_s_3_export_configuration.rs"
        );
        include!("types/imagebuilder/get_distribution_configurations_filter.rs");
        include!("types/imagebuilder/get_image_image_scanning_configuration.rs");
        include!(
            "types/imagebuilder/get_image_image_scanning_configuration_ecr_configuration.rs"
        );
        include!("types/imagebuilder/get_image_image_tests_configuration.rs");
        include!("types/imagebuilder/get_image_output_resource.rs");
        include!("types/imagebuilder/get_image_output_resource_ami.rs");
        include!("types/imagebuilder/get_image_output_resource_container.rs");
        include!(
            "types/imagebuilder/get_image_pipeline_image_scanning_configuration.rs"
        );
        include!(
            "types/imagebuilder/get_image_pipeline_image_scanning_configuration_ecr_configuration.rs"
        );
        include!("types/imagebuilder/get_image_pipeline_image_tests_configuration.rs");
        include!("types/imagebuilder/get_image_pipeline_schedule.rs");
        include!("types/imagebuilder/get_image_pipelines_filter.rs");
        include!("types/imagebuilder/get_image_recipe_block_device_mapping.rs");
        include!("types/imagebuilder/get_image_recipe_block_device_mapping_eb.rs");
        include!("types/imagebuilder/get_image_recipe_component.rs");
        include!("types/imagebuilder/get_image_recipe_component_parameter.rs");
        include!("types/imagebuilder/get_image_recipes_filter.rs");
        include!(
            "types/imagebuilder/get_infrastructure_configuration_instance_metadata_option.rs"
        );
        include!("types/imagebuilder/get_infrastructure_configuration_logging.rs");
        include!(
            "types/imagebuilder/get_infrastructure_configuration_logging_s_3_log.rs"
        );
        include!("types/imagebuilder/get_infrastructure_configurations_filter.rs");
    }
    pub mod inspector {
        include!("types/inspector/assessment_template_event_subscription.rs");
    }
    pub mod inspector2 {
        include!("types/inspector2/organization_configuration_auto_enable.rs");
    }
    pub mod iot {
        include!("types/iot/billing_group_metadata.rs");
        include!("types/iot/billing_group_properties.rs");
        include!("types/iot/ca_certificate_registration_config.rs");
        include!("types/iot/ca_certificate_validity.rs");
        include!("types/iot/domain_configuration_authorizer_config.rs");
        include!("types/iot/domain_configuration_tls_config.rs");
        include!(
            "types/iot/indexing_configuration_thing_group_indexing_configuration.rs"
        );
        include!(
            "types/iot/indexing_configuration_thing_group_indexing_configuration_custom_field.rs"
        );
        include!(
            "types/iot/indexing_configuration_thing_group_indexing_configuration_managed_field.rs"
        );
        include!("types/iot/indexing_configuration_thing_indexing_configuration.rs");
        include!(
            "types/iot/indexing_configuration_thing_indexing_configuration_custom_field.rs"
        );
        include!(
            "types/iot/indexing_configuration_thing_indexing_configuration_filter.rs"
        );
        include!(
            "types/iot/indexing_configuration_thing_indexing_configuration_managed_field.rs"
        );
        include!("types/iot/provisioning_template_pre_provisioning_hook.rs");
        include!("types/iot/thing_group_metadata.rs");
        include!("types/iot/thing_group_metadata_root_to_parent_group.rs");
        include!("types/iot/thing_group_properties.rs");
        include!("types/iot/thing_group_properties_attribute_payload.rs");
        include!("types/iot/thing_type_properties.rs");
        include!("types/iot/topic_rule_cloudwatch_alarm.rs");
        include!("types/iot/topic_rule_cloudwatch_log.rs");
        include!("types/iot/topic_rule_cloudwatch_metric.rs");
        include!("types/iot/topic_rule_destination_vpc_configuration.rs");
        include!("types/iot/topic_rule_dynamodb.rs");
        include!("types/iot/topic_rule_dynamodbv_2.rs");
        include!("types/iot/topic_rule_dynamodbv_2_put_item.rs");
        include!("types/iot/topic_rule_elasticsearch.rs");
        include!("types/iot/topic_rule_error_action.rs");
        include!("types/iot/topic_rule_error_action_cloudwatch_alarm.rs");
        include!("types/iot/topic_rule_error_action_cloudwatch_logs.rs");
        include!("types/iot/topic_rule_error_action_cloudwatch_metric.rs");
        include!("types/iot/topic_rule_error_action_dynamodb.rs");
        include!("types/iot/topic_rule_error_action_dynamodbv_2.rs");
        include!("types/iot/topic_rule_error_action_dynamodbv_2_put_item.rs");
        include!("types/iot/topic_rule_error_action_elasticsearch.rs");
        include!("types/iot/topic_rule_error_action_firehose.rs");
        include!("types/iot/topic_rule_error_action_http.rs");
        include!("types/iot/topic_rule_error_action_http_http_header.rs");
        include!("types/iot/topic_rule_error_action_iot_analytics.rs");
        include!("types/iot/topic_rule_error_action_iot_events.rs");
        include!("types/iot/topic_rule_error_action_kafka.rs");
        include!("types/iot/topic_rule_error_action_kafka_header.rs");
        include!("types/iot/topic_rule_error_action_kinesis.rs");
        include!("types/iot/topic_rule_error_action_lambda.rs");
        include!("types/iot/topic_rule_error_action_republish.rs");
        include!("types/iot/topic_rule_error_action_s_3.rs");
        include!("types/iot/topic_rule_error_action_sns.rs");
        include!("types/iot/topic_rule_error_action_sqs.rs");
        include!("types/iot/topic_rule_error_action_step_functions.rs");
        include!("types/iot/topic_rule_error_action_timestream.rs");
        include!("types/iot/topic_rule_error_action_timestream_dimension.rs");
        include!("types/iot/topic_rule_error_action_timestream_timestamp.rs");
        include!("types/iot/topic_rule_firehose.rs");
        include!("types/iot/topic_rule_http.rs");
        include!("types/iot/topic_rule_http_http_header.rs");
        include!("types/iot/topic_rule_iot_analytic.rs");
        include!("types/iot/topic_rule_iot_event.rs");
        include!("types/iot/topic_rule_kafka.rs");
        include!("types/iot/topic_rule_kafka_header.rs");
        include!("types/iot/topic_rule_kinesis.rs");
        include!("types/iot/topic_rule_lambda.rs");
        include!("types/iot/topic_rule_republish.rs");
        include!("types/iot/topic_rule_s_3.rs");
        include!("types/iot/topic_rule_sns.rs");
        include!("types/iot/topic_rule_sqs.rs");
        include!("types/iot/topic_rule_step_function.rs");
        include!("types/iot/topic_rule_timestream.rs");
        include!("types/iot/topic_rule_timestream_dimension.rs");
        include!("types/iot/topic_rule_timestream_timestamp.rs");
    }
    pub mod ivs {
        include!("types/ivs/recording_configuration_destination_configuration.rs");
        include!("types/ivs/recording_configuration_destination_configuration_s_3.rs");
        include!("types/ivs/recording_configuration_thumbnail_configuration.rs");
    }
    pub mod ivschat {
        include!("types/ivschat/logging_configuration_destination_configuration.rs");
        include!(
            "types/ivschat/logging_configuration_destination_configuration_cloudwatch_logs.rs"
        );
        include!(
            "types/ivschat/logging_configuration_destination_configuration_firehose.rs"
        );
        include!("types/ivschat/logging_configuration_destination_configuration_s_3.rs");
        include!("types/ivschat/room_message_review_handler.rs");
    }
    pub mod kendra {
        include!("types/kendra/data_source_configuration.rs");
        include!("types/kendra/data_source_configuration_s_3_configuration.rs");
        include!(
            "types/kendra/data_source_configuration_s_3_configuration_access_control_list_configuration.rs"
        );
        include!(
            "types/kendra/data_source_configuration_s_3_configuration_documents_metadata_configuration.rs"
        );
        include!("types/kendra/data_source_configuration_web_crawler_configuration.rs");
        include!(
            "types/kendra/data_source_configuration_web_crawler_configuration_authentication_configuration.rs"
        );
        include!(
            "types/kendra/data_source_configuration_web_crawler_configuration_authentication_configuration_basic_authentication.rs"
        );
        include!(
            "types/kendra/data_source_configuration_web_crawler_configuration_proxy_configuration.rs"
        );
        include!(
            "types/kendra/data_source_configuration_web_crawler_configuration_urls.rs"
        );
        include!(
            "types/kendra/data_source_configuration_web_crawler_configuration_urls_seed_url_configuration.rs"
        );
        include!(
            "types/kendra/data_source_configuration_web_crawler_configuration_urls_site_maps_configuration.rs"
        );
        include!("types/kendra/data_source_custom_document_enrichment_configuration.rs");
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_inline_configuration.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_inline_configuration_condition.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_inline_configuration_condition_condition_on_value.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_inline_configuration_target.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_inline_configuration_target_target_document_attribute_value.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_post_extraction_hook_configuration.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_post_extraction_hook_configuration_invocation_condition.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_post_extraction_hook_configuration_invocation_condition_condition_on_value.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_pre_extraction_hook_configuration.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_pre_extraction_hook_configuration_invocation_condition.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_pre_extraction_hook_configuration_invocation_condition_condition_on_value.rs"
        );
        include!("types/kendra/experience_configuration.rs");
        include!(
            "types/kendra/experience_configuration_content_source_configuration.rs"
        );
        include!("types/kendra/experience_configuration_user_identity_configuration.rs");
        include!("types/kendra/experience_endpoint.rs");
        include!("types/kendra/faq_s_3_path.rs");
        include!("types/kendra/index_capacity_units.rs");
        include!("types/kendra/index_document_metadata_configuration_update.rs");
        include!(
            "types/kendra/index_document_metadata_configuration_update_relevance.rs"
        );
        include!("types/kendra/index_document_metadata_configuration_update_search.rs");
        include!("types/kendra/index_index_statistic.rs");
        include!("types/kendra/index_index_statistic_faq_statistic.rs");
        include!("types/kendra/index_index_statistic_text_document_statistic.rs");
        include!("types/kendra/index_server_side_encryption_configuration.rs");
        include!("types/kendra/index_user_group_resolution_configuration.rs");
        include!("types/kendra/index_user_token_configurations.rs");
        include!(
            "types/kendra/index_user_token_configurations_json_token_type_configuration.rs"
        );
        include!(
            "types/kendra/index_user_token_configurations_jwt_token_type_configuration.rs"
        );
        include!("types/kendra/query_suggestions_block_list_source_s_3_path.rs");
        include!("types/kendra/thesaurus_source_s_3_path.rs");
        include!("types/kendra/get_experience_configuration.rs");
        include!(
            "types/kendra/get_experience_configuration_content_source_configuration.rs"
        );
        include!(
            "types/kendra/get_experience_configuration_user_identity_configuration.rs"
        );
        include!("types/kendra/get_experience_endpoint.rs");
        include!("types/kendra/get_faq_s_3_path.rs");
        include!("types/kendra/get_index_capacity_unit.rs");
        include!("types/kendra/get_index_document_metadata_configuration_update.rs");
        include!(
            "types/kendra/get_index_document_metadata_configuration_update_relevance.rs"
        );
        include!(
            "types/kendra/get_index_document_metadata_configuration_update_search.rs"
        );
        include!("types/kendra/get_index_index_statistic.rs");
        include!("types/kendra/get_index_index_statistic_faq_statistic.rs");
        include!("types/kendra/get_index_index_statistic_text_document_statistic.rs");
        include!("types/kendra/get_index_server_side_encryption_configuration.rs");
        include!("types/kendra/get_index_user_group_resolution_configuration.rs");
        include!("types/kendra/get_index_user_token_configuration.rs");
        include!(
            "types/kendra/get_index_user_token_configuration_json_token_type_configuration.rs"
        );
        include!(
            "types/kendra/get_index_user_token_configuration_jwt_token_type_configuration.rs"
        );
        include!("types/kendra/get_query_suggestions_block_list_source_s_3_path.rs");
        include!("types/kendra/get_thesaurus_source_s_3_path.rs");
    }
    pub mod keyspaces {
        include!("types/keyspaces/keyspace_replication_specification.rs");
        include!("types/keyspaces/table_capacity_specification.rs");
        include!("types/keyspaces/table_client_side_timestamps.rs");
        include!("types/keyspaces/table_comment.rs");
        include!("types/keyspaces/table_encryption_specification.rs");
        include!("types/keyspaces/table_point_in_time_recovery.rs");
        include!("types/keyspaces/table_schema_definition.rs");
        include!("types/keyspaces/table_schema_definition_clustering_key.rs");
        include!("types/keyspaces/table_schema_definition_column.rs");
        include!("types/keyspaces/table_schema_definition_partition_key.rs");
        include!("types/keyspaces/table_schema_definition_static_column.rs");
        include!("types/keyspaces/table_ttl.rs");
    }
    pub mod kinesis {
        include!("types/kinesis/analytics_application_cloudwatch_logging_options.rs");
        include!("types/kinesis/analytics_application_inputs.rs");
        include!("types/kinesis/analytics_application_inputs_kinesis_firehose.rs");
        include!("types/kinesis/analytics_application_inputs_kinesis_stream.rs");
        include!("types/kinesis/analytics_application_inputs_parallelism.rs");
        include!(
            "types/kinesis/analytics_application_inputs_processing_configuration.rs"
        );
        include!(
            "types/kinesis/analytics_application_inputs_processing_configuration_lambda.rs"
        );
        include!("types/kinesis/analytics_application_inputs_schema.rs");
        include!("types/kinesis/analytics_application_inputs_schema_record_column.rs");
        include!("types/kinesis/analytics_application_inputs_schema_record_format.rs");
        include!(
            "types/kinesis/analytics_application_inputs_schema_record_format_mapping_parameters.rs"
        );
        include!(
            "types/kinesis/analytics_application_inputs_schema_record_format_mapping_parameters_csv.rs"
        );
        include!(
            "types/kinesis/analytics_application_inputs_schema_record_format_mapping_parameters_json.rs"
        );
        include!(
            "types/kinesis/analytics_application_inputs_starting_position_configuration.rs"
        );
        include!("types/kinesis/analytics_application_output.rs");
        include!("types/kinesis/analytics_application_output_kinesis_firehose.rs");
        include!("types/kinesis/analytics_application_output_kinesis_stream.rs");
        include!("types/kinesis/analytics_application_output_lambda.rs");
        include!("types/kinesis/analytics_application_output_schema.rs");
        include!("types/kinesis/analytics_application_reference_data_sources.rs");
        include!("types/kinesis/analytics_application_reference_data_sources_s_3.rs");
        include!("types/kinesis/analytics_application_reference_data_sources_schema.rs");
        include!(
            "types/kinesis/analytics_application_reference_data_sources_schema_record_column.rs"
        );
        include!(
            "types/kinesis/analytics_application_reference_data_sources_schema_record_format.rs"
        );
        include!(
            "types/kinesis/analytics_application_reference_data_sources_schema_record_format_mapping_parameters.rs"
        );
        include!(
            "types/kinesis/analytics_application_reference_data_sources_schema_record_format_mapping_parameters_csv.rs"
        );
        include!(
            "types/kinesis/analytics_application_reference_data_sources_schema_record_format_mapping_parameters_json.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration_vpc_config.rs"
        );
        include!("types/kinesis/firehose_delivery_stream_extended_s_3_configuration.rs");
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_input_format_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_input_format_configuration_deserializer.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_input_format_configuration_deserializer_hive_json_ser_de.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_input_format_configuration_deserializer_open_x_json_ser_de.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_output_format_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_output_format_configuration_serializer.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_output_format_configuration_serializer_orc_ser_de.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_output_format_configuration_serializer_parquet_ser_de.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_schema_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_dynamic_partitioning_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_s_3_backup_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_s_3_backup_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_request_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_request_configuration_common_attribute.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_secrets_manager_configuration.rs"
        );
        include!("types/kinesis/firehose_delivery_stream_iceberg_configuration.rs");
        include!(
            "types/kinesis/firehose_delivery_stream_iceberg_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_iceberg_configuration_destination_table_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_iceberg_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_iceberg_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_iceberg_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_iceberg_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_iceberg_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_kinesis_source_configuration.rs"
        );
        include!("types/kinesis/firehose_delivery_stream_msk_source_configuration.rs");
        include!(
            "types/kinesis/firehose_delivery_stream_msk_source_configuration_authentication_configuration.rs"
        );
        include!("types/kinesis/firehose_delivery_stream_opensearch_configuration.rs");
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_document_id_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_vpc_config.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration_vpc_config.rs"
        );
        include!("types/kinesis/firehose_delivery_stream_redshift_configuration.rs");
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_s_3_backup_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_s_3_backup_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_secrets_manager_configuration.rs"
        );
        include!("types/kinesis/firehose_delivery_stream_server_side_encryption.rs");
        include!("types/kinesis/firehose_delivery_stream_snowflake_configuration.rs");
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_secrets_manager_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_snowflake_role_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_snowflake_vpc_configuration.rs"
        );
        include!("types/kinesis/firehose_delivery_stream_splunk_configuration.rs");
        include!(
            "types/kinesis/firehose_delivery_stream_splunk_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_splunk_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_splunk_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_splunk_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_splunk_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_splunk_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_splunk_configuration_secrets_manager_configuration.rs"
        );
        include!("types/kinesis/stream_stream_mode_details.rs");
        include!("types/kinesis/get_stream_stream_mode_detail.rs");
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
}

interface register-interface {
    use pulumi-engine.{engine};
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
        version: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record register-resource-result {
        fields: list<register-resource-result-field>
    }

    register: func(engine: borrow<engine>, request: register-resource-request) -> register-resource-result;

    record resource-invoke-result-field {
        name: string,
        output: output
    }

    record resource-invoke-request {
        token: string,
        version: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record resource-invoke-result {
        fields: list<resource-invoke-result-field>
    }

    invoke: func(engine: borrow<engine>, request: resource-invoke-request) -> resource-invoke-result;
}",
        with : { "component:pulumi-wasm/output-interface@0.0.0-DEV" :
        pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
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
