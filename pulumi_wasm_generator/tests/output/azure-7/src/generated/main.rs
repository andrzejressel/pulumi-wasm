pub mod kusto {
    include!("resources/kusto/attached_database_configuration.rs");
    include!("resources/kusto/cluster.rs");
    include!("resources/kusto/cluster_customer_managed_key.rs");
    include!("resources/kusto/cluster_managed_private_endpoint.rs");
    include!("resources/kusto/cluster_principal_assignment.rs");
    include!("resources/kusto/cosmosdb_data_connection.rs");
    include!("resources/kusto/database.rs");
    include!("resources/kusto/database_principal_assignment.rs");
    include!("resources/kusto/event_grid_data_connection.rs");
    include!("resources/kusto/eventhub_data_connection.rs");
    include!("resources/kusto/iot_hub_data_connection.rs");
    include!("resources/kusto/script.rs");
}
pub mod lb {
    include!("resources/lb/backend_address_pool.rs");
    include!("resources/lb/backend_address_pool_address.rs");
    include!("resources/lb/load_balancer.rs");
    include!("resources/lb/nat_pool.rs");
    include!("resources/lb/nat_rule.rs");
    include!("resources/lb/outbound_rule.rs");
    include!("resources/lb/probe.rs");
    include!("resources/lb/rule.rs");
}
pub mod lighthouse {
    include!("resources/lighthouse/assignment.rs");
    include!("resources/lighthouse/definition.rs");
}
pub mod loadtest {
    include!("resources/loadtest/load_test.rs");
}
pub mod loganalytics {
    include!("resources/loganalytics/cluster.rs");
    include!("resources/loganalytics/cluster_customer_managed_key.rs");
    include!("resources/loganalytics/data_export_rule.rs");
    include!("resources/loganalytics/data_source_windows_event.rs");
    include!("resources/loganalytics/data_source_windows_performance_counter.rs");
    include!("resources/loganalytics/linked_service.rs");
    include!("resources/loganalytics/linked_storage_account.rs");
    include!("resources/loganalytics/query_pack.rs");
    include!("resources/loganalytics/saved_search.rs");
    include!("resources/loganalytics/storage_insights.rs");
    include!("resources/loganalytics/workspace_table.rs");
}
pub mod logicapps {
    include!("resources/logicapps/action_custom.rs");
    include!("resources/logicapps/action_http.rs");
    include!("resources/logicapps/integration_account.rs");
    include!("resources/logicapps/integration_account_agreement.rs");
    include!("resources/logicapps/integration_account_assembly.rs");
    include!("resources/logicapps/integration_account_batch_configuration.rs");
    include!("resources/logicapps/integration_account_certificate.rs");
    include!("resources/logicapps/integration_account_map.rs");
    include!("resources/logicapps/integration_account_partner.rs");
    include!("resources/logicapps/integration_account_schema.rs");
    include!("resources/logicapps/integration_account_session.rs");
    include!("resources/logicapps/standard.rs");
    include!("resources/logicapps/trigger_custom.rs");
    include!("resources/logicapps/trigger_http_request.rs");
    include!("resources/logicapps/trigger_recurrence.rs");
    include!("resources/logicapps/workflow.rs");
}
pub mod machinelearning {
    include!("resources/machinelearning/compute_cluster.rs");
    include!("resources/machinelearning/compute_instance.rs");
    include!("resources/machinelearning/datastore_blobstorage.rs");
    include!("resources/machinelearning/datastore_datalake_gen_2.rs");
    include!("resources/machinelearning/datastore_fileshare.rs");
    include!("resources/machinelearning/inference_cluster.rs");
    include!("resources/machinelearning/synapse_spark.rs");
    include!("resources/machinelearning/workspace.rs");
    include!("resources/machinelearning/workspace_network_outbound_rule_fqdn.rs");
}
pub mod maintenance {
    include!("resources/maintenance/assignment_dedicated_host.rs");
    include!("resources/maintenance/assignment_dynamic_scope.rs");
    include!("resources/maintenance/assignment_virtual_machine.rs");
    include!("resources/maintenance/assignment_virtual_machine_scale_set.rs");
    include!("resources/maintenance/configuration.rs");
}
pub mod managedapplication {
    include!("resources/managedapplication/application.rs");
    include!("resources/managedapplication/definition.rs");
}
pub mod managedlustre {
    include!("resources/managedlustre/file_system.rs");
}
pub mod functions {
    pub mod kusto {
        include!("functions/kusto/get_cluster.rs");
        include!("functions/kusto/get_database.rs");
    }
    pub mod lb {
        include!("functions/lb/get_backend_address_pool.rs");
        include!("functions/lb/get_lb.rs");
        include!("functions/lb/get_lb_outbound_rule.rs");
        include!("functions/lb/get_lb_rule.rs");
    }
    pub mod loadtest {
        include!("functions/loadtest/get.rs");
    }
    pub mod logicapps {
        include!("functions/logicapps/get_integration_account.rs");
        include!("functions/logicapps/get_standard.rs");
        include!("functions/logicapps/get_workflow.rs");
    }
    pub mod machinelearning {
        include!("functions/machinelearning/get_workspace.rs");
    }
    pub mod maintenance {
        include!("functions/maintenance/get_configuration.rs");
        include!("functions/maintenance/get_public_configurations.rs");
    }
    pub mod managedapplication {
        include!("functions/managedapplication/get_definition.rs");
    }
}
pub mod types {
    pub mod kusto {
        include!("types/kusto/attached_database_configuration_sharing.rs");
        include!("types/kusto/cluster_identity.rs");
        include!("types/kusto/cluster_language_extension.rs");
        include!("types/kusto/cluster_optimized_auto_scale.rs");
        include!("types/kusto/cluster_sku.rs");
        include!("types/kusto/cluster_virtual_network_configuration.rs");
        include!("types/kusto/get_cluster_identity.rs");
    }
    pub mod lb {
        include!(
            "types/lb/backend_address_pool_address_inbound_nat_rule_port_mapping.rs"
        );
        include!("types/lb/backend_address_pool_tunnel_interface.rs");
        include!("types/lb/load_balancer_frontend_ip_configuration.rs");
        include!("types/lb/outbound_rule_frontend_ip_configuration.rs");
        include!("types/lb/get_backend_address_pool_backend_address.rs");
        include!(
            "types/lb/get_backend_address_pool_backend_address_inbound_nat_rule_port_mapping.rs"
        );
        include!("types/lb/get_backend_address_pool_backend_ip_configuration.rs");
        include!("types/lb/get_lb_frontend_ip_configuration.rs");
        include!("types/lb/get_lb_outbound_rule_frontend_ip_configuration.rs");
    }
    pub mod lighthouse {
        include!("types/lighthouse/definition_authorization.rs");
        include!("types/lighthouse/definition_eligible_authorization.rs");
        include!(
            "types/lighthouse/definition_eligible_authorization_just_in_time_access_policy.rs"
        );
        include!(
            "types/lighthouse/definition_eligible_authorization_just_in_time_access_policy_approver.rs"
        );
        include!("types/lighthouse/definition_plan.rs");
    }
    pub mod loadtest {
        include!("types/loadtest/load_test_encryption.rs");
        include!("types/loadtest/load_test_encryption_identity.rs");
        include!("types/loadtest/load_test_identity.rs");
        include!("types/loadtest/get_encryption.rs");
        include!("types/loadtest/get_encryption_identity.rs");
        include!("types/loadtest/get_identity.rs");
    }
    pub mod loganalytics {
        include!("types/loganalytics/cluster_identity.rs");
    }
    pub mod logicapps {
        include!("types/logicapps/action_http_run_after.rs");
        include!("types/logicapps/integration_account_agreement_guest_identity.rs");
        include!("types/logicapps/integration_account_agreement_host_identity.rs");
        include!(
            "types/logicapps/integration_account_batch_configuration_release_criteria.rs"
        );
        include!(
            "types/logicapps/integration_account_batch_configuration_release_criteria_recurrence.rs"
        );
        include!(
            "types/logicapps/integration_account_batch_configuration_release_criteria_recurrence_schedule.rs"
        );
        include!(
            "types/logicapps/integration_account_batch_configuration_release_criteria_recurrence_schedule_monthly.rs"
        );
        include!("types/logicapps/integration_account_certificate_key_vault_key.rs");
        include!("types/logicapps/integration_account_partner_business_identity.rs");
        include!("types/logicapps/standard_connection_string.rs");
        include!("types/logicapps/standard_identity.rs");
        include!("types/logicapps/standard_site_config.rs");
        include!("types/logicapps/standard_site_config_cors.rs");
        include!("types/logicapps/standard_site_config_ip_restriction.rs");
        include!("types/logicapps/standard_site_config_ip_restriction_headers.rs");
        include!("types/logicapps/standard_site_config_scm_ip_restriction.rs");
        include!("types/logicapps/standard_site_config_scm_ip_restriction_headers.rs");
        include!("types/logicapps/standard_site_credential.rs");
        include!("types/logicapps/trigger_recurrence_schedule.rs");
        include!("types/logicapps/workflow_access_control.rs");
        include!("types/logicapps/workflow_access_control_action.rs");
        include!("types/logicapps/workflow_access_control_content.rs");
        include!("types/logicapps/workflow_access_control_trigger.rs");
        include!(
            "types/logicapps/workflow_access_control_trigger_open_authentication_policy.rs"
        );
        include!(
            "types/logicapps/workflow_access_control_trigger_open_authentication_policy_claim.rs"
        );
        include!("types/logicapps/workflow_access_control_workflow_management.rs");
        include!("types/logicapps/workflow_identity.rs");
        include!("types/logicapps/get_standard_connection_string.rs");
        include!("types/logicapps/get_standard_identity.rs");
        include!("types/logicapps/get_standard_site_config.rs");
        include!("types/logicapps/get_standard_site_config_cors.rs");
        include!("types/logicapps/get_standard_site_config_ip_restriction.rs");
        include!("types/logicapps/get_standard_site_config_ip_restriction_headers.rs");
        include!("types/logicapps/get_standard_site_config_scm_ip_restriction.rs");
        include!(
            "types/logicapps/get_standard_site_config_scm_ip_restriction_headers.rs"
        );
        include!("types/logicapps/get_standard_site_credential.rs");
        include!("types/logicapps/get_workflow_identity.rs");
    }
    pub mod machinelearning {
        include!("types/machinelearning/compute_cluster_identity.rs");
        include!("types/machinelearning/compute_cluster_scale_settings.rs");
        include!("types/machinelearning/compute_cluster_ssh.rs");
        include!("types/machinelearning/compute_instance_assign_to_user.rs");
        include!("types/machinelearning/compute_instance_identity.rs");
        include!("types/machinelearning/compute_instance_ssh.rs");
        include!("types/machinelearning/inference_cluster_identity.rs");
        include!("types/machinelearning/inference_cluster_ssl.rs");
        include!("types/machinelearning/synapse_spark_identity.rs");
        include!("types/machinelearning/workspace_encryption.rs");
        include!("types/machinelearning/workspace_feature_store.rs");
        include!("types/machinelearning/workspace_identity.rs");
        include!("types/machinelearning/workspace_managed_network.rs");
        include!("types/machinelearning/workspace_serverless_compute.rs");
        include!("types/machinelearning/get_workspace_identity.rs");
    }
    pub mod maintenance {
        include!("types/maintenance/assignment_dynamic_scope_filter.rs");
        include!("types/maintenance/assignment_dynamic_scope_filter_tag.rs");
        include!("types/maintenance/configuration_install_patches.rs");
        include!("types/maintenance/configuration_install_patches_linux.rs");
        include!("types/maintenance/configuration_install_patches_window.rs");
        include!("types/maintenance/configuration_window.rs");
        include!("types/maintenance/get_configuration_install_patch.rs");
        include!("types/maintenance/get_configuration_install_patch_linux.rs");
        include!("types/maintenance/get_configuration_install_patch_window.rs");
        include!("types/maintenance/get_configuration_window.rs");
        include!("types/maintenance/get_public_configurations_config.rs");
    }
    pub mod managedapplication {
        include!("types/managedapplication/application_plan.rs");
        include!("types/managedapplication/definition_authorization.rs");
    }
    pub mod managedlustre {
        include!("types/managedlustre/file_system_encryption_key.rs");
        include!("types/managedlustre/file_system_hsm_setting.rs");
        include!("types/managedlustre/file_system_identity.rs");
        include!("types/managedlustre/file_system_maintenance_window.rs");
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
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_AZURE: [u8; 45] = *b"{\"version\":\"6.14.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "6.14.0".to_string()
}
