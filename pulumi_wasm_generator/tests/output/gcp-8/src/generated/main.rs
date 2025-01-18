pub mod memorystore {
    include!("resources/memorystore/instance.rs");
}
pub mod migrationcenter {
    include!("resources/migrationcenter/group.rs");
    include!("resources/migrationcenter/preference_set.rs");
}
pub mod ml {
    include!("resources/ml/engine_model.rs");
}
pub mod monitoring {
    include!("resources/monitoring/alert_policy.rs");
    include!("resources/monitoring/custom_service.rs");
    include!("resources/monitoring/dashboard.rs");
    include!("resources/monitoring/generic_service.rs");
    include!("resources/monitoring/group.rs");
    include!("resources/monitoring/metric_descriptor.rs");
    include!("resources/monitoring/monitored_project.rs");
    include!("resources/monitoring/notification_channel.rs");
    include!("resources/monitoring/slo.rs");
    include!("resources/monitoring/uptime_check_config.rs");
}
pub mod netapp {
    include!("resources/netapp/active_directory.rs");
    include!("resources/netapp/backup.rs");
    include!("resources/netapp/backup_policy.rs");
    include!("resources/netapp/backup_vault.rs");
    include!("resources/netapp/kmsconfig.rs");
    include!("resources/netapp/storage_pool.rs");
    include!("resources/netapp/volume.rs");
    include!("resources/netapp/volume_replication.rs");
    include!("resources/netapp/volume_snapshot.rs");
}
pub mod networkconnectivity {
    include!("resources/networkconnectivity/group.rs");
    include!("resources/networkconnectivity/hub.rs");
    include!("resources/networkconnectivity/internal_range.rs");
    include!("resources/networkconnectivity/policy_based_route.rs");
    include!("resources/networkconnectivity/regional_endpoint.rs");
    include!("resources/networkconnectivity/service_connection_policy.rs");
    include!("resources/networkconnectivity/spoke.rs");
}
pub mod networkmanagement {
    include!("resources/networkmanagement/connectivity_test.rs");
    include!("resources/networkmanagement/vpc_flow_logs_config.rs");
}
pub mod networksecurity {
    include!("resources/networksecurity/address_group.rs");
    include!("resources/networksecurity/address_group_iam_binding.rs");
    include!("resources/networksecurity/address_group_iam_member.rs");
    include!("resources/networksecurity/address_group_iam_policy.rs");
    include!("resources/networksecurity/authorization_policy.rs");
    include!("resources/networksecurity/authz_policy.rs");
    include!("resources/networksecurity/client_tls_policy.rs");
    include!("resources/networksecurity/firewall_endpoint.rs");
    include!("resources/networksecurity/firewall_endpoint_association.rs");
    include!("resources/networksecurity/gateway_security_policy.rs");
    include!("resources/networksecurity/gateway_security_policy_rule.rs");
    include!("resources/networksecurity/intercept_deployment.rs");
    include!("resources/networksecurity/intercept_deployment_group.rs");
    include!("resources/networksecurity/mirroring_deployment.rs");
    include!("resources/networksecurity/mirroring_deployment_group.rs");
    include!("resources/networksecurity/mirroring_endpoint_group.rs");
    include!("resources/networksecurity/mirroring_endpoint_group_association.rs");
    include!("resources/networksecurity/security_profile.rs");
    include!("resources/networksecurity/security_profile_group.rs");
    include!("resources/networksecurity/server_tls_policy.rs");
    include!("resources/networksecurity/tls_inspection_policy.rs");
    include!("resources/networksecurity/url_list.rs");
}
pub mod networkservices {
    include!("resources/networkservices/authz_extension.rs");
    include!("resources/networkservices/edge_cache_keyset.rs");
    include!("resources/networkservices/edge_cache_origin.rs");
    include!("resources/networkservices/edge_cache_service.rs");
    include!("resources/networkservices/endpoint_policy.rs");
    include!("resources/networkservices/gateway.rs");
    include!("resources/networkservices/grpc_route.rs");
    include!("resources/networkservices/http_route.rs");
    include!("resources/networkservices/lb_route_extension.rs");
    include!("resources/networkservices/lb_traffic_extension.rs");
    include!("resources/networkservices/mesh.rs");
    include!("resources/networkservices/service_binding.rs");
    include!("resources/networkservices/service_lb_policies.rs");
    include!("resources/networkservices/tcp_route.rs");
    include!("resources/networkservices/tls_route.rs");
}
pub mod notebooks {
    include!("resources/notebooks/environment.rs");
    include!("resources/notebooks/instance.rs");
    include!("resources/notebooks/instance_iam_binding.rs");
    include!("resources/notebooks/instance_iam_member.rs");
    include!("resources/notebooks/instance_iam_policy.rs");
    include!("resources/notebooks/location.rs");
    include!("resources/notebooks/runtime.rs");
    include!("resources/notebooks/runtime_iam_binding.rs");
    include!("resources/notebooks/runtime_iam_member.rs");
    include!("resources/notebooks/runtime_iam_policy.rs");
}
pub mod functions {
    pub mod monitoring {
        include!("functions/monitoring/get_app_engine_service.rs");
        include!("functions/monitoring/get_cluster_istio_service.rs");
        include!("functions/monitoring/get_istio_canonical_service.rs");
        include!("functions/monitoring/get_mesh_istio_service.rs");
        include!("functions/monitoring/get_notification_channel.rs");
        include!("functions/monitoring/get_secret_version.rs");
        include!("functions/monitoring/get_uptime_check_i_ps.rs");
    }
    pub mod networksecurity {
        include!("functions/networksecurity/get_address_group_iam_policy.rs");
    }
    pub mod notebooks {
        include!("functions/notebooks/get_instance_iam_policy.rs");
        include!("functions/notebooks/get_runtime_iam_policy.rs");
    }
}
pub mod types {
    pub mod memorystore {
        include!("types/memorystore/instance_desired_psc_auto_connection.rs");
        include!("types/memorystore/instance_discovery_endpoint.rs");
        include!("types/memorystore/instance_node_config.rs");
        include!("types/memorystore/instance_persistence_config.rs");
        include!("types/memorystore/instance_persistence_config_aof_config.rs");
        include!("types/memorystore/instance_persistence_config_rdb_config.rs");
        include!("types/memorystore/instance_psc_auto_connection.rs");
        include!("types/memorystore/instance_state_info.rs");
        include!("types/memorystore/instance_state_info_update_info.rs");
        include!("types/memorystore/instance_zone_distribution_config.rs");
    }
    pub mod migrationcenter {
        include!("types/migrationcenter/preference_set_virtual_machine_preferences.rs");
        include!(
            "types/migrationcenter/preference_set_virtual_machine_preferences_compute_engine_preferences.rs"
        );
        include!(
            "types/migrationcenter/preference_set_virtual_machine_preferences_compute_engine_preferences_machine_preferences.rs"
        );
        include!(
            "types/migrationcenter/preference_set_virtual_machine_preferences_compute_engine_preferences_machine_preferences_allowed_machine_series.rs"
        );
        include!(
            "types/migrationcenter/preference_set_virtual_machine_preferences_region_preferences.rs"
        );
        include!(
            "types/migrationcenter/preference_set_virtual_machine_preferences_sole_tenancy_preferences.rs"
        );
        include!(
            "types/migrationcenter/preference_set_virtual_machine_preferences_sole_tenancy_preferences_node_type.rs"
        );
        include!(
            "types/migrationcenter/preference_set_virtual_machine_preferences_vmware_engine_preferences.rs"
        );
    }
    pub mod ml {
        include!("types/ml/engine_model_default_version.rs");
    }
    pub mod monitoring {
        include!("types/monitoring/alert_policy_alert_strategy.rs");
        include!(
            "types/monitoring/alert_policy_alert_strategy_notification_channel_strategy.rs"
        );
        include!(
            "types/monitoring/alert_policy_alert_strategy_notification_rate_limit.rs"
        );
        include!("types/monitoring/alert_policy_condition.rs");
        include!("types/monitoring/alert_policy_condition_condition_absent.rs");
        include!(
            "types/monitoring/alert_policy_condition_condition_absent_aggregation.rs"
        );
        include!("types/monitoring/alert_policy_condition_condition_absent_trigger.rs");
        include!("types/monitoring/alert_policy_condition_condition_matched_log.rs");
        include!(
            "types/monitoring/alert_policy_condition_condition_monitoring_query_language.rs"
        );
        include!(
            "types/monitoring/alert_policy_condition_condition_monitoring_query_language_trigger.rs"
        );
        include!(
            "types/monitoring/alert_policy_condition_condition_prometheus_query_language.rs"
        );
        include!("types/monitoring/alert_policy_condition_condition_threshold.rs");
        include!(
            "types/monitoring/alert_policy_condition_condition_threshold_aggregation.rs"
        );
        include!(
            "types/monitoring/alert_policy_condition_condition_threshold_denominator_aggregation.rs"
        );
        include!(
            "types/monitoring/alert_policy_condition_condition_threshold_forecast_options.rs"
        );
        include!(
            "types/monitoring/alert_policy_condition_condition_threshold_trigger.rs"
        );
        include!("types/monitoring/alert_policy_creation_record.rs");
        include!("types/monitoring/alert_policy_documentation.rs");
        include!("types/monitoring/alert_policy_documentation_link.rs");
        include!("types/monitoring/custom_service_telemetry.rs");
        include!("types/monitoring/generic_service_basic_service.rs");
        include!("types/monitoring/generic_service_telemetry.rs");
        include!("types/monitoring/metric_descriptor_label.rs");
        include!("types/monitoring/metric_descriptor_metadata.rs");
        include!("types/monitoring/notification_channel_sensitive_labels.rs");
        include!("types/monitoring/slo_basic_sli.rs");
        include!("types/monitoring/slo_basic_sli_availability.rs");
        include!("types/monitoring/slo_basic_sli_latency.rs");
        include!("types/monitoring/slo_request_based_sli.rs");
        include!("types/monitoring/slo_request_based_sli_distribution_cut.rs");
        include!("types/monitoring/slo_request_based_sli_distribution_cut_range.rs");
        include!("types/monitoring/slo_request_based_sli_good_total_ratio.rs");
        include!("types/monitoring/slo_windows_based_sli.rs");
        include!("types/monitoring/slo_windows_based_sli_good_total_ratio_threshold.rs");
        include!(
            "types/monitoring/slo_windows_based_sli_good_total_ratio_threshold_basic_sli_performance.rs"
        );
        include!(
            "types/monitoring/slo_windows_based_sli_good_total_ratio_threshold_basic_sli_performance_availability.rs"
        );
        include!(
            "types/monitoring/slo_windows_based_sli_good_total_ratio_threshold_basic_sli_performance_latency.rs"
        );
        include!(
            "types/monitoring/slo_windows_based_sli_good_total_ratio_threshold_performance.rs"
        );
        include!(
            "types/monitoring/slo_windows_based_sli_good_total_ratio_threshold_performance_distribution_cut.rs"
        );
        include!(
            "types/monitoring/slo_windows_based_sli_good_total_ratio_threshold_performance_distribution_cut_range.rs"
        );
        include!(
            "types/monitoring/slo_windows_based_sli_good_total_ratio_threshold_performance_good_total_ratio.rs"
        );
        include!("types/monitoring/slo_windows_based_sli_metric_mean_in_range.rs");
        include!("types/monitoring/slo_windows_based_sli_metric_mean_in_range_range.rs");
        include!("types/monitoring/slo_windows_based_sli_metric_sum_in_range.rs");
        include!("types/monitoring/slo_windows_based_sli_metric_sum_in_range_range.rs");
        include!("types/monitoring/uptime_check_config_content_matcher.rs");
        include!(
            "types/monitoring/uptime_check_config_content_matcher_json_path_matcher.rs"
        );
        include!("types/monitoring/uptime_check_config_http_check.rs");
        include!(
            "types/monitoring/uptime_check_config_http_check_accepted_response_status_code.rs"
        );
        include!("types/monitoring/uptime_check_config_http_check_auth_info.rs");
        include!("types/monitoring/uptime_check_config_http_check_ping_config.rs");
        include!(
            "types/monitoring/uptime_check_config_http_check_service_agent_authentication.rs"
        );
        include!("types/monitoring/uptime_check_config_monitored_resource.rs");
        include!("types/monitoring/uptime_check_config_resource_group.rs");
        include!("types/monitoring/uptime_check_config_synthetic_monitor.rs");
        include!(
            "types/monitoring/uptime_check_config_synthetic_monitor_cloud_function_v_2.rs"
        );
        include!("types/monitoring/uptime_check_config_tcp_check.rs");
        include!("types/monitoring/uptime_check_config_tcp_check_ping_config.rs");
        include!("types/monitoring/get_app_engine_service_telemetry.rs");
        include!("types/monitoring/get_cluster_istio_service_telemetry.rs");
        include!("types/monitoring/get_istio_canonical_service_telemetry.rs");
        include!("types/monitoring/get_mesh_istio_service_telemetry.rs");
        include!("types/monitoring/get_notification_channel_sensitive_label.rs");
        include!("types/monitoring/get_uptime_check_i_ps_uptime_check_ip.rs");
    }
    pub mod netapp {
        include!("types/netapp/volume_backup_config.rs");
        include!("types/netapp/volume_export_policy.rs");
        include!("types/netapp/volume_export_policy_rule.rs");
        include!("types/netapp/volume_mount_option.rs");
        include!("types/netapp/volume_replication_destination_volume_parameters.rs");
        include!("types/netapp/volume_replication_transfer_stat.rs");
        include!("types/netapp/volume_restore_parameters.rs");
        include!("types/netapp/volume_snapshot_policy.rs");
        include!("types/netapp/volume_snapshot_policy_daily_schedule.rs");
        include!("types/netapp/volume_snapshot_policy_hourly_schedule.rs");
        include!("types/netapp/volume_snapshot_policy_monthly_schedule.rs");
        include!("types/netapp/volume_snapshot_policy_weekly_schedule.rs");
        include!("types/netapp/volume_tiering_policy.rs");
    }
    pub mod networkconnectivity {
        include!("types/networkconnectivity/group_auto_accept.rs");
        include!("types/networkconnectivity/hub_routing_vpc.rs");
        include!("types/networkconnectivity/internal_range_migration.rs");
        include!("types/networkconnectivity/policy_based_route_filter.rs");
        include!(
            "types/networkconnectivity/policy_based_route_interconnect_attachment.rs"
        );
        include!("types/networkconnectivity/policy_based_route_virtual_machine.rs");
        include!("types/networkconnectivity/policy_based_route_warning.rs");
        include!("types/networkconnectivity/service_connection_policy_psc_config.rs");
        include!(
            "types/networkconnectivity/service_connection_policy_psc_connection.rs"
        );
        include!(
            "types/networkconnectivity/service_connection_policy_psc_connection_error.rs"
        );
        include!(
            "types/networkconnectivity/service_connection_policy_psc_connection_error_info.rs"
        );
        include!("types/networkconnectivity/spoke_linked_interconnect_attachments.rs");
        include!("types/networkconnectivity/spoke_linked_producer_vpc_network.rs");
        include!("types/networkconnectivity/spoke_linked_router_appliance_instances.rs");
        include!(
            "types/networkconnectivity/spoke_linked_router_appliance_instances_instance.rs"
        );
        include!("types/networkconnectivity/spoke_linked_vpc_network.rs");
        include!("types/networkconnectivity/spoke_linked_vpn_tunnels.rs");
    }
    pub mod networkmanagement {
        include!("types/networkmanagement/connectivity_test_destination.rs");
        include!("types/networkmanagement/connectivity_test_source.rs");
    }
    pub mod networksecurity {
        include!("types/networksecurity/address_group_iam_binding_condition.rs");
        include!("types/networksecurity/address_group_iam_member_condition.rs");
        include!("types/networksecurity/authorization_policy_rule.rs");
        include!("types/networksecurity/authorization_policy_rule_destination.rs");
        include!(
            "types/networksecurity/authorization_policy_rule_destination_http_header_match.rs"
        );
        include!("types/networksecurity/authorization_policy_rule_source.rs");
        include!("types/networksecurity/authz_policy_custom_provider.rs");
        include!(
            "types/networksecurity/authz_policy_custom_provider_authz_extension.rs"
        );
        include!("types/networksecurity/authz_policy_custom_provider_cloud_iap.rs");
        include!("types/networksecurity/authz_policy_http_rule.rs");
        include!("types/networksecurity/authz_policy_http_rule_from.rs");
        include!("types/networksecurity/authz_policy_http_rule_from_not_source.rs");
        include!(
            "types/networksecurity/authz_policy_http_rule_from_not_source_principal.rs"
        );
        include!(
            "types/networksecurity/authz_policy_http_rule_from_not_source_resource.rs"
        );
        include!(
            "types/networksecurity/authz_policy_http_rule_from_not_source_resource_iam_service_account.rs"
        );
        include!(
            "types/networksecurity/authz_policy_http_rule_from_not_source_resource_tag_value_id_set.rs"
        );
        include!("types/networksecurity/authz_policy_http_rule_from_source.rs");
        include!(
            "types/networksecurity/authz_policy_http_rule_from_source_principal.rs"
        );
        include!("types/networksecurity/authz_policy_http_rule_from_source_resource.rs");
        include!(
            "types/networksecurity/authz_policy_http_rule_from_source_resource_iam_service_account.rs"
        );
        include!(
            "types/networksecurity/authz_policy_http_rule_from_source_resource_tag_value_id_set.rs"
        );
        include!("types/networksecurity/authz_policy_http_rule_to.rs");
        include!("types/networksecurity/authz_policy_http_rule_to_operation.rs");
        include!(
            "types/networksecurity/authz_policy_http_rule_to_operation_header_set.rs"
        );
        include!(
            "types/networksecurity/authz_policy_http_rule_to_operation_header_set_header.rs"
        );
        include!(
            "types/networksecurity/authz_policy_http_rule_to_operation_header_set_header_value.rs"
        );
        include!("types/networksecurity/authz_policy_http_rule_to_operation_host.rs");
        include!("types/networksecurity/authz_policy_http_rule_to_operation_path.rs");
        include!("types/networksecurity/authz_policy_target.rs");
        include!("types/networksecurity/client_tls_policy_client_certificate.rs");
        include!(
            "types/networksecurity/client_tls_policy_client_certificate_certificate_provider_instance.rs"
        );
        include!(
            "types/networksecurity/client_tls_policy_client_certificate_grpc_endpoint.rs"
        );
        include!("types/networksecurity/client_tls_policy_server_validation_ca.rs");
        include!(
            "types/networksecurity/client_tls_policy_server_validation_ca_certificate_provider_instance.rs"
        );
        include!(
            "types/networksecurity/client_tls_policy_server_validation_ca_grpc_endpoint.rs"
        );
        include!(
            "types/networksecurity/intercept_deployment_group_connected_endpoint_group.rs"
        );
        include!(
            "types/networksecurity/mirroring_deployment_group_connected_endpoint_group.rs"
        );
        include!(
            "types/networksecurity/mirroring_endpoint_group_association_locations_detail.rs"
        );
        include!("types/networksecurity/security_profile_threat_prevention_profile.rs");
        include!(
            "types/networksecurity/security_profile_threat_prevention_profile_severity_override.rs"
        );
        include!(
            "types/networksecurity/security_profile_threat_prevention_profile_threat_override.rs"
        );
        include!("types/networksecurity/server_tls_policy_mtls_policy.rs");
        include!(
            "types/networksecurity/server_tls_policy_mtls_policy_client_validation_ca.rs"
        );
        include!(
            "types/networksecurity/server_tls_policy_mtls_policy_client_validation_ca_certificate_provider_instance.rs"
        );
        include!(
            "types/networksecurity/server_tls_policy_mtls_policy_client_validation_ca_grpc_endpoint.rs"
        );
        include!("types/networksecurity/server_tls_policy_server_certificate.rs");
        include!(
            "types/networksecurity/server_tls_policy_server_certificate_certificate_provider_instance.rs"
        );
        include!(
            "types/networksecurity/server_tls_policy_server_certificate_grpc_endpoint.rs"
        );
    }
    pub mod networkservices {
        include!("types/networkservices/edge_cache_keyset_public_key.rs");
        include!("types/networkservices/edge_cache_keyset_validation_shared_key.rs");
        include!("types/networkservices/edge_cache_origin_aws_v_4_authentication.rs");
        include!("types/networkservices/edge_cache_origin_origin_override_action.rs");
        include!(
            "types/networkservices/edge_cache_origin_origin_override_action_header_action.rs"
        );
        include!(
            "types/networkservices/edge_cache_origin_origin_override_action_header_action_request_headers_to_add.rs"
        );
        include!(
            "types/networkservices/edge_cache_origin_origin_override_action_url_rewrite.rs"
        );
        include!("types/networkservices/edge_cache_origin_origin_redirect.rs");
        include!("types/networkservices/edge_cache_origin_timeout.rs");
        include!("types/networkservices/edge_cache_service_log_config.rs");
        include!("types/networkservices/edge_cache_service_routing.rs");
        include!("types/networkservices/edge_cache_service_routing_host_rule.rs");
        include!("types/networkservices/edge_cache_service_routing_path_matcher.rs");
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_header_action.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_header_action_request_header_to_add.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_header_action_request_header_to_remove.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_header_action_response_header_to_add.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_header_action_response_header_to_remove.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_match_rule.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_match_rule_header_match.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_match_rule_query_parameter_match.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_route_action.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_route_action_cdn_policy.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_route_action_cdn_policy_add_signatures.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_route_action_cdn_policy_cache_key_policy.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_route_action_cdn_policy_signed_token_options.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_route_action_cors_policy.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_route_action_url_rewrite.rs"
        );
        include!(
            "types/networkservices/edge_cache_service_routing_path_matcher_route_rule_url_redirect.rs"
        );
        include!("types/networkservices/endpoint_policy_endpoint_matcher.rs");
        include!(
            "types/networkservices/endpoint_policy_endpoint_matcher_metadata_label_matcher.rs"
        );
        include!(
            "types/networkservices/endpoint_policy_endpoint_matcher_metadata_label_matcher_metadata_label.rs"
        );
        include!("types/networkservices/endpoint_policy_traffic_port_selector.rs");
        include!("types/networkservices/grpc_route_rule.rs");
        include!("types/networkservices/grpc_route_rule_action.rs");
        include!("types/networkservices/grpc_route_rule_action_destination.rs");
        include!(
            "types/networkservices/grpc_route_rule_action_fault_injection_policy.rs"
        );
        include!(
            "types/networkservices/grpc_route_rule_action_fault_injection_policy_abort.rs"
        );
        include!(
            "types/networkservices/grpc_route_rule_action_fault_injection_policy_delay.rs"
        );
        include!("types/networkservices/grpc_route_rule_action_retry_policy.rs");
        include!("types/networkservices/grpc_route_rule_match.rs");
        include!("types/networkservices/grpc_route_rule_match_header.rs");
        include!("types/networkservices/grpc_route_rule_match_method.rs");
        include!("types/networkservices/http_route_rule.rs");
        include!("types/networkservices/http_route_rule_action.rs");
        include!("types/networkservices/http_route_rule_action_cors_policy.rs");
        include!("types/networkservices/http_route_rule_action_destination.rs");
        include!(
            "types/networkservices/http_route_rule_action_fault_injection_policy.rs"
        );
        include!(
            "types/networkservices/http_route_rule_action_fault_injection_policy_abort.rs"
        );
        include!(
            "types/networkservices/http_route_rule_action_fault_injection_policy_delay.rs"
        );
        include!("types/networkservices/http_route_rule_action_redirect.rs");
        include!(
            "types/networkservices/http_route_rule_action_request_header_modifier.rs"
        );
        include!(
            "types/networkservices/http_route_rule_action_request_mirror_policy.rs"
        );
        include!(
            "types/networkservices/http_route_rule_action_request_mirror_policy_destination.rs"
        );
        include!(
            "types/networkservices/http_route_rule_action_response_header_modifier.rs"
        );
        include!("types/networkservices/http_route_rule_action_retry_policy.rs");
        include!("types/networkservices/http_route_rule_action_url_rewrite.rs");
        include!("types/networkservices/http_route_rule_match.rs");
        include!("types/networkservices/http_route_rule_match_header.rs");
        include!("types/networkservices/http_route_rule_match_header_range_match.rs");
        include!("types/networkservices/http_route_rule_match_query_parameter.rs");
        include!("types/networkservices/lb_route_extension_extension_chain.rs");
        include!(
            "types/networkservices/lb_route_extension_extension_chain_extension.rs"
        );
        include!(
            "types/networkservices/lb_route_extension_extension_chain_match_condition.rs"
        );
        include!("types/networkservices/lb_traffic_extension_extension_chain.rs");
        include!(
            "types/networkservices/lb_traffic_extension_extension_chain_extension.rs"
        );
        include!(
            "types/networkservices/lb_traffic_extension_extension_chain_match_condition.rs"
        );
        include!("types/networkservices/service_lb_policies_auto_capacity_drain.rs");
        include!("types/networkservices/service_lb_policies_failover_config.rs");
        include!("types/networkservices/tcp_route_rule.rs");
        include!("types/networkservices/tcp_route_rule_action.rs");
        include!("types/networkservices/tcp_route_rule_action_destination.rs");
        include!("types/networkservices/tcp_route_rule_match.rs");
        include!("types/networkservices/tls_route_rule.rs");
        include!("types/networkservices/tls_route_rule_action.rs");
        include!("types/networkservices/tls_route_rule_action_destination.rs");
        include!("types/networkservices/tls_route_rule_match.rs");
    }
    pub mod notebooks {
        include!("types/notebooks/environment_container_image.rs");
        include!("types/notebooks/environment_vm_image.rs");
        include!("types/notebooks/instance_accelerator_config.rs");
        include!("types/notebooks/instance_container_image.rs");
        include!("types/notebooks/instance_iam_binding_condition.rs");
        include!("types/notebooks/instance_iam_member_condition.rs");
        include!("types/notebooks/instance_reservation_affinity.rs");
        include!("types/notebooks/instance_shielded_instance_config.rs");
        include!("types/notebooks/instance_vm_image.rs");
        include!("types/notebooks/runtime_access_config.rs");
        include!("types/notebooks/runtime_iam_binding_condition.rs");
        include!("types/notebooks/runtime_iam_member_condition.rs");
        include!("types/notebooks/runtime_metric.rs");
        include!("types/notebooks/runtime_software_config.rs");
        include!("types/notebooks/runtime_software_config_kernel.rs");
        include!("types/notebooks/runtime_virtual_machine.rs");
        include!("types/notebooks/runtime_virtual_machine_virtual_machine_config.rs");
        include!(
            "types/notebooks/runtime_virtual_machine_virtual_machine_config_accelerator_config.rs"
        );
        include!(
            "types/notebooks/runtime_virtual_machine_virtual_machine_config_container_image.rs"
        );
        include!(
            "types/notebooks/runtime_virtual_machine_virtual_machine_config_data_disk.rs"
        );
        include!(
            "types/notebooks/runtime_virtual_machine_virtual_machine_config_data_disk_initialize_params.rs"
        );
        include!(
            "types/notebooks/runtime_virtual_machine_virtual_machine_config_encryption_config.rs"
        );
        include!(
            "types/notebooks/runtime_virtual_machine_virtual_machine_config_shielded_instance_config.rs"
        );
    }
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-gcp {
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
#[link_section = "pulumi_wasm_provider::gcp"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_GCP: [u8; 45] = *b"{\"version\":\"8.12.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "8.12.1".to_string()
}
