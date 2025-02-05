pub mod mwaa {
    include!("resources/mwaa/environment.rs");
}
pub mod neptune {
    include!("resources/neptune/cluster.rs");
    include!("resources/neptune/cluster_endpoint.rs");
    include!("resources/neptune/cluster_instance.rs");
    include!("resources/neptune/cluster_parameter_group.rs");
    include!("resources/neptune/cluster_snapshot.rs");
    include!("resources/neptune/event_subscription.rs");
    include!("resources/neptune/global_cluster.rs");
    include!("resources/neptune/parameter_group.rs");
    include!("resources/neptune/subnet_group.rs");
}
pub mod networkfirewall {
    include!("resources/networkfirewall/firewall.rs");
    include!("resources/networkfirewall/firewall_policy.rs");
    include!("resources/networkfirewall/logging_configuration.rs");
    include!("resources/networkfirewall/resource_policy.rs");
    include!("resources/networkfirewall/rule_group.rs");
    include!("resources/networkfirewall/tls_inspection_configuration.rs");
}
pub mod networkmanager {
    include!("resources/networkmanager/attachment_accepter.rs");
    include!("resources/networkmanager/connect_attachment.rs");
    include!("resources/networkmanager/connect_peer.rs");
    include!("resources/networkmanager/connection.rs");
    include!("resources/networkmanager/core_network.rs");
    include!("resources/networkmanager/core_network_policy_attachment.rs");
    include!("resources/networkmanager/customer_gateway_association.rs");
    include!("resources/networkmanager/device.rs");
    include!("resources/networkmanager/dx_gateway_attachment.rs");
    include!("resources/networkmanager/global_network.rs");
    include!("resources/networkmanager/link.rs");
    include!("resources/networkmanager/link_association.rs");
    include!("resources/networkmanager/site.rs");
    include!("resources/networkmanager/site_to_site_vpn_attachment.rs");
    include!("resources/networkmanager/transit_gateway_connect_peer_association.rs");
    include!("resources/networkmanager/transit_gateway_peering.rs");
    include!("resources/networkmanager/transit_gateway_registration.rs");
    include!("resources/networkmanager/transit_gateway_route_table_attachment.rs");
    include!("resources/networkmanager/vpc_attachment.rs");
}
pub mod networkmonitor {
    include!("resources/networkmonitor/monitor.rs");
    include!("resources/networkmonitor/probe.rs");
}
pub mod oam {
    include!("resources/oam/link.rs");
    include!("resources/oam/sink.rs");
    include!("resources/oam/sink_policy.rs");
}
pub mod opensearch {
    include!("resources/opensearch/authorize_vpc_endpoint_access.rs");
    include!("resources/opensearch/domain.rs");
    include!("resources/opensearch/domain_policy.rs");
    include!("resources/opensearch/domain_saml_options.rs");
    include!("resources/opensearch/inbound_connection_accepter.rs");
    include!("resources/opensearch/outbound_connection.rs");
    include!("resources/opensearch/package.rs");
    include!("resources/opensearch/package_association.rs");
    include!("resources/opensearch/serverless_access_policy.rs");
    include!("resources/opensearch/serverless_collection.rs");
    include!("resources/opensearch/serverless_lifecycle_policy.rs");
    include!("resources/opensearch/serverless_security_config.rs");
    include!("resources/opensearch/serverless_security_policy.rs");
    include!("resources/opensearch/serverless_vpc_endpoint.rs");
    include!("resources/opensearch/vpc_endpoint.rs");
}
pub mod opensearchingest {
    include!("resources/opensearchingest/pipeline.rs");
}
pub mod opsworks {
    include!("resources/opsworks/application.rs");
    include!("resources/opsworks/custom_layer.rs");
    include!("resources/opsworks/ecs_cluster_layer.rs");
    include!("resources/opsworks/ganglia_layer.rs");
    include!("resources/opsworks/haproxy_layer.rs");
    include!("resources/opsworks/instance.rs");
    include!("resources/opsworks/java_app_layer.rs");
    include!("resources/opsworks/memcached_layer.rs");
    include!("resources/opsworks/mysql_layer.rs");
    include!("resources/opsworks/nodejs_app_layer.rs");
    include!("resources/opsworks/permission.rs");
    include!("resources/opsworks/php_app_layer.rs");
    include!("resources/opsworks/rails_app_layer.rs");
    include!("resources/opsworks/rds_db_instance.rs");
    include!("resources/opsworks/stack.rs");
    include!("resources/opsworks/static_web_layer.rs");
    include!("resources/opsworks/user_profile.rs");
}
pub mod organizations {
    include!("resources/organizations/account.rs");
    include!("resources/organizations/delegated_administrator.rs");
    include!("resources/organizations/organization.rs");
    include!("resources/organizations/organizational_unit.rs");
    include!("resources/organizations/policy.rs");
    include!("resources/organizations/policy_attachment.rs");
    include!("resources/organizations/resource_policy.rs");
}
pub mod functions {
    pub mod neptune {
        include!("functions/neptune/get_engine_version.rs");
        include!("functions/neptune/get_orderable_db_instance.rs");
    }
    pub mod networkfirewall {
        include!("functions/networkfirewall/get_firewall.rs");
        include!("functions/networkfirewall/get_firewall_policy.rs");
        include!("functions/networkfirewall/get_resource_policy.rs");
    }
    pub mod networkmanager {
        include!("functions/networkmanager/get_connection.rs");
        include!("functions/networkmanager/get_connections.rs");
        include!("functions/networkmanager/get_core_network_policy_document.rs");
        include!("functions/networkmanager/get_device.rs");
        include!("functions/networkmanager/get_devices.rs");
        include!("functions/networkmanager/get_global_network.rs");
        include!("functions/networkmanager/get_global_networks.rs");
        include!("functions/networkmanager/get_link.rs");
        include!("functions/networkmanager/get_links.rs");
        include!("functions/networkmanager/get_site.rs");
        include!("functions/networkmanager/get_sites.rs");
    }
    pub mod oam {
        include!("functions/oam/get_link.rs");
        include!("functions/oam/get_links.rs");
        include!("functions/oam/get_sink.rs");
        include!("functions/oam/get_sinks.rs");
    }
    pub mod opensearch {
        include!("functions/opensearch/get_domain.rs");
        include!("functions/opensearch/get_serverless_access_policy.rs");
        include!("functions/opensearch/get_serverless_collection.rs");
        include!("functions/opensearch/get_serverless_lifecycle_policy.rs");
        include!("functions/opensearch/get_serverless_security_config.rs");
        include!("functions/opensearch/get_serverless_security_policy.rs");
        include!("functions/opensearch/get_serverless_vpc_endpoint.rs");
    }
    pub mod organizations {
        include!("functions/organizations/get_delegated_administrators.rs");
        include!("functions/organizations/get_delegated_services.rs");
        include!("functions/organizations/get_organization.rs");
        include!("functions/organizations/get_organizational_unit.rs");
        include!("functions/organizations/get_organizational_unit_child_accounts.rs");
        include!(
            "functions/organizations/get_organizational_unit_descendant_accounts.rs"
        );
        include!(
            "functions/organizations/get_organizational_unit_descendant_organizational_units.rs"
        );
        include!("functions/organizations/get_organizational_units.rs");
        include!("functions/organizations/get_policies.rs");
        include!("functions/organizations/get_policies_for_target.rs");
        include!("functions/organizations/get_policy.rs");
        include!("functions/organizations/get_resource_tags.rs");
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
    pub mod mwaa {
        include!("types/mwaa/environment_last_updated.rs");
        include!("types/mwaa/environment_last_updated_error.rs");
        include!("types/mwaa/environment_logging_configuration.rs");
        include!("types/mwaa/environment_logging_configuration_dag_processing_logs.rs");
        include!("types/mwaa/environment_logging_configuration_scheduler_logs.rs");
        include!("types/mwaa/environment_logging_configuration_task_logs.rs");
        include!("types/mwaa/environment_logging_configuration_webserver_logs.rs");
        include!("types/mwaa/environment_logging_configuration_worker_logs.rs");
        include!("types/mwaa/environment_network_configuration.rs");
    }
    pub mod neptune {
        include!("types/neptune/cluster_parameter_group_parameter.rs");
        include!("types/neptune/cluster_serverless_v_2_scaling_configuration.rs");
        include!("types/neptune/global_cluster_global_cluster_member.rs");
        include!("types/neptune/parameter_group_parameter.rs");
    }
    pub mod networkfirewall {
        include!("types/networkfirewall/firewall_encryption_configuration.rs");
        include!("types/networkfirewall/firewall_firewall_status.rs");
        include!("types/networkfirewall/firewall_firewall_status_sync_state.rs");
        include!(
            "types/networkfirewall/firewall_firewall_status_sync_state_attachment.rs"
        );
        include!("types/networkfirewall/firewall_policy_encryption_configuration.rs");
        include!("types/networkfirewall/firewall_policy_firewall_policy.rs");
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_policy_variables.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_policy_variables_rule_variable.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_policy_variables_rule_variable_ip_set.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateful_engine_options.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateful_engine_options_flow_timeouts.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateful_rule_group_reference.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateful_rule_group_reference_override.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateless_custom_action.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateless_custom_action_action_definition.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateless_custom_action_action_definition_publish_metric_action.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateless_custom_action_action_definition_publish_metric_action_dimension.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateless_rule_group_reference.rs"
        );
        include!("types/networkfirewall/firewall_subnet_mapping.rs");
        include!("types/networkfirewall/logging_configuration_logging_configuration.rs");
        include!(
            "types/networkfirewall/logging_configuration_logging_configuration_log_destination_config.rs"
        );
        include!("types/networkfirewall/rule_group_encryption_configuration.rs");
        include!("types/networkfirewall/rule_group_rule_group.rs");
        include!("types/networkfirewall/rule_group_rule_group_reference_sets.rs");
        include!(
            "types/networkfirewall/rule_group_rule_group_reference_sets_ip_set_reference.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_reference_sets_ip_set_reference_ip_set_reference.rs"
        );
        include!("types/networkfirewall/rule_group_rule_group_rule_variables.rs");
        include!("types/networkfirewall/rule_group_rule_group_rule_variables_ip_set.rs");
        include!(
            "types/networkfirewall/rule_group_rule_group_rule_variables_ip_set_ip_set.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rule_variables_port_set.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rule_variables_port_set_port_set.rs"
        );
        include!("types/networkfirewall/rule_group_rule_group_rules_source.rs");
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_rules_source_list.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateful_rule.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateful_rule_header.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateful_rule_rule_option.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_custom_action.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_custom_action_action_definition.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_custom_action_action_definition_publish_metric_action.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_custom_action_action_definition_publish_metric_action_dimension.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule_rule_definition.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule_rule_definition_match_attributes.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule_rule_definition_match_attributes_destination.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule_rule_definition_match_attributes_destination_port.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule_rule_definition_match_attributes_source.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule_rule_definition_match_attributes_source_port.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule_rule_definition_match_attributes_tcp_flag.rs"
        );
        include!("types/networkfirewall/rule_group_rule_group_stateful_rule_options.rs");
        include!("types/networkfirewall/tls_inspection_configuration_certificate.rs");
        include!(
            "types/networkfirewall/tls_inspection_configuration_certificate_authority.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_encryption_configuration.rs"
        );
        include!("types/networkfirewall/tls_inspection_configuration_timeouts.rs");
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration_check_certificate_revocation_status.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration_scope.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration_scope_destination.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration_scope_destination_port.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration_scope_source.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration_scope_source_port.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration_server_certificate.rs"
        );
        include!("types/networkfirewall/get_firewall_encryption_configuration.rs");
        include!("types/networkfirewall/get_firewall_firewall_status.rs");
        include!(
            "types/networkfirewall/get_firewall_firewall_status_capacity_usage_summary.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_firewall_status_capacity_usage_summary_cidr.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_firewall_status_capacity_usage_summary_cidr_ip_set_reference.rs"
        );
        include!("types/networkfirewall/get_firewall_firewall_status_sync_state.rs");
        include!(
            "types/networkfirewall/get_firewall_firewall_status_sync_state_attachment.rs"
        );
        include!("types/networkfirewall/get_firewall_policy_firewall_policy.rs");
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateful_engine_option.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateful_rule_group_reference.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateful_rule_group_reference_override.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateless_custom_action.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateless_custom_action_action_definition.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateless_custom_action_action_definition_publish_metric_action.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateless_custom_action_action_definition_publish_metric_action_dimension.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateless_rule_group_reference.rs"
        );
        include!("types/networkfirewall/get_firewall_subnet_mapping.rs");
    }
    pub mod networkmanager {
        include!("types/networkmanager/connect_attachment_options.rs");
        include!("types/networkmanager/connect_peer_bgp_options.rs");
        include!("types/networkmanager/connect_peer_configuration.rs");
        include!("types/networkmanager/connect_peer_configuration_bgp_configuration.rs");
        include!("types/networkmanager/core_network_edge.rs");
        include!("types/networkmanager/core_network_segment.rs");
        include!("types/networkmanager/device_aws_location.rs");
        include!("types/networkmanager/device_location.rs");
        include!("types/networkmanager/dx_gateway_attachment_timeouts.rs");
        include!("types/networkmanager/link_bandwidth.rs");
        include!("types/networkmanager/site_location.rs");
        include!("types/networkmanager/vpc_attachment_options.rs");
        include!(
            "types/networkmanager/get_core_network_policy_document_attachment_policy.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_attachment_policy_action.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_attachment_policy_condition.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_core_network_configuration.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_core_network_configuration_edge_location.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_network_function_group.rs"
        );
        include!("types/networkmanager/get_core_network_policy_document_segment.rs");
        include!(
            "types/networkmanager/get_core_network_policy_document_segment_action.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_segment_action_via.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_segment_action_via_with_edge_override.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_segment_action_when_sent_to.rs"
        );
        include!("types/networkmanager/get_device_aws_location.rs");
        include!("types/networkmanager/get_device_location.rs");
        include!("types/networkmanager/get_link_bandwidth.rs");
        include!("types/networkmanager/get_site_location.rs");
    }
    pub mod oam {
        include!("types/oam/link_link_configuration.rs");
        include!("types/oam/link_link_configuration_log_group_configuration.rs");
        include!("types/oam/link_link_configuration_metric_configuration.rs");
        include!("types/oam/get_link_link_configuration.rs");
        include!("types/oam/get_link_link_configuration_log_group_configuration.rs");
        include!("types/oam/get_link_link_configuration_metric_configuration.rs");
    }
    pub mod opensearch {
        include!(
            "types/opensearch/authorize_vpc_endpoint_access_authorized_principal.rs"
        );
        include!("types/opensearch/domain_advanced_security_options.rs");
        include!(
            "types/opensearch/domain_advanced_security_options_master_user_options.rs"
        );
        include!("types/opensearch/domain_auto_tune_options.rs");
        include!("types/opensearch/domain_auto_tune_options_maintenance_schedule.rs");
        include!(
            "types/opensearch/domain_auto_tune_options_maintenance_schedule_duration.rs"
        );
        include!("types/opensearch/domain_cluster_config.rs");
        include!("types/opensearch/domain_cluster_config_cold_storage_options.rs");
        include!("types/opensearch/domain_cluster_config_zone_awareness_config.rs");
        include!("types/opensearch/domain_cognito_options.rs");
        include!("types/opensearch/domain_domain_endpoint_options.rs");
        include!("types/opensearch/domain_ebs_options.rs");
        include!("types/opensearch/domain_encrypt_at_rest.rs");
        include!("types/opensearch/domain_log_publishing_option.rs");
        include!("types/opensearch/domain_node_to_node_encryption.rs");
        include!("types/opensearch/domain_off_peak_window_options.rs");
        include!("types/opensearch/domain_off_peak_window_options_off_peak_window.rs");
        include!(
            "types/opensearch/domain_off_peak_window_options_off_peak_window_window_start_time.rs"
        );
        include!("types/opensearch/domain_saml_options_saml_options.rs");
        include!("types/opensearch/domain_saml_options_saml_options_idp.rs");
        include!("types/opensearch/domain_snapshot_options.rs");
        include!("types/opensearch/domain_software_update_options.rs");
        include!("types/opensearch/domain_vpc_options.rs");
        include!("types/opensearch/outbound_connection_connection_properties.rs");
        include!(
            "types/opensearch/outbound_connection_connection_properties_cross_cluster_search.rs"
        );
        include!("types/opensearch/outbound_connection_local_domain_info.rs");
        include!("types/opensearch/outbound_connection_remote_domain_info.rs");
        include!("types/opensearch/package_package_source.rs");
        include!("types/opensearch/serverless_collection_timeouts.rs");
        include!("types/opensearch/serverless_security_config_saml_options.rs");
        include!("types/opensearch/serverless_vpc_endpoint_timeouts.rs");
        include!("types/opensearch/vpc_endpoint_vpc_options.rs");
        include!("types/opensearch/get_domain_advanced_security_option.rs");
        include!("types/opensearch/get_domain_auto_tune_option.rs");
        include!("types/opensearch/get_domain_auto_tune_option_maintenance_schedule.rs");
        include!(
            "types/opensearch/get_domain_auto_tune_option_maintenance_schedule_duration.rs"
        );
        include!("types/opensearch/get_domain_cluster_config.rs");
        include!("types/opensearch/get_domain_cluster_config_cold_storage_option.rs");
        include!("types/opensearch/get_domain_cluster_config_zone_awareness_config.rs");
        include!("types/opensearch/get_domain_cognito_option.rs");
        include!("types/opensearch/get_domain_ebs_option.rs");
        include!("types/opensearch/get_domain_encryption_at_rest.rs");
        include!("types/opensearch/get_domain_log_publishing_option.rs");
        include!("types/opensearch/get_domain_node_to_node_encryption.rs");
        include!("types/opensearch/get_domain_off_peak_window_options.rs");
        include!(
            "types/opensearch/get_domain_off_peak_window_options_off_peak_window.rs"
        );
        include!(
            "types/opensearch/get_domain_off_peak_window_options_off_peak_window_window_start_time.rs"
        );
        include!("types/opensearch/get_domain_snapshot_option.rs");
        include!("types/opensearch/get_domain_software_update_option.rs");
        include!("types/opensearch/get_domain_vpc_option.rs");
        include!("types/opensearch/get_serverless_security_config_saml_options.rs");
    }
    pub mod opensearchingest {
        include!("types/opensearchingest/pipeline_buffer_options.rs");
        include!("types/opensearchingest/pipeline_encryption_at_rest_options.rs");
        include!("types/opensearchingest/pipeline_log_publishing_options.rs");
        include!(
            "types/opensearchingest/pipeline_log_publishing_options_cloudwatch_log_destination.rs"
        );
        include!("types/opensearchingest/pipeline_timeouts.rs");
        include!("types/opensearchingest/pipeline_vpc_options.rs");
    }
    pub mod opsworks {
        include!("types/opsworks/application_app_source.rs");
        include!("types/opsworks/application_environment.rs");
        include!("types/opsworks/application_ssl_configuration.rs");
        include!("types/opsworks/custom_layer_cloudwatch_configuration.rs");
        include!("types/opsworks/custom_layer_cloudwatch_configuration_log_stream.rs");
        include!("types/opsworks/custom_layer_ebs_volume.rs");
        include!("types/opsworks/custom_layer_load_based_auto_scaling.rs");
        include!("types/opsworks/custom_layer_load_based_auto_scaling_downscaling.rs");
        include!("types/opsworks/custom_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/ecs_cluster_layer_cloudwatch_configuration.rs");
        include!(
            "types/opsworks/ecs_cluster_layer_cloudwatch_configuration_log_stream.rs"
        );
        include!("types/opsworks/ecs_cluster_layer_ebs_volume.rs");
        include!("types/opsworks/ecs_cluster_layer_load_based_auto_scaling.rs");
        include!(
            "types/opsworks/ecs_cluster_layer_load_based_auto_scaling_downscaling.rs"
        );
        include!(
            "types/opsworks/ecs_cluster_layer_load_based_auto_scaling_upscaling.rs"
        );
        include!("types/opsworks/ganglia_layer_cloudwatch_configuration.rs");
        include!("types/opsworks/ganglia_layer_cloudwatch_configuration_log_stream.rs");
        include!("types/opsworks/ganglia_layer_ebs_volume.rs");
        include!("types/opsworks/ganglia_layer_load_based_auto_scaling.rs");
        include!("types/opsworks/ganglia_layer_load_based_auto_scaling_downscaling.rs");
        include!("types/opsworks/ganglia_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/haproxy_layer_cloudwatch_configuration.rs");
        include!("types/opsworks/haproxy_layer_cloudwatch_configuration_log_stream.rs");
        include!("types/opsworks/haproxy_layer_ebs_volume.rs");
        include!("types/opsworks/haproxy_layer_load_based_auto_scaling.rs");
        include!("types/opsworks/haproxy_layer_load_based_auto_scaling_downscaling.rs");
        include!("types/opsworks/haproxy_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/instance_ebs_block_device.rs");
        include!("types/opsworks/instance_ephemeral_block_device.rs");
        include!("types/opsworks/instance_root_block_device.rs");
        include!("types/opsworks/java_app_layer_cloudwatch_configuration.rs");
        include!("types/opsworks/java_app_layer_cloudwatch_configuration_log_stream.rs");
        include!("types/opsworks/java_app_layer_ebs_volume.rs");
        include!("types/opsworks/java_app_layer_load_based_auto_scaling.rs");
        include!("types/opsworks/java_app_layer_load_based_auto_scaling_downscaling.rs");
        include!("types/opsworks/java_app_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/memcached_layer_cloudwatch_configuration.rs");
        include!(
            "types/opsworks/memcached_layer_cloudwatch_configuration_log_stream.rs"
        );
        include!("types/opsworks/memcached_layer_ebs_volume.rs");
        include!("types/opsworks/memcached_layer_load_based_auto_scaling.rs");
        include!(
            "types/opsworks/memcached_layer_load_based_auto_scaling_downscaling.rs"
        );
        include!("types/opsworks/memcached_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/mysql_layer_cloudwatch_configuration.rs");
        include!("types/opsworks/mysql_layer_cloudwatch_configuration_log_stream.rs");
        include!("types/opsworks/mysql_layer_ebs_volume.rs");
        include!("types/opsworks/mysql_layer_load_based_auto_scaling.rs");
        include!("types/opsworks/mysql_layer_load_based_auto_scaling_downscaling.rs");
        include!("types/opsworks/mysql_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/nodejs_app_layer_cloudwatch_configuration.rs");
        include!(
            "types/opsworks/nodejs_app_layer_cloudwatch_configuration_log_stream.rs"
        );
        include!("types/opsworks/nodejs_app_layer_ebs_volume.rs");
        include!("types/opsworks/nodejs_app_layer_load_based_auto_scaling.rs");
        include!(
            "types/opsworks/nodejs_app_layer_load_based_auto_scaling_downscaling.rs"
        );
        include!("types/opsworks/nodejs_app_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/php_app_layer_cloudwatch_configuration.rs");
        include!("types/opsworks/php_app_layer_cloudwatch_configuration_log_stream.rs");
        include!("types/opsworks/php_app_layer_ebs_volume.rs");
        include!("types/opsworks/php_app_layer_load_based_auto_scaling.rs");
        include!("types/opsworks/php_app_layer_load_based_auto_scaling_downscaling.rs");
        include!("types/opsworks/php_app_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/rails_app_layer_cloudwatch_configuration.rs");
        include!(
            "types/opsworks/rails_app_layer_cloudwatch_configuration_log_stream.rs"
        );
        include!("types/opsworks/rails_app_layer_ebs_volume.rs");
        include!("types/opsworks/rails_app_layer_load_based_auto_scaling.rs");
        include!(
            "types/opsworks/rails_app_layer_load_based_auto_scaling_downscaling.rs"
        );
        include!("types/opsworks/rails_app_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/stack_custom_cookbooks_source.rs");
        include!("types/opsworks/static_web_layer_cloudwatch_configuration.rs");
        include!(
            "types/opsworks/static_web_layer_cloudwatch_configuration_log_stream.rs"
        );
        include!("types/opsworks/static_web_layer_ebs_volume.rs");
        include!("types/opsworks/static_web_layer_load_based_auto_scaling.rs");
        include!(
            "types/opsworks/static_web_layer_load_based_auto_scaling_downscaling.rs"
        );
        include!("types/opsworks/static_web_layer_load_based_auto_scaling_upscaling.rs");
    }
    pub mod organizations {
        include!("types/organizations/organization_account.rs");
        include!("types/organizations/organization_non_master_account.rs");
        include!("types/organizations/organization_root.rs");
        include!("types/organizations/organization_root_policy_type.rs");
        include!("types/organizations/organizational_unit_account.rs");
        include!(
            "types/organizations/get_delegated_administrators_delegated_administrator.rs"
        );
        include!("types/organizations/get_delegated_services_delegated_service.rs");
        include!("types/organizations/get_organization_account.rs");
        include!("types/organizations/get_organization_non_master_account.rs");
        include!("types/organizations/get_organization_root.rs");
        include!("types/organizations/get_organization_root_policy_type.rs");
        include!(
            "types/organizations/get_organizational_unit_child_accounts_account.rs"
        );
        include!(
            "types/organizations/get_organizational_unit_descendant_accounts_account.rs"
        );
        include!(
            "types/organizations/get_organizational_unit_descendant_organizational_units_children.rs"
        );
        include!("types/organizations/get_organizational_units_child.rs");
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

    resource register-output {
        extract-field: func(field-name: string) -> output;
    }
}

interface register-interface {
    use pulumi-engine.{engine};
    use output-interface.{output, register-output};

    record object-field {
        name: string,
        value: borrow<output>
    }

    record register-resource-request {
        %type: string,
        name: string,
        version: string,
        object: list<object-field>,
    }

    register: func(engine: borrow<engine>, request: register-resource-request) -> register-output;

    record resource-invoke-request {
        token: string,
        version: string,
        object: list<object-field>,
    }

    invoke: func(engine: borrow<engine>, request: resource-invoke-request) -> register-output;
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
