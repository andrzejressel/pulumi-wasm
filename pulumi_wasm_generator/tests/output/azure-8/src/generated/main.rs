pub mod management {
    include!("resources/management/group.rs");
    include!("resources/management/group_policy_assignment.rs");
    include!("resources/management/group_policy_exemption.rs");
    include!("resources/management/group_policy_remediation.rs");
    include!("resources/management/group_subscription_association.rs");
    include!("resources/management/group_template_deployment.rs");
    include!("resources/management/lock.rs");
    include!("resources/management/private_link.rs");
    include!("resources/management/private_link_association.rs");
}
pub mod managementgroups {
    include!("resources/managementgroups/management_group.rs");
}
pub mod managementresource {
    include!("resources/managementresource/manangement_lock.rs");
}
pub mod maps {
    include!("resources/maps/account.rs");
    include!("resources/maps/creator.rs");
}
pub mod marketplace {
    include!("resources/marketplace/agreement.rs");
    include!("resources/marketplace/role_assignment.rs");
}
pub mod mixedreality {
    include!("resources/mixedreality/spatial_anchors_account.rs");
}
pub mod mobile {
    include!("resources/mobile/network.rs");
    include!("resources/mobile/network_attached_data_network.rs");
    include!("resources/mobile/network_data_network.rs");
    include!("resources/mobile/network_packet_core_control_plane.rs");
    include!("resources/mobile/network_packet_core_data_plane.rs");
    include!("resources/mobile/network_service.rs");
    include!("resources/mobile/network_sim.rs");
    include!("resources/mobile/network_sim_group.rs");
    include!("resources/mobile/network_sim_policy.rs");
    include!("resources/mobile/network_site.rs");
    include!("resources/mobile/network_slice.rs");
}
pub mod monitoring {
    include!("resources/monitoring/aad_diagnostic_setting.rs");
    include!("resources/monitoring/action_group.rs");
    include!("resources/monitoring/activity_log_alert.rs");
    include!("resources/monitoring/alert_processing_rule_action_group.rs");
    include!("resources/monitoring/alert_processing_rule_suppression.rs");
    include!("resources/monitoring/alert_prometheus_rule_group.rs");
    include!("resources/monitoring/autoscale_setting.rs");
    include!("resources/monitoring/data_collection_endpoint.rs");
    include!("resources/monitoring/data_collection_rule.rs");
    include!("resources/monitoring/data_collection_rule_association.rs");
    include!("resources/monitoring/diagnostic_setting.rs");
    include!("resources/monitoring/metric_alert.rs");
    include!("resources/monitoring/private_link_scope.rs");
    include!("resources/monitoring/private_link_scoped_service.rs");
    include!("resources/monitoring/scheduled_query_rules_alert.rs");
    include!("resources/monitoring/scheduled_query_rules_alert_v_2.rs");
    include!("resources/monitoring/scheduled_query_rules_log.rs");
    include!("resources/monitoring/smart_detector_alert_rule.rs");
    include!("resources/monitoring/workspace.rs");
}
pub mod msi {
    include!("resources/msi/user_assigned_identity.rs");
}
pub mod mssql {
    include!("resources/mssql/database.rs");
    include!("resources/mssql/database_extended_auditing_policy.rs");
    include!("resources/mssql/database_vulnerability_assessment_rule_baseline.rs");
    include!("resources/mssql/elastic_pool.rs");
    include!("resources/mssql/failover_group.rs");
    include!("resources/mssql/firewall_rule.rs");
    include!("resources/mssql/job_agent.rs");
    include!("resources/mssql/job_credential.rs");
    include!("resources/mssql/managed_database.rs");
    include!("resources/mssql/managed_instance.rs");
    include!("resources/mssql/managed_instance_active_directory_administrator.rs");
    include!("resources/mssql/managed_instance_failover_group.rs");
    include!("resources/mssql/managed_instance_security_alert_policy.rs");
    include!("resources/mssql/managed_instance_transparent_data_encryption.rs");
    include!("resources/mssql/managed_instance_vulnerability_assessment.rs");
    include!("resources/mssql/outbound_firewall_rule.rs");
    include!("resources/mssql/server.rs");
    include!("resources/mssql/server_dns_alias.rs");
    include!("resources/mssql/server_extended_auditing_policy.rs");
    include!("resources/mssql/server_microsoft_support_auditing_policy.rs");
    include!("resources/mssql/server_security_alert_policy.rs");
    include!("resources/mssql/server_transparent_data_encryption.rs");
    include!("resources/mssql/server_vulnerability_assessment.rs");
    include!("resources/mssql/virtual_machine.rs");
    include!("resources/mssql/virtual_machine_availability_group_listener.rs");
    include!("resources/mssql/virtual_machine_group.rs");
    include!("resources/mssql/virtual_network_rule.rs");
}
pub mod functions {
    pub mod management {
        include!("functions/management/get_group.rs");
        include!("functions/management/get_group_template_deployment.rs");
    }
    pub mod managementgroups {
        include!("functions/managementgroups/get_management_group.rs");
    }
    pub mod maps {
        include!("functions/maps/get_account.rs");
    }
    pub mod marketplace {
        include!("functions/marketplace/get_agreement.rs");
    }
    pub mod mixedreality {
        include!("functions/mixedreality/get_spatial_anchors_account.rs");
    }
    pub mod mobile {
        include!("functions/mobile/get_network.rs");
        include!("functions/mobile/get_network_attached_data_network.rs");
        include!("functions/mobile/get_network_data_network.rs");
        include!("functions/mobile/get_network_packet_core_control_plane.rs");
        include!("functions/mobile/get_network_packet_core_data_plane.rs");
        include!("functions/mobile/get_network_service.rs");
        include!("functions/mobile/get_network_sim.rs");
        include!("functions/mobile/get_network_sim_group.rs");
        include!("functions/mobile/get_network_sim_policy.rs");
        include!("functions/mobile/get_network_site.rs");
        include!("functions/mobile/get_network_slice.rs");
    }
    pub mod monitoring {
        include!("functions/monitoring/get_action_group.rs");
        include!("functions/monitoring/get_data_collection_endpoint.rs");
        include!("functions/monitoring/get_data_collection_rule.rs");
        include!("functions/monitoring/get_diagnostic_categories.rs");
        include!("functions/monitoring/get_scheduled_query_rules_alert.rs");
        include!("functions/monitoring/get_scheduled_query_rules_log.rs");
        include!("functions/monitoring/get_workspace.rs");
    }
    pub mod mssql {
        include!("functions/mssql/get_database.rs");
        include!("functions/mssql/get_elastic_pool.rs");
        include!("functions/mssql/get_managed_database.rs");
        include!("functions/mssql/get_managed_instance.rs");
        include!("functions/mssql/get_server.rs");
    }
}
pub mod types {
    pub mod management {
        include!("types/management/group_policy_assignment_identity.rs");
        include!("types/management/group_policy_assignment_non_compliance_message.rs");
        include!("types/management/group_policy_assignment_override.rs");
        include!("types/management/group_policy_assignment_override_selector.rs");
        include!("types/management/group_policy_assignment_resource_selector.rs");
        include!(
            "types/management/group_policy_assignment_resource_selector_selector.rs"
        );
    }
    pub mod maps {
        include!("types/maps/account_cors.rs");
        include!("types/maps/account_data_store.rs");
        include!("types/maps/account_identity.rs");
    }
    pub mod mobile {
        include!(
            "types/mobile/network_attached_data_network_network_address_port_translation.rs"
        );
        include!(
            "types/mobile/network_attached_data_network_network_address_port_translation_port_range.rs"
        );
        include!("types/mobile/network_packet_core_control_plane_identity.rs");
        include!(
            "types/mobile/network_packet_core_control_plane_local_diagnostics_access.rs"
        );
        include!("types/mobile/network_packet_core_control_plane_platform.rs");
        include!("types/mobile/network_service_pcc_rule.rs");
        include!("types/mobile/network_service_pcc_rule_qos_policy.rs");
        include!(
            "types/mobile/network_service_pcc_rule_qos_policy_guaranteed_bit_rate.rs"
        );
        include!("types/mobile/network_service_pcc_rule_qos_policy_maximum_bit_rate.rs");
        include!("types/mobile/network_service_pcc_rule_service_data_flow_template.rs");
        include!("types/mobile/network_service_service_qos_policy.rs");
        include!("types/mobile/network_service_service_qos_policy_maximum_bit_rate.rs");
        include!("types/mobile/network_sim_group_identity.rs");
        include!("types/mobile/network_sim_policy_slice.rs");
        include!("types/mobile/network_sim_policy_slice_data_network.rs");
        include!(
            "types/mobile/network_sim_policy_slice_data_network_session_aggregate_maximum_bit_rate.rs"
        );
        include!(
            "types/mobile/network_sim_policy_user_equipment_aggregate_maximum_bit_rate.rs"
        );
        include!("types/mobile/network_sim_static_ip_configuration.rs");
        include!(
            "types/mobile/network_slice_single_network_slice_selection_assistance_information.rs"
        );
        include!(
            "types/mobile/get_network_attached_data_network_network_address_port_translation.rs"
        );
        include!(
            "types/mobile/get_network_attached_data_network_network_address_port_translation_port_range.rs"
        );
        include!("types/mobile/get_network_packet_core_control_plane_identity.rs");
        include!(
            "types/mobile/get_network_packet_core_control_plane_local_diagnostics_access.rs"
        );
        include!("types/mobile/get_network_packet_core_control_plane_platform.rs");
        include!("types/mobile/get_network_service_pcc_rule.rs");
        include!("types/mobile/get_network_service_pcc_rule_qos_policy.rs");
        include!(
            "types/mobile/get_network_service_pcc_rule_qos_policy_guaranteed_bit_rate.rs"
        );
        include!(
            "types/mobile/get_network_service_pcc_rule_qos_policy_maximum_bit_rate.rs"
        );
        include!(
            "types/mobile/get_network_service_pcc_rule_service_data_flow_template.rs"
        );
        include!("types/mobile/get_network_service_service_qos_policy.rs");
        include!(
            "types/mobile/get_network_service_service_qos_policy_maximum_bit_rate.rs"
        );
        include!("types/mobile/get_network_sim_group_identity.rs");
        include!("types/mobile/get_network_sim_policy_slice.rs");
        include!("types/mobile/get_network_sim_policy_slice_data_network.rs");
        include!(
            "types/mobile/get_network_sim_policy_slice_data_network_session_aggregate_maximum_bit_rate.rs"
        );
        include!(
            "types/mobile/get_network_sim_policy_user_equipment_aggregate_maximum_bit_rate.rs"
        );
        include!("types/mobile/get_network_sim_static_ip_configuration.rs");
        include!(
            "types/mobile/get_network_slice_single_network_slice_selection_assistance_information.rs"
        );
    }
    pub mod monitoring {
        include!("types/monitoring/aad_diagnostic_setting_enabled_log.rs");
        include!(
            "types/monitoring/aad_diagnostic_setting_enabled_log_retention_policy.rs"
        );
        include!("types/monitoring/action_group_arm_role_receiver.rs");
        include!("types/monitoring/action_group_automation_runbook_receiver.rs");
        include!("types/monitoring/action_group_azure_app_push_receiver.rs");
        include!("types/monitoring/action_group_azure_function_receiver.rs");
        include!("types/monitoring/action_group_email_receiver.rs");
        include!("types/monitoring/action_group_event_hub_receiver.rs");
        include!("types/monitoring/action_group_itsm_receiver.rs");
        include!("types/monitoring/action_group_logic_app_receiver.rs");
        include!("types/monitoring/action_group_sms_receiver.rs");
        include!("types/monitoring/action_group_voice_receiver.rs");
        include!("types/monitoring/action_group_webhook_receiver.rs");
        include!("types/monitoring/action_group_webhook_receiver_aad_auth.rs");
        include!("types/monitoring/activity_log_alert_action.rs");
        include!("types/monitoring/activity_log_alert_criteria.rs");
        include!("types/monitoring/activity_log_alert_criteria_resource_health.rs");
        include!("types/monitoring/activity_log_alert_criteria_service_health.rs");
        include!("types/monitoring/alert_processing_rule_action_group_condition.rs");
        include!(
            "types/monitoring/alert_processing_rule_action_group_condition_alert_context.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_action_group_condition_alert_rule_id.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_action_group_condition_alert_rule_name.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_action_group_condition_description.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_action_group_condition_monitor_condition.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_action_group_condition_monitor_service.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_action_group_condition_severity.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_action_group_condition_signal_type.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_action_group_condition_target_resource.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_action_group_condition_target_resource_group.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_action_group_condition_target_resource_type.rs"
        );
        include!("types/monitoring/alert_processing_rule_action_group_schedule.rs");
        include!(
            "types/monitoring/alert_processing_rule_action_group_schedule_recurrence.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_action_group_schedule_recurrence_daily.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_action_group_schedule_recurrence_monthly.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_action_group_schedule_recurrence_weekly.rs"
        );
        include!("types/monitoring/alert_processing_rule_suppression_condition.rs");
        include!(
            "types/monitoring/alert_processing_rule_suppression_condition_alert_context.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_suppression_condition_alert_rule_id.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_suppression_condition_alert_rule_name.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_suppression_condition_description.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_suppression_condition_monitor_condition.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_suppression_condition_monitor_service.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_suppression_condition_severity.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_suppression_condition_signal_type.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_suppression_condition_target_resource.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_suppression_condition_target_resource_group.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_suppression_condition_target_resource_type.rs"
        );
        include!("types/monitoring/alert_processing_rule_suppression_schedule.rs");
        include!(
            "types/monitoring/alert_processing_rule_suppression_schedule_recurrence.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_suppression_schedule_recurrence_daily.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_suppression_schedule_recurrence_monthly.rs"
        );
        include!(
            "types/monitoring/alert_processing_rule_suppression_schedule_recurrence_weekly.rs"
        );
        include!("types/monitoring/alert_prometheus_rule_group_rule.rs");
        include!("types/monitoring/alert_prometheus_rule_group_rule_action.rs");
        include!(
            "types/monitoring/alert_prometheus_rule_group_rule_alert_resolution.rs"
        );
        include!("types/monitoring/autoscale_setting_notification.rs");
        include!("types/monitoring/autoscale_setting_notification_email.rs");
        include!("types/monitoring/autoscale_setting_notification_webhook.rs");
        include!("types/monitoring/autoscale_setting_predictive.rs");
        include!("types/monitoring/autoscale_setting_profile.rs");
        include!("types/monitoring/autoscale_setting_profile_capacity.rs");
        include!("types/monitoring/autoscale_setting_profile_fixed_date.rs");
        include!("types/monitoring/autoscale_setting_profile_recurrence.rs");
        include!("types/monitoring/autoscale_setting_profile_rule.rs");
        include!("types/monitoring/autoscale_setting_profile_rule_metric_trigger.rs");
        include!(
            "types/monitoring/autoscale_setting_profile_rule_metric_trigger_dimension.rs"
        );
        include!("types/monitoring/autoscale_setting_profile_rule_scale_action.rs");
        include!("types/monitoring/data_collection_rule_data_flow.rs");
        include!("types/monitoring/data_collection_rule_data_sources.rs");
        include!("types/monitoring/data_collection_rule_data_sources_data_import.rs");
        include!(
            "types/monitoring/data_collection_rule_data_sources_data_import_event_hub_data_source.rs"
        );
        include!("types/monitoring/data_collection_rule_data_sources_extension.rs");
        include!("types/monitoring/data_collection_rule_data_sources_iis_log.rs");
        include!("types/monitoring/data_collection_rule_data_sources_log_file.rs");
        include!(
            "types/monitoring/data_collection_rule_data_sources_log_file_settings.rs"
        );
        include!(
            "types/monitoring/data_collection_rule_data_sources_log_file_settings_text.rs"
        );
        include!(
            "types/monitoring/data_collection_rule_data_sources_performance_counter.rs"
        );
        include!(
            "types/monitoring/data_collection_rule_data_sources_platform_telemetry.rs"
        );
        include!(
            "types/monitoring/data_collection_rule_data_sources_prometheus_forwarder.rs"
        );
        include!(
            "types/monitoring/data_collection_rule_data_sources_prometheus_forwarder_label_include_filter.rs"
        );
        include!("types/monitoring/data_collection_rule_data_sources_syslog.rs");
        include!(
            "types/monitoring/data_collection_rule_data_sources_windows_event_log.rs"
        );
        include!(
            "types/monitoring/data_collection_rule_data_sources_windows_firewall_log.rs"
        );
        include!("types/monitoring/data_collection_rule_destinations.rs");
        include!(
            "types/monitoring/data_collection_rule_destinations_azure_monitor_metrics.rs"
        );
        include!("types/monitoring/data_collection_rule_destinations_event_hub.rs");
        include!(
            "types/monitoring/data_collection_rule_destinations_event_hub_direct.rs"
        );
        include!("types/monitoring/data_collection_rule_destinations_log_analytic.rs");
        include!(
            "types/monitoring/data_collection_rule_destinations_monitor_account.rs"
        );
        include!("types/monitoring/data_collection_rule_destinations_storage_blob.rs");
        include!(
            "types/monitoring/data_collection_rule_destinations_storage_blob_direct.rs"
        );
        include!(
            "types/monitoring/data_collection_rule_destinations_storage_table_direct.rs"
        );
        include!("types/monitoring/data_collection_rule_identity.rs");
        include!("types/monitoring/data_collection_rule_stream_declaration.rs");
        include!("types/monitoring/data_collection_rule_stream_declaration_column.rs");
        include!("types/monitoring/diagnostic_setting_enabled_log.rs");
        include!("types/monitoring/diagnostic_setting_enabled_log_retention_policy.rs");
        include!("types/monitoring/diagnostic_setting_metric.rs");
        include!("types/monitoring/diagnostic_setting_metric_retention_policy.rs");
        include!("types/monitoring/metric_alert_action.rs");
        include!(
            "types/monitoring/metric_alert_application_insights_web_test_location_availability_criteria.rs"
        );
        include!("types/monitoring/metric_alert_criteria.rs");
        include!("types/monitoring/metric_alert_criteria_dimension.rs");
        include!("types/monitoring/metric_alert_dynamic_criteria.rs");
        include!("types/monitoring/metric_alert_dynamic_criteria_dimension.rs");
        include!("types/monitoring/scheduled_query_rules_alert_action.rs");
        include!("types/monitoring/scheduled_query_rules_alert_trigger.rs");
        include!(
            "types/monitoring/scheduled_query_rules_alert_trigger_metric_trigger.rs"
        );
        include!("types/monitoring/scheduled_query_rules_alert_v_2_action.rs");
        include!("types/monitoring/scheduled_query_rules_alert_v_2_criteria.rs");
        include!(
            "types/monitoring/scheduled_query_rules_alert_v_2_criteria_dimension.rs"
        );
        include!(
            "types/monitoring/scheduled_query_rules_alert_v_2_criteria_failing_periods.rs"
        );
        include!("types/monitoring/scheduled_query_rules_alert_v_2_identity.rs");
        include!("types/monitoring/scheduled_query_rules_log_criteria.rs");
        include!("types/monitoring/scheduled_query_rules_log_criteria_dimension.rs");
        include!("types/monitoring/smart_detector_alert_rule_action_group.rs");
        include!("types/monitoring/get_action_group_arm_role_receiver.rs");
        include!("types/monitoring/get_action_group_automation_runbook_receiver.rs");
        include!("types/monitoring/get_action_group_azure_app_push_receiver.rs");
        include!("types/monitoring/get_action_group_azure_function_receiver.rs");
        include!("types/monitoring/get_action_group_email_receiver.rs");
        include!("types/monitoring/get_action_group_event_hub_receiver.rs");
        include!("types/monitoring/get_action_group_itsm_receiver.rs");
        include!("types/monitoring/get_action_group_logic_app_receiver.rs");
        include!("types/monitoring/get_action_group_sms_receiver.rs");
        include!("types/monitoring/get_action_group_voice_receiver.rs");
        include!("types/monitoring/get_action_group_webhook_receiver.rs");
        include!("types/monitoring/get_action_group_webhook_receiver_aad_auth.rs");
        include!("types/monitoring/get_data_collection_rule_data_flow.rs");
        include!("types/monitoring/get_data_collection_rule_data_source.rs");
        include!("types/monitoring/get_data_collection_rule_data_source_data_import.rs");
        include!(
            "types/monitoring/get_data_collection_rule_data_source_data_import_event_hub_data_source.rs"
        );
        include!("types/monitoring/get_data_collection_rule_data_source_extension.rs");
        include!("types/monitoring/get_data_collection_rule_data_source_iis_log.rs");
        include!("types/monitoring/get_data_collection_rule_data_source_log_file.rs");
        include!(
            "types/monitoring/get_data_collection_rule_data_source_log_file_setting.rs"
        );
        include!(
            "types/monitoring/get_data_collection_rule_data_source_log_file_setting_text.rs"
        );
        include!(
            "types/monitoring/get_data_collection_rule_data_source_performance_counter.rs"
        );
        include!(
            "types/monitoring/get_data_collection_rule_data_source_platform_telemetry.rs"
        );
        include!(
            "types/monitoring/get_data_collection_rule_data_source_prometheus_forwarder.rs"
        );
        include!(
            "types/monitoring/get_data_collection_rule_data_source_prometheus_forwarder_label_include_filter.rs"
        );
        include!("types/monitoring/get_data_collection_rule_data_source_syslog.rs");
        include!(
            "types/monitoring/get_data_collection_rule_data_source_windows_event_log.rs"
        );
        include!(
            "types/monitoring/get_data_collection_rule_data_source_windows_firewall_log.rs"
        );
        include!("types/monitoring/get_data_collection_rule_destination.rs");
        include!(
            "types/monitoring/get_data_collection_rule_destination_azure_monitor_metric.rs"
        );
        include!("types/monitoring/get_data_collection_rule_destination_event_hub.rs");
        include!(
            "types/monitoring/get_data_collection_rule_destination_event_hub_direct.rs"
        );
        include!(
            "types/monitoring/get_data_collection_rule_destination_log_analytic.rs"
        );
        include!(
            "types/monitoring/get_data_collection_rule_destination_monitor_account.rs"
        );
        include!(
            "types/monitoring/get_data_collection_rule_destination_storage_blob.rs"
        );
        include!(
            "types/monitoring/get_data_collection_rule_destination_storage_blob_direct.rs"
        );
        include!(
            "types/monitoring/get_data_collection_rule_destination_storage_table_direct.rs"
        );
        include!("types/monitoring/get_data_collection_rule_identity.rs");
        include!("types/monitoring/get_data_collection_rule_stream_declaration.rs");
        include!(
            "types/monitoring/get_data_collection_rule_stream_declaration_column.rs"
        );
        include!("types/monitoring/get_scheduled_query_rules_alert_action.rs");
        include!("types/monitoring/get_scheduled_query_rules_alert_trigger.rs");
        include!(
            "types/monitoring/get_scheduled_query_rules_alert_trigger_metric_trigger.rs"
        );
        include!("types/monitoring/get_scheduled_query_rules_log_criteria.rs");
        include!("types/monitoring/get_scheduled_query_rules_log_criteria_dimension.rs");
    }
    pub mod mssql {
        include!("types/mssql/database_identity.rs");
        include!("types/mssql/database_import.rs");
        include!("types/mssql/database_long_term_retention_policy.rs");
        include!("types/mssql/database_short_term_retention_policy.rs");
        include!("types/mssql/database_threat_detection_policy.rs");
        include!(
            "types/mssql/database_vulnerability_assessment_rule_baseline_baseline_result.rs"
        );
        include!("types/mssql/elastic_pool_per_database_settings.rs");
        include!("types/mssql/elastic_pool_sku.rs");
        include!("types/mssql/failover_group_partner_server.rs");
        include!("types/mssql/failover_group_read_write_endpoint_failover_policy.rs");
        include!("types/mssql/managed_database_long_term_retention_policy.rs");
        include!("types/mssql/managed_database_point_in_time_restore.rs");
        include!("types/mssql/managed_instance_failover_group_partner_region.rs");
        include!(
            "types/mssql/managed_instance_failover_group_read_write_endpoint_failover_policy.rs"
        );
        include!("types/mssql/managed_instance_identity.rs");
        include!(
            "types/mssql/managed_instance_vulnerability_assessment_recurring_scans.rs"
        );
        include!("types/mssql/server_azuread_administrator.rs");
        include!("types/mssql/server_identity.rs");
        include!("types/mssql/server_vulnerability_assessment_recurring_scans.rs");
        include!("types/mssql/virtual_machine_assessment.rs");
        include!("types/mssql/virtual_machine_assessment_schedule.rs");
        include!("types/mssql/virtual_machine_auto_backup.rs");
        include!("types/mssql/virtual_machine_auto_backup_manual_schedule.rs");
        include!("types/mssql/virtual_machine_auto_patching.rs");
        include!(
            "types/mssql/virtual_machine_availability_group_listener_load_balancer_configuration.rs"
        );
        include!(
            "types/mssql/virtual_machine_availability_group_listener_multi_subnet_ip_configuration.rs"
        );
        include!("types/mssql/virtual_machine_availability_group_listener_replica.rs");
        include!("types/mssql/virtual_machine_group_wsfc_domain_profile.rs");
        include!("types/mssql/virtual_machine_key_vault_credential.rs");
        include!("types/mssql/virtual_machine_sql_instance.rs");
        include!("types/mssql/virtual_machine_storage_configuration.rs");
        include!("types/mssql/virtual_machine_storage_configuration_data_settings.rs");
        include!("types/mssql/virtual_machine_storage_configuration_log_settings.rs");
        include!(
            "types/mssql/virtual_machine_storage_configuration_temp_db_settings.rs"
        );
        include!("types/mssql/virtual_machine_wsfc_domain_credential.rs");
        include!("types/mssql/get_database_identity.rs");
        include!("types/mssql/get_elastic_pool_skus.rs");
        include!("types/mssql/get_managed_database_long_term_retention_policy.rs");
        include!("types/mssql/get_managed_database_point_in_time_restore.rs");
        include!("types/mssql/get_managed_instance_identity.rs");
        include!("types/mssql/get_server_identity.rs");
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
#[link_section = "pulumi_wasm_provider::azure"]
#[no_mangle]
pub static PULUMI_WASM_PROVIDER_azure: [u8; 6] = *b"6.14.0";
