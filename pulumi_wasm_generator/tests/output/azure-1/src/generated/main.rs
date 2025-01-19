pub mod arcmachine {
    include!("resources/arcmachine/arc_machine.rs");
    include!("resources/arcmachine/automanage_configuration_assignment.rs");
    include!("resources/arcmachine/extension.rs");
}
pub mod armmsi {
    include!("resources/armmsi/federated_identity_credential.rs");
}
pub mod attestation {
    include!("resources/attestation/provider.rs");
}
pub mod authorization {
    include!("resources/authorization/assignment.rs");
    include!("resources/authorization/role_definition.rs");
    include!("resources/authorization/user_assigned_identity.rs");
}
pub mod automanage {
    include!("resources/automanage/configuration.rs");
}
pub mod automation {
    include!("resources/automation/account.rs");
    include!("resources/automation/bool_variable.rs");
    include!("resources/automation/certificate.rs");
    include!("resources/automation/connection.rs");
    include!("resources/automation/connection_certificate.rs");
    include!("resources/automation/connection_classic_certificate.rs");
    include!("resources/automation/connection_service_principal.rs");
    include!("resources/automation/connection_type.rs");
    include!("resources/automation/credential.rs");
    include!("resources/automation/date_time_variable.rs");
    include!("resources/automation/dsc_configuration.rs");
    include!("resources/automation/dsc_node_configuration.rs");
    include!("resources/automation/hybrid_runbook_worker.rs");
    include!("resources/automation/hybrid_runbook_worker_group.rs");
    include!("resources/automation/int_variable.rs");
    include!("resources/automation/job_schedule.rs");
    include!("resources/automation/module.rs");
    include!("resources/automation/powershell_72_module.rs");
    include!("resources/automation/python_3_package.rs");
    include!("resources/automation/run_book.rs");
    include!("resources/automation/schedule.rs");
    include!("resources/automation/software_update_configuration.rs");
    include!("resources/automation/source_control.rs");
    include!("resources/automation/string_variable.rs");
    include!("resources/automation/variable_object.rs");
    include!("resources/automation/watcher.rs");
    include!("resources/automation/webhook.rs");
}
pub mod avs {
    include!("resources/avs/cluster.rs");
    include!("resources/avs/express_route_authorization.rs");
    include!("resources/avs/netapp_volume_attachment.rs");
    include!("resources/avs/private_cloud.rs");
}
pub mod backup {
    include!("resources/backup/container_storage_account.rs");
    include!("resources/backup/policy_file_share.rs");
    include!("resources/backup/policy_vm.rs");
    include!("resources/backup/policy_vm_workload.rs");
    include!("resources/backup/protected_file_share.rs");
    include!("resources/backup/protected_vm.rs");
}
pub mod batch {
    include!("resources/batch/account.rs");
    include!("resources/batch/application.rs");
    include!("resources/batch/certificate.rs");
    include!("resources/batch/job.rs");
    include!("resources/batch/pool.rs");
}
pub mod billing {
    include!("resources/billing/account_cost_management_export.rs");
}
pub mod functions {
    pub mod arcmachine {
        include!("functions/arcmachine/get.rs");
    }
    pub mod attestation {
        include!("functions/attestation/get_provider.rs");
    }
    pub mod authorization {
        include!("functions/authorization/get_role_definition.rs");
        include!("functions/authorization/get_user_assigned_identity.rs");
    }
    pub mod automation {
        include!("functions/automation/get_account.rs");
        include!("functions/automation/get_bool_variable.rs");
        include!("functions/automation/get_date_time_variable.rs");
        include!("functions/automation/get_int_variable.rs");
        include!("functions/automation/get_runbook.rs");
        include!("functions/automation/get_string_variable.rs");
        include!("functions/automation/get_variable_object.rs");
        include!("functions/automation/get_variables.rs");
    }
    pub mod avs {
        include!("functions/avs/get_private_cloud.rs");
    }
    pub mod backup {
        include!("functions/backup/get_policy_fileshare.rs");
        include!("functions/backup/get_policy_vm.rs");
    }
    pub mod batch {
        include!("functions/batch/get_account.rs");
        include!("functions/batch/get_application.rs");
        include!("functions/batch/get_certificate.rs");
        include!("functions/batch/get_pool.rs");
    }
    pub mod billing {
        include!("functions/billing/get_enrollment_account_scope.rs");
        include!("functions/billing/get_mca_account_scope.rs");
        include!("functions/billing/get_mpa_account_scope.rs");
    }
}
pub mod types {
    pub mod arcmachine {
        include!("types/arcmachine/arc_machine_identity.rs");
        include!("types/arcmachine/get_agent.rs");
        include!("types/arcmachine/get_agent_extensions_allow_list.rs");
        include!("types/arcmachine/get_agent_extensions_block_list.rs");
        include!("types/arcmachine/get_cloud_metadata.rs");
        include!("types/arcmachine/get_identity.rs");
        include!("types/arcmachine/get_location_data.rs");
        include!("types/arcmachine/get_os_profile.rs");
        include!("types/arcmachine/get_os_profile_linux.rs");
        include!("types/arcmachine/get_os_profile_linux_patch.rs");
        include!("types/arcmachine/get_os_profile_window.rs");
        include!("types/arcmachine/get_os_profile_window_patch.rs");
        include!("types/arcmachine/get_service_status.rs");
        include!("types/arcmachine/get_service_status_extension_service.rs");
        include!("types/arcmachine/get_service_status_guest_configuration_service.rs");
    }
    pub mod authorization {
        include!("types/authorization/role_definition_permission.rs");
        include!("types/authorization/get_role_definition_permission.rs");
    }
    pub mod automanage {
        include!("types/automanage/configuration_antimalware.rs");
        include!("types/automanage/configuration_antimalware_exclusions.rs");
        include!("types/automanage/configuration_azure_security_baseline.rs");
        include!("types/automanage/configuration_backup.rs");
        include!("types/automanage/configuration_backup_retention_policy.rs");
        include!(
            "types/automanage/configuration_backup_retention_policy_daily_schedule.rs"
        );
        include!(
            "types/automanage/configuration_backup_retention_policy_daily_schedule_retention_duration.rs"
        );
        include!(
            "types/automanage/configuration_backup_retention_policy_weekly_schedule.rs"
        );
        include!(
            "types/automanage/configuration_backup_retention_policy_weekly_schedule_retention_duration.rs"
        );
        include!("types/automanage/configuration_backup_schedule_policy.rs");
    }
    pub mod automation {
        include!("types/automation/account_encryption.rs");
        include!("types/automation/account_identity.rs");
        include!("types/automation/account_private_endpoint_connection.rs");
        include!("types/automation/connection_type_field.rs");
        include!("types/automation/module_module_link.rs");
        include!("types/automation/module_module_link_hash.rs");
        include!("types/automation/powershell_72_module_module_link.rs");
        include!("types/automation/powershell_72_module_module_link_hash.rs");
        include!("types/automation/run_book_draft.rs");
        include!("types/automation/run_book_draft_content_link.rs");
        include!("types/automation/run_book_draft_content_link_hash.rs");
        include!("types/automation/run_book_draft_parameter.rs");
        include!("types/automation/run_book_job_schedule.rs");
        include!("types/automation/run_book_publish_content_link.rs");
        include!("types/automation/run_book_publish_content_link_hash.rs");
        include!("types/automation/schedule_monthly_occurrence.rs");
        include!("types/automation/software_update_configuration_linux.rs");
        include!("types/automation/software_update_configuration_post_task.rs");
        include!("types/automation/software_update_configuration_pre_task.rs");
        include!("types/automation/software_update_configuration_schedule.rs");
        include!(
            "types/automation/software_update_configuration_schedule_monthly_occurrence.rs"
        );
        include!("types/automation/software_update_configuration_target.rs");
        include!("types/automation/software_update_configuration_target_azure_query.rs");
        include!(
            "types/automation/software_update_configuration_target_azure_query_tag.rs"
        );
        include!(
            "types/automation/software_update_configuration_target_non_azure_query.rs"
        );
        include!("types/automation/software_update_configuration_windows.rs");
        include!("types/automation/source_control_security.rs");
        include!("types/automation/get_account_identity.rs");
        include!("types/automation/get_account_private_endpoint_connection.rs");
        include!("types/automation/get_variables_bool.rs");
        include!("types/automation/get_variables_datetime.rs");
        include!("types/automation/get_variables_encrypted.rs");
        include!("types/automation/get_variables_int.rs");
        include!("types/automation/get_variables_null.rs");
        include!("types/automation/get_variables_object.rs");
        include!("types/automation/get_variables_string.rs");
    }
    pub mod avs {
        include!("types/avs/private_cloud_circuit.rs");
        include!("types/avs/private_cloud_management_cluster.rs");
        include!("types/avs/get_private_cloud_circuit.rs");
        include!("types/avs/get_private_cloud_management_cluster.rs");
    }
    pub mod backup {
        include!("types/backup/policy_file_share_backup.rs");
        include!("types/backup/policy_file_share_backup_hourly.rs");
        include!("types/backup/policy_file_share_retention_daily.rs");
        include!("types/backup/policy_file_share_retention_monthly.rs");
        include!("types/backup/policy_file_share_retention_weekly.rs");
        include!("types/backup/policy_file_share_retention_yearly.rs");
        include!("types/backup/policy_vm_backup.rs");
        include!("types/backup/policy_vm_instant_restore_resource_group.rs");
        include!("types/backup/policy_vm_retention_daily.rs");
        include!("types/backup/policy_vm_retention_monthly.rs");
        include!("types/backup/policy_vm_retention_weekly.rs");
        include!("types/backup/policy_vm_retention_yearly.rs");
        include!("types/backup/policy_vm_tiering_policy.rs");
        include!("types/backup/policy_vm_tiering_policy_archived_restore_point.rs");
        include!("types/backup/policy_vm_workload_protection_policy.rs");
        include!("types/backup/policy_vm_workload_protection_policy_backup.rs");
        include!("types/backup/policy_vm_workload_protection_policy_retention_daily.rs");
        include!(
            "types/backup/policy_vm_workload_protection_policy_retention_monthly.rs"
        );
        include!(
            "types/backup/policy_vm_workload_protection_policy_retention_weekly.rs"
        );
        include!(
            "types/backup/policy_vm_workload_protection_policy_retention_yearly.rs"
        );
        include!(
            "types/backup/policy_vm_workload_protection_policy_simple_retention.rs"
        );
        include!("types/backup/policy_vm_workload_settings.rs");
    }
    pub mod batch {
        include!("types/batch/account_encryption.rs");
        include!("types/batch/account_identity.rs");
        include!("types/batch/account_key_vault_reference.rs");
        include!("types/batch/account_network_profile.rs");
        include!("types/batch/account_network_profile_account_access.rs");
        include!("types/batch/account_network_profile_account_access_ip_rule.rs");
        include!("types/batch/account_network_profile_node_management_access.rs");
        include!(
            "types/batch/account_network_profile_node_management_access_ip_rule.rs"
        );
        include!("types/batch/pool_auto_scale.rs");
        include!("types/batch/pool_certificate.rs");
        include!("types/batch/pool_container_configuration.rs");
        include!("types/batch/pool_container_configuration_container_registry.rs");
        include!("types/batch/pool_data_disk.rs");
        include!("types/batch/pool_disk_encryption.rs");
        include!("types/batch/pool_extension.rs");
        include!("types/batch/pool_fixed_scale.rs");
        include!("types/batch/pool_identity.rs");
        include!("types/batch/pool_mount.rs");
        include!("types/batch/pool_mount_azure_blob_file_system.rs");
        include!("types/batch/pool_mount_azure_file_share.rs");
        include!("types/batch/pool_mount_cifs_mount.rs");
        include!("types/batch/pool_mount_nfs_mount.rs");
        include!("types/batch/pool_network_configuration.rs");
        include!("types/batch/pool_network_configuration_endpoint_configuration.rs");
        include!(
            "types/batch/pool_network_configuration_endpoint_configuration_network_security_group_rule.rs"
        );
        include!("types/batch/pool_node_placement.rs");
        include!("types/batch/pool_start_task.rs");
        include!("types/batch/pool_start_task_container.rs");
        include!("types/batch/pool_start_task_container_registry.rs");
        include!("types/batch/pool_start_task_resource_file.rs");
        include!("types/batch/pool_start_task_user_identity.rs");
        include!("types/batch/pool_start_task_user_identity_auto_user.rs");
        include!("types/batch/pool_storage_image_reference.rs");
        include!("types/batch/pool_task_scheduling_policy.rs");
        include!("types/batch/pool_user_account.rs");
        include!("types/batch/pool_user_account_linux_user_configuration.rs");
        include!("types/batch/pool_user_account_windows_user_configuration.rs");
        include!("types/batch/pool_window.rs");
        include!("types/batch/get_account_encryption.rs");
        include!("types/batch/get_account_key_vault_reference.rs");
        include!("types/batch/get_pool_auto_scale.rs");
        include!("types/batch/get_pool_certificate.rs");
        include!("types/batch/get_pool_container_configuration.rs");
        include!("types/batch/get_pool_container_configuration_container_registry.rs");
        include!("types/batch/get_pool_data_disk.rs");
        include!("types/batch/get_pool_disk_encryption.rs");
        include!("types/batch/get_pool_extension.rs");
        include!("types/batch/get_pool_fixed_scale.rs");
        include!("types/batch/get_pool_mount.rs");
        include!("types/batch/get_pool_mount_azure_blob_file_system.rs");
        include!("types/batch/get_pool_mount_azure_file_share.rs");
        include!("types/batch/get_pool_mount_cifs_mount.rs");
        include!("types/batch/get_pool_mount_nfs_mount.rs");
        include!("types/batch/get_pool_network_configuration.rs");
        include!("types/batch/get_pool_network_configuration_endpoint_configuration.rs");
        include!(
            "types/batch/get_pool_network_configuration_endpoint_configuration_network_security_group_rule.rs"
        );
        include!("types/batch/get_pool_node_placement.rs");
        include!("types/batch/get_pool_start_task.rs");
        include!("types/batch/get_pool_start_task_container.rs");
        include!("types/batch/get_pool_start_task_container_registry.rs");
        include!("types/batch/get_pool_start_task_resource_file.rs");
        include!("types/batch/get_pool_start_task_user_identity.rs");
        include!("types/batch/get_pool_start_task_user_identity_auto_user.rs");
        include!("types/batch/get_pool_storage_image_reference.rs");
        include!("types/batch/get_pool_task_scheduling_policy.rs");
        include!("types/batch/get_pool_user_account.rs");
        include!("types/batch/get_pool_user_account_linux_user_configuration.rs");
        include!("types/batch/get_pool_user_account_windows_user_configuration.rs");
        include!("types/batch/get_pool_window.rs");
    }
    pub mod billing {
        include!("types/billing/account_cost_management_export_export_data_options.rs");
        include!(
            "types/billing/account_cost_management_export_export_data_storage_location.rs"
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
