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
pub mod storage {
    include!("resources/storage/account.rs");
    include!("resources/storage/account_network_rules.rs");
    include!("resources/storage/account_queue_properties.rs");
    include!("resources/storage/account_static_website.rs");
    include!("resources/storage/blob.rs");
    include!("resources/storage/blob_inventory_policy.rs");
    include!("resources/storage/container.rs");
    include!("resources/storage/container_immutability_policy.rs");
    include!("resources/storage/customer_managed_key.rs");
    include!("resources/storage/data_lake_gen_2_filesystem.rs");
    include!("resources/storage/data_lake_gen_2_path.rs");
    include!("resources/storage/encryption_scope.rs");
    include!("resources/storage/local_user.rs");
    include!("resources/storage/management_policy.rs");
    include!("resources/storage/mover.rs");
    include!("resources/storage/mover_agent.rs");
    include!("resources/storage/mover_job_definition.rs");
    include!("resources/storage/mover_project.rs");
    include!("resources/storage/mover_source_endpoint.rs");
    include!("resources/storage/mover_target_endpoint.rs");
    include!("resources/storage/object_replication.rs");
    include!("resources/storage/queue.rs");
    include!("resources/storage/share.rs");
    include!("resources/storage/share_directory.rs");
    include!("resources/storage/share_file.rs");
    include!("resources/storage/sync.rs");
    include!("resources/storage/sync_cloud_endpoint.rs");
    include!("resources/storage/sync_group.rs");
    include!("resources/storage/sync_server_endpoint.rs");
    include!("resources/storage/table.rs");
    include!("resources/storage/table_entity.rs");
    include!("resources/storage/zip_blob.rs");
}
pub mod streamanalytics {
    include!("resources/streamanalytics/cluster.rs");
    include!("resources/streamanalytics/function_java_script_udf.rs");
    include!("resources/streamanalytics/function_javascript_uda.rs");
    include!("resources/streamanalytics/job.rs");
    include!("resources/streamanalytics/job_schedule.rs");
    include!("resources/streamanalytics/managed_private_endpoint.rs");
    include!("resources/streamanalytics/output_blob.rs");
    include!("resources/streamanalytics/output_cosmosdb.rs");
    include!("resources/streamanalytics/output_event_hub.rs");
    include!("resources/streamanalytics/output_function.rs");
    include!("resources/streamanalytics/output_mssql.rs");
    include!("resources/streamanalytics/output_powerbi.rs");
    include!("resources/streamanalytics/output_service_bus_queue.rs");
    include!("resources/streamanalytics/output_servicebus_topic.rs");
    include!("resources/streamanalytics/output_synapse.rs");
    include!("resources/streamanalytics/output_table.rs");
    include!("resources/streamanalytics/reference_input_blob.rs");
    include!("resources/streamanalytics/reference_input_mssql.rs");
    include!("resources/streamanalytics/stream_input_blob.rs");
    include!("resources/streamanalytics/stream_input_event_hub.rs");
    include!("resources/streamanalytics/stream_input_event_hub_v_2.rs");
    include!("resources/streamanalytics/stream_input_iot_hub.rs");
}
pub mod synapse {
    include!("resources/synapse/firewall_rule.rs");
    include!("resources/synapse/integration_runtime_azure.rs");
    include!("resources/synapse/integration_runtime_self_hosted.rs");
    include!("resources/synapse/linked_service.rs");
    include!("resources/synapse/managed_private_endpoint.rs");
    include!("resources/synapse/private_link_hub.rs");
    include!("resources/synapse/role_assignment.rs");
    include!("resources/synapse/spark_pool.rs");
    include!("resources/synapse/sql_pool.rs");
    include!("resources/synapse/sql_pool_extended_auditing_policy.rs");
    include!("resources/synapse/sql_pool_security_alert_policy.rs");
    include!("resources/synapse/sql_pool_vulnerability_assessment.rs");
    include!("resources/synapse/sql_pool_vulnerability_assessment_baseline.rs");
    include!("resources/synapse/sql_pool_workload_classifier.rs");
    include!("resources/synapse/sql_pool_workload_group.rs");
    include!("resources/synapse/workspace.rs");
    include!("resources/synapse/workspace_aad_admin.rs");
    include!("resources/synapse/workspace_extended_auditing_policy.rs");
    include!("resources/synapse/workspace_key.rs");
    include!("resources/synapse/workspace_security_alert_policy.rs");
    include!("resources/synapse/workspace_sql_aad_admin.rs");
    include!("resources/synapse/workspace_vulnerability_assessment.rs");
}
pub mod systemcenter {
    include!("resources/systemcenter/virtual_machine_manager_availability_set.rs");
    include!("resources/systemcenter/virtual_machine_manager_cloud.rs");
    include!("resources/systemcenter/virtual_machine_manager_server.rs");
    include!(
        "resources/systemcenter/virtual_machine_manager_virtual_machine_template.rs"
    );
    include!("resources/systemcenter/virtual_machine_manager_virtual_network.rs");
}
pub mod trafficmanager {
    include!("resources/trafficmanager/profile.rs");
}
pub mod trustedsigning {
    include!("resources/trustedsigning/account.rs");
}
pub mod videoindexer {
    include!("resources/videoindexer/account.rs");
}
pub mod functions {
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
    pub mod storage {
        include!("functions/storage/get_account.rs");
        include!("functions/storage/get_account_blob_container_sas.rs");
        include!("functions/storage/get_account_sas.rs");
        include!("functions/storage/get_blob.rs");
        include!("functions/storage/get_containers.rs");
        include!("functions/storage/get_encryption_scope.rs");
        include!("functions/storage/get_policy.rs");
        include!("functions/storage/get_queue.rs");
        include!("functions/storage/get_share.rs");
        include!("functions/storage/get_storage_container.rs");
        include!("functions/storage/get_sync.rs");
        include!("functions/storage/get_sync_group.rs");
        include!("functions/storage/get_table.rs");
        include!("functions/storage/get_table_entities.rs");
        include!("functions/storage/get_table_entity.rs");
    }
    pub mod streamanalytics {
        include!("functions/streamanalytics/get_job.rs");
    }
    pub mod synapse {
        include!("functions/synapse/get_workspace.rs");
    }
    pub mod systemcenter {
        include!(
            "functions/systemcenter/get_virtual_machine_manager_inventory_items.rs"
        );
    }
    pub mod trafficmanager {
        include!("functions/trafficmanager/get_geographical_location.rs");
    }
}
pub mod types {
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
    pub mod storage {
        include!("types/storage/account_azure_files_authentication.rs");
        include!("types/storage/account_azure_files_authentication_active_directory.rs");
        include!("types/storage/account_blob_properties.rs");
        include!(
            "types/storage/account_blob_properties_container_delete_retention_policy.rs"
        );
        include!("types/storage/account_blob_properties_cors_rule.rs");
        include!("types/storage/account_blob_properties_delete_retention_policy.rs");
        include!("types/storage/account_blob_properties_restore_policy.rs");
        include!("types/storage/account_custom_domain.rs");
        include!("types/storage/account_customer_managed_key.rs");
        include!("types/storage/account_identity.rs");
        include!("types/storage/account_immutability_policy.rs");
        include!("types/storage/account_network_rules.rs");
        include!("types/storage/account_network_rules_private_link_access.rs");
        include!("types/storage/account_network_rules_private_link_access_rule.rs");
        include!("types/storage/account_queue_properties.rs");
        include!("types/storage/account_queue_properties_cors_rule.rs");
        include!("types/storage/account_queue_properties_hour_metrics.rs");
        include!("types/storage/account_queue_properties_logging.rs");
        include!("types/storage/account_queue_properties_minute_metrics.rs");
        include!("types/storage/account_routing.rs");
        include!("types/storage/account_sas_policy.rs");
        include!("types/storage/account_share_properties.rs");
        include!("types/storage/account_share_properties_cors_rule.rs");
        include!("types/storage/account_share_properties_retention_policy.rs");
        include!("types/storage/account_share_properties_smb.rs");
        include!("types/storage/account_static_website.rs");
        include!("types/storage/blob_inventory_policy_rule.rs");
        include!("types/storage/blob_inventory_policy_rule_filter.rs");
        include!("types/storage/data_lake_gen_2_filesystem_ace.rs");
        include!("types/storage/data_lake_gen_2_path_ace.rs");
        include!("types/storage/local_user_permission_scope.rs");
        include!("types/storage/local_user_permission_scope_permissions.rs");
        include!("types/storage/local_user_ssh_authorized_key.rs");
        include!("types/storage/management_policy_rule.rs");
        include!("types/storage/management_policy_rule_actions.rs");
        include!("types/storage/management_policy_rule_actions_base_blob.rs");
        include!("types/storage/management_policy_rule_actions_snapshot.rs");
        include!("types/storage/management_policy_rule_actions_version.rs");
        include!("types/storage/management_policy_rule_filters.rs");
        include!("types/storage/management_policy_rule_filters_match_blob_index_tag.rs");
        include!("types/storage/object_replication_rule.rs");
        include!("types/storage/share_acl.rs");
        include!("types/storage/share_acl_access_policy.rs");
        include!("types/storage/table_acl.rs");
        include!("types/storage/table_acl_access_policy.rs");
        include!("types/storage/get_account_azure_files_authentication.rs");
        include!(
            "types/storage/get_account_azure_files_authentication_active_directory.rs"
        );
        include!("types/storage/get_account_blob_container_sas_permissions.rs");
        include!("types/storage/get_account_custom_domain.rs");
        include!("types/storage/get_account_identity.rs");
        include!("types/storage/get_account_sas_permissions.rs");
        include!("types/storage/get_account_sas_resource_types.rs");
        include!("types/storage/get_account_sas_services.rs");
        include!("types/storage/get_containers_container.rs");
        include!("types/storage/get_policy_rule.rs");
        include!("types/storage/get_policy_rule_action.rs");
        include!("types/storage/get_policy_rule_action_base_blob.rs");
        include!("types/storage/get_policy_rule_action_snapshot.rs");
        include!("types/storage/get_policy_rule_action_version.rs");
        include!("types/storage/get_policy_rule_filter.rs");
        include!("types/storage/get_policy_rule_filter_match_blob_index_tag.rs");
        include!("types/storage/get_share_acl.rs");
        include!("types/storage/get_share_acl_access_policy.rs");
        include!("types/storage/get_table_acl.rs");
        include!("types/storage/get_table_acl_access_policy.rs");
        include!("types/storage/get_table_entities_item.rs");
    }
    pub mod streamanalytics {
        include!("types/streamanalytics/function_java_script_udf_input.rs");
        include!("types/streamanalytics/function_java_script_udf_output.rs");
        include!("types/streamanalytics/function_javascript_uda_input.rs");
        include!("types/streamanalytics/function_javascript_uda_output.rs");
        include!("types/streamanalytics/job_identity.rs");
        include!("types/streamanalytics/job_job_storage_account.rs");
        include!("types/streamanalytics/output_blob_serialization.rs");
        include!("types/streamanalytics/output_event_hub_serialization.rs");
        include!("types/streamanalytics/output_service_bus_queue_serialization.rs");
        include!("types/streamanalytics/output_servicebus_topic_serialization.rs");
        include!("types/streamanalytics/reference_input_blob_serialization.rs");
        include!("types/streamanalytics/stream_input_blob_serialization.rs");
        include!("types/streamanalytics/stream_input_event_hub_serialization.rs");
        include!("types/streamanalytics/stream_input_event_hub_v_2_serialization.rs");
        include!("types/streamanalytics/stream_input_iot_hub_serialization.rs");
        include!("types/streamanalytics/get_job_identity.rs");
    }
    pub mod synapse {
        include!("types/synapse/linked_service_integration_runtime.rs");
        include!("types/synapse/spark_pool_auto_pause.rs");
        include!("types/synapse/spark_pool_auto_scale.rs");
        include!("types/synapse/spark_pool_library_requirement.rs");
        include!("types/synapse/spark_pool_spark_config.rs");
        include!("types/synapse/sql_pool_restore.rs");
        include!("types/synapse/sql_pool_vulnerability_assessment_baseline_baseline.rs");
        include!("types/synapse/sql_pool_vulnerability_assessment_recurring_scans.rs");
        include!("types/synapse/workspace_azure_devops_repo.rs");
        include!("types/synapse/workspace_customer_managed_key.rs");
        include!("types/synapse/workspace_github_repo.rs");
        include!("types/synapse/workspace_identity.rs");
        include!("types/synapse/workspace_vulnerability_assessment_recurring_scans.rs");
        include!("types/synapse/get_workspace_identity.rs");
    }
    pub mod systemcenter {
        include!(
            "types/systemcenter/get_virtual_machine_manager_inventory_items_inventory_item.rs"
        );
    }
    pub mod trafficmanager {
        include!("types/trafficmanager/profile_dns_config.rs");
        include!("types/trafficmanager/profile_monitor_config.rs");
        include!("types/trafficmanager/profile_monitor_config_custom_header.rs");
    }
    pub mod videoindexer {
        include!("types/videoindexer/account_identity.rs");
        include!("types/videoindexer/account_storage.rs");
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
