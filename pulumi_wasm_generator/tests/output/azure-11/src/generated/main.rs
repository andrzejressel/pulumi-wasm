pub mod relay {
    include!("resources/relay/hybrid_connection.rs");
    include!("resources/relay/hybrid_connection_authorization_rule.rs");
    include!("resources/relay/namespace.rs");
    include!("resources/relay/namespace_authorization_rule.rs");
}
pub mod role {
    include!("resources/role/assignment.rs");
    include!("resources/role/definition.rs");
}
pub mod search {
    include!("resources/search/service.rs");
    include!("resources/search/shared_private_link_service.rs");
}
pub mod securitycenter {
    include!("resources/securitycenter/advanced_threat_protection.rs");
    include!("resources/securitycenter/assessment.rs");
    include!("resources/securitycenter/assessment_policy.rs");
    include!("resources/securitycenter/auto_provisioning.rs");
    include!("resources/securitycenter/automation.rs");
    include!("resources/securitycenter/contact.rs");
    include!(
        "resources/securitycenter/server_vulnerability_assessment_virtual_machine.rs"
    );
    include!("resources/securitycenter/server_vulnerability_assessments_setting.rs");
    include!("resources/securitycenter/setting.rs");
    include!("resources/securitycenter/storage_defender.rs");
    include!("resources/securitycenter/subscription_pricing.rs");
    include!("resources/securitycenter/workspace.rs");
}
pub mod sentinel {
    include!("resources/sentinel/alert_rule_anomaly_built_in.rs");
    include!("resources/sentinel/alert_rule_anomaly_duplicate.rs");
    include!("resources/sentinel/alert_rule_fusion.rs");
    include!("resources/sentinel/alert_rule_machine_learning_behavior_analytics.rs");
    include!("resources/sentinel/alert_rule_ms_security_incident.rs");
    include!("resources/sentinel/alert_rule_nrt.rs");
    include!("resources/sentinel/alert_rule_scheduled.rs");
    include!("resources/sentinel/alert_rule_threat_intelligence.rs");
    include!("resources/sentinel/authomation_rule.rs");
    include!("resources/sentinel/automation_rule.rs");
    include!("resources/sentinel/data_connector_aws_cloud_trail.rs");
    include!("resources/sentinel/data_connector_aws_s_3.rs");
    include!("resources/sentinel/data_connector_azure_active_directory.rs");
    include!("resources/sentinel/data_connector_azure_advanced_thread_protection.rs");
    include!("resources/sentinel/data_connector_azure_security_center.rs");
    include!("resources/sentinel/data_connector_dynamics_365.rs");
    include!("resources/sentinel/data_connector_iot.rs");
    include!("resources/sentinel/data_connector_microsoft_cloud_app_security.rs");
    include!(
        "resources/sentinel/data_connector_microsoft_defender_advanced_threat_protection.rs"
    );
    include!("resources/sentinel/data_connector_microsoft_threat_intelligence.rs");
    include!("resources/sentinel/data_connector_microsoft_threat_protection.rs");
    include!("resources/sentinel/data_connector_office_365.rs");
    include!("resources/sentinel/data_connector_office_365_project.rs");
    include!("resources/sentinel/data_connector_office_atp.rs");
    include!("resources/sentinel/data_connector_office_irm.rs");
    include!("resources/sentinel/data_connector_office_power_bi.rs");
    include!("resources/sentinel/data_connector_threat_intelligence.rs");
    include!("resources/sentinel/data_connector_threat_intelligence_taxii.rs");
    include!("resources/sentinel/log_analytics_workspace_onboarding.rs");
    include!("resources/sentinel/metadata.rs");
    include!("resources/sentinel/threat_intelligence_indicator.rs");
    include!("resources/sentinel/watchlist.rs");
    include!("resources/sentinel/watchlist_item.rs");
}
pub mod servicebus {
    include!("resources/servicebus/namespace.rs");
    include!("resources/servicebus/namespace_authorization_rule.rs");
    include!("resources/servicebus/namespace_disaster_recovery_config.rs");
    include!("resources/servicebus/queue.rs");
    include!("resources/servicebus/queue_authorization_rule.rs");
    include!("resources/servicebus/subscription.rs");
    include!("resources/servicebus/subscription_rule.rs");
    include!("resources/servicebus/topic.rs");
    include!("resources/servicebus/topic_authorization_rule.rs");
}
pub mod servicefabric {
    include!("resources/servicefabric/cluster.rs");
    include!("resources/servicefabric/managed_cluster.rs");
}
pub mod signalr {
    include!("resources/signalr/service.rs");
    include!("resources/signalr/service_custom_certificate.rs");
    include!("resources/signalr/service_custom_domain.rs");
    include!("resources/signalr/service_network_acl.rs");
    include!("resources/signalr/shared_private_link_resource.rs");
}
pub mod siterecovery {
    include!("resources/siterecovery/fabric.rs");
    include!("resources/siterecovery/hyper_v_replication_policy.rs");
    include!("resources/siterecovery/hyper_v_replication_policy_association.rs");
    include!("resources/siterecovery/hyper_v_site.rs");
    include!("resources/siterecovery/hyperv_network_mapping.rs");
    include!("resources/siterecovery/network_mapping.rs");
    include!("resources/siterecovery/protection_container.rs");
    include!("resources/siterecovery/protection_container_mapping.rs");
    include!("resources/siterecovery/replicated_vm.rs");
    include!("resources/siterecovery/replication_policy.rs");
    include!("resources/siterecovery/replication_recovery_plan.rs");
    include!("resources/siterecovery/vm_ware_replication_policy.rs");
    include!("resources/siterecovery/vmware_replicated_vm.rs");
    include!("resources/siterecovery/vmware_replication_policy_association.rs");
}
pub mod stack {
    include!("resources/stack/hci_cluster.rs");
    include!("resources/stack/hci_deployment_setting.rs");
    include!("resources/stack/hci_extension.rs");
    include!("resources/stack/hci_logical_network.rs");
    include!("resources/stack/hci_marketplace_gallery_image.rs");
    include!("resources/stack/hci_network_interface.rs");
    include!("resources/stack/hci_storage_path.rs");
    include!("resources/stack/hci_virtual_hard_disk.rs");
}
pub mod functions {
    pub mod role {
        include!("functions/role/get_role_definition.rs");
    }
    pub mod search {
        include!("functions/search/get_service.rs");
    }
    pub mod sentinel {
        include!("functions/sentinel/get_alert_rule.rs");
        include!("functions/sentinel/get_alert_rule_anomaly.rs");
        include!("functions/sentinel/get_alert_rule_template.rs");
    }
    pub mod servicebus {
        include!("functions/servicebus/get_namespace.rs");
        include!("functions/servicebus/get_namespace_authorization_rule.rs");
        include!("functions/servicebus/get_namespace_disaster_recovery_config.rs");
        include!("functions/servicebus/get_queue.rs");
        include!("functions/servicebus/get_queue_authorization_rule.rs");
        include!("functions/servicebus/get_subscription.rs");
        include!("functions/servicebus/get_topic.rs");
        include!("functions/servicebus/get_topic_authorization_rule.rs");
    }
    pub mod signalr {
        include!("functions/signalr/get_service.rs");
    }
    pub mod siterecovery {
        include!("functions/siterecovery/get_fabric.rs");
        include!("functions/siterecovery/get_protection_container.rs");
        include!("functions/siterecovery/get_replication_policy.rs");
        include!("functions/siterecovery/get_replication_recovery_plan.rs");
    }
    pub mod stack {
        include!("functions/stack/get_hci_cluster.rs");
    }
}
pub mod types {
    pub mod role {
        include!("types/role/definition_permission.rs");
        include!("types/role/get_role_definition_permission.rs");
    }
    pub mod search {
        include!("types/search/service_identity.rs");
        include!("types/search/service_query_key.rs");
        include!("types/search/get_service_identity.rs");
        include!("types/search/get_service_query_key.rs");
    }
    pub mod securitycenter {
        include!("types/securitycenter/assessment_status.rs");
        include!("types/securitycenter/automation_action.rs");
        include!("types/securitycenter/automation_source.rs");
        include!("types/securitycenter/automation_source_rule_set.rs");
        include!("types/securitycenter/automation_source_rule_set_rule.rs");
        include!("types/securitycenter/subscription_pricing_extension.rs");
    }
    pub mod sentinel {
        include!(
            "types/sentinel/alert_rule_anomaly_built_in_multi_select_observation.rs"
        );
        include!(
            "types/sentinel/alert_rule_anomaly_built_in_prioritized_exclude_observation.rs"
        );
        include!(
            "types/sentinel/alert_rule_anomaly_built_in_required_data_connector.rs"
        );
        include!(
            "types/sentinel/alert_rule_anomaly_built_in_single_select_observation.rs"
        );
        include!("types/sentinel/alert_rule_anomaly_built_in_threshold_observation.rs");
        include!(
            "types/sentinel/alert_rule_anomaly_duplicate_multi_select_observation.rs"
        );
        include!(
            "types/sentinel/alert_rule_anomaly_duplicate_prioritized_exclude_observation.rs"
        );
        include!(
            "types/sentinel/alert_rule_anomaly_duplicate_required_data_connector.rs"
        );
        include!(
            "types/sentinel/alert_rule_anomaly_duplicate_single_select_observation.rs"
        );
        include!("types/sentinel/alert_rule_anomaly_duplicate_threshold_observation.rs");
        include!("types/sentinel/alert_rule_fusion_source.rs");
        include!("types/sentinel/alert_rule_fusion_source_sub_type.rs");
        include!("types/sentinel/alert_rule_nrt_alert_details_override.rs");
        include!(
            "types/sentinel/alert_rule_nrt_alert_details_override_dynamic_property.rs"
        );
        include!("types/sentinel/alert_rule_nrt_entity_mapping.rs");
        include!("types/sentinel/alert_rule_nrt_entity_mapping_field_mapping.rs");
        include!("types/sentinel/alert_rule_nrt_event_grouping.rs");
        include!("types/sentinel/alert_rule_nrt_incident.rs");
        include!("types/sentinel/alert_rule_nrt_incident_grouping.rs");
        include!("types/sentinel/alert_rule_nrt_sentinel_entity_mapping.rs");
        include!("types/sentinel/alert_rule_scheduled_alert_details_override.rs");
        include!(
            "types/sentinel/alert_rule_scheduled_alert_details_override_dynamic_property.rs"
        );
        include!("types/sentinel/alert_rule_scheduled_entity_mapping.rs");
        include!("types/sentinel/alert_rule_scheduled_entity_mapping_field_mapping.rs");
        include!("types/sentinel/alert_rule_scheduled_event_grouping.rs");
        include!("types/sentinel/alert_rule_scheduled_incident.rs");
        include!("types/sentinel/alert_rule_scheduled_incident_grouping.rs");
        include!("types/sentinel/alert_rule_scheduled_sentinel_entity_mapping.rs");
        include!("types/sentinel/authomation_rule_action_incident.rs");
        include!("types/sentinel/authomation_rule_action_playbook.rs");
        include!("types/sentinel/automation_rule_action_incident.rs");
        include!("types/sentinel/automation_rule_action_playbook.rs");
        include!("types/sentinel/metadata_author.rs");
        include!("types/sentinel/metadata_category.rs");
        include!("types/sentinel/metadata_source.rs");
        include!("types/sentinel/metadata_support.rs");
        include!("types/sentinel/threat_intelligence_indicator_external_reference.rs");
        include!("types/sentinel/threat_intelligence_indicator_granular_marking.rs");
        include!("types/sentinel/threat_intelligence_indicator_kill_chain_phase.rs");
        include!("types/sentinel/threat_intelligence_indicator_parsed_pattern.rs");
        include!(
            "types/sentinel/threat_intelligence_indicator_parsed_pattern_pattern_type_value.rs"
        );
        include!("types/sentinel/get_alert_rule_anomaly_multi_select_observation.rs");
        include!(
            "types/sentinel/get_alert_rule_anomaly_prioritized_exclude_observation.rs"
        );
        include!("types/sentinel/get_alert_rule_anomaly_required_data_connector.rs");
        include!("types/sentinel/get_alert_rule_anomaly_single_select_observation.rs");
        include!("types/sentinel/get_alert_rule_anomaly_threshold_observation.rs");
        include!("types/sentinel/get_alert_rule_template_nrt_template.rs");
        include!("types/sentinel/get_alert_rule_template_scheduled_template.rs");
        include!("types/sentinel/get_alert_rule_template_security_incident_template.rs");
    }
    pub mod servicebus {
        include!("types/servicebus/namespace_customer_managed_key.rs");
        include!("types/servicebus/namespace_identity.rs");
        include!("types/servicebus/namespace_network_rule_set.rs");
        include!("types/servicebus/namespace_network_rule_set_network_rule.rs");
        include!("types/servicebus/subscription_client_scoped_subscription.rs");
        include!("types/servicebus/subscription_rule_correlation_filter.rs");
    }
    pub mod servicefabric {
        include!("types/servicefabric/cluster_azure_active_directory.rs");
        include!("types/servicefabric/cluster_certificate.rs");
        include!("types/servicefabric/cluster_certificate_common_names.rs");
        include!("types/servicefabric/cluster_certificate_common_names_common_name.rs");
        include!("types/servicefabric/cluster_client_certificate_common_name.rs");
        include!("types/servicefabric/cluster_client_certificate_thumbprint.rs");
        include!("types/servicefabric/cluster_diagnostics_config.rs");
        include!("types/servicefabric/cluster_fabric_setting.rs");
        include!("types/servicefabric/cluster_node_type.rs");
        include!("types/servicefabric/cluster_node_type_application_ports.rs");
        include!("types/servicefabric/cluster_node_type_ephemeral_ports.rs");
        include!("types/servicefabric/cluster_reverse_proxy_certificate.rs");
        include!(
            "types/servicefabric/cluster_reverse_proxy_certificate_common_names.rs"
        );
        include!(
            "types/servicefabric/cluster_reverse_proxy_certificate_common_names_common_name.rs"
        );
        include!("types/servicefabric/cluster_upgrade_policy.rs");
        include!("types/servicefabric/cluster_upgrade_policy_delta_health_policy.rs");
        include!("types/servicefabric/cluster_upgrade_policy_health_policy.rs");
        include!("types/servicefabric/managed_cluster_authentication.rs");
        include!(
            "types/servicefabric/managed_cluster_authentication_active_directory.rs"
        );
        include!("types/servicefabric/managed_cluster_authentication_certificate.rs");
        include!("types/servicefabric/managed_cluster_custom_fabric_setting.rs");
        include!("types/servicefabric/managed_cluster_lb_rule.rs");
        include!("types/servicefabric/managed_cluster_node_type.rs");
        include!("types/servicefabric/managed_cluster_node_type_vm_secret.rs");
        include!(
            "types/servicefabric/managed_cluster_node_type_vm_secret_certificate.rs"
        );
    }
    pub mod signalr {
        include!("types/signalr/service_cor.rs");
        include!("types/signalr/service_identity.rs");
        include!("types/signalr/service_live_trace.rs");
        include!("types/signalr/service_network_acl_private_endpoint.rs");
        include!("types/signalr/service_network_acl_public_network.rs");
        include!("types/signalr/service_sku.rs");
        include!("types/signalr/service_upstream_endpoint.rs");
    }
    pub mod siterecovery {
        include!("types/siterecovery/protection_container_mapping_automatic_update.rs");
        include!("types/siterecovery/replicated_vm_managed_disk.rs");
        include!(
            "types/siterecovery/replicated_vm_managed_disk_target_disk_encryption.rs"
        );
        include!(
            "types/siterecovery/replicated_vm_managed_disk_target_disk_encryption_disk_encryption_key.rs"
        );
        include!(
            "types/siterecovery/replicated_vm_managed_disk_target_disk_encryption_key_encryption_key.rs"
        );
        include!("types/siterecovery/replicated_vm_network_interface.rs");
        include!("types/siterecovery/replicated_vm_unmanaged_disk.rs");
        include!(
            "types/siterecovery/replication_recovery_plan_azure_to_azure_settings.rs"
        );
        include!("types/siterecovery/replication_recovery_plan_boot_recovery_group.rs");
        include!(
            "types/siterecovery/replication_recovery_plan_boot_recovery_group_post_action.rs"
        );
        include!(
            "types/siterecovery/replication_recovery_plan_boot_recovery_group_pre_action.rs"
        );
        include!(
            "types/siterecovery/replication_recovery_plan_failover_recovery_group.rs"
        );
        include!(
            "types/siterecovery/replication_recovery_plan_failover_recovery_group_post_action.rs"
        );
        include!(
            "types/siterecovery/replication_recovery_plan_failover_recovery_group_pre_action.rs"
        );
        include!(
            "types/siterecovery/replication_recovery_plan_shutdown_recovery_group.rs"
        );
        include!(
            "types/siterecovery/replication_recovery_plan_shutdown_recovery_group_post_action.rs"
        );
        include!(
            "types/siterecovery/replication_recovery_plan_shutdown_recovery_group_pre_action.rs"
        );
        include!("types/siterecovery/vmware_replicated_vm_managed_disk.rs");
        include!("types/siterecovery/vmware_replicated_vm_network_interface.rs");
        include!(
            "types/siterecovery/get_replication_recovery_plan_azure_to_azure_setting.rs"
        );
        include!("types/siterecovery/get_replication_recovery_plan_recovery_group.rs");
        include!(
            "types/siterecovery/get_replication_recovery_plan_recovery_group_post_action.rs"
        );
        include!(
            "types/siterecovery/get_replication_recovery_plan_recovery_group_pre_action.rs"
        );
    }
    pub mod stack {
        include!("types/stack/hci_cluster_identity.rs");
        include!("types/stack/hci_deployment_setting_scale_unit.rs");
        include!("types/stack/hci_deployment_setting_scale_unit_cluster.rs");
        include!("types/stack/hci_deployment_setting_scale_unit_host_network.rs");
        include!("types/stack/hci_deployment_setting_scale_unit_host_network_intent.rs");
        include!(
            "types/stack/hci_deployment_setting_scale_unit_host_network_intent_adapter_property_override.rs"
        );
        include!(
            "types/stack/hci_deployment_setting_scale_unit_host_network_intent_qos_policy_override.rs"
        );
        include!(
            "types/stack/hci_deployment_setting_scale_unit_host_network_intent_virtual_switch_configuration_override.rs"
        );
        include!(
            "types/stack/hci_deployment_setting_scale_unit_host_network_storage_network.rs"
        );
        include!(
            "types/stack/hci_deployment_setting_scale_unit_infrastructure_network.rs"
        );
        include!(
            "types/stack/hci_deployment_setting_scale_unit_infrastructure_network_ip_pool.rs"
        );
        include!("types/stack/hci_deployment_setting_scale_unit_optional_service.rs");
        include!("types/stack/hci_deployment_setting_scale_unit_physical_node.rs");
        include!("types/stack/hci_deployment_setting_scale_unit_storage.rs");
        include!("types/stack/hci_logical_network_subnet.rs");
        include!("types/stack/hci_logical_network_subnet_ip_pool.rs");
        include!("types/stack/hci_logical_network_subnet_route.rs");
        include!("types/stack/hci_marketplace_gallery_image_identifier.rs");
        include!("types/stack/hci_network_interface_ip_configuration.rs");
        include!("types/stack/get_hci_cluster_identity.rs");
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
