pub mod fms {
    include!("resources/fms/admin_account.rs");
    include!("resources/fms/policy.rs");
    include!("resources/fms/resource_set.rs");
}
pub mod fsx {
    include!("resources/fsx/backup.rs");
    include!("resources/fsx/data_repository_association.rs");
    include!("resources/fsx/file_cache.rs");
    include!("resources/fsx/lustre_file_system.rs");
    include!("resources/fsx/ontap_file_system.rs");
    include!("resources/fsx/ontap_storage_virtual_machine.rs");
    include!("resources/fsx/ontap_volume.rs");
    include!("resources/fsx/open_zfs_file_system.rs");
    include!("resources/fsx/open_zfs_snapshot.rs");
    include!("resources/fsx/open_zfs_volume.rs");
    include!("resources/fsx/windows_file_system.rs");
}
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
pub mod functions {
    pub mod fsx {
        include!("functions/fsx/get_ontap_file_system.rs");
        include!("functions/fsx/get_ontap_storage_virtual_machine.rs");
        include!("functions/fsx/get_ontap_storage_virtual_machines.rs");
        include!("functions/fsx/get_open_zfs_snapshot.rs");
        include!("functions/fsx/get_windows_file_system.rs");
    }
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
    pub mod fms {
        include!("types/fms/policy_exclude_map.rs");
        include!("types/fms/policy_include_map.rs");
        include!("types/fms/policy_security_service_policy_data.rs");
        include!("types/fms/policy_security_service_policy_data_policy_option.rs");
        include!(
            "types/fms/policy_security_service_policy_data_policy_option_network_firewall_policy.rs"
        );
        include!(
            "types/fms/policy_security_service_policy_data_policy_option_third_party_firewall_policy.rs"
        );
        include!("types/fms/resource_set_resource_set.rs");
        include!("types/fms/resource_set_timeouts.rs");
    }
    pub mod fsx {
        include!("types/fsx/data_repository_association_s_3.rs");
        include!("types/fsx/data_repository_association_s_3_auto_export_policy.rs");
        include!("types/fsx/data_repository_association_s_3_auto_import_policy.rs");
        include!("types/fsx/file_cache_data_repository_association.rs");
        include!("types/fsx/file_cache_data_repository_association_nf.rs");
        include!("types/fsx/file_cache_lustre_configuration.rs");
        include!("types/fsx/file_cache_lustre_configuration_log_configuration.rs");
        include!("types/fsx/file_cache_lustre_configuration_metadata_configuration.rs");
        include!("types/fsx/lustre_file_system_log_configuration.rs");
        include!("types/fsx/lustre_file_system_metadata_configuration.rs");
        include!("types/fsx/lustre_file_system_root_squash_configuration.rs");
        include!("types/fsx/ontap_file_system_disk_iops_configuration.rs");
        include!("types/fsx/ontap_file_system_endpoint.rs");
        include!("types/fsx/ontap_file_system_endpoint_intercluster.rs");
        include!("types/fsx/ontap_file_system_endpoint_management.rs");
        include!(
            "types/fsx/ontap_storage_virtual_machine_active_directory_configuration.rs"
        );
        include!(
            "types/fsx/ontap_storage_virtual_machine_active_directory_configuration_self_managed_active_directory_configuration.rs"
        );
        include!("types/fsx/ontap_storage_virtual_machine_endpoint.rs");
        include!("types/fsx/ontap_storage_virtual_machine_endpoint_iscsi.rs");
        include!("types/fsx/ontap_storage_virtual_machine_endpoint_management.rs");
        include!("types/fsx/ontap_storage_virtual_machine_endpoint_nf.rs");
        include!("types/fsx/ontap_storage_virtual_machine_endpoint_smb.rs");
        include!("types/fsx/ontap_volume_aggregate_configuration.rs");
        include!("types/fsx/ontap_volume_snaplock_configuration.rs");
        include!("types/fsx/ontap_volume_snaplock_configuration_autocommit_period.rs");
        include!("types/fsx/ontap_volume_snaplock_configuration_retention_period.rs");
        include!(
            "types/fsx/ontap_volume_snaplock_configuration_retention_period_default_retention.rs"
        );
        include!(
            "types/fsx/ontap_volume_snaplock_configuration_retention_period_maximum_retention.rs"
        );
        include!(
            "types/fsx/ontap_volume_snaplock_configuration_retention_period_minimum_retention.rs"
        );
        include!("types/fsx/ontap_volume_tiering_policy.rs");
        include!("types/fsx/open_zfs_file_system_disk_iops_configuration.rs");
        include!("types/fsx/open_zfs_file_system_root_volume_configuration.rs");
        include!(
            "types/fsx/open_zfs_file_system_root_volume_configuration_nfs_exports.rs"
        );
        include!(
            "types/fsx/open_zfs_file_system_root_volume_configuration_nfs_exports_client_configuration.rs"
        );
        include!(
            "types/fsx/open_zfs_file_system_root_volume_configuration_user_and_group_quota.rs"
        );
        include!("types/fsx/open_zfs_volume_nfs_exports.rs");
        include!("types/fsx/open_zfs_volume_nfs_exports_client_configuration.rs");
        include!("types/fsx/open_zfs_volume_origin_snapshot.rs");
        include!("types/fsx/open_zfs_volume_user_and_group_quota.rs");
        include!("types/fsx/windows_file_system_audit_log_configuration.rs");
        include!("types/fsx/windows_file_system_disk_iops_configuration.rs");
        include!("types/fsx/windows_file_system_self_managed_active_directory.rs");
        include!("types/fsx/get_ontap_file_system_disk_iops_configuration.rs");
        include!("types/fsx/get_ontap_file_system_endpoint.rs");
        include!("types/fsx/get_ontap_file_system_endpoint_intercluster.rs");
        include!("types/fsx/get_ontap_file_system_endpoint_management.rs");
        include!(
            "types/fsx/get_ontap_storage_virtual_machine_active_directory_configuration.rs"
        );
        include!(
            "types/fsx/get_ontap_storage_virtual_machine_active_directory_configuration_self_managed_active_directory_configuration.rs"
        );
        include!("types/fsx/get_ontap_storage_virtual_machine_endpoint.rs");
        include!("types/fsx/get_ontap_storage_virtual_machine_endpoint_iscsi.rs");
        include!("types/fsx/get_ontap_storage_virtual_machine_endpoint_management.rs");
        include!("types/fsx/get_ontap_storage_virtual_machine_endpoint_nf.rs");
        include!("types/fsx/get_ontap_storage_virtual_machine_endpoint_smb.rs");
        include!("types/fsx/get_ontap_storage_virtual_machine_filter.rs");
        include!(
            "types/fsx/get_ontap_storage_virtual_machine_lifecycle_transition_reason.rs"
        );
        include!("types/fsx/get_ontap_storage_virtual_machines_filter.rs");
        include!("types/fsx/get_open_zfs_snapshot_filter.rs");
        include!("types/fsx/get_windows_file_system_audit_log_configuration.rs");
        include!("types/fsx/get_windows_file_system_disk_iops_configuration.rs");
    }
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
        constructor(value: string, secret: bool);
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
static PULUMI_WASM_PROVIDER_AWS: [u8; 45] = *b"{\"version\":\"6.66.2\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "6.66.2".to_string()
}
