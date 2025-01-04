pub mod gamelift {
    include!("resources/gamelift/alias.rs");
    include!("resources/gamelift/build.rs");
    include!("resources/gamelift/fleet.rs");
    include!("resources/gamelift/game_server_group.rs");
    include!("resources/gamelift/game_session_queue.rs");
    include!("resources/gamelift/matchmaking_configuration.rs");
    include!("resources/gamelift/matchmaking_rule_set.rs");
    include!("resources/gamelift/script.rs");
}
pub mod glacier {
    include!("resources/glacier/vault.rs");
    include!("resources/glacier/vault_lock.rs");
}
pub mod globalaccelerator {
    include!("resources/globalaccelerator/accelerator.rs");
    include!("resources/globalaccelerator/cross_account_attachment.rs");
    include!("resources/globalaccelerator/custom_routing_accelerator.rs");
    include!("resources/globalaccelerator/custom_routing_endpoint_group.rs");
    include!("resources/globalaccelerator/custom_routing_listener.rs");
    include!("resources/globalaccelerator/endpoint_group.rs");
    include!("resources/globalaccelerator/listener.rs");
}
pub mod glue {
    include!("resources/glue/catalog_database.rs");
    include!("resources/glue/catalog_table.rs");
    include!("resources/glue/catalog_table_optimizer.rs");
    include!("resources/glue/classifier.rs");
    include!("resources/glue/connection.rs");
    include!("resources/glue/crawler.rs");
    include!("resources/glue/data_catalog_encryption_settings.rs");
    include!("resources/glue/data_quality_ruleset.rs");
    include!("resources/glue/dev_endpoint.rs");
    include!("resources/glue/job.rs");
    include!("resources/glue/ml_transform.rs");
    include!("resources/glue/partition.rs");
    include!("resources/glue/partition_index.rs");
    include!("resources/glue/registry.rs");
    include!("resources/glue/resource_policy.rs");
    include!("resources/glue/schema.rs");
    include!("resources/glue/security_configuration.rs");
    include!("resources/glue/trigger.rs");
    include!("resources/glue/user_defined_function.rs");
    include!("resources/glue/workflow.rs");
}
pub mod grafana {
    include!("resources/grafana/license_association.rs");
    include!("resources/grafana/role_association.rs");
    include!("resources/grafana/workspace.rs");
    include!("resources/grafana/workspace_api_key.rs");
    include!("resources/grafana/workspace_saml_configuration.rs");
    include!("resources/grafana/workspace_service_account.rs");
    include!("resources/grafana/workspace_service_account_token.rs");
}
pub mod guardduty {
    include!("resources/guardduty/detector.rs");
    include!("resources/guardduty/detector_feature.rs");
    include!("resources/guardduty/filter.rs");
    include!("resources/guardduty/ip_set.rs");
    include!("resources/guardduty/invite_accepter.rs");
    include!("resources/guardduty/malware_protection_plan.rs");
    include!("resources/guardduty/member.rs");
    include!("resources/guardduty/organization_admin_account.rs");
    include!("resources/guardduty/organization_configuration.rs");
    include!("resources/guardduty/organization_configuration_feature.rs");
    include!("resources/guardduty/publishing_destination.rs");
    include!("resources/guardduty/threat_intel_set.rs");
}
pub mod iam {
    include!("resources/iam/access_key.rs");
    include!("resources/iam/account_alias.rs");
    include!("resources/iam/account_password_policy.rs");
    include!("resources/iam/group.rs");
    include!("resources/iam/group_membership.rs");
    include!("resources/iam/group_policies_exclusive.rs");
    include!("resources/iam/group_policy.rs");
    include!("resources/iam/group_policy_attachment.rs");
    include!("resources/iam/group_policy_attachments_exclusive.rs");
    include!("resources/iam/instance_profile.rs");
    include!("resources/iam/open_id_connect_provider.rs");
    include!("resources/iam/organizations_features.rs");
    include!("resources/iam/policy.rs");
    include!("resources/iam/policy_attachment.rs");
    include!("resources/iam/role.rs");
    include!("resources/iam/role_policies_exclusive.rs");
    include!("resources/iam/role_policy.rs");
    include!("resources/iam/role_policy_attachment.rs");
    include!("resources/iam/role_policy_attachments_exclusive.rs");
    include!("resources/iam/saml_provider.rs");
    include!("resources/iam/security_token_service_preferences.rs");
    include!("resources/iam/server_certificate.rs");
    include!("resources/iam/service_linked_role.rs");
    include!("resources/iam/service_specific_credential.rs");
    include!("resources/iam/signing_certificate.rs");
    include!("resources/iam/ssh_key.rs");
    include!("resources/iam/user.rs");
    include!("resources/iam/user_group_membership.rs");
    include!("resources/iam/user_login_profile.rs");
    include!("resources/iam/user_policies_exclusive.rs");
    include!("resources/iam/user_policy.rs");
    include!("resources/iam/user_policy_attachment.rs");
    include!("resources/iam/user_policy_attachments_exclusive.rs");
    include!("resources/iam/virtual_mfa_device.rs");
}
pub mod identitystore {
    include!("resources/identitystore/group.rs");
    include!("resources/identitystore/group_membership.rs");
    include!("resources/identitystore/user.rs");
}
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
pub mod functions {
    pub mod globalaccelerator {
        include!("functions/globalaccelerator/get_accelerator.rs");
        include!("functions/globalaccelerator/get_custom_routing_accelerator.rs");
    }
    pub mod glue {
        include!("functions/glue/get_catalog_table.rs");
        include!("functions/glue/get_connection.rs");
        include!("functions/glue/get_data_catalog_encryption_settings.rs");
        include!("functions/glue/get_registry.rs");
        include!("functions/glue/get_script.rs");
    }
    pub mod grafana {
        include!("functions/grafana/get_workspace.rs");
    }
    pub mod guardduty {
        include!("functions/guardduty/get_detector.rs");
        include!("functions/guardduty/get_finding_ids.rs");
    }
    pub mod iam {
        include!("functions/iam/get_access_keys.rs");
        include!("functions/iam/get_account_alias.rs");
        include!("functions/iam/get_group.rs");
        include!("functions/iam/get_instance_profile.rs");
        include!("functions/iam/get_instance_profiles.rs");
        include!("functions/iam/get_open_id_connect_provider.rs");
        include!("functions/iam/get_policy.rs");
        include!("functions/iam/get_policy_document.rs");
        include!("functions/iam/get_principal_policy_simulation.rs");
        include!("functions/iam/get_role.rs");
        include!("functions/iam/get_roles.rs");
        include!("functions/iam/get_saml_provider.rs");
        include!("functions/iam/get_server_certificate.rs");
        include!("functions/iam/get_session_context.rs");
        include!("functions/iam/get_user.rs");
        include!("functions/iam/get_user_ssh_key.rs");
        include!("functions/iam/get_users.rs");
    }
    pub mod identitystore {
        include!("functions/identitystore/get_group.rs");
        include!("functions/identitystore/get_groups.rs");
        include!("functions/identitystore/get_user.rs");
    }
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
    pub mod gamelift {
        include!("types/gamelift/alias_routing_strategy.rs");
        include!("types/gamelift/build_storage_location.rs");
        include!("types/gamelift/fleet_certificate_configuration.rs");
        include!("types/gamelift/fleet_ec_2_inbound_permission.rs");
        include!("types/gamelift/fleet_resource_creation_limit_policy.rs");
        include!("types/gamelift/fleet_runtime_configuration.rs");
        include!("types/gamelift/fleet_runtime_configuration_server_process.rs");
        include!("types/gamelift/game_server_group_auto_scaling_policy.rs");
        include!(
            "types/gamelift/game_server_group_auto_scaling_policy_target_tracking_configuration.rs"
        );
        include!("types/gamelift/game_server_group_instance_definition.rs");
        include!("types/gamelift/game_server_group_launch_template.rs");
        include!("types/gamelift/game_session_queue_player_latency_policy.rs");
        include!("types/gamelift/matchmaking_configuration_game_property.rs");
        include!("types/gamelift/script_storage_location.rs");
    }
    pub mod glacier {
        include!("types/glacier/vault_notification.rs");
    }
    pub mod globalaccelerator {
        include!("types/globalaccelerator/accelerator_attributes.rs");
        include!("types/globalaccelerator/accelerator_ip_set.rs");
        include!("types/globalaccelerator/cross_account_attachment_resource.rs");
        include!("types/globalaccelerator/custom_routing_accelerator_attributes.rs");
        include!("types/globalaccelerator/custom_routing_accelerator_ip_set.rs");
        include!(
            "types/globalaccelerator/custom_routing_endpoint_group_destination_configuration.rs"
        );
        include!(
            "types/globalaccelerator/custom_routing_endpoint_group_endpoint_configuration.rs"
        );
        include!("types/globalaccelerator/custom_routing_listener_port_range.rs");
        include!("types/globalaccelerator/endpoint_group_endpoint_configuration.rs");
        include!("types/globalaccelerator/endpoint_group_port_override.rs");
        include!("types/globalaccelerator/listener_port_range.rs");
        include!("types/globalaccelerator/get_accelerator_attribute.rs");
        include!("types/globalaccelerator/get_accelerator_ip_set.rs");
        include!("types/globalaccelerator/get_custom_routing_accelerator_attribute.rs");
        include!("types/globalaccelerator/get_custom_routing_accelerator_ip_set.rs");
    }
    pub mod glue {
        include!("types/glue/catalog_database_create_table_default_permission.rs");
        include!(
            "types/glue/catalog_database_create_table_default_permission_principal.rs"
        );
        include!("types/glue/catalog_database_federated_database.rs");
        include!("types/glue/catalog_database_target_database.rs");
        include!("types/glue/catalog_table_open_table_format_input.rs");
        include!("types/glue/catalog_table_open_table_format_input_iceberg_input.rs");
        include!("types/glue/catalog_table_optimizer_configuration.rs");
        include!(
            "types/glue/catalog_table_optimizer_configuration_orphan_file_deletion_configuration.rs"
        );
        include!(
            "types/glue/catalog_table_optimizer_configuration_orphan_file_deletion_configuration_iceberg_configuration.rs"
        );
        include!(
            "types/glue/catalog_table_optimizer_configuration_retention_configuration.rs"
        );
        include!(
            "types/glue/catalog_table_optimizer_configuration_retention_configuration_iceberg_configuration.rs"
        );
        include!("types/glue/catalog_table_partition_index.rs");
        include!("types/glue/catalog_table_partition_key.rs");
        include!("types/glue/catalog_table_storage_descriptor.rs");
        include!("types/glue/catalog_table_storage_descriptor_column.rs");
        include!("types/glue/catalog_table_storage_descriptor_schema_reference.rs");
        include!(
            "types/glue/catalog_table_storage_descriptor_schema_reference_schema_id.rs"
        );
        include!("types/glue/catalog_table_storage_descriptor_ser_de_info.rs");
        include!("types/glue/catalog_table_storage_descriptor_skewed_info.rs");
        include!("types/glue/catalog_table_storage_descriptor_sort_column.rs");
        include!("types/glue/catalog_table_target_table.rs");
        include!("types/glue/classifier_csv_classifier.rs");
        include!("types/glue/classifier_grok_classifier.rs");
        include!("types/glue/classifier_json_classifier.rs");
        include!("types/glue/classifier_xml_classifier.rs");
        include!("types/glue/connection_physical_connection_requirements.rs");
        include!("types/glue/crawler_catalog_target.rs");
        include!("types/glue/crawler_delta_target.rs");
        include!("types/glue/crawler_dynamodb_target.rs");
        include!("types/glue/crawler_hudi_target.rs");
        include!("types/glue/crawler_iceberg_target.rs");
        include!("types/glue/crawler_jdbc_target.rs");
        include!("types/glue/crawler_lake_formation_configuration.rs");
        include!("types/glue/crawler_lineage_configuration.rs");
        include!("types/glue/crawler_mongodb_target.rs");
        include!("types/glue/crawler_recrawl_policy.rs");
        include!("types/glue/crawler_s_3_target.rs");
        include!("types/glue/crawler_schema_change_policy.rs");
        include!(
            "types/glue/data_catalog_encryption_settings_data_catalog_encryption_settings.rs"
        );
        include!(
            "types/glue/data_catalog_encryption_settings_data_catalog_encryption_settings_connection_password_encryption.rs"
        );
        include!(
            "types/glue/data_catalog_encryption_settings_data_catalog_encryption_settings_encryption_at_rest.rs"
        );
        include!("types/glue/data_quality_ruleset_target_table.rs");
        include!("types/glue/job_command.rs");
        include!("types/glue/job_execution_property.rs");
        include!("types/glue/job_notification_property.rs");
        include!("types/glue/ml_transform_input_record_table.rs");
        include!("types/glue/ml_transform_parameters.rs");
        include!("types/glue/ml_transform_parameters_find_matches_parameters.rs");
        include!("types/glue/ml_transform_schema.rs");
        include!("types/glue/partition_index_partition_index.rs");
        include!("types/glue/partition_storage_descriptor.rs");
        include!("types/glue/partition_storage_descriptor_column.rs");
        include!("types/glue/partition_storage_descriptor_ser_de_info.rs");
        include!("types/glue/partition_storage_descriptor_skewed_info.rs");
        include!("types/glue/partition_storage_descriptor_sort_column.rs");
        include!("types/glue/security_configuration_encryption_configuration.rs");
        include!(
            "types/glue/security_configuration_encryption_configuration_cloudwatch_encryption.rs"
        );
        include!(
            "types/glue/security_configuration_encryption_configuration_job_bookmarks_encryption.rs"
        );
        include!(
            "types/glue/security_configuration_encryption_configuration_s_3_encryption.rs"
        );
        include!("types/glue/trigger_action.rs");
        include!("types/glue/trigger_action_notification_property.rs");
        include!("types/glue/trigger_event_batching_condition.rs");
        include!("types/glue/trigger_predicate.rs");
        include!("types/glue/trigger_predicate_condition.rs");
        include!("types/glue/user_defined_function_resource_uri.rs");
        include!("types/glue/get_catalog_table_partition_index.rs");
        include!("types/glue/get_catalog_table_partition_key.rs");
        include!("types/glue/get_catalog_table_storage_descriptor.rs");
        include!("types/glue/get_catalog_table_storage_descriptor_column.rs");
        include!("types/glue/get_catalog_table_storage_descriptor_schema_reference.rs");
        include!(
            "types/glue/get_catalog_table_storage_descriptor_schema_reference_schema_id.rs"
        );
        include!("types/glue/get_catalog_table_storage_descriptor_ser_de_info.rs");
        include!("types/glue/get_catalog_table_storage_descriptor_skewed_info.rs");
        include!("types/glue/get_catalog_table_storage_descriptor_sort_column.rs");
        include!("types/glue/get_catalog_table_target_table.rs");
        include!("types/glue/get_connection_physical_connection_requirement.rs");
        include!(
            "types/glue/get_data_catalog_encryption_settings_data_catalog_encryption_setting.rs"
        );
        include!(
            "types/glue/get_data_catalog_encryption_settings_data_catalog_encryption_setting_connection_password_encryption.rs"
        );
        include!(
            "types/glue/get_data_catalog_encryption_settings_data_catalog_encryption_setting_encryption_at_rest.rs"
        );
        include!("types/glue/get_script_dag_edge.rs");
        include!("types/glue/get_script_dag_node.rs");
        include!("types/glue/get_script_dag_node_arg.rs");
    }
    pub mod grafana {
        include!("types/grafana/workspace_network_access_control.rs");
        include!("types/grafana/workspace_vpc_configuration.rs");
    }
    pub mod guardduty {
        include!("types/guardduty/detector_datasources.rs");
        include!("types/guardduty/detector_datasources_kubernetes.rs");
        include!("types/guardduty/detector_datasources_kubernetes_audit_logs.rs");
        include!("types/guardduty/detector_datasources_malware_protection.rs");
        include!(
            "types/guardduty/detector_datasources_malware_protection_scan_ec_2_instance_with_findings.rs"
        );
        include!(
            "types/guardduty/detector_datasources_malware_protection_scan_ec_2_instance_with_findings_ebs_volumes.rs"
        );
        include!("types/guardduty/detector_datasources_s_3_logs.rs");
        include!("types/guardduty/detector_feature_additional_configuration.rs");
        include!("types/guardduty/filter_finding_criteria.rs");
        include!("types/guardduty/filter_finding_criteria_criterion.rs");
        include!("types/guardduty/malware_protection_plan_action.rs");
        include!("types/guardduty/malware_protection_plan_action_tagging.rs");
        include!("types/guardduty/malware_protection_plan_protected_resource.rs");
        include!(
            "types/guardduty/malware_protection_plan_protected_resource_s_3_bucket.rs"
        );
        include!("types/guardduty/organization_configuration_datasources.rs");
        include!("types/guardduty/organization_configuration_datasources_kubernetes.rs");
        include!(
            "types/guardduty/organization_configuration_datasources_kubernetes_audit_logs.rs"
        );
        include!(
            "types/guardduty/organization_configuration_datasources_malware_protection.rs"
        );
        include!(
            "types/guardduty/organization_configuration_datasources_malware_protection_scan_ec_2_instance_with_findings.rs"
        );
        include!(
            "types/guardduty/organization_configuration_datasources_malware_protection_scan_ec_2_instance_with_findings_ebs_volumes.rs"
        );
        include!("types/guardduty/organization_configuration_datasources_s_3_logs.rs");
        include!(
            "types/guardduty/organization_configuration_feature_additional_configuration.rs"
        );
        include!("types/guardduty/get_detector_feature.rs");
        include!("types/guardduty/get_detector_feature_additional_configuration.rs");
    }
    pub mod iam {
        include!("types/iam/role_inline_policy.rs");
        include!("types/iam/get_access_keys_access_key.rs");
        include!("types/iam/get_group_user.rs");
        include!("types/iam/get_policy_document_statement.rs");
        include!("types/iam/get_policy_document_statement_condition.rs");
        include!("types/iam/get_policy_document_statement_not_principal.rs");
        include!("types/iam/get_policy_document_statement_principal.rs");
        include!("types/iam/get_principal_policy_simulation_context.rs");
        include!("types/iam/get_principal_policy_simulation_result.rs");
        include!(
            "types/iam/get_principal_policy_simulation_result_matched_statement.rs"
        );
        include!("types/iam/get_role_role_last_used.rs");
    }
    pub mod identitystore {
        include!("types/identitystore/group_external_id.rs");
        include!("types/identitystore/user_addresses.rs");
        include!("types/identitystore/user_emails.rs");
        include!("types/identitystore/user_external_id.rs");
        include!("types/identitystore/user_name.rs");
        include!("types/identitystore/user_phone_numbers.rs");
        include!("types/identitystore/get_group_alternate_identifier.rs");
        include!("types/identitystore/get_group_alternate_identifier_external_id.rs");
        include!(
            "types/identitystore/get_group_alternate_identifier_unique_attribute.rs"
        );
        include!("types/identitystore/get_group_external_id.rs");
        include!("types/identitystore/get_group_filter.rs");
        include!("types/identitystore/get_groups_group.rs");
        include!("types/identitystore/get_groups_group_external_id.rs");
        include!("types/identitystore/get_user_address.rs");
        include!("types/identitystore/get_user_alternate_identifier.rs");
        include!("types/identitystore/get_user_alternate_identifier_external_id.rs");
        include!(
            "types/identitystore/get_user_alternate_identifier_unique_attribute.rs"
        );
        include!("types/identitystore/get_user_email.rs");
        include!("types/identitystore/get_user_external_id.rs");
        include!("types/identitystore/get_user_filter.rs");
        include!("types/identitystore/get_user_name.rs");
        include!("types/identitystore/get_user_phone_number.rs");
    }
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
