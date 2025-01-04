pub mod containerservice {
    include!("resources/containerservice/cluster_trusted_access_role_binding.rs");
    include!("resources/containerservice/connected_registry.rs");
    include!("resources/containerservice/fleet_member.rs");
    include!("resources/containerservice/fleet_update_run.rs");
    include!("resources/containerservice/fleet_update_strategy.rs");
    include!("resources/containerservice/flux_configuration.rs");
    include!("resources/containerservice/group.rs");
    include!("resources/containerservice/kubernetes_cluster.rs");
    include!("resources/containerservice/kubernetes_cluster_extension.rs");
    include!("resources/containerservice/kubernetes_cluster_node_pool.rs");
    include!("resources/containerservice/kubernetes_fleet_manager.rs");
    include!("resources/containerservice/registry.rs");
    include!("resources/containerservice/registry_agent_pool.rs");
    include!("resources/containerservice/registry_cache_rule.rs");
    include!("resources/containerservice/registry_scope_map.rs");
    include!("resources/containerservice/registry_task.rs");
    include!("resources/containerservice/registry_task_schedule_run_now.rs");
    include!("resources/containerservice/registry_token.rs");
    include!("resources/containerservice/registry_webhook.rs");
    include!("resources/containerservice/registry_webook.rs");
    include!("resources/containerservice/token_password.rs");
}
pub mod functions {
    pub mod containerservice {
        include!("functions/containerservice/get_cluster_node_pool.rs");
        include!("functions/containerservice/get_group.rs");
        include!("functions/containerservice/get_kubernetes_cluster.rs");
        include!("functions/containerservice/get_kubernetes_node_pool_snapshot.rs");
        include!("functions/containerservice/get_kubernetes_service_versions.rs");
        include!("functions/containerservice/get_registry.rs");
        include!("functions/containerservice/get_registry_cache_rule.rs");
        include!("functions/containerservice/get_registry_scope_map.rs");
        include!("functions/containerservice/get_registry_token.rs");
    }
}
pub mod types {
    pub mod containerservice {
        include!("types/containerservice/connected_registry_notification.rs");
        include!("types/containerservice/fleet_update_run_managed_cluster_update.rs");
        include!(
            "types/containerservice/fleet_update_run_managed_cluster_update_node_image_selection.rs"
        );
        include!(
            "types/containerservice/fleet_update_run_managed_cluster_update_upgrade.rs"
        );
        include!("types/containerservice/fleet_update_run_stage.rs");
        include!("types/containerservice/fleet_update_run_stage_group.rs");
        include!("types/containerservice/fleet_update_strategy_stage.rs");
        include!("types/containerservice/fleet_update_strategy_stage_group.rs");
        include!("types/containerservice/flux_configuration_blob_storage.rs");
        include!(
            "types/containerservice/flux_configuration_blob_storage_managed_identity.rs"
        );
        include!(
            "types/containerservice/flux_configuration_blob_storage_service_principal.rs"
        );
        include!("types/containerservice/flux_configuration_bucket.rs");
        include!("types/containerservice/flux_configuration_git_repository.rs");
        include!("types/containerservice/flux_configuration_kustomization.rs");
        include!("types/containerservice/group_container.rs");
        include!("types/containerservice/group_container_liveness_probe.rs");
        include!("types/containerservice/group_container_liveness_probe_http_get.rs");
        include!("types/containerservice/group_container_port.rs");
        include!("types/containerservice/group_container_readiness_probe.rs");
        include!("types/containerservice/group_container_readiness_probe_http_get.rs");
        include!("types/containerservice/group_container_security.rs");
        include!("types/containerservice/group_container_volume.rs");
        include!("types/containerservice/group_container_volume_git_repo.rs");
        include!("types/containerservice/group_diagnostics.rs");
        include!("types/containerservice/group_diagnostics_log_analytics.rs");
        include!("types/containerservice/group_dns_config.rs");
        include!("types/containerservice/group_exposed_port.rs");
        include!("types/containerservice/group_identity.rs");
        include!("types/containerservice/group_image_registry_credential.rs");
        include!("types/containerservice/group_init_container.rs");
        include!("types/containerservice/group_init_container_security.rs");
        include!("types/containerservice/group_init_container_volume.rs");
        include!("types/containerservice/group_init_container_volume_git_repo.rs");
        include!("types/containerservice/kubernetes_cluster_aci_connector_linux.rs");
        include!(
            "types/containerservice/kubernetes_cluster_aci_connector_linux_connector_identity.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_api_server_access_profile.rs"
        );
        include!("types/containerservice/kubernetes_cluster_auto_scaler_profile.rs");
        include!(
            "types/containerservice/kubernetes_cluster_azure_active_directory_role_based_access_control.rs"
        );
        include!("types/containerservice/kubernetes_cluster_confidential_computing.rs");
        include!("types/containerservice/kubernetes_cluster_default_node_pool.rs");
        include!(
            "types/containerservice/kubernetes_cluster_default_node_pool_kubelet_config.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_default_node_pool_linux_os_config.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_default_node_pool_linux_os_config_sysctl_config.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_default_node_pool_node_network_profile.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_default_node_pool_node_network_profile_allowed_host_port.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_default_node_pool_upgrade_settings.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_extension_aks_assigned_identity.rs"
        );
        include!("types/containerservice/kubernetes_cluster_extension_plan.rs");
        include!("types/containerservice/kubernetes_cluster_http_proxy_config.rs");
        include!("types/containerservice/kubernetes_cluster_identity.rs");
        include!(
            "types/containerservice/kubernetes_cluster_ingress_application_gateway.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_ingress_application_gateway_ingress_application_gateway_identity.rs"
        );
        include!("types/containerservice/kubernetes_cluster_key_management_service.rs");
        include!(
            "types/containerservice/kubernetes_cluster_key_vault_secrets_provider.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_key_vault_secrets_provider_secret_identity.rs"
        );
        include!("types/containerservice/kubernetes_cluster_kube_admin_config.rs");
        include!("types/containerservice/kubernetes_cluster_kube_config.rs");
        include!("types/containerservice/kubernetes_cluster_kubelet_identity.rs");
        include!("types/containerservice/kubernetes_cluster_linux_profile.rs");
        include!("types/containerservice/kubernetes_cluster_linux_profile_ssh_key.rs");
        include!("types/containerservice/kubernetes_cluster_maintenance_window.rs");
        include!(
            "types/containerservice/kubernetes_cluster_maintenance_window_allowed.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_maintenance_window_auto_upgrade.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_maintenance_window_auto_upgrade_not_allowed.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_maintenance_window_node_os.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_maintenance_window_node_os_not_allowed.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_maintenance_window_not_allowed.rs"
        );
        include!("types/containerservice/kubernetes_cluster_microsoft_defender.rs");
        include!("types/containerservice/kubernetes_cluster_monitor_metrics.rs");
        include!("types/containerservice/kubernetes_cluster_network_profile.rs");
        include!(
            "types/containerservice/kubernetes_cluster_network_profile_load_balancer_profile.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_network_profile_nat_gateway_profile.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_node_pool_kubelet_config.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_node_pool_linux_os_config.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_node_pool_linux_os_config_sysctl_config.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_node_pool_node_network_profile.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_node_pool_node_network_profile_allowed_host_port.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_node_pool_upgrade_settings.rs"
        );
        include!(
            "types/containerservice/kubernetes_cluster_node_pool_windows_profile.rs"
        );
        include!("types/containerservice/kubernetes_cluster_oms_agent.rs");
        include!(
            "types/containerservice/kubernetes_cluster_oms_agent_oms_agent_identity.rs"
        );
        include!("types/containerservice/kubernetes_cluster_service_mesh_profile.rs");
        include!(
            "types/containerservice/kubernetes_cluster_service_mesh_profile_certificate_authority.rs"
        );
        include!("types/containerservice/kubernetes_cluster_service_principal.rs");
        include!("types/containerservice/kubernetes_cluster_storage_profile.rs");
        include!("types/containerservice/kubernetes_cluster_web_app_routing.rs");
        include!(
            "types/containerservice/kubernetes_cluster_web_app_routing_web_app_routing_identity.rs"
        );
        include!("types/containerservice/kubernetes_cluster_windows_profile.rs");
        include!("types/containerservice/kubernetes_cluster_windows_profile_gmsa.rs");
        include!(
            "types/containerservice/kubernetes_cluster_workload_autoscaler_profile.rs"
        );
        include!("types/containerservice/kubernetes_fleet_manager_hub_profile.rs");
        include!("types/containerservice/registry_encryption.rs");
        include!("types/containerservice/registry_georeplication.rs");
        include!("types/containerservice/registry_identity.rs");
        include!("types/containerservice/registry_network_rule_set.rs");
        include!("types/containerservice/registry_network_rule_set_ip_rule.rs");
        include!("types/containerservice/registry_task_agent_setting.rs");
        include!("types/containerservice/registry_task_base_image_trigger.rs");
        include!("types/containerservice/registry_task_docker_step.rs");
        include!("types/containerservice/registry_task_encoded_step.rs");
        include!("types/containerservice/registry_task_file_step.rs");
        include!("types/containerservice/registry_task_identity.rs");
        include!("types/containerservice/registry_task_platform.rs");
        include!("types/containerservice/registry_task_registry_credential.rs");
        include!("types/containerservice/registry_task_registry_credential_custom.rs");
        include!("types/containerservice/registry_task_registry_credential_source.rs");
        include!("types/containerservice/registry_task_source_trigger.rs");
        include!(
            "types/containerservice/registry_task_source_trigger_authentication.rs"
        );
        include!("types/containerservice/registry_task_timer_trigger.rs");
        include!("types/containerservice/token_password_password_1.rs");
        include!("types/containerservice/token_password_password_2.rs");
        include!("types/containerservice/get_cluster_node_pool_upgrade_setting.rs");
        include!("types/containerservice/get_group_identity.rs");
        include!("types/containerservice/get_kubernetes_cluster_aci_connector_linux.rs");
        include!("types/containerservice/get_kubernetes_cluster_agent_pool_profile.rs");
        include!(
            "types/containerservice/get_kubernetes_cluster_agent_pool_profile_upgrade_setting.rs"
        );
        include!(
            "types/containerservice/get_kubernetes_cluster_azure_active_directory_role_based_access_control.rs"
        );
        include!("types/containerservice/get_kubernetes_cluster_identity.rs");
        include!(
            "types/containerservice/get_kubernetes_cluster_ingress_application_gateway.rs"
        );
        include!(
            "types/containerservice/get_kubernetes_cluster_ingress_application_gateway_ingress_application_gateway_identity.rs"
        );
        include!(
            "types/containerservice/get_kubernetes_cluster_key_management_service.rs"
        );
        include!(
            "types/containerservice/get_kubernetes_cluster_key_vault_secrets_provider.rs"
        );
        include!(
            "types/containerservice/get_kubernetes_cluster_key_vault_secrets_provider_secret_identity.rs"
        );
        include!("types/containerservice/get_kubernetes_cluster_kube_admin_config.rs");
        include!("types/containerservice/get_kubernetes_cluster_kube_config.rs");
        include!("types/containerservice/get_kubernetes_cluster_kubelet_identity.rs");
        include!("types/containerservice/get_kubernetes_cluster_linux_profile.rs");
        include!(
            "types/containerservice/get_kubernetes_cluster_linux_profile_ssh_key.rs"
        );
        include!("types/containerservice/get_kubernetes_cluster_microsoft_defender.rs");
        include!("types/containerservice/get_kubernetes_cluster_network_profile.rs");
        include!("types/containerservice/get_kubernetes_cluster_oms_agent.rs");
        include!(
            "types/containerservice/get_kubernetes_cluster_oms_agent_oms_agent_identity.rs"
        );
        include!(
            "types/containerservice/get_kubernetes_cluster_service_mesh_profile.rs"
        );
        include!(
            "types/containerservice/get_kubernetes_cluster_service_mesh_profile_certificate_authority.rs"
        );
        include!("types/containerservice/get_kubernetes_cluster_service_principal.rs");
        include!("types/containerservice/get_kubernetes_cluster_storage_profile.rs");
        include!("types/containerservice/get_kubernetes_cluster_windows_profile.rs");
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
