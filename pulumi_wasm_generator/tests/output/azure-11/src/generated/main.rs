pub mod recoveryservices {
    include!("resources/recoveryservices/vault.rs");
    include!("resources/recoveryservices/vault_resource_guard_association.rs");
}
pub mod redhatopenshift {
    include!("resources/redhatopenshift/cluster.rs");
}
pub mod redis {
    include!("resources/redis/cache.rs");
    include!("resources/redis/cache_access_policy.rs");
    include!("resources/redis/cache_access_policy_assignment.rs");
    include!("resources/redis/enterprise_cluster.rs");
    include!("resources/redis/enterprise_database.rs");
    include!("resources/redis/firewall_rule.rs");
    include!("resources/redis/linked_server.rs");
}
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
pub mod functions {
    pub mod recoveryservices {
        include!("functions/recoveryservices/get_vault.rs");
    }
    pub mod redis {
        include!("functions/redis/get_cache.rs");
        include!("functions/redis/get_enterprise_database.rs");
    }
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
}
pub mod types {
    pub mod recoveryservices {
        include!("types/recoveryservices/vault_encryption.rs");
        include!("types/recoveryservices/vault_identity.rs");
        include!("types/recoveryservices/vault_monitoring.rs");
        include!("types/recoveryservices/get_vault_identity.rs");
    }
    pub mod redhatopenshift {
        include!("types/redhatopenshift/cluster_api_server_profile.rs");
        include!("types/redhatopenshift/cluster_cluster_profile.rs");
        include!("types/redhatopenshift/cluster_ingress_profile.rs");
        include!("types/redhatopenshift/cluster_main_profile.rs");
        include!("types/redhatopenshift/cluster_network_profile.rs");
        include!("types/redhatopenshift/cluster_service_principal.rs");
        include!("types/redhatopenshift/cluster_worker_profile.rs");
    }
    pub mod redis {
        include!("types/redis/cache_identity.rs");
        include!("types/redis/cache_patch_schedule.rs");
        include!("types/redis/cache_redis_configuration.rs");
        include!("types/redis/enterprise_database_module.rs");
        include!("types/redis/get_cache_patch_schedule.rs");
        include!("types/redis/get_cache_redis_configuration.rs");
    }
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
#[link_section = "pulumi_wasm_provider::azure"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_AZURE: [u8; 45] = *b"{\"version\":\"6.14.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "6.14.0".to_string()
}
