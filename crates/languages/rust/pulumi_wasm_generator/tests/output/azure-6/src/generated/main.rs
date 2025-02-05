pub mod frontdoor {
    include!("resources/frontdoor/custom_https_configuration.rs");
    include!("resources/frontdoor/firewall_policy.rs");
    include!("resources/frontdoor/frontdoor.rs");
    include!("resources/frontdoor/rules_engine.rs");
}
pub mod graph {
    include!("resources/graph/services_account.rs");
}
pub mod hdinsight {
    include!("resources/hdinsight/h_base_cluster.rs");
    include!("resources/hdinsight/hadoop_cluster.rs");
    include!("resources/hdinsight/interactive_query_cluster.rs");
    include!("resources/hdinsight/kafka_cluster.rs");
    include!("resources/hdinsight/spark_cluster.rs");
}
pub mod healthcare {
    include!("resources/healthcare/dicom_service.rs");
    include!("resources/healthcare/fhir_service.rs");
    include!("resources/healthcare/medtech_service.rs");
    include!("resources/healthcare/medtech_service_fhir_destination.rs");
    include!("resources/healthcare/service.rs");
    include!("resources/healthcare/workspace.rs");
}
pub mod hpc {
    include!("resources/hpc/cache.rs");
    include!("resources/hpc/cache_access_policy.rs");
    include!("resources/hpc/cache_blob_nfs_target.rs");
    include!("resources/hpc/cache_blob_target.rs");
    include!("resources/hpc/cache_nfs_target.rs");
}
pub mod hsm {
    include!("resources/hsm/module.rs");
}
pub mod iot {
    include!("resources/iot/certificate.rs");
    include!("resources/iot/consumer_group.rs");
    include!("resources/iot/dps_shared_access_policy.rs");
    include!("resources/iot/endpoint_cosmosdb_account.rs");
    include!("resources/iot/endpoint_eventhub.rs");
    include!("resources/iot/endpoint_servicebus_queue.rs");
    include!("resources/iot/endpoint_servicebus_topic.rs");
    include!("resources/iot/endpoint_storage_container.rs");
    include!("resources/iot/enrichment.rs");
    include!("resources/iot/fallback_route.rs");
    include!("resources/iot/file_upload.rs");
    include!("resources/iot/io_t_hub.rs");
    include!("resources/iot/iot_hub_certificate.rs");
    include!("resources/iot/iot_hub_device_update_account.rs");
    include!("resources/iot/iot_hub_device_update_instance.rs");
    include!("resources/iot/iot_hub_dps.rs");
    include!("resources/iot/route.rs");
    include!("resources/iot/security_device_group.rs");
    include!("resources/iot/security_solution.rs");
    include!("resources/iot/shared_access_policy.rs");
}
pub mod iotcentral {
    include!("resources/iotcentral/application.rs");
    include!("resources/iotcentral/application_network_rule_set.rs");
    include!("resources/iotcentral/organization.rs");
}
pub mod keyvault {
    include!("resources/keyvault/access_policy.rs");
    include!("resources/keyvault/certifiate.rs");
    include!("resources/keyvault/certificate.rs");
    include!("resources/keyvault/certificate_contacts.rs");
    include!("resources/keyvault/certificate_issuer.rs");
    include!("resources/keyvault/key.rs");
    include!("resources/keyvault/key_vault.rs");
    include!("resources/keyvault/managed_hardware_security_module.rs");
    include!("resources/keyvault/managed_hardware_security_module_key.rs");
    include!(
        "resources/keyvault/managed_hardware_security_module_key_rotation_policy.rs"
    );
    include!("resources/keyvault/managed_hardware_security_module_role_assignment.rs");
    include!("resources/keyvault/managed_hardware_security_module_role_definition.rs");
    include!("resources/keyvault/managed_storage_account.rs");
    include!("resources/keyvault/managed_storage_account_sas_token_definition.rs");
    include!("resources/keyvault/secret.rs");
}
pub mod functions {
    pub mod hdinsight {
        include!("functions/hdinsight/get_cluster.rs");
    }
    pub mod healthcare {
        include!("functions/healthcare/get_dicom_service.rs");
        include!("functions/healthcare/get_fhir_service.rs");
        include!("functions/healthcare/get_medtech_service.rs");
        include!("functions/healthcare/get_service.rs");
        include!("functions/healthcare/get_workspace.rs");
    }
    pub mod iot {
        include!("functions/iot/get_dps.rs");
        include!("functions/iot/get_dps_shared_access_policy.rs");
        include!("functions/iot/get_iot_hub.rs");
        include!("functions/iot/get_shared_access_policy.rs");
    }
    pub mod keyvault {
        include!("functions/keyvault/get_access_policy.rs");
        include!("functions/keyvault/get_certificate.rs");
        include!("functions/keyvault/get_certificate_data.rs");
        include!("functions/keyvault/get_certificate_issuer.rs");
        include!("functions/keyvault/get_certificates.rs");
        include!("functions/keyvault/get_encrypted_value.rs");
        include!("functions/keyvault/get_key.rs");
        include!("functions/keyvault/get_key_vault.rs");
        include!("functions/keyvault/get_managed_hardware_security_module.rs");
        include!("functions/keyvault/get_managed_hardware_security_module_key.rs");
        include!(
            "functions/keyvault/get_managed_hardware_security_module_role_definition.rs"
        );
        include!("functions/keyvault/get_secret.rs");
        include!("functions/keyvault/get_secrets.rs");
    }
}
pub mod types {
    pub mod frontdoor {
        include!(
            "types/frontdoor/custom_https_configuration_custom_https_configuration.rs"
        );
        include!("types/frontdoor/firewall_policy_custom_rule.rs");
        include!("types/frontdoor/firewall_policy_custom_rule_match_condition.rs");
        include!("types/frontdoor/firewall_policy_managed_rule.rs");
        include!("types/frontdoor/firewall_policy_managed_rule_exclusion.rs");
        include!("types/frontdoor/firewall_policy_managed_rule_override.rs");
        include!("types/frontdoor/firewall_policy_managed_rule_override_exclusion.rs");
        include!("types/frontdoor/firewall_policy_managed_rule_override_rule.rs");
        include!(
            "types/frontdoor/firewall_policy_managed_rule_override_rule_exclusion.rs"
        );
        include!("types/frontdoor/frontdoor_backend_pool.rs");
        include!("types/frontdoor/frontdoor_backend_pool_backend.rs");
        include!("types/frontdoor/frontdoor_backend_pool_health_probe.rs");
        include!("types/frontdoor/frontdoor_backend_pool_load_balancing.rs");
        include!("types/frontdoor/frontdoor_backend_pool_setting.rs");
        include!("types/frontdoor/frontdoor_explicit_resource_order.rs");
        include!("types/frontdoor/frontdoor_frontend_endpoint.rs");
        include!("types/frontdoor/frontdoor_routing_rule.rs");
        include!("types/frontdoor/frontdoor_routing_rule_forwarding_configuration.rs");
        include!("types/frontdoor/frontdoor_routing_rule_redirect_configuration.rs");
        include!("types/frontdoor/rules_engine_rule.rs");
        include!("types/frontdoor/rules_engine_rule_action.rs");
        include!("types/frontdoor/rules_engine_rule_action_request_header.rs");
        include!("types/frontdoor/rules_engine_rule_action_response_header.rs");
        include!("types/frontdoor/rules_engine_rule_match_condition.rs");
    }
    pub mod hdinsight {
        include!("types/hdinsight/h_base_cluster_component_version.rs");
        include!("types/hdinsight/h_base_cluster_compute_isolation.rs");
        include!("types/hdinsight/h_base_cluster_disk_encryption.rs");
        include!("types/hdinsight/h_base_cluster_extension.rs");
        include!("types/hdinsight/h_base_cluster_gateway.rs");
        include!("types/hdinsight/h_base_cluster_metastores.rs");
        include!("types/hdinsight/h_base_cluster_metastores_ambari.rs");
        include!("types/hdinsight/h_base_cluster_metastores_hive.rs");
        include!("types/hdinsight/h_base_cluster_metastores_oozie.rs");
        include!("types/hdinsight/h_base_cluster_monitor.rs");
        include!("types/hdinsight/h_base_cluster_network.rs");
        include!("types/hdinsight/h_base_cluster_private_link_configuration.rs");
        include!(
            "types/hdinsight/h_base_cluster_private_link_configuration_ip_configuration.rs"
        );
        include!("types/hdinsight/h_base_cluster_roles.rs");
        include!("types/hdinsight/h_base_cluster_roles_head_node.rs");
        include!("types/hdinsight/h_base_cluster_roles_head_node_script_action.rs");
        include!("types/hdinsight/h_base_cluster_roles_worker_node.rs");
        include!("types/hdinsight/h_base_cluster_roles_worker_node_autoscale.rs");
        include!(
            "types/hdinsight/h_base_cluster_roles_worker_node_autoscale_recurrence.rs"
        );
        include!(
            "types/hdinsight/h_base_cluster_roles_worker_node_autoscale_recurrence_schedule.rs"
        );
        include!("types/hdinsight/h_base_cluster_roles_worker_node_script_action.rs");
        include!("types/hdinsight/h_base_cluster_roles_zookeeper_node.rs");
        include!("types/hdinsight/h_base_cluster_roles_zookeeper_node_script_action.rs");
        include!("types/hdinsight/h_base_cluster_security_profile.rs");
        include!("types/hdinsight/h_base_cluster_storage_account.rs");
        include!("types/hdinsight/h_base_cluster_storage_account_gen_2.rs");
        include!("types/hdinsight/hadoop_cluster_component_version.rs");
        include!("types/hdinsight/hadoop_cluster_compute_isolation.rs");
        include!("types/hdinsight/hadoop_cluster_disk_encryption.rs");
        include!("types/hdinsight/hadoop_cluster_extension.rs");
        include!("types/hdinsight/hadoop_cluster_gateway.rs");
        include!("types/hdinsight/hadoop_cluster_metastores.rs");
        include!("types/hdinsight/hadoop_cluster_metastores_ambari.rs");
        include!("types/hdinsight/hadoop_cluster_metastores_hive.rs");
        include!("types/hdinsight/hadoop_cluster_metastores_oozie.rs");
        include!("types/hdinsight/hadoop_cluster_monitor.rs");
        include!("types/hdinsight/hadoop_cluster_network.rs");
        include!("types/hdinsight/hadoop_cluster_private_link_configuration.rs");
        include!(
            "types/hdinsight/hadoop_cluster_private_link_configuration_ip_configuration.rs"
        );
        include!("types/hdinsight/hadoop_cluster_roles.rs");
        include!("types/hdinsight/hadoop_cluster_roles_edge_node.rs");
        include!("types/hdinsight/hadoop_cluster_roles_edge_node_https_endpoint.rs");
        include!(
            "types/hdinsight/hadoop_cluster_roles_edge_node_install_script_action.rs"
        );
        include!(
            "types/hdinsight/hadoop_cluster_roles_edge_node_uninstall_script_action.rs"
        );
        include!("types/hdinsight/hadoop_cluster_roles_head_node.rs");
        include!("types/hdinsight/hadoop_cluster_roles_head_node_script_action.rs");
        include!("types/hdinsight/hadoop_cluster_roles_worker_node.rs");
        include!("types/hdinsight/hadoop_cluster_roles_worker_node_autoscale.rs");
        include!(
            "types/hdinsight/hadoop_cluster_roles_worker_node_autoscale_capacity.rs"
        );
        include!(
            "types/hdinsight/hadoop_cluster_roles_worker_node_autoscale_recurrence.rs"
        );
        include!(
            "types/hdinsight/hadoop_cluster_roles_worker_node_autoscale_recurrence_schedule.rs"
        );
        include!("types/hdinsight/hadoop_cluster_roles_worker_node_script_action.rs");
        include!("types/hdinsight/hadoop_cluster_roles_zookeeper_node.rs");
        include!("types/hdinsight/hadoop_cluster_roles_zookeeper_node_script_action.rs");
        include!("types/hdinsight/hadoop_cluster_security_profile.rs");
        include!("types/hdinsight/hadoop_cluster_storage_account.rs");
        include!("types/hdinsight/hadoop_cluster_storage_account_gen_2.rs");
        include!("types/hdinsight/interactive_query_cluster_component_version.rs");
        include!("types/hdinsight/interactive_query_cluster_compute_isolation.rs");
        include!("types/hdinsight/interactive_query_cluster_disk_encryption.rs");
        include!("types/hdinsight/interactive_query_cluster_extension.rs");
        include!("types/hdinsight/interactive_query_cluster_gateway.rs");
        include!("types/hdinsight/interactive_query_cluster_metastores.rs");
        include!("types/hdinsight/interactive_query_cluster_metastores_ambari.rs");
        include!("types/hdinsight/interactive_query_cluster_metastores_hive.rs");
        include!("types/hdinsight/interactive_query_cluster_metastores_oozie.rs");
        include!("types/hdinsight/interactive_query_cluster_monitor.rs");
        include!("types/hdinsight/interactive_query_cluster_network.rs");
        include!(
            "types/hdinsight/interactive_query_cluster_private_link_configuration.rs"
        );
        include!(
            "types/hdinsight/interactive_query_cluster_private_link_configuration_ip_configuration.rs"
        );
        include!("types/hdinsight/interactive_query_cluster_roles.rs");
        include!("types/hdinsight/interactive_query_cluster_roles_head_node.rs");
        include!(
            "types/hdinsight/interactive_query_cluster_roles_head_node_script_action.rs"
        );
        include!("types/hdinsight/interactive_query_cluster_roles_worker_node.rs");
        include!(
            "types/hdinsight/interactive_query_cluster_roles_worker_node_autoscale.rs"
        );
        include!(
            "types/hdinsight/interactive_query_cluster_roles_worker_node_autoscale_recurrence.rs"
        );
        include!(
            "types/hdinsight/interactive_query_cluster_roles_worker_node_autoscale_recurrence_schedule.rs"
        );
        include!(
            "types/hdinsight/interactive_query_cluster_roles_worker_node_script_action.rs"
        );
        include!("types/hdinsight/interactive_query_cluster_roles_zookeeper_node.rs");
        include!(
            "types/hdinsight/interactive_query_cluster_roles_zookeeper_node_script_action.rs"
        );
        include!("types/hdinsight/interactive_query_cluster_security_profile.rs");
        include!("types/hdinsight/interactive_query_cluster_storage_account.rs");
        include!("types/hdinsight/interactive_query_cluster_storage_account_gen_2.rs");
        include!("types/hdinsight/kafka_cluster_component_version.rs");
        include!("types/hdinsight/kafka_cluster_compute_isolation.rs");
        include!("types/hdinsight/kafka_cluster_disk_encryption.rs");
        include!("types/hdinsight/kafka_cluster_extension.rs");
        include!("types/hdinsight/kafka_cluster_gateway.rs");
        include!("types/hdinsight/kafka_cluster_metastores.rs");
        include!("types/hdinsight/kafka_cluster_metastores_ambari.rs");
        include!("types/hdinsight/kafka_cluster_metastores_hive.rs");
        include!("types/hdinsight/kafka_cluster_metastores_oozie.rs");
        include!("types/hdinsight/kafka_cluster_monitor.rs");
        include!("types/hdinsight/kafka_cluster_network.rs");
        include!("types/hdinsight/kafka_cluster_private_link_configuration.rs");
        include!(
            "types/hdinsight/kafka_cluster_private_link_configuration_ip_configuration.rs"
        );
        include!("types/hdinsight/kafka_cluster_rest_proxy.rs");
        include!("types/hdinsight/kafka_cluster_roles.rs");
        include!("types/hdinsight/kafka_cluster_roles_head_node.rs");
        include!("types/hdinsight/kafka_cluster_roles_head_node_script_action.rs");
        include!("types/hdinsight/kafka_cluster_roles_kafka_management_node.rs");
        include!(
            "types/hdinsight/kafka_cluster_roles_kafka_management_node_script_action.rs"
        );
        include!("types/hdinsight/kafka_cluster_roles_worker_node.rs");
        include!("types/hdinsight/kafka_cluster_roles_worker_node_script_action.rs");
        include!("types/hdinsight/kafka_cluster_roles_zookeeper_node.rs");
        include!("types/hdinsight/kafka_cluster_roles_zookeeper_node_script_action.rs");
        include!("types/hdinsight/kafka_cluster_security_profile.rs");
        include!("types/hdinsight/kafka_cluster_storage_account.rs");
        include!("types/hdinsight/kafka_cluster_storage_account_gen_2.rs");
        include!("types/hdinsight/spark_cluster_component_version.rs");
        include!("types/hdinsight/spark_cluster_compute_isolation.rs");
        include!("types/hdinsight/spark_cluster_disk_encryption.rs");
        include!("types/hdinsight/spark_cluster_extension.rs");
        include!("types/hdinsight/spark_cluster_gateway.rs");
        include!("types/hdinsight/spark_cluster_metastores.rs");
        include!("types/hdinsight/spark_cluster_metastores_ambari.rs");
        include!("types/hdinsight/spark_cluster_metastores_hive.rs");
        include!("types/hdinsight/spark_cluster_metastores_oozie.rs");
        include!("types/hdinsight/spark_cluster_monitor.rs");
        include!("types/hdinsight/spark_cluster_network.rs");
        include!("types/hdinsight/spark_cluster_private_link_configuration.rs");
        include!(
            "types/hdinsight/spark_cluster_private_link_configuration_ip_configuration.rs"
        );
        include!("types/hdinsight/spark_cluster_roles.rs");
        include!("types/hdinsight/spark_cluster_roles_head_node.rs");
        include!("types/hdinsight/spark_cluster_roles_head_node_script_action.rs");
        include!("types/hdinsight/spark_cluster_roles_worker_node.rs");
        include!("types/hdinsight/spark_cluster_roles_worker_node_autoscale.rs");
        include!(
            "types/hdinsight/spark_cluster_roles_worker_node_autoscale_capacity.rs"
        );
        include!(
            "types/hdinsight/spark_cluster_roles_worker_node_autoscale_recurrence.rs"
        );
        include!(
            "types/hdinsight/spark_cluster_roles_worker_node_autoscale_recurrence_schedule.rs"
        );
        include!("types/hdinsight/spark_cluster_roles_worker_node_script_action.rs");
        include!("types/hdinsight/spark_cluster_roles_zookeeper_node.rs");
        include!("types/hdinsight/spark_cluster_roles_zookeeper_node_script_action.rs");
        include!("types/hdinsight/spark_cluster_security_profile.rs");
        include!("types/hdinsight/spark_cluster_storage_account.rs");
        include!("types/hdinsight/spark_cluster_storage_account_gen_2.rs");
        include!("types/hdinsight/get_cluster_gateway.rs");
    }
    pub mod healthcare {
        include!("types/healthcare/dicom_service_authentication.rs");
        include!("types/healthcare/dicom_service_identity.rs");
        include!("types/healthcare/dicom_service_private_endpoint.rs");
        include!("types/healthcare/fhir_service_authentication.rs");
        include!("types/healthcare/fhir_service_cors.rs");
        include!("types/healthcare/fhir_service_identity.rs");
        include!("types/healthcare/fhir_service_oci_artifact.rs");
        include!("types/healthcare/medtech_service_identity.rs");
        include!("types/healthcare/service_authentication_configuration.rs");
        include!("types/healthcare/service_cors_configuration.rs");
        include!("types/healthcare/service_identity.rs");
        include!("types/healthcare/workspace_private_endpoint_connection.rs");
        include!("types/healthcare/get_dicom_service_authentication.rs");
        include!("types/healthcare/get_dicom_service_identity.rs");
        include!("types/healthcare/get_dicom_service_private_endpoint.rs");
        include!("types/healthcare/get_fhir_service_authentication.rs");
        include!("types/healthcare/get_fhir_service_cor.rs");
        include!("types/healthcare/get_fhir_service_identity.rs");
        include!("types/healthcare/get_medtech_service_identity.rs");
        include!("types/healthcare/get_service_authentication_configuration.rs");
        include!("types/healthcare/get_service_cors_configuration.rs");
    }
    pub mod hpc {
        include!("types/hpc/cache_access_policy_access_rule.rs");
        include!("types/hpc/cache_default_access_policy.rs");
        include!("types/hpc/cache_default_access_policy_access_rule.rs");
        include!("types/hpc/cache_directory_active_directory.rs");
        include!("types/hpc/cache_directory_flat_file.rs");
        include!("types/hpc/cache_directory_ldap.rs");
        include!("types/hpc/cache_directory_ldap_bind.rs");
        include!("types/hpc/cache_dns.rs");
        include!("types/hpc/cache_identity.rs");
        include!("types/hpc/cache_nfs_target_namespace_junction.rs");
    }
    pub mod hsm {
        include!("types/hsm/module_management_network_profile.rs");
        include!("types/hsm/module_network_profile.rs");
    }
    pub mod iot {
        include!("types/iot/io_t_hub_cloud_to_device.rs");
        include!("types/iot/io_t_hub_cloud_to_device_feedback.rs");
        include!("types/iot/io_t_hub_endpoint.rs");
        include!("types/iot/io_t_hub_enrichment.rs");
        include!("types/iot/io_t_hub_fallback_route.rs");
        include!("types/iot/io_t_hub_file_upload.rs");
        include!("types/iot/io_t_hub_identity.rs");
        include!("types/iot/io_t_hub_network_rule_set.rs");
        include!("types/iot/io_t_hub_network_rule_set_ip_rule.rs");
        include!("types/iot/io_t_hub_route.rs");
        include!("types/iot/io_t_hub_shared_access_policy.rs");
        include!("types/iot/io_t_hub_sku.rs");
        include!("types/iot/iot_hub_device_update_account_identity.rs");
        include!(
            "types/iot/iot_hub_device_update_instance_diagnostic_storage_account.rs"
        );
        include!("types/iot/iot_hub_dps_ip_filter_rule.rs");
        include!("types/iot/iot_hub_dps_linked_hub.rs");
        include!("types/iot/iot_hub_dps_sku.rs");
        include!("types/iot/security_device_group_allow_rule.rs");
        include!("types/iot/security_device_group_range_rule.rs");
        include!("types/iot/security_solution_additional_workspace.rs");
        include!("types/iot/security_solution_recommendations_enabled.rs");
        include!("types/iot/get_iot_hub_identity.rs");
    }
    pub mod iotcentral {
        include!("types/iotcentral/application_identity.rs");
        include!("types/iotcentral/application_network_rule_set_ip_rule.rs");
    }
    pub mod keyvault {
        include!("types/keyvault/certifiate_certificate.rs");
        include!("types/keyvault/certifiate_certificate_attribute.rs");
        include!("types/keyvault/certifiate_certificate_policy.rs");
        include!("types/keyvault/certifiate_certificate_policy_issuer_parameters.rs");
        include!("types/keyvault/certifiate_certificate_policy_key_properties.rs");
        include!("types/keyvault/certifiate_certificate_policy_lifetime_action.rs");
        include!(
            "types/keyvault/certifiate_certificate_policy_lifetime_action_action.rs"
        );
        include!(
            "types/keyvault/certifiate_certificate_policy_lifetime_action_trigger.rs"
        );
        include!("types/keyvault/certifiate_certificate_policy_secret_properties.rs");
        include!(
            "types/keyvault/certifiate_certificate_policy_x_509_certificate_properties.rs"
        );
        include!(
            "types/keyvault/certifiate_certificate_policy_x_509_certificate_properties_subject_alternative_names.rs"
        );
        include!("types/keyvault/certificate_certificate.rs");
        include!("types/keyvault/certificate_certificate_attribute.rs");
        include!("types/keyvault/certificate_certificate_policy.rs");
        include!("types/keyvault/certificate_certificate_policy_issuer_parameters.rs");
        include!("types/keyvault/certificate_certificate_policy_key_properties.rs");
        include!("types/keyvault/certificate_certificate_policy_lifetime_action.rs");
        include!(
            "types/keyvault/certificate_certificate_policy_lifetime_action_action.rs"
        );
        include!(
            "types/keyvault/certificate_certificate_policy_lifetime_action_trigger.rs"
        );
        include!("types/keyvault/certificate_certificate_policy_secret_properties.rs");
        include!(
            "types/keyvault/certificate_certificate_policy_x_509_certificate_properties.rs"
        );
        include!(
            "types/keyvault/certificate_certificate_policy_x_509_certificate_properties_subject_alternative_names.rs"
        );
        include!("types/keyvault/certificate_contacts_contact.rs");
        include!("types/keyvault/certificate_issuer_admin.rs");
        include!("types/keyvault/key_rotation_policy.rs");
        include!("types/keyvault/key_rotation_policy_automatic.rs");
        include!("types/keyvault/key_vault_access_policy.rs");
        include!("types/keyvault/key_vault_contact.rs");
        include!("types/keyvault/key_vault_network_acls.rs");
        include!("types/keyvault/managed_hardware_security_module_network_acls.rs");
        include!(
            "types/keyvault/managed_hardware_security_module_role_definition_permission.rs"
        );
        include!("types/keyvault/get_certificate_certificate_policy.rs");
        include!(
            "types/keyvault/get_certificate_certificate_policy_issuer_parameter.rs"
        );
        include!("types/keyvault/get_certificate_certificate_policy_key_property.rs");
        include!("types/keyvault/get_certificate_certificate_policy_lifetime_action.rs");
        include!(
            "types/keyvault/get_certificate_certificate_policy_lifetime_action_action.rs"
        );
        include!(
            "types/keyvault/get_certificate_certificate_policy_lifetime_action_trigger.rs"
        );
        include!("types/keyvault/get_certificate_certificate_policy_secret_property.rs");
        include!(
            "types/keyvault/get_certificate_certificate_policy_x_509_certificate_property.rs"
        );
        include!(
            "types/keyvault/get_certificate_certificate_policy_x_509_certificate_property_subject_alternative_name.rs"
        );
        include!("types/keyvault/get_certificate_issuer_admin.rs");
        include!("types/keyvault/get_certificates_certificate.rs");
        include!("types/keyvault/get_key_vault_access_policy.rs");
        include!("types/keyvault/get_key_vault_network_acl.rs");
        include!(
            "types/keyvault/get_managed_hardware_security_module_role_definition_permission.rs"
        );
        include!("types/keyvault/get_secrets_secret.rs");
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
        pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
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
