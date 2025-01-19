include!("resources/container.rs");
include!("resources/image.rs");
include!("resources/network.rs");
include!("resources/plugin.rs");
include!("resources/registry_image.rs");
include!("resources/remote_image.rs");
include!("resources/secret.rs");
include!("resources/service.rs");
include!("resources/service_config.rs");
include!("resources/tag.rs");
include!("resources/volume.rs");
pub mod functions {
    include!("functions/get_logs.rs");
    include!("functions/get_network.rs");
    include!("functions/get_plugin.rs");
    include!("functions/get_registry_image.rs");
    include!("functions/get_remote_image.rs");
}
pub mod types {
    pub mod config {
        include!("types/config/registry_auth.rs");
    }
    include!("types/builder_version.rs");
    include!("types/cache_from.rs");
    include!("types/container_capabilities.rs");
    include!("types/container_device.rs");
    include!("types/container_healthcheck.rs");
    include!("types/container_host.rs");
    include!("types/container_label.rs");
    include!("types/container_mount.rs");
    include!("types/container_mount_bind_options.rs");
    include!("types/container_mount_tmpfs_options.rs");
    include!("types/container_mount_volume_options.rs");
    include!("types/container_mount_volume_options_label.rs");
    include!("types/container_network_data.rs");
    include!("types/container_networks_advanced.rs");
    include!("types/container_port.rs");
    include!("types/container_ulimit.rs");
    include!("types/container_upload.rs");
    include!("types/container_volume.rs");
    include!("types/docker_build.rs");
    include!("types/network_ipam_config.rs");
    include!("types/network_label.rs");
    include!("types/plugin_grant_permission.rs");
    include!("types/provider_registry_auth.rs");
    include!("types/registry.rs");
    include!("types/remote_image_build.rs");
    include!("types/remote_image_build_auth_config.rs");
    include!("types/remote_image_build_ulimit.rs");
    include!("types/secret_label.rs");
    include!("types/service_auth.rs");
    include!("types/service_converge_config.rs");
    include!("types/service_endpoint_spec.rs");
    include!("types/service_endpoint_spec_port.rs");
    include!("types/service_label.rs");
    include!("types/service_mode.rs");
    include!("types/service_mode_replicated.rs");
    include!("types/service_rollback_config.rs");
    include!("types/service_task_spec.rs");
    include!("types/service_task_spec_container_spec.rs");
    include!("types/service_task_spec_container_spec_config.rs");
    include!("types/service_task_spec_container_spec_dns_config.rs");
    include!("types/service_task_spec_container_spec_healthcheck.rs");
    include!("types/service_task_spec_container_spec_host.rs");
    include!("types/service_task_spec_container_spec_label.rs");
    include!("types/service_task_spec_container_spec_mount.rs");
    include!("types/service_task_spec_container_spec_mount_bind_options.rs");
    include!("types/service_task_spec_container_spec_mount_tmpfs_options.rs");
    include!("types/service_task_spec_container_spec_mount_volume_options.rs");
    include!("types/service_task_spec_container_spec_mount_volume_options_label.rs");
    include!("types/service_task_spec_container_spec_privileges.rs");
    include!("types/service_task_spec_container_spec_privileges_credential_spec.rs");
    include!("types/service_task_spec_container_spec_privileges_se_linux_context.rs");
    include!("types/service_task_spec_container_spec_secret.rs");
    include!("types/service_task_spec_log_driver.rs");
    include!("types/service_task_spec_networks_advanced.rs");
    include!("types/service_task_spec_placement.rs");
    include!("types/service_task_spec_placement_platform.rs");
    include!("types/service_task_spec_resources.rs");
    include!("types/service_task_spec_resources_limits.rs");
    include!("types/service_task_spec_resources_reservation.rs");
    include!("types/service_task_spec_resources_reservation_generic_resources.rs");
    include!("types/service_task_spec_restart_policy.rs");
    include!("types/service_update_config.rs");
    include!("types/volume_label.rs");
    include!("types/get_network_ipam_config.rs");
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-docker {
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
#[link_section = "pulumi_wasm_provider::docker"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_DOCKER: [u8; 44] = *b"{\"version\":\"4.5.3\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "4.5.3".to_string()
}
