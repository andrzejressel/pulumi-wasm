pub mod pipes {
    include!("resources/pipes/pipe.rs");
}
pub mod qldb {
    include!("resources/qldb/ledger.rs");
    include!("resources/qldb/stream.rs");
}
pub mod quicksight {
    include!("resources/quicksight/account_subscription.rs");
    include!("resources/quicksight/analysis.rs");
    include!("resources/quicksight/dashboard.rs");
    include!("resources/quicksight/data_set.rs");
    include!("resources/quicksight/data_source.rs");
    include!("resources/quicksight/folder.rs");
    include!("resources/quicksight/folder_membership.rs");
    include!("resources/quicksight/group.rs");
    include!("resources/quicksight/group_membership.rs");
    include!("resources/quicksight/iam_policy_assignment.rs");
    include!("resources/quicksight/ingestion.rs");
    include!("resources/quicksight/namespace.rs");
    include!("resources/quicksight/refresh_schedule.rs");
    include!("resources/quicksight/template.rs");
    include!("resources/quicksight/template_alias.rs");
    include!("resources/quicksight/theme.rs");
    include!("resources/quicksight/user.rs");
    include!("resources/quicksight/vpc_connection.rs");
}
pub mod ram {
    include!("resources/ram/principal_association.rs");
    include!("resources/ram/resource_association.rs");
    include!("resources/ram/resource_share.rs");
    include!("resources/ram/resource_share_accepter.rs");
    include!("resources/ram/sharing_with_organization.rs");
}
pub mod rbin {
    include!("resources/rbin/rule.rs");
}
pub mod rds {
    include!("resources/rds/certificate.rs");
    include!("resources/rds/cluster.rs");
    include!("resources/rds/cluster_activity_stream.rs");
    include!("resources/rds/cluster_endpoint.rs");
    include!("resources/rds/cluster_instance.rs");
    include!("resources/rds/cluster_parameter_group.rs");
    include!("resources/rds/cluster_role_association.rs");
    include!("resources/rds/cluster_snapshot.rs");
    include!("resources/rds/cluster_snapshot_copy.rs");
    include!("resources/rds/custom_db_engine_version.rs");
    include!("resources/rds/event_subscription.rs");
    include!("resources/rds/export_task.rs");
    include!("resources/rds/global_cluster.rs");
    include!("resources/rds/instance.rs");
    include!("resources/rds/instance_automated_backups_replication.rs");
    include!("resources/rds/instance_desired_state.rs");
    include!("resources/rds/integration.rs");
    include!("resources/rds/option_group.rs");
    include!("resources/rds/parameter_group.rs");
    include!("resources/rds/proxy.rs");
    include!("resources/rds/proxy_default_target_group.rs");
    include!("resources/rds/proxy_endpoint.rs");
    include!("resources/rds/proxy_target.rs");
    include!("resources/rds/reserved_instance.rs");
    include!("resources/rds/role_association.rs");
    include!("resources/rds/snapshot.rs");
    include!("resources/rds/snapshot_copy.rs");
    include!("resources/rds/subnet_group.rs");
}
pub mod redshift {
    include!("resources/redshift/authentication_profile.rs");
    include!("resources/redshift/cluster.rs");
    include!("resources/redshift/cluster_iam_roles.rs");
    include!("resources/redshift/cluster_snapshot.rs");
    include!("resources/redshift/data_share_authorization.rs");
    include!("resources/redshift/data_share_consumer_association.rs");
    include!("resources/redshift/endpoint_access.rs");
    include!("resources/redshift/endpoint_authorization.rs");
    include!("resources/redshift/event_subscription.rs");
    include!("resources/redshift/hsm_client_certificate.rs");
    include!("resources/redshift/hsm_configuration.rs");
    include!("resources/redshift/logging.rs");
    include!("resources/redshift/parameter_group.rs");
    include!("resources/redshift/partner.rs");
    include!("resources/redshift/resource_policy.rs");
    include!("resources/redshift/scheduled_action.rs");
    include!("resources/redshift/snapshot_copy.rs");
    include!("resources/redshift/snapshot_copy_grant.rs");
    include!("resources/redshift/snapshot_schedule.rs");
    include!("resources/redshift/snapshot_schedule_association.rs");
    include!("resources/redshift/subnet_group.rs");
    include!("resources/redshift/usage_limit.rs");
}
pub mod redshiftdata {
    include!("resources/redshiftdata/statement.rs");
}
pub mod functions {
    pub mod polly {
        include!("functions/polly/get_voices.rs");
    }
    pub mod pricing {
        include!("functions/pricing/get_product.rs");
    }
    pub mod qldb {
        include!("functions/qldb/get_ledger.rs");
    }
    pub mod quicksight {
        include!("functions/quicksight/get_analysis.rs");
        include!("functions/quicksight/get_data_set.rs");
        include!("functions/quicksight/get_quicksight_analysis.rs");
        include!("functions/quicksight/get_quicksight_group.rs");
        include!("functions/quicksight/get_quicksight_user.rs");
        include!("functions/quicksight/get_theme.rs");
    }
    pub mod ram {
        include!("functions/ram/get_resource_share.rs");
    }
    pub mod rds {
        include!("functions/rds/get_certificate.rs");
        include!("functions/rds/get_cluster.rs");
        include!("functions/rds/get_cluster_parameter_group.rs");
        include!("functions/rds/get_cluster_snapshot.rs");
        include!("functions/rds/get_clusters.rs");
        include!("functions/rds/get_engine_version.rs");
        include!("functions/rds/get_event_categories.rs");
        include!("functions/rds/get_instance.rs");
        include!("functions/rds/get_instances.rs");
        include!("functions/rds/get_orderable_db_instance.rs");
        include!("functions/rds/get_parameter_group.rs");
        include!("functions/rds/get_proxy.rs");
        include!("functions/rds/get_reserved_instance_offering.rs");
        include!("functions/rds/get_snapshot.rs");
        include!("functions/rds/get_subnet_group.rs");
    }
    pub mod redshift {
        include!("functions/redshift/get_cluster.rs");
        include!("functions/redshift/get_cluster_credentials.rs");
        include!("functions/redshift/get_data_shares.rs");
        include!("functions/redshift/get_orderable_cluster.rs");
        include!("functions/redshift/get_producer_data_shares.rs");
        include!("functions/redshift/get_service_account.rs");
        include!("functions/redshift/get_subnet_group.rs");
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
    pub mod pipes {
        include!("types/pipes/pipe_enrichment_parameters.rs");
        include!("types/pipes/pipe_enrichment_parameters_http_parameters.rs");
        include!("types/pipes/pipe_log_configuration.rs");
        include!(
            "types/pipes/pipe_log_configuration_cloudwatch_logs_log_destination.rs"
        );
        include!("types/pipes/pipe_log_configuration_firehose_log_destination.rs");
        include!("types/pipes/pipe_log_configuration_s_3_log_destination.rs");
        include!("types/pipes/pipe_source_parameters.rs");
        include!("types/pipes/pipe_source_parameters_activemq_broker_parameters.rs");
        include!(
            "types/pipes/pipe_source_parameters_activemq_broker_parameters_credentials.rs"
        );
        include!("types/pipes/pipe_source_parameters_dynamodb_stream_parameters.rs");
        include!(
            "types/pipes/pipe_source_parameters_dynamodb_stream_parameters_dead_letter_config.rs"
        );
        include!("types/pipes/pipe_source_parameters_filter_criteria.rs");
        include!("types/pipes/pipe_source_parameters_filter_criteria_filter.rs");
        include!("types/pipes/pipe_source_parameters_kinesis_stream_parameters.rs");
        include!(
            "types/pipes/pipe_source_parameters_kinesis_stream_parameters_dead_letter_config.rs"
        );
        include!(
            "types/pipes/pipe_source_parameters_managed_streaming_kafka_parameters.rs"
        );
        include!(
            "types/pipes/pipe_source_parameters_managed_streaming_kafka_parameters_credentials.rs"
        );
        include!("types/pipes/pipe_source_parameters_rabbitmq_broker_parameters.rs");
        include!(
            "types/pipes/pipe_source_parameters_rabbitmq_broker_parameters_credentials.rs"
        );
        include!("types/pipes/pipe_source_parameters_self_managed_kafka_parameters.rs");
        include!(
            "types/pipes/pipe_source_parameters_self_managed_kafka_parameters_credentials.rs"
        );
        include!(
            "types/pipes/pipe_source_parameters_self_managed_kafka_parameters_vpc.rs"
        );
        include!("types/pipes/pipe_source_parameters_sqs_queue_parameters.rs");
        include!("types/pipes/pipe_target_parameters.rs");
        include!("types/pipes/pipe_target_parameters_batch_job_parameters.rs");
        include!(
            "types/pipes/pipe_target_parameters_batch_job_parameters_array_properties.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_batch_job_parameters_container_overrides.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_batch_job_parameters_container_overrides_environment.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_batch_job_parameters_container_overrides_resource_requirement.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_batch_job_parameters_depends_on.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_batch_job_parameters_retry_strategy.rs"
        );
        include!("types/pipes/pipe_target_parameters_cloudwatch_logs_parameters.rs");
        include!("types/pipes/pipe_target_parameters_ecs_task_parameters.rs");
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_capacity_provider_strategy.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_network_configuration.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_network_configuration_aws_vpc_configuration.rs"
        );
        include!("types/pipes/pipe_target_parameters_ecs_task_parameters_overrides.rs");
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_overrides_container_override.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_overrides_container_override_environment.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_overrides_container_override_environment_file.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_overrides_container_override_resource_requirement.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_overrides_ephemeral_storage.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_overrides_inference_accelerator_override.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_placement_constraint.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_placement_strategy.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_eventbridge_event_bus_parameters.rs"
        );
        include!("types/pipes/pipe_target_parameters_http_parameters.rs");
        include!("types/pipes/pipe_target_parameters_kinesis_stream_parameters.rs");
        include!("types/pipes/pipe_target_parameters_lambda_function_parameters.rs");
        include!("types/pipes/pipe_target_parameters_redshift_data_parameters.rs");
        include!("types/pipes/pipe_target_parameters_sagemaker_pipeline_parameters.rs");
        include!(
            "types/pipes/pipe_target_parameters_sagemaker_pipeline_parameters_pipeline_parameter.rs"
        );
        include!("types/pipes/pipe_target_parameters_sqs_queue_parameters.rs");
        include!(
            "types/pipes/pipe_target_parameters_step_function_state_machine_parameters.rs"
        );
    }
    pub mod polly {
        include!("types/polly/get_voices_voice.rs");
    }
    pub mod pricing {
        include!("types/pricing/get_product_filter.rs");
    }
    pub mod qldb {
        include!("types/qldb/stream_kinesis_configuration.rs");
    }
    pub mod quicksight {
        include!("types/quicksight/analysis_parameters.rs");
        include!("types/quicksight/analysis_parameters_date_time_parameter.rs");
        include!("types/quicksight/analysis_parameters_decimal_parameter.rs");
        include!("types/quicksight/analysis_parameters_integer_parameter.rs");
        include!("types/quicksight/analysis_parameters_string_parameter.rs");
        include!("types/quicksight/analysis_permission.rs");
        include!("types/quicksight/analysis_source_entity.rs");
        include!("types/quicksight/analysis_source_entity_source_template.rs");
        include!(
            "types/quicksight/analysis_source_entity_source_template_data_set_reference.rs"
        );
        include!("types/quicksight/dashboard_dashboard_publish_options.rs");
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_ad_hoc_filtering_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_data_point_drill_up_down_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_data_point_menu_label_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_data_point_tooltip_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_export_to_csv_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_export_with_hidden_fields_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_sheet_controls_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_sheet_layout_element_maximization_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_visual_axis_sort_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_visual_menu_option.rs"
        );
        include!("types/quicksight/dashboard_parameters.rs");
        include!("types/quicksight/dashboard_parameters_date_time_parameter.rs");
        include!("types/quicksight/dashboard_parameters_decimal_parameter.rs");
        include!("types/quicksight/dashboard_parameters_integer_parameter.rs");
        include!("types/quicksight/dashboard_parameters_string_parameter.rs");
        include!("types/quicksight/dashboard_permission.rs");
        include!("types/quicksight/dashboard_source_entity.rs");
        include!("types/quicksight/dashboard_source_entity_source_template.rs");
        include!(
            "types/quicksight/dashboard_source_entity_source_template_data_set_reference.rs"
        );
        include!("types/quicksight/data_set_column_group.rs");
        include!("types/quicksight/data_set_column_group_geo_spatial_column_group.rs");
        include!("types/quicksight/data_set_column_level_permission_rule.rs");
        include!("types/quicksight/data_set_data_set_usage_configuration.rs");
        include!("types/quicksight/data_set_field_folder.rs");
        include!("types/quicksight/data_set_logical_table_map.rs");
        include!("types/quicksight/data_set_logical_table_map_data_transform.rs");
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_cast_column_type_operation.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_create_columns_operation.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_create_columns_operation_column.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_filter_operation.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_project_operation.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_rename_column_operation.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_tag_column_operation.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_tag_column_operation_tag.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_tag_column_operation_tag_column_description.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_untag_column_operation.rs"
        );
        include!("types/quicksight/data_set_logical_table_map_source.rs");
        include!(
            "types/quicksight/data_set_logical_table_map_source_join_instruction.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_source_join_instruction_left_join_key_properties.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_source_join_instruction_right_join_key_properties.rs"
        );
        include!("types/quicksight/data_set_output_column.rs");
        include!("types/quicksight/data_set_permission.rs");
        include!("types/quicksight/data_set_physical_table_map.rs");
        include!("types/quicksight/data_set_physical_table_map_custom_sql.rs");
        include!("types/quicksight/data_set_physical_table_map_custom_sql_column.rs");
        include!("types/quicksight/data_set_physical_table_map_relational_table.rs");
        include!(
            "types/quicksight/data_set_physical_table_map_relational_table_input_column.rs"
        );
        include!("types/quicksight/data_set_physical_table_map_s_3_source.rs");
        include!(
            "types/quicksight/data_set_physical_table_map_s_3_source_input_column.rs"
        );
        include!(
            "types/quicksight/data_set_physical_table_map_s_3_source_upload_settings.rs"
        );
        include!("types/quicksight/data_set_refresh_properties.rs");
        include!(
            "types/quicksight/data_set_refresh_properties_refresh_configuration.rs"
        );
        include!(
            "types/quicksight/data_set_refresh_properties_refresh_configuration_incremental_refresh.rs"
        );
        include!(
            "types/quicksight/data_set_refresh_properties_refresh_configuration_incremental_refresh_lookback_window.rs"
        );
        include!("types/quicksight/data_set_row_level_permission_data_set.rs");
        include!("types/quicksight/data_set_row_level_permission_tag_configuration.rs");
        include!(
            "types/quicksight/data_set_row_level_permission_tag_configuration_tag_rule.rs"
        );
        include!("types/quicksight/data_source_credentials.rs");
        include!("types/quicksight/data_source_credentials_credential_pair.rs");
        include!("types/quicksight/data_source_parameters.rs");
        include!("types/quicksight/data_source_parameters_amazon_elasticsearch.rs");
        include!("types/quicksight/data_source_parameters_athena.rs");
        include!("types/quicksight/data_source_parameters_aurora.rs");
        include!("types/quicksight/data_source_parameters_aurora_postgresql.rs");
        include!("types/quicksight/data_source_parameters_aws_iot_analytics.rs");
        include!("types/quicksight/data_source_parameters_databricks.rs");
        include!("types/quicksight/data_source_parameters_jira.rs");
        include!("types/quicksight/data_source_parameters_maria_db.rs");
        include!("types/quicksight/data_source_parameters_mysql.rs");
        include!("types/quicksight/data_source_parameters_oracle.rs");
        include!("types/quicksight/data_source_parameters_postgresql.rs");
        include!("types/quicksight/data_source_parameters_presto.rs");
        include!("types/quicksight/data_source_parameters_rds.rs");
        include!("types/quicksight/data_source_parameters_redshift.rs");
        include!("types/quicksight/data_source_parameters_s_3.rs");
        include!(
            "types/quicksight/data_source_parameters_s_3_manifest_file_location.rs"
        );
        include!("types/quicksight/data_source_parameters_service_now.rs");
        include!("types/quicksight/data_source_parameters_snowflake.rs");
        include!("types/quicksight/data_source_parameters_spark.rs");
        include!("types/quicksight/data_source_parameters_sql_server.rs");
        include!("types/quicksight/data_source_parameters_teradata.rs");
        include!("types/quicksight/data_source_parameters_twitter.rs");
        include!("types/quicksight/data_source_permission.rs");
        include!("types/quicksight/data_source_ssl_properties.rs");
        include!("types/quicksight/data_source_vpc_connection_properties.rs");
        include!("types/quicksight/folder_permission.rs");
        include!("types/quicksight/iam_policy_assignment_identities.rs");
        include!("types/quicksight/namespace_timeouts.rs");
        include!("types/quicksight/refresh_schedule_schedule.rs");
        include!("types/quicksight/refresh_schedule_schedule_schedule_frequency.rs");
        include!(
            "types/quicksight/refresh_schedule_schedule_schedule_frequency_refresh_on_day.rs"
        );
        include!("types/quicksight/template_permission.rs");
        include!("types/quicksight/template_source_entity.rs");
        include!("types/quicksight/template_source_entity_source_analysis.rs");
        include!(
            "types/quicksight/template_source_entity_source_analysis_data_set_reference.rs"
        );
        include!("types/quicksight/template_source_entity_source_template.rs");
        include!("types/quicksight/theme_configuration.rs");
        include!("types/quicksight/theme_configuration_data_color_palette.rs");
        include!("types/quicksight/theme_configuration_sheet.rs");
        include!("types/quicksight/theme_configuration_sheet_tile.rs");
        include!("types/quicksight/theme_configuration_sheet_tile_border.rs");
        include!("types/quicksight/theme_configuration_sheet_tile_layout.rs");
        include!("types/quicksight/theme_configuration_sheet_tile_layout_gutter.rs");
        include!("types/quicksight/theme_configuration_sheet_tile_layout_margin.rs");
        include!("types/quicksight/theme_configuration_typography.rs");
        include!("types/quicksight/theme_configuration_typography_font_family.rs");
        include!("types/quicksight/theme_configuration_ui_color_palette.rs");
        include!("types/quicksight/theme_permission.rs");
        include!("types/quicksight/vpc_connection_timeouts.rs");
        include!("types/quicksight/get_analysis_permission.rs");
        include!("types/quicksight/get_data_set_column_group.rs");
        include!(
            "types/quicksight/get_data_set_column_group_geo_spatial_column_group.rs"
        );
        include!("types/quicksight/get_data_set_column_level_permission_rule.rs");
        include!("types/quicksight/get_data_set_data_set_usage_configuration.rs");
        include!("types/quicksight/get_data_set_field_folder.rs");
        include!("types/quicksight/get_data_set_logical_table_map.rs");
        include!("types/quicksight/get_data_set_logical_table_map_data_transform.rs");
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_cast_column_type_operation.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_create_columns_operation.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_create_columns_operation_column.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_filter_operation.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_project_operation.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_rename_column_operation.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_tag_column_operation.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_tag_column_operation_tag.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_tag_column_operation_tag_column_description.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_untag_column_operation.rs"
        );
        include!("types/quicksight/get_data_set_logical_table_map_source.rs");
        include!(
            "types/quicksight/get_data_set_logical_table_map_source_join_instruction.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_source_join_instruction_left_join_key_property.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_source_join_instruction_right_join_key_property.rs"
        );
        include!("types/quicksight/get_data_set_permission.rs");
        include!("types/quicksight/get_data_set_physical_table_map.rs");
        include!("types/quicksight/get_data_set_physical_table_map_custom_sql.rs");
        include!(
            "types/quicksight/get_data_set_physical_table_map_custom_sql_column.rs"
        );
        include!("types/quicksight/get_data_set_physical_table_map_relational_table.rs");
        include!(
            "types/quicksight/get_data_set_physical_table_map_relational_table_input_column.rs"
        );
        include!("types/quicksight/get_data_set_physical_table_map_s_3_source.rs");
        include!(
            "types/quicksight/get_data_set_physical_table_map_s_3_source_input_column.rs"
        );
        include!(
            "types/quicksight/get_data_set_physical_table_map_s_3_source_upload_setting.rs"
        );
        include!("types/quicksight/get_data_set_row_level_permission_data_set.rs");
        include!(
            "types/quicksight/get_data_set_row_level_permission_tag_configuration.rs"
        );
        include!(
            "types/quicksight/get_data_set_row_level_permission_tag_configuration_tag_rule.rs"
        );
        include!("types/quicksight/get_quicksight_analysis_permission.rs");
        include!("types/quicksight/get_theme_configuration.rs");
        include!("types/quicksight/get_theme_configuration_data_color_palette.rs");
        include!("types/quicksight/get_theme_configuration_sheet.rs");
        include!("types/quicksight/get_theme_configuration_sheet_tile.rs");
        include!("types/quicksight/get_theme_configuration_sheet_tile_border.rs");
        include!("types/quicksight/get_theme_configuration_sheet_tile_layout.rs");
        include!("types/quicksight/get_theme_configuration_sheet_tile_layout_gutter.rs");
        include!("types/quicksight/get_theme_configuration_sheet_tile_layout_margin.rs");
        include!("types/quicksight/get_theme_configuration_typography.rs");
        include!("types/quicksight/get_theme_configuration_typography_font_family.rs");
        include!("types/quicksight/get_theme_configuration_ui_color_palette.rs");
        include!("types/quicksight/get_theme_permission.rs");
    }
    pub mod ram {
        include!("types/ram/get_resource_share_filter.rs");
    }
    pub mod rbin {
        include!("types/rbin/rule_lock_configuration.rs");
        include!("types/rbin/rule_lock_configuration_unlock_delay.rs");
        include!("types/rbin/rule_resource_tag.rs");
        include!("types/rbin/rule_retention_period.rs");
    }
    pub mod rds {
        include!("types/rds/cluster_master_user_secret.rs");
        include!("types/rds/cluster_parameter_group_parameter.rs");
        include!("types/rds/cluster_restore_to_point_in_time.rs");
        include!("types/rds/cluster_s_3_import.rs");
        include!("types/rds/cluster_scaling_configuration.rs");
        include!("types/rds/cluster_serverlessv_2_scaling_configuration.rs");
        include!("types/rds/cluster_snapshot_copy_timeouts.rs");
        include!("types/rds/export_task_timeouts.rs");
        include!("types/rds/global_cluster_global_cluster_member.rs");
        include!("types/rds/instance_blue_green_update.rs");
        include!("types/rds/instance_desired_state_timeouts.rs");
        include!("types/rds/instance_listener_endpoint.rs");
        include!("types/rds/instance_master_user_secret.rs");
        include!("types/rds/instance_restore_to_point_in_time.rs");
        include!("types/rds/instance_s_3_import.rs");
        include!("types/rds/integration_timeouts.rs");
        include!("types/rds/option_group_option.rs");
        include!("types/rds/option_group_option_option_setting.rs");
        include!("types/rds/parameter_group_parameter.rs");
        include!("types/rds/proxy_auth.rs");
        include!("types/rds/proxy_default_target_group_connection_pool_config.rs");
        include!("types/rds/reserved_instance_recurring_charge.rs");
        include!("types/rds/get_cluster_master_user_secret.rs");
        include!("types/rds/get_clusters_filter.rs");
        include!("types/rds/get_engine_version_filter.rs");
        include!("types/rds/get_instance_master_user_secret.rs");
        include!("types/rds/get_instances_filter.rs");
        include!("types/rds/get_proxy_auth.rs");
    }
    pub mod redshift {
        include!("types/redshift/cluster_cluster_node.rs");
        include!("types/redshift/cluster_logging.rs");
        include!("types/redshift/cluster_snapshot_copy.rs");
        include!("types/redshift/endpoint_access_vpc_endpoint.rs");
        include!("types/redshift/endpoint_access_vpc_endpoint_network_interface.rs");
        include!("types/redshift/parameter_group_parameter.rs");
        include!("types/redshift/scheduled_action_target_action.rs");
        include!("types/redshift/scheduled_action_target_action_pause_cluster.rs");
        include!("types/redshift/scheduled_action_target_action_resize_cluster.rs");
        include!("types/redshift/scheduled_action_target_action_resume_cluster.rs");
        include!("types/redshift/get_cluster_cluster_node.rs");
        include!("types/redshift/get_data_shares_data_share.rs");
        include!("types/redshift/get_producer_data_shares_data_share.rs");
    }
    pub mod redshiftdata {
        include!("types/redshiftdata/statement_parameter.rs");
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
