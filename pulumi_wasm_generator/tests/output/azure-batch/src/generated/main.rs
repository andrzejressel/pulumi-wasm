pub mod batch {
    include!("resources/batch/account.rs");
    include!("resources/batch/application.rs");
    include!("resources/batch/certificate.rs");
    include!("resources/batch/job.rs");
    include!("resources/batch/pool.rs");
}
pub mod functions {
    pub mod batch {
        include!("functions/batch/get_account.rs");
        include!("functions/batch/get_application.rs");
        include!("functions/batch/get_certificate.rs");
        include!("functions/batch/get_pool.rs");
    }
}
pub mod types {
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
