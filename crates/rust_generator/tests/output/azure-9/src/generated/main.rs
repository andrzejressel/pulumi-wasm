pub mod mysql {
    include!("resources/mysql/flexible_database.rs");
    include!("resources/mysql/flexible_server.rs");
    include!("resources/mysql/flexible_server_active_directory_administratory.rs");
    include!("resources/mysql/flexible_server_configuration.rs");
    include!("resources/mysql/flexible_server_firewall_rule.rs");
}
pub mod netapp {
    include!("resources/netapp/account.rs");
    include!("resources/netapp/account_encryption.rs");
    include!("resources/netapp/backup_policy.rs");
    include!("resources/netapp/backup_vault.rs");
    include!("resources/netapp/pool.rs");
    include!("resources/netapp/snapshot.rs");
    include!("resources/netapp/snapshot_policy.rs");
    include!("resources/netapp/volume.rs");
    include!("resources/netapp/volume_group_sap_hana.rs");
    include!("resources/netapp/volume_quota_rule.rs");
}
pub mod network {
    include!("resources/network/application_gateway.rs");
    include!("resources/network/application_security_group.rs");
    include!("resources/network/bgp_connection.rs");
    include!("resources/network/ddos_protection_plan.rs");
    include!("resources/network/express_route_circuit.rs");
    include!("resources/network/express_route_circuit_authorization.rs");
    include!("resources/network/express_route_circuit_connection.rs");
    include!("resources/network/express_route_circuit_peering.rs");
    include!("resources/network/express_route_connection.rs");
    include!("resources/network/express_route_gateway.rs");
    include!("resources/network/express_route_port.rs");
    include!("resources/network/express_route_port_authorization.rs");
    include!("resources/network/firewall.rs");
    include!("resources/network/firewall_application_rule_collection.rs");
    include!("resources/network/firewall_nat_rule_collection.rs");
    include!("resources/network/firewall_network_rule_collection.rs");
    include!("resources/network/firewall_policy.rs");
    include!("resources/network/firewall_policy_rule_collection_group.rs");
    include!("resources/network/ip_group.rs");
    include!("resources/network/ip_group_cidr.rs");
    include!("resources/network/local_network_gateway.rs");
    include!("resources/network/nat_gateway.rs");
    include!("resources/network/nat_gateway_public_ip_association.rs");
    include!("resources/network/nat_gateway_public_ip_prefix_association.rs");
    include!("resources/network/network_connection_monitor.rs");
    include!("resources/network/network_interface.rs");
    include!(
        "resources/network/network_interface_application_gateway_backend_address_pool_association.rs"
    );
    include!(
        "resources/network/network_interface_application_security_group_association.rs"
    );
    include!("resources/network/network_interface_backend_address_pool_association.rs");
    include!("resources/network/network_interface_nat_rule_association.rs");
    include!("resources/network/network_interface_security_group_association.rs");
    include!("resources/network/network_manager.rs");
    include!("resources/network/network_manager_admin_rule.rs");
    include!("resources/network/network_manager_admin_rule_collection.rs");
    include!("resources/network/network_manager_connectivity_configuration.rs");
    include!("resources/network/network_manager_deployment.rs");
    include!("resources/network/network_manager_management_group_connection.rs");
    include!("resources/network/network_manager_network_group.rs");
    include!("resources/network/network_manager_scope_connection.rs");
    include!("resources/network/network_manager_security_admin_configuration.rs");
    include!("resources/network/network_manager_static_member.rs");
    include!("resources/network/network_manager_subscription_connection.rs");
    include!("resources/network/network_security_group.rs");
    include!("resources/network/network_security_rule.rs");
    include!("resources/network/network_watcher.rs");
    include!("resources/network/network_watcher_flow_log.rs");
    include!("resources/network/point_to_point_vpn_gateway.rs");
    include!("resources/network/profile.rs");
    include!("resources/network/public_ip.rs");
    include!("resources/network/public_ip_prefix.rs");
    include!("resources/network/route.rs");
    include!("resources/network/route_filter.rs");
    include!("resources/network/route_map.rs");
    include!("resources/network/route_server.rs");
    include!("resources/network/route_server_bgp_connection.rs");
    include!("resources/network/route_table.rs");
    include!("resources/network/routing_intent.rs");
    include!("resources/network/security_partner_provider.rs");
    include!("resources/network/subnet.rs");
    include!("resources/network/subnet_nat_gateway_association.rs");
    include!("resources/network/subnet_network_security_group_association.rs");
    include!("resources/network/subnet_route_table_association.rs");
    include!("resources/network/subnet_service_endpoint_storage_policy.rs");
    include!("resources/network/traffic_manager_azure_endpoint.rs");
    include!("resources/network/traffic_manager_external_endpoint.rs");
    include!("resources/network/traffic_manager_nested_endpoint.rs");
    include!("resources/network/traffic_manager_profile.rs");
    include!("resources/network/virtual_hub.rs");
    include!("resources/network/virtual_hub_connection.rs");
    include!("resources/network/virtual_hub_ip.rs");
    include!("resources/network/virtual_hub_route_table.rs");
    include!("resources/network/virtual_hub_route_table_route.rs");
    include!("resources/network/virtual_network.rs");
    include!("resources/network/virtual_network_dns_servers.rs");
    include!("resources/network/virtual_network_gateway.rs");
    include!("resources/network/virtual_network_gateway_connection.rs");
    include!("resources/network/virtual_network_gateway_nat_rule.rs");
    include!("resources/network/virtual_network_peering.rs");
    include!("resources/network/virtual_wan.rs");
    include!("resources/network/vnp_gateway_nat_rule.rs");
    include!("resources/network/vpn_gateway.rs");
    include!("resources/network/vpn_gateway_connection.rs");
    include!("resources/network/vpn_server_configuration.rs");
    include!("resources/network/vpn_server_configuration_policy_group.rs");
    include!("resources/network/vpn_site.rs");
}
pub mod networkfunction {
    include!("resources/networkfunction/azure_traffic_collector.rs");
    include!("resources/networkfunction/collector_policy.rs");
}
pub mod newrelic {
    include!("resources/newrelic/monitor.rs");
    include!("resources/newrelic/tag_rule.rs");
}
pub mod nginx {
    include!("resources/nginx/certificate.rs");
    include!("resources/nginx/configuration.rs");
    include!("resources/nginx/deployment.rs");
}
pub mod notificationhub {
    include!("resources/notificationhub/authorization_rule.rs");
    include!("resources/notificationhub/hub.rs");
    include!("resources/notificationhub/namespace.rs");
}
pub mod operationalinsights {
    include!("resources/operationalinsights/analytics_solution.rs");
    include!("resources/operationalinsights/analytics_workspace.rs");
    include!("resources/operationalinsights/query_pack_query.rs");
}
pub mod oracle {
    include!("resources/oracle/autonomous_database.rs");
    include!("resources/oracle/cloud_vm_cluster.rs");
    include!("resources/oracle/exadata_infrastructure.rs");
}
pub mod orbital {
    include!("resources/orbital/contact.rs");
    include!("resources/orbital/contact_profile.rs");
    include!("resources/orbital/spacecraft.rs");
}
pub mod functions {
    pub mod mysql {
        include!("functions/mysql/get_flexible_server.rs");
    }
    pub mod netapp {
        include!("functions/netapp/get_account.rs");
        include!("functions/netapp/get_account_encryption.rs");
        include!("functions/netapp/get_backup_policy.rs");
        include!("functions/netapp/get_backup_vault.rs");
        include!("functions/netapp/get_pool.rs");
        include!("functions/netapp/get_snapshot.rs");
        include!("functions/netapp/get_snapshot_policy.rs");
        include!("functions/netapp/get_volume.rs");
        include!("functions/netapp/get_volume_group_sap_hana.rs");
        include!("functions/netapp/get_volume_quota_rule.rs");
    }
    pub mod network {
        include!("functions/network/get_application_gateway.rs");
        include!("functions/network/get_application_security_group.rs");
        include!("functions/network/get_express_route_circuit.rs");
        include!("functions/network/get_firewall.rs");
        include!("functions/network/get_firewall_policy.rs");
        include!("functions/network/get_gateway_connection.rs");
        include!("functions/network/get_ip_group.rs");
        include!("functions/network/get_ip_groups.rs");
        include!("functions/network/get_local_network_gateway.rs");
        include!("functions/network/get_nat_gateway.rs");
        include!("functions/network/get_network_ddos_protection_plan.rs");
        include!("functions/network/get_network_interface.rs");
        include!("functions/network/get_network_manager.rs");
        include!("functions/network/get_network_manager_connectivity_configuration.rs");
        include!("functions/network/get_network_manager_network_group.rs");
        include!("functions/network/get_network_security_group.rs");
        include!("functions/network/get_network_watcher.rs");
        include!("functions/network/get_public_ip.rs");
        include!("functions/network/get_public_i_ps.rs");
        include!("functions/network/get_public_ip_prefix.rs");
        include!("functions/network/get_route_filter.rs");
        include!("functions/network/get_route_table.rs");
        include!("functions/network/get_service_tags.rs");
        include!("functions/network/get_subnet.rs");
        include!("functions/network/get_traffic_manager.rs");
        include!("functions/network/get_traffic_manager_profile.rs");
        include!("functions/network/get_virtual_hub.rs");
        include!("functions/network/get_virtual_hub_connection.rs");
        include!("functions/network/get_virtual_hub_route_table.rs");
        include!("functions/network/get_virtual_network.rs");
        include!("functions/network/get_virtual_network_gateway.rs");
        include!("functions/network/get_virtual_network_peering.rs");
        include!("functions/network/get_virtual_wan.rs");
        include!("functions/network/get_vpn_gateway.rs");
        include!("functions/network/get_vpn_server_configuration.rs");
    }
    pub mod nginx {
        include!("functions/nginx/get_certificate.rs");
        include!("functions/nginx/get_configuration.rs");
        include!("functions/nginx/get_deployment.rs");
    }
    pub mod notificationhub {
        include!("functions/notificationhub/get_hub.rs");
        include!("functions/notificationhub/get_namespace.rs");
    }
    pub mod operationalinsights {
        include!("functions/operationalinsights/get_analytics_workspace.rs");
    }
    pub mod oracle {
        include!("functions/oracle/get_adbs_character_sets.rs");
        include!("functions/oracle/get_adbs_national_character_sets.rs");
        include!("functions/oracle/get_autonomous_database.rs");
        include!("functions/oracle/get_cloud_vm_cluster.rs");
        include!("functions/oracle/get_db_nodes.rs");
        include!("functions/oracle/get_db_servers.rs");
        include!("functions/oracle/get_db_system_shapes.rs");
        include!("functions/oracle/get_exadata_infrastructure.rs");
        include!("functions/oracle/get_gi_versions.rs");
    }
}
pub mod types {
    pub mod mysql {
        include!("types/mysql/flexible_server_customer_managed_key.rs");
        include!("types/mysql/flexible_server_high_availability.rs");
        include!("types/mysql/flexible_server_identity.rs");
        include!("types/mysql/flexible_server_maintenance_window.rs");
        include!("types/mysql/flexible_server_storage.rs");
        include!("types/mysql/get_flexible_server_high_availability.rs");
        include!("types/mysql/get_flexible_server_maintenance_window.rs");
        include!("types/mysql/get_flexible_server_storage.rs");
    }
    pub mod netapp {
        include!("types/netapp/account_active_directory.rs");
        include!("types/netapp/account_identity.rs");
        include!("types/netapp/snapshot_policy_daily_schedule.rs");
        include!("types/netapp/snapshot_policy_hourly_schedule.rs");
        include!("types/netapp/snapshot_policy_monthly_schedule.rs");
        include!("types/netapp/snapshot_policy_weekly_schedule.rs");
        include!("types/netapp/volume_data_protection_backup_policy.rs");
        include!("types/netapp/volume_data_protection_replication.rs");
        include!("types/netapp/volume_data_protection_snapshot_policy.rs");
        include!("types/netapp/volume_export_policy_rule.rs");
        include!("types/netapp/volume_group_sap_hana_volume.rs");
        include!(
            "types/netapp/volume_group_sap_hana_volume_data_protection_replication.rs"
        );
        include!(
            "types/netapp/volume_group_sap_hana_volume_data_protection_snapshot_policy.rs"
        );
        include!("types/netapp/volume_group_sap_hana_volume_export_policy_rule.rs");
        include!("types/netapp/get_account_identity.rs");
        include!("types/netapp/get_snapshot_policy_daily_schedule.rs");
        include!("types/netapp/get_snapshot_policy_hourly_schedule.rs");
        include!("types/netapp/get_snapshot_policy_monthly_schedule.rs");
        include!("types/netapp/get_snapshot_policy_weekly_schedule.rs");
        include!("types/netapp/get_volume_data_protection_backup_policy.rs");
        include!("types/netapp/get_volume_data_protection_replication.rs");
        include!("types/netapp/get_volume_group_sap_hana_volume.rs");
        include!(
            "types/netapp/get_volume_group_sap_hana_volume_data_protection_replication.rs"
        );
        include!(
            "types/netapp/get_volume_group_sap_hana_volume_data_protection_snapshot_policy.rs"
        );
        include!("types/netapp/get_volume_group_sap_hana_volume_export_policy_rule.rs");
    }
    pub mod network {
        include!("types/network/application_gateway_authentication_certificate.rs");
        include!("types/network/application_gateway_autoscale_configuration.rs");
        include!("types/network/application_gateway_backend_address_pool.rs");
        include!("types/network/application_gateway_backend_http_setting.rs");
        include!(
            "types/network/application_gateway_backend_http_setting_authentication_certificate.rs"
        );
        include!(
            "types/network/application_gateway_backend_http_setting_connection_draining.rs"
        );
        include!("types/network/application_gateway_custom_error_configuration.rs");
        include!("types/network/application_gateway_frontend_ip_configuration.rs");
        include!("types/network/application_gateway_frontend_port.rs");
        include!("types/network/application_gateway_gateway_ip_configuration.rs");
        include!("types/network/application_gateway_global.rs");
        include!("types/network/application_gateway_http_listener.rs");
        include!(
            "types/network/application_gateway_http_listener_custom_error_configuration.rs"
        );
        include!("types/network/application_gateway_identity.rs");
        include!("types/network/application_gateway_private_endpoint_connection.rs");
        include!("types/network/application_gateway_private_link_configuration.rs");
        include!(
            "types/network/application_gateway_private_link_configuration_ip_configuration.rs"
        );
        include!("types/network/application_gateway_probe.rs");
        include!("types/network/application_gateway_probe_match.rs");
        include!("types/network/application_gateway_redirect_configuration.rs");
        include!("types/network/application_gateway_request_routing_rule.rs");
        include!("types/network/application_gateway_rewrite_rule_set.rs");
        include!("types/network/application_gateway_rewrite_rule_set_rewrite_rule.rs");
        include!(
            "types/network/application_gateway_rewrite_rule_set_rewrite_rule_condition.rs"
        );
        include!(
            "types/network/application_gateway_rewrite_rule_set_rewrite_rule_request_header_configuration.rs"
        );
        include!(
            "types/network/application_gateway_rewrite_rule_set_rewrite_rule_response_header_configuration.rs"
        );
        include!(
            "types/network/application_gateway_rewrite_rule_set_rewrite_rule_url.rs"
        );
        include!("types/network/application_gateway_sku.rs");
        include!("types/network/application_gateway_ssl_certificate.rs");
        include!("types/network/application_gateway_ssl_policy.rs");
        include!("types/network/application_gateway_ssl_profile.rs");
        include!("types/network/application_gateway_ssl_profile_ssl_policy.rs");
        include!("types/network/application_gateway_trusted_client_certificate.rs");
        include!("types/network/application_gateway_trusted_root_certificate.rs");
        include!("types/network/application_gateway_url_path_map.rs");
        include!("types/network/application_gateway_url_path_map_path_rule.rs");
        include!("types/network/application_gateway_waf_configuration.rs");
        include!(
            "types/network/application_gateway_waf_configuration_disabled_rule_group.rs"
        );
        include!("types/network/application_gateway_waf_configuration_exclusion.rs");
        include!("types/network/express_route_circuit_peering_ipv_6.rs");
        include!(
            "types/network/express_route_circuit_peering_ipv_6_microsoft_peering.rs"
        );
        include!(
            "types/network/express_route_circuit_peering_microsoft_peering_config.rs"
        );
        include!("types/network/express_route_circuit_sku.rs");
        include!("types/network/express_route_connection_routing.rs");
        include!(
            "types/network/express_route_connection_routing_propagated_route_table.rs"
        );
        include!("types/network/express_route_port_identity.rs");
        include!("types/network/express_route_port_link_1.rs");
        include!("types/network/express_route_port_link_2.rs");
        include!("types/network/firewall_application_rule_collection_rule.rs");
        include!("types/network/firewall_application_rule_collection_rule_protocol.rs");
        include!("types/network/firewall_ip_configuration.rs");
        include!("types/network/firewall_management_ip_configuration.rs");
        include!("types/network/firewall_nat_rule_collection_rule.rs");
        include!("types/network/firewall_network_rule_collection_rule.rs");
        include!("types/network/firewall_policy_dns.rs");
        include!("types/network/firewall_policy_explicit_proxy.rs");
        include!("types/network/firewall_policy_identity.rs");
        include!("types/network/firewall_policy_insights.rs");
        include!("types/network/firewall_policy_insights_log_analytics_workspace.rs");
        include!("types/network/firewall_policy_intrusion_detection.rs");
        include!(
            "types/network/firewall_policy_intrusion_detection_signature_override.rs"
        );
        include!("types/network/firewall_policy_intrusion_detection_traffic_bypass.rs");
        include!(
            "types/network/firewall_policy_rule_collection_group_application_rule_collection.rs"
        );
        include!(
            "types/network/firewall_policy_rule_collection_group_application_rule_collection_rule.rs"
        );
        include!(
            "types/network/firewall_policy_rule_collection_group_application_rule_collection_rule_http_header.rs"
        );
        include!(
            "types/network/firewall_policy_rule_collection_group_application_rule_collection_rule_protocol.rs"
        );
        include!(
            "types/network/firewall_policy_rule_collection_group_nat_rule_collection.rs"
        );
        include!(
            "types/network/firewall_policy_rule_collection_group_nat_rule_collection_rule.rs"
        );
        include!(
            "types/network/firewall_policy_rule_collection_group_network_rule_collection.rs"
        );
        include!(
            "types/network/firewall_policy_rule_collection_group_network_rule_collection_rule.rs"
        );
        include!("types/network/firewall_policy_threat_intelligence_allowlist.rs");
        include!("types/network/firewall_policy_tls_certificate.rs");
        include!("types/network/firewall_virtual_hub.rs");
        include!("types/network/local_network_gateway_bgp_settings.rs");
        include!("types/network/network_connection_monitor_endpoint.rs");
        include!("types/network/network_connection_monitor_endpoint_filter.rs");
        include!("types/network/network_connection_monitor_endpoint_filter_item.rs");
        include!("types/network/network_connection_monitor_test_configuration.rs");
        include!(
            "types/network/network_connection_monitor_test_configuration_http_configuration.rs"
        );
        include!(
            "types/network/network_connection_monitor_test_configuration_http_configuration_request_header.rs"
        );
        include!(
            "types/network/network_connection_monitor_test_configuration_icmp_configuration.rs"
        );
        include!(
            "types/network/network_connection_monitor_test_configuration_success_threshold.rs"
        );
        include!(
            "types/network/network_connection_monitor_test_configuration_tcp_configuration.rs"
        );
        include!("types/network/network_connection_monitor_test_group.rs");
        include!("types/network/network_interface_ip_configuration.rs");
        include!("types/network/network_manager_admin_rule_destination.rs");
        include!("types/network/network_manager_admin_rule_source.rs");
        include!(
            "types/network/network_manager_connectivity_configuration_applies_to_group.rs"
        );
        include!("types/network/network_manager_connectivity_configuration_hub.rs");
        include!("types/network/network_manager_cross_tenant_scope.rs");
        include!("types/network/network_manager_scope.rs");
        include!("types/network/network_security_group_security_rule.rs");
        include!("types/network/network_watcher_flow_log_retention_policy.rs");
        include!("types/network/network_watcher_flow_log_traffic_analytics.rs");
        include!("types/network/point_to_point_vpn_gateway_connection_configuration.rs");
        include!(
            "types/network/point_to_point_vpn_gateway_connection_configuration_route.rs"
        );
        include!(
            "types/network/point_to_point_vpn_gateway_connection_configuration_route_propagated_route_table.rs"
        );
        include!(
            "types/network/point_to_point_vpn_gateway_connection_configuration_vpn_client_address_pool.rs"
        );
        include!("types/network/profile_container_network_interface.rs");
        include!(
            "types/network/profile_container_network_interface_ip_configuration.rs"
        );
        include!("types/network/route_filter_rule.rs");
        include!("types/network/route_map_rule.rs");
        include!("types/network/route_map_rule_action.rs");
        include!("types/network/route_map_rule_action_parameter.rs");
        include!("types/network/route_map_rule_match_criterion.rs");
        include!("types/network/route_table_route.rs");
        include!("types/network/routing_intent_routing_policy.rs");
        include!("types/network/subnet_delegation.rs");
        include!("types/network/subnet_delegation_service_delegation.rs");
        include!("types/network/subnet_service_endpoint_storage_policy_definition.rs");
        include!("types/network/traffic_manager_azure_endpoint_custom_header.rs");
        include!("types/network/traffic_manager_azure_endpoint_subnet.rs");
        include!("types/network/traffic_manager_external_endpoint_custom_header.rs");
        include!("types/network/traffic_manager_external_endpoint_subnet.rs");
        include!("types/network/traffic_manager_nested_endpoint_custom_header.rs");
        include!("types/network/traffic_manager_nested_endpoint_subnet.rs");
        include!("types/network/traffic_manager_profile_dns_config.rs");
        include!("types/network/traffic_manager_profile_monitor_config.rs");
        include!(
            "types/network/traffic_manager_profile_monitor_config_custom_header.rs"
        );
        include!("types/network/virtual_hub_connection_routing.rs");
        include!(
            "types/network/virtual_hub_connection_routing_propagated_route_table.rs"
        );
        include!("types/network/virtual_hub_connection_routing_static_vnet_route.rs");
        include!("types/network/virtual_hub_route.rs");
        include!("types/network/virtual_hub_route_table_route.rs");
        include!("types/network/virtual_network_ddos_protection_plan.rs");
        include!("types/network/virtual_network_encryption.rs");
        include!("types/network/virtual_network_gateway_bgp_settings.rs");
        include!(
            "types/network/virtual_network_gateway_bgp_settings_peering_address.rs"
        );
        include!(
            "types/network/virtual_network_gateway_connection_custom_bgp_addresses.rs"
        );
        include!("types/network/virtual_network_gateway_connection_ipsec_policy.rs");
        include!(
            "types/network/virtual_network_gateway_connection_traffic_selector_policy.rs"
        );
        include!("types/network/virtual_network_gateway_custom_route.rs");
        include!("types/network/virtual_network_gateway_ip_configuration.rs");
        include!("types/network/virtual_network_gateway_nat_rule_external_mapping.rs");
        include!("types/network/virtual_network_gateway_nat_rule_internal_mapping.rs");
        include!("types/network/virtual_network_gateway_policy_group.rs");
        include!("types/network/virtual_network_gateway_policy_group_policy_member.rs");
        include!("types/network/virtual_network_gateway_vpn_client_configuration.rs");
        include!(
            "types/network/virtual_network_gateway_vpn_client_configuration_ipsec_policy.rs"
        );
        include!(
            "types/network/virtual_network_gateway_vpn_client_configuration_radius_server.rs"
        );
        include!(
            "types/network/virtual_network_gateway_vpn_client_configuration_revoked_certificate.rs"
        );
        include!(
            "types/network/virtual_network_gateway_vpn_client_configuration_root_certificate.rs"
        );
        include!(
            "types/network/virtual_network_gateway_vpn_client_configuration_virtual_network_gateway_client_connection.rs"
        );
        include!("types/network/virtual_network_subnet.rs");
        include!("types/network/virtual_network_subnet_delegation.rs");
        include!(
            "types/network/virtual_network_subnet_delegation_service_delegation.rs"
        );
        include!("types/network/vnp_gateway_nat_rule_external_mapping.rs");
        include!("types/network/vnp_gateway_nat_rule_internal_mapping.rs");
        include!("types/network/vpn_gateway_bgp_settings.rs");
        include!(
            "types/network/vpn_gateway_bgp_settings_instance_0_bgp_peering_address.rs"
        );
        include!(
            "types/network/vpn_gateway_bgp_settings_instance_1_bgp_peering_address.rs"
        );
        include!("types/network/vpn_gateway_connection_routing.rs");
        include!(
            "types/network/vpn_gateway_connection_routing_propagated_route_table.rs"
        );
        include!("types/network/vpn_gateway_connection_traffic_selector_policy.rs");
        include!("types/network/vpn_gateway_connection_vpn_link.rs");
        include!("types/network/vpn_gateway_connection_vpn_link_custom_bgp_address.rs");
        include!("types/network/vpn_gateway_connection_vpn_link_ipsec_policy.rs");
        include!(
            "types/network/vpn_server_configuration_azure_active_directory_authentication.rs"
        );
        include!("types/network/vpn_server_configuration_client_revoked_certificate.rs");
        include!("types/network/vpn_server_configuration_client_root_certificate.rs");
        include!("types/network/vpn_server_configuration_ipsec_policy.rs");
        include!("types/network/vpn_server_configuration_policy_group_policy.rs");
        include!("types/network/vpn_server_configuration_radius.rs");
        include!(
            "types/network/vpn_server_configuration_radius_client_root_certificate.rs"
        );
        include!("types/network/vpn_server_configuration_radius_server.rs");
        include!(
            "types/network/vpn_server_configuration_radius_server_root_certificate.rs"
        );
        include!("types/network/vpn_site_link.rs");
        include!("types/network/vpn_site_link_bgp.rs");
        include!("types/network/vpn_site_o_365_policy.rs");
        include!("types/network/vpn_site_o_365_policy_traffic_category.rs");
        include!("types/network/get_application_gateway_authentication_certificate.rs");
        include!("types/network/get_application_gateway_autoscale_configuration.rs");
        include!("types/network/get_application_gateway_backend_address_pool.rs");
        include!("types/network/get_application_gateway_backend_http_setting.rs");
        include!(
            "types/network/get_application_gateway_backend_http_setting_authentication_certificate.rs"
        );
        include!(
            "types/network/get_application_gateway_backend_http_setting_connection_draining.rs"
        );
        include!("types/network/get_application_gateway_custom_error_configuration.rs");
        include!("types/network/get_application_gateway_frontend_ip_configuration.rs");
        include!("types/network/get_application_gateway_frontend_port.rs");
        include!("types/network/get_application_gateway_gateway_ip_configuration.rs");
        include!("types/network/get_application_gateway_global.rs");
        include!("types/network/get_application_gateway_http_listener.rs");
        include!(
            "types/network/get_application_gateway_http_listener_custom_error_configuration.rs"
        );
        include!("types/network/get_application_gateway_identity.rs");
        include!("types/network/get_application_gateway_private_endpoint_connection.rs");
        include!("types/network/get_application_gateway_private_link_configuration.rs");
        include!(
            "types/network/get_application_gateway_private_link_configuration_ip_configuration.rs"
        );
        include!("types/network/get_application_gateway_probe.rs");
        include!("types/network/get_application_gateway_probe_match.rs");
        include!("types/network/get_application_gateway_redirect_configuration.rs");
        include!("types/network/get_application_gateway_request_routing_rule.rs");
        include!("types/network/get_application_gateway_rewrite_rule_set.rs");
        include!(
            "types/network/get_application_gateway_rewrite_rule_set_rewrite_rule.rs"
        );
        include!(
            "types/network/get_application_gateway_rewrite_rule_set_rewrite_rule_condition.rs"
        );
        include!(
            "types/network/get_application_gateway_rewrite_rule_set_rewrite_rule_request_header_configuration.rs"
        );
        include!(
            "types/network/get_application_gateway_rewrite_rule_set_rewrite_rule_response_header_configuration.rs"
        );
        include!(
            "types/network/get_application_gateway_rewrite_rule_set_rewrite_rule_url.rs"
        );
        include!("types/network/get_application_gateway_skus.rs");
        include!("types/network/get_application_gateway_ssl_certificate.rs");
        include!("types/network/get_application_gateway_ssl_policy.rs");
        include!("types/network/get_application_gateway_ssl_profile.rs");
        include!("types/network/get_application_gateway_ssl_profile_ssl_policy.rs");
        include!("types/network/get_application_gateway_trusted_client_certificate.rs");
        include!("types/network/get_application_gateway_trusted_root_certificate.rs");
        include!("types/network/get_application_gateway_url_path_map.rs");
        include!("types/network/get_application_gateway_url_path_map_path_rule.rs");
        include!("types/network/get_application_gateway_waf_configuration.rs");
        include!(
            "types/network/get_application_gateway_waf_configuration_disabled_rule_group.rs"
        );
        include!("types/network/get_application_gateway_waf_configuration_exclusion.rs");
        include!("types/network/get_express_route_circuit_peering.rs");
        include!("types/network/get_express_route_circuit_service_provider_property.rs");
        include!("types/network/get_express_route_circuit_sku.rs");
        include!("types/network/get_firewall_ip_configuration.rs");
        include!("types/network/get_firewall_management_ip_configuration.rs");
        include!("types/network/get_firewall_policy_dn.rs");
        include!("types/network/get_firewall_policy_threat_intelligence_allowlist.rs");
        include!("types/network/get_firewall_virtual_hub.rs");
        include!("types/network/get_gateway_connection_ipsec_policy.rs");
        include!("types/network/get_gateway_connection_traffic_selector_policy.rs");
        include!("types/network/get_local_network_gateway_bgp_setting.rs");
        include!("types/network/get_network_interface_ip_configuration.rs");
        include!(
            "types/network/get_network_manager_connectivity_configuration_applies_to_group.rs"
        );
        include!("types/network/get_network_manager_connectivity_configuration_hub.rs");
        include!("types/network/get_network_manager_cross_tenant_scope.rs");
        include!("types/network/get_network_manager_scope.rs");
        include!("types/network/get_network_security_group_security_rule.rs");
        include!("types/network/get_public_i_ps_public_ip.rs");
        include!("types/network/get_route_filter_rule.rs");
        include!("types/network/get_route_table_route.rs");
        include!("types/network/get_traffic_manager_profile_dns_config.rs");
        include!("types/network/get_traffic_manager_profile_monitor_config.rs");
        include!(
            "types/network/get_traffic_manager_profile_monitor_config_custom_header.rs"
        );
        include!("types/network/get_virtual_hub_connection_routing.rs");
        include!(
            "types/network/get_virtual_hub_connection_routing_propagated_route_table.rs"
        );
        include!(
            "types/network/get_virtual_hub_connection_routing_static_vnet_route.rs"
        );
        include!("types/network/get_virtual_hub_route_table_route.rs");
        include!("types/network/get_virtual_network_gateway_bgp_setting.rs");
        include!("types/network/get_virtual_network_gateway_custom_route.rs");
        include!("types/network/get_virtual_network_gateway_ip_configuration.rs");
        include!(
            "types/network/get_virtual_network_gateway_vpn_client_configuration.rs"
        );
        include!(
            "types/network/get_virtual_network_gateway_vpn_client_configuration_revoked_certificate.rs"
        );
        include!(
            "types/network/get_virtual_network_gateway_vpn_client_configuration_root_certificate.rs"
        );
        include!("types/network/get_vpn_gateway_bgp_setting.rs");
        include!(
            "types/network/get_vpn_gateway_bgp_setting_instance_0_bgp_peering_address.rs"
        );
        include!(
            "types/network/get_vpn_gateway_bgp_setting_instance_1_bgp_peering_address.rs"
        );
        include!(
            "types/network/get_vpn_server_configuration_azure_active_directory_authentication.rs"
        );
        include!(
            "types/network/get_vpn_server_configuration_client_revoked_certificate.rs"
        );
        include!(
            "types/network/get_vpn_server_configuration_client_root_certificate.rs"
        );
        include!("types/network/get_vpn_server_configuration_ipsec_policy.rs");
        include!("types/network/get_vpn_server_configuration_radius.rs");
        include!(
            "types/network/get_vpn_server_configuration_radius_client_root_certificate.rs"
        );
        include!("types/network/get_vpn_server_configuration_radius_server.rs");
        include!(
            "types/network/get_vpn_server_configuration_radius_server_root_certificate.rs"
        );
    }
    pub mod networkfunction {
        include!("types/networkfunction/collector_policy_ipfx_emission.rs");
        include!("types/networkfunction/collector_policy_ipfx_ingestion.rs");
    }
    pub mod newrelic {
        include!("types/newrelic/monitor_identity.rs");
        include!("types/newrelic/monitor_plan.rs");
        include!("types/newrelic/monitor_user.rs");
        include!("types/newrelic/tag_rule_log_tag_filter.rs");
        include!("types/newrelic/tag_rule_metric_tag_filter.rs");
    }
    pub mod nginx {
        include!("types/nginx/configuration_config_file.rs");
        include!("types/nginx/configuration_protected_file.rs");
        include!("types/nginx/deployment_auto_scale_profile.rs");
        include!("types/nginx/deployment_frontend_private.rs");
        include!("types/nginx/deployment_frontend_public.rs");
        include!("types/nginx/deployment_identity.rs");
        include!("types/nginx/deployment_logging_storage_account.rs");
        include!("types/nginx/deployment_network_interface.rs");
        include!("types/nginx/get_configuration_config_file.rs");
        include!("types/nginx/get_configuration_protected_file.rs");
        include!("types/nginx/get_deployment_auto_scale_profile.rs");
        include!("types/nginx/get_deployment_frontend_private.rs");
        include!("types/nginx/get_deployment_frontend_public.rs");
        include!("types/nginx/get_deployment_identity.rs");
        include!("types/nginx/get_deployment_logging_storage_account.rs");
        include!("types/nginx/get_deployment_network_interface.rs");
    }
    pub mod notificationhub {
        include!("types/notificationhub/hub_apns_credential.rs");
        include!("types/notificationhub/hub_browser_credential.rs");
        include!("types/notificationhub/hub_gcm_credential.rs");
        include!("types/notificationhub/get_hub_apns_credential.rs");
        include!("types/notificationhub/get_hub_gcm_credential.rs");
        include!("types/notificationhub/get_namespace_sku.rs");
    }
    pub mod operationalinsights {
        include!("types/operationalinsights/analytics_solution_plan.rs");
        include!("types/operationalinsights/analytics_workspace_identity.rs");
    }
    pub mod oracle {
        include!("types/oracle/cloud_vm_cluster_data_collection_options.rs");
        include!("types/oracle/exadata_infrastructure_maintenance_window.rs");
        include!("types/oracle/get_adbs_character_sets_character_set.rs");
        include!("types/oracle/get_adbs_national_character_sets_character_set.rs");
        include!("types/oracle/get_cloud_vm_cluster_data_collection_option.rs");
        include!("types/oracle/get_cloud_vm_cluster_iorm_config_cach.rs");
        include!("types/oracle/get_cloud_vm_cluster_iorm_config_cach_db_plan.rs");
        include!("types/oracle/get_db_nodes_db_node.rs");
        include!("types/oracle/get_db_servers_db_server.rs");
        include!("types/oracle/get_db_system_shapes_db_system_shape.rs");
        include!("types/oracle/get_exadata_infrastructure_estimated_patching_time.rs");
        include!("types/oracle/get_exadata_infrastructure_maintenance_window.rs");
    }
    pub mod orbital {
        include!("types/orbital/contact_profile_link.rs");
        include!("types/orbital/contact_profile_link_channel.rs");
        include!("types/orbital/contact_profile_link_channel_end_point.rs");
        include!("types/orbital/spacecraft_link.rs");
    }
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_gestalt_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-gestalt@0.0.0-DEV;

world world-azure {
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
        clone: func() -> output;
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
        with : { "component:pulumi-gestalt/output-interface@0.0.0-DEV" :
        pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::output_interface
        } }
    );
}
#[link_section = "pulumi_gestalt_provider::azure"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_AZURE: [u8; 45] = *b"{\"version\":\"6.14.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "6.14.0".to_string()
}
