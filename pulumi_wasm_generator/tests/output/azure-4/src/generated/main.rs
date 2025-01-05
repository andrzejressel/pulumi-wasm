pub mod databricks {
    include!("resources/databricks/access_connector.rs");
    include!("resources/databricks/virtual_network_peering.rs");
    include!("resources/databricks/workspace.rs");
    include!("resources/databricks/workspace_customer_managed_key.rs");
    include!("resources/databricks/workspace_root_dbfs_customer_managed_key.rs");
}
pub mod datadog {
    include!("resources/datadog/monitor.rs");
    include!("resources/datadog/monitor_sso_configuration.rs");
    include!("resources/datadog/monitor_tag_rule.rs");
}
pub mod datafactory {
    include!("resources/datafactory/credential_service_principal.rs");
    include!("resources/datafactory/credential_user_managed_identity.rs");
    include!("resources/datafactory/custom_dataset.rs");
    include!("resources/datafactory/data_flow.rs");
    include!("resources/datafactory/dataset_azure_blob.rs");
    include!("resources/datafactory/dataset_azure_sql_table.rs");
    include!("resources/datafactory/dataset_binary.rs");
    include!("resources/datafactory/dataset_cosmos_db_api.rs");
    include!("resources/datafactory/dataset_delimited_text.rs");
    include!("resources/datafactory/dataset_http.rs");
    include!("resources/datafactory/dataset_json.rs");
    include!("resources/datafactory/dataset_mysql.rs");
    include!("resources/datafactory/dataset_parquet.rs");
    include!("resources/datafactory/dataset_postgresql.rs");
    include!("resources/datafactory/dataset_snowflake.rs");
    include!("resources/datafactory/dataset_sql_server_table.rs");
    include!("resources/datafactory/factory.rs");
    include!("resources/datafactory/flowlet_data_flow.rs");
    include!("resources/datafactory/integration_runtime_rule.rs");
    include!("resources/datafactory/integration_runtime_self_hosted.rs");
    include!("resources/datafactory/integration_runtime_ssis.rs");
    include!("resources/datafactory/linked_custom_service.rs");
    include!("resources/datafactory/linked_service_azure_blob_storage.rs");
    include!("resources/datafactory/linked_service_azure_databricks.rs");
    include!("resources/datafactory/linked_service_azure_file_storage.rs");
    include!("resources/datafactory/linked_service_azure_function.rs");
    include!("resources/datafactory/linked_service_azure_search.rs");
    include!("resources/datafactory/linked_service_azure_sql_database.rs");
    include!("resources/datafactory/linked_service_azure_table_storage.rs");
    include!("resources/datafactory/linked_service_cosmos_db.rs");
    include!("resources/datafactory/linked_service_cosmos_db_mongo_api.rs");
    include!("resources/datafactory/linked_service_data_lake_storage_gen_2.rs");
    include!("resources/datafactory/linked_service_key_vault.rs");
    include!("resources/datafactory/linked_service_kusto.rs");
    include!("resources/datafactory/linked_service_mysql.rs");
    include!("resources/datafactory/linked_service_odata.rs");
    include!("resources/datafactory/linked_service_odbc.rs");
    include!("resources/datafactory/linked_service_postgresql.rs");
    include!("resources/datafactory/linked_service_sftp.rs");
    include!("resources/datafactory/linked_service_snowflake.rs");
    include!("resources/datafactory/linked_service_sql_server.rs");
    include!("resources/datafactory/linked_service_synapse.rs");
    include!("resources/datafactory/linked_service_web.rs");
    include!("resources/datafactory/managed_private_endpoint.rs");
    include!("resources/datafactory/pipeline.rs");
    include!("resources/datafactory/trigger_blob_event.rs");
    include!("resources/datafactory/trigger_custom_event.rs");
    include!("resources/datafactory/trigger_schedule.rs");
    include!("resources/datafactory/trigger_tumbling_window.rs");
}
pub mod dataprotection {
    include!("resources/dataprotection/backup_instance_blog_storage.rs");
    include!("resources/dataprotection/backup_instance_disk.rs");
    include!("resources/dataprotection/backup_instance_kubernetes_cluster.rs");
    include!("resources/dataprotection/backup_instance_mysql_flexible_server.rs");
    include!("resources/dataprotection/backup_instance_postgresql.rs");
    include!("resources/dataprotection/backup_instance_postgresql_flexible_server.rs");
    include!("resources/dataprotection/backup_policy_blob_storage.rs");
    include!("resources/dataprotection/backup_policy_disk.rs");
    include!("resources/dataprotection/backup_policy_kubernetes_cluster.rs");
    include!("resources/dataprotection/backup_policy_mysql_flexible_server.rs");
    include!("resources/dataprotection/backup_policy_postgresql.rs");
    include!("resources/dataprotection/backup_policy_postgresql_flexible_server.rs");
    include!("resources/dataprotection/backup_vault.rs");
    include!("resources/dataprotection/resource_guard.rs");
}
pub mod datashare {
    include!("resources/datashare/account.rs");
    include!("resources/datashare/dataset_blob_storage.rs");
    include!("resources/datashare/dataset_data_lake_gen_2.rs");
    include!("resources/datashare/dataset_kusto_cluster.rs");
    include!("resources/datashare/dataset_kusto_database.rs");
    include!("resources/datashare/share.rs");
}
pub mod desktopvirtualization {
    include!("resources/desktopvirtualization/application.rs");
    include!("resources/desktopvirtualization/application_group.rs");
    include!("resources/desktopvirtualization/host_pool.rs");
    include!("resources/desktopvirtualization/scaling_plan.rs");
    include!("resources/desktopvirtualization/scaling_plan_host_pool_association.rs");
    include!("resources/desktopvirtualization/workspace.rs");
    include!(
        "resources/desktopvirtualization/workspace_application_group_association.rs"
    );
    include!("resources/desktopvirtualization/get_host_pool_registration_info.rs");
}
pub mod devcenter {
    include!("resources/devcenter/attached_network.rs");
    include!("resources/devcenter/catalog.rs");
    include!("resources/devcenter/dev_box_definition.rs");
    include!("resources/devcenter/dev_center.rs");
    include!("resources/devcenter/environment_type.rs");
    include!("resources/devcenter/gallery.rs");
    include!("resources/devcenter/network_connection.rs");
    include!("resources/devcenter/project.rs");
    include!("resources/devcenter/project_environment_type.rs");
    include!("resources/devcenter/project_pool.rs");
}
pub mod devtest {
    include!("resources/devtest/global_vm_shutdown_schedule.rs");
    include!("resources/devtest/lab.rs");
    include!("resources/devtest/linux_virtual_machine.rs");
    include!("resources/devtest/policy.rs");
    include!("resources/devtest/schedule.rs");
    include!("resources/devtest/virtual_network.rs");
    include!("resources/devtest/windows_virtual_machine.rs");
}
pub mod digitaltwins {
    include!("resources/digitaltwins/endpoint_event_grid.rs");
    include!("resources/digitaltwins/endpoint_event_hub.rs");
    include!("resources/digitaltwins/endpoint_servicebus.rs");
    include!("resources/digitaltwins/instance.rs");
    include!("resources/digitaltwins/time_series_database_connection.rs");
}
pub mod dns {
    include!("resources/dns/a_record.rs");
    include!("resources/dns/aaaa_record.rs");
    include!("resources/dns/c_name_record.rs");
    include!("resources/dns/caa_record.rs");
    include!("resources/dns/mx_record.rs");
    include!("resources/dns/ns_record.rs");
    include!("resources/dns/ptr_record.rs");
    include!("resources/dns/srv_record.rs");
    include!("resources/dns/txt_record.rs");
    include!("resources/dns/zone.rs");
}
pub mod functions {
    pub mod databricks {
        include!("functions/databricks/get_access_connector.rs");
        include!("functions/databricks/get_workspace.rs");
        include!("functions/databricks/get_workspace_private_endpoint_connection.rs");
    }
    pub mod datafactory {
        include!("functions/datafactory/get_factory.rs");
        include!("functions/datafactory/get_trigger_schedule.rs");
        include!("functions/datafactory/get_trigger_schedules.rs");
    }
    pub mod dataprotection {
        include!("functions/dataprotection/get_backup_vault.rs");
    }
    pub mod datashare {
        include!("functions/datashare/get_account.rs");
        include!("functions/datashare/get_dataset_blob_storage.rs");
        include!("functions/datashare/get_dataset_data_lake_gen_2.rs");
        include!("functions/datashare/get_dataset_kusto_cluster.rs");
        include!("functions/datashare/get_dataset_kusto_database.rs");
        include!("functions/datashare/get_share.rs");
    }
    pub mod desktopvirtualization {
        include!("functions/desktopvirtualization/get_application_group.rs");
        include!("functions/desktopvirtualization/get_host_pool.rs");
        include!("functions/desktopvirtualization/get_workspace.rs");
    }
    pub mod devtest {
        include!("functions/devtest/get_lab.rs");
        include!("functions/devtest/get_virtual_network.rs");
    }
    pub mod digitaltwins {
        include!("functions/digitaltwins/get_instance.rs");
    }
    pub mod dns {
        include!("functions/dns/get_aaaa_record.rs");
        include!("functions/dns/get_a_record.rs");
        include!("functions/dns/get_caa_record.rs");
        include!("functions/dns/get_cname_record.rs");
        include!("functions/dns/get_mx_record.rs");
        include!("functions/dns/get_ns_record.rs");
        include!("functions/dns/get_ptr_record.rs");
        include!("functions/dns/get_soa_record.rs");
        include!("functions/dns/get_srv_record.rs");
        include!("functions/dns/get_txt_record.rs");
        include!("functions/dns/get_zone.rs");
    }
}
pub mod types {
    pub mod databricks {
        include!("types/databricks/access_connector_identity.rs");
        include!("types/databricks/workspace_custom_parameters.rs");
        include!("types/databricks/workspace_enhanced_security_compliance.rs");
        include!("types/databricks/workspace_managed_disk_identity.rs");
        include!("types/databricks/workspace_storage_account_identity.rs");
        include!("types/databricks/get_access_connector_identity.rs");
        include!("types/databricks/get_workspace_enhanced_security_compliance.rs");
        include!("types/databricks/get_workspace_managed_disk_identity.rs");
        include!(
            "types/databricks/get_workspace_private_endpoint_connection_connection.rs"
        );
        include!("types/databricks/get_workspace_storage_account_identity.rs");
    }
    pub mod datadog {
        include!("types/datadog/monitor_datadog_organization.rs");
        include!("types/datadog/monitor_identity.rs");
        include!("types/datadog/monitor_tag_rule_log.rs");
        include!("types/datadog/monitor_tag_rule_log_filter.rs");
        include!("types/datadog/monitor_tag_rule_metric.rs");
        include!("types/datadog/monitor_tag_rule_metric_filter.rs");
        include!("types/datadog/monitor_user.rs");
    }
    pub mod datafactory {
        include!(
            "types/datafactory/credential_service_principal_service_principal_key.rs"
        );
        include!("types/datafactory/custom_dataset_linked_service.rs");
        include!("types/datafactory/data_flow_sink.rs");
        include!("types/datafactory/data_flow_sink_dataset.rs");
        include!("types/datafactory/data_flow_sink_flowlet.rs");
        include!("types/datafactory/data_flow_sink_linked_service.rs");
        include!("types/datafactory/data_flow_sink_rejected_linked_service.rs");
        include!("types/datafactory/data_flow_sink_schema_linked_service.rs");
        include!("types/datafactory/data_flow_source.rs");
        include!("types/datafactory/data_flow_source_dataset.rs");
        include!("types/datafactory/data_flow_source_flowlet.rs");
        include!("types/datafactory/data_flow_source_linked_service.rs");
        include!("types/datafactory/data_flow_source_rejected_linked_service.rs");
        include!("types/datafactory/data_flow_source_schema_linked_service.rs");
        include!("types/datafactory/data_flow_transformation.rs");
        include!("types/datafactory/data_flow_transformation_dataset.rs");
        include!("types/datafactory/data_flow_transformation_flowlet.rs");
        include!("types/datafactory/data_flow_transformation_linked_service.rs");
        include!("types/datafactory/dataset_azure_blob_schema_column.rs");
        include!("types/datafactory/dataset_azure_sql_table_schema_column.rs");
        include!("types/datafactory/dataset_binary_azure_blob_storage_location.rs");
        include!("types/datafactory/dataset_binary_compression.rs");
        include!("types/datafactory/dataset_binary_http_server_location.rs");
        include!("types/datafactory/dataset_binary_sftp_server_location.rs");
        include!("types/datafactory/dataset_cosmos_db_api_schema_column.rs");
        include!("types/datafactory/dataset_delimited_text_azure_blob_fs_location.rs");
        include!(
            "types/datafactory/dataset_delimited_text_azure_blob_storage_location.rs"
        );
        include!("types/datafactory/dataset_delimited_text_http_server_location.rs");
        include!("types/datafactory/dataset_delimited_text_schema_column.rs");
        include!("types/datafactory/dataset_http_schema_column.rs");
        include!("types/datafactory/dataset_json_azure_blob_storage_location.rs");
        include!("types/datafactory/dataset_json_http_server_location.rs");
        include!("types/datafactory/dataset_json_schema_column.rs");
        include!("types/datafactory/dataset_mysql_schema_column.rs");
        include!("types/datafactory/dataset_parquet_azure_blob_fs_location.rs");
        include!("types/datafactory/dataset_parquet_azure_blob_storage_location.rs");
        include!("types/datafactory/dataset_parquet_http_server_location.rs");
        include!("types/datafactory/dataset_parquet_schema_column.rs");
        include!("types/datafactory/dataset_postgresql_schema_column.rs");
        include!("types/datafactory/dataset_snowflake_schema_column.rs");
        include!("types/datafactory/dataset_sql_server_table_schema_column.rs");
        include!("types/datafactory/factory_github_configuration.rs");
        include!("types/datafactory/factory_global_parameter.rs");
        include!("types/datafactory/factory_identity.rs");
        include!("types/datafactory/factory_vsts_configuration.rs");
        include!("types/datafactory/flowlet_data_flow_sink.rs");
        include!("types/datafactory/flowlet_data_flow_sink_dataset.rs");
        include!("types/datafactory/flowlet_data_flow_sink_flowlet.rs");
        include!("types/datafactory/flowlet_data_flow_sink_linked_service.rs");
        include!("types/datafactory/flowlet_data_flow_sink_rejected_linked_service.rs");
        include!("types/datafactory/flowlet_data_flow_sink_schema_linked_service.rs");
        include!("types/datafactory/flowlet_data_flow_source.rs");
        include!("types/datafactory/flowlet_data_flow_source_dataset.rs");
        include!("types/datafactory/flowlet_data_flow_source_flowlet.rs");
        include!("types/datafactory/flowlet_data_flow_source_linked_service.rs");
        include!(
            "types/datafactory/flowlet_data_flow_source_rejected_linked_service.rs"
        );
        include!("types/datafactory/flowlet_data_flow_source_schema_linked_service.rs");
        include!("types/datafactory/flowlet_data_flow_transformation.rs");
        include!("types/datafactory/flowlet_data_flow_transformation_dataset.rs");
        include!("types/datafactory/flowlet_data_flow_transformation_flowlet.rs");
        include!("types/datafactory/flowlet_data_flow_transformation_linked_service.rs");
        include!(
            "types/datafactory/integration_runtime_self_hosted_rbac_authorization.rs"
        );
        include!("types/datafactory/integration_runtime_ssis_catalog_info.rs");
        include!("types/datafactory/integration_runtime_ssis_copy_compute_scale.rs");
        include!("types/datafactory/integration_runtime_ssis_custom_setup_script.rs");
        include!("types/datafactory/integration_runtime_ssis_express_custom_setup.rs");
        include!(
            "types/datafactory/integration_runtime_ssis_express_custom_setup_command_key.rs"
        );
        include!(
            "types/datafactory/integration_runtime_ssis_express_custom_setup_command_key_key_vault_password.rs"
        );
        include!(
            "types/datafactory/integration_runtime_ssis_express_custom_setup_component.rs"
        );
        include!(
            "types/datafactory/integration_runtime_ssis_express_custom_setup_component_key_vault_license.rs"
        );
        include!(
            "types/datafactory/integration_runtime_ssis_express_vnet_integration.rs"
        );
        include!("types/datafactory/integration_runtime_ssis_package_store.rs");
        include!(
            "types/datafactory/integration_runtime_ssis_pipeline_external_compute_scale.rs"
        );
        include!("types/datafactory/integration_runtime_ssis_proxy.rs");
        include!("types/datafactory/integration_runtime_ssis_vnet_integration.rs");
        include!("types/datafactory/linked_custom_service_integration_runtime.rs");
        include!(
            "types/datafactory/linked_service_azure_blob_storage_key_vault_sas_token.rs"
        );
        include!(
            "types/datafactory/linked_service_azure_blob_storage_service_principal_linked_key_vault_key.rs"
        );
        include!("types/datafactory/linked_service_azure_databricks_instance_pool.rs");
        include!(
            "types/datafactory/linked_service_azure_databricks_key_vault_password.rs"
        );
        include!(
            "types/datafactory/linked_service_azure_databricks_new_cluster_config.rs"
        );
        include!(
            "types/datafactory/linked_service_azure_file_storage_key_vault_password.rs"
        );
        include!("types/datafactory/linked_service_azure_function_key_vault_key.rs");
        include!(
            "types/datafactory/linked_service_azure_sql_database_key_vault_connection_string.rs"
        );
        include!(
            "types/datafactory/linked_service_azure_sql_database_key_vault_password.rs"
        );
        include!("types/datafactory/linked_service_odata_basic_authentication.rs");
        include!("types/datafactory/linked_service_odbc_basic_authentication.rs");
        include!("types/datafactory/linked_service_snowflake_key_vault_password.rs");
        include!(
            "types/datafactory/linked_service_sql_server_key_vault_connection_string.rs"
        );
        include!("types/datafactory/linked_service_sql_server_key_vault_password.rs");
        include!("types/datafactory/linked_service_synapse_key_vault_password.rs");
        include!("types/datafactory/trigger_blob_event_pipeline.rs");
        include!("types/datafactory/trigger_custom_event_pipeline.rs");
        include!("types/datafactory/trigger_schedule_pipeline.rs");
        include!("types/datafactory/trigger_schedule_schedule.rs");
        include!("types/datafactory/trigger_schedule_schedule_monthly.rs");
        include!("types/datafactory/trigger_tumbling_window_pipeline.rs");
        include!("types/datafactory/trigger_tumbling_window_retry.rs");
        include!("types/datafactory/trigger_tumbling_window_trigger_dependency.rs");
        include!("types/datafactory/get_factory_github_configuration.rs");
        include!("types/datafactory/get_factory_identity.rs");
        include!("types/datafactory/get_factory_vsts_configuration.rs");
        include!("types/datafactory/get_trigger_schedule_schedule.rs");
        include!("types/datafactory/get_trigger_schedule_schedule_monthly.rs");
    }
    pub mod dataprotection {
        include!(
            "types/dataprotection/backup_instance_kubernetes_cluster_backup_datasource_parameters.rs"
        );
        include!("types/dataprotection/backup_policy_blob_storage_retention_rule.rs");
        include!(
            "types/dataprotection/backup_policy_blob_storage_retention_rule_criteria.rs"
        );
        include!(
            "types/dataprotection/backup_policy_blob_storage_retention_rule_life_cycle.rs"
        );
        include!("types/dataprotection/backup_policy_disk_retention_rule.rs");
        include!("types/dataprotection/backup_policy_disk_retention_rule_criteria.rs");
        include!(
            "types/dataprotection/backup_policy_kubernetes_cluster_default_retention_rule.rs"
        );
        include!(
            "types/dataprotection/backup_policy_kubernetes_cluster_default_retention_rule_life_cycle.rs"
        );
        include!(
            "types/dataprotection/backup_policy_kubernetes_cluster_retention_rule.rs"
        );
        include!(
            "types/dataprotection/backup_policy_kubernetes_cluster_retention_rule_criteria.rs"
        );
        include!(
            "types/dataprotection/backup_policy_kubernetes_cluster_retention_rule_life_cycle.rs"
        );
        include!(
            "types/dataprotection/backup_policy_mysql_flexible_server_default_retention_rule.rs"
        );
        include!(
            "types/dataprotection/backup_policy_mysql_flexible_server_default_retention_rule_life_cycle.rs"
        );
        include!(
            "types/dataprotection/backup_policy_mysql_flexible_server_retention_rule.rs"
        );
        include!(
            "types/dataprotection/backup_policy_mysql_flexible_server_retention_rule_criteria.rs"
        );
        include!(
            "types/dataprotection/backup_policy_mysql_flexible_server_retention_rule_life_cycle.rs"
        );
        include!(
            "types/dataprotection/backup_policy_postgresql_flexible_server_default_retention_rule.rs"
        );
        include!(
            "types/dataprotection/backup_policy_postgresql_flexible_server_default_retention_rule_life_cycle.rs"
        );
        include!(
            "types/dataprotection/backup_policy_postgresql_flexible_server_retention_rule.rs"
        );
        include!(
            "types/dataprotection/backup_policy_postgresql_flexible_server_retention_rule_criteria.rs"
        );
        include!(
            "types/dataprotection/backup_policy_postgresql_flexible_server_retention_rule_life_cycle.rs"
        );
        include!("types/dataprotection/backup_policy_postgresql_retention_rule.rs");
        include!(
            "types/dataprotection/backup_policy_postgresql_retention_rule_criteria.rs"
        );
        include!("types/dataprotection/backup_vault_identity.rs");
        include!("types/dataprotection/get_backup_vault_identity.rs");
    }
    pub mod datashare {
        include!("types/datashare/account_identity.rs");
        include!("types/datashare/dataset_blob_storage_storage_account.rs");
        include!("types/datashare/share_snapshot_schedule.rs");
        include!("types/datashare/get_account_identity.rs");
        include!("types/datashare/get_dataset_blob_storage_storage_account.rs");
        include!("types/datashare/get_share_snapshot_schedule.rs");
    }
    pub mod desktopvirtualization {
        include!("types/desktopvirtualization/host_pool_scheduled_agent_updates.rs");
        include!(
            "types/desktopvirtualization/host_pool_scheduled_agent_updates_schedule.rs"
        );
        include!("types/desktopvirtualization/scaling_plan_host_pool.rs");
        include!("types/desktopvirtualization/scaling_plan_schedule.rs");
        include!("types/desktopvirtualization/get_host_pool_scheduled_agent_update.rs");
        include!(
            "types/desktopvirtualization/get_host_pool_scheduled_agent_update_schedule.rs"
        );
    }
    pub mod devcenter {
        include!("types/devcenter/catalog_catalog_adogit.rs");
        include!("types/devcenter/catalog_catalog_github.rs");
        include!("types/devcenter/dev_center_identity.rs");
        include!("types/devcenter/project_environment_type_identity.rs");
        include!("types/devcenter/project_environment_type_user_role_assignment.rs");
    }
    pub mod devtest {
        include!("types/devtest/global_vm_shutdown_schedule_notification_settings.rs");
        include!("types/devtest/linux_virtual_machine_gallery_image_reference.rs");
        include!("types/devtest/linux_virtual_machine_inbound_nat_rule.rs");
        include!("types/devtest/schedule_daily_recurrence.rs");
        include!("types/devtest/schedule_hourly_recurrence.rs");
        include!("types/devtest/schedule_notification_settings.rs");
        include!("types/devtest/schedule_weekly_recurrence.rs");
        include!("types/devtest/virtual_network_subnet.rs");
        include!("types/devtest/virtual_network_subnet_shared_public_ip_address.rs");
        include!(
            "types/devtest/virtual_network_subnet_shared_public_ip_address_allowed_port.rs"
        );
        include!("types/devtest/windows_virtual_machine_gallery_image_reference.rs");
        include!("types/devtest/windows_virtual_machine_inbound_nat_rule.rs");
        include!("types/devtest/get_virtual_network_allowed_subnet.rs");
        include!("types/devtest/get_virtual_network_subnet_override.rs");
    }
    pub mod digitaltwins {
        include!("types/digitaltwins/instance_identity.rs");
    }
    pub mod dns {
        include!("types/dns/caa_record_record.rs");
        include!("types/dns/mx_record_record.rs");
        include!("types/dns/srv_record_record.rs");
        include!("types/dns/txt_record_record.rs");
        include!("types/dns/zone_soa_record.rs");
        include!("types/dns/get_caa_record_record.rs");
        include!("types/dns/get_mx_record_record.rs");
        include!("types/dns/get_srv_record_record.rs");
        include!("types/dns/get_txt_record_record.rs");
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
