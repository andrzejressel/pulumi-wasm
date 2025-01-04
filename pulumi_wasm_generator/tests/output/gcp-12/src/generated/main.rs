pub mod workbench {
    include!("resources/workbench/instance.rs");
    include!("resources/workbench/instance_iam_binding.rs");
    include!("resources/workbench/instance_iam_member.rs");
    include!("resources/workbench/instance_iam_policy.rs");
}
pub mod workflows {
    include!("resources/workflows/workflow.rs");
}
pub mod workstations {
    include!("resources/workstations/workstation.rs");
    include!("resources/workstations/workstation_cluster.rs");
    include!("resources/workstations/workstation_config.rs");
    include!("resources/workstations/workstation_config_iam_binding.rs");
    include!("resources/workstations/workstation_config_iam_member.rs");
    include!("resources/workstations/workstation_config_iam_policy.rs");
    include!("resources/workstations/workstation_iam_binding.rs");
    include!("resources/workstations/workstation_iam_member.rs");
    include!("resources/workstations/workstation_iam_policy.rs");
}
pub mod functions {
    pub mod workbench {
        include!("functions/workbench/get_instance_iam_policy.rs");
    }
    pub mod workstations {
        include!("functions/workstations/get_workstation_config_iam_policy.rs");
        include!("functions/workstations/get_workstation_iam_policy.rs");
    }
}
pub mod types {
    pub mod workbench {
        include!("types/workbench/instance_gce_setup.rs");
        include!("types/workbench/instance_gce_setup_accelerator_config.rs");
        include!("types/workbench/instance_gce_setup_boot_disk.rs");
        include!("types/workbench/instance_gce_setup_container_image.rs");
        include!("types/workbench/instance_gce_setup_data_disks.rs");
        include!("types/workbench/instance_gce_setup_network_interface.rs");
        include!(
            "types/workbench/instance_gce_setup_network_interface_access_config.rs"
        );
        include!("types/workbench/instance_gce_setup_service_account.rs");
        include!("types/workbench/instance_gce_setup_shielded_instance_config.rs");
        include!("types/workbench/instance_gce_setup_vm_image.rs");
        include!("types/workbench/instance_health_info.rs");
        include!("types/workbench/instance_iam_binding_condition.rs");
        include!("types/workbench/instance_iam_member_condition.rs");
        include!("types/workbench/instance_upgrade_history.rs");
    }
    pub mod workstations {
        include!("types/workstations/workstation_cluster_condition.rs");
        include!("types/workstations/workstation_cluster_domain_config.rs");
        include!("types/workstations/workstation_cluster_private_cluster_config.rs");
        include!("types/workstations/workstation_config_allowed_port.rs");
        include!("types/workstations/workstation_config_condition.rs");
        include!("types/workstations/workstation_config_container.rs");
        include!("types/workstations/workstation_config_encryption_key.rs");
        include!("types/workstations/workstation_config_ephemeral_directory.rs");
        include!("types/workstations/workstation_config_ephemeral_directory_gce_pd.rs");
        include!("types/workstations/workstation_config_host.rs");
        include!("types/workstations/workstation_config_host_gce_instance.rs");
        include!(
            "types/workstations/workstation_config_host_gce_instance_accelerator.rs"
        );
        include!(
            "types/workstations/workstation_config_host_gce_instance_boost_config.rs"
        );
        include!(
            "types/workstations/workstation_config_host_gce_instance_boost_config_accelerator.rs"
        );
        include!(
            "types/workstations/workstation_config_host_gce_instance_confidential_instance_config.rs"
        );
        include!(
            "types/workstations/workstation_config_host_gce_instance_shielded_instance_config.rs"
        );
        include!("types/workstations/workstation_config_iam_binding_condition.rs");
        include!("types/workstations/workstation_config_iam_member_condition.rs");
        include!("types/workstations/workstation_config_persistent_directory.rs");
        include!("types/workstations/workstation_config_persistent_directory_gce_pd.rs");
        include!("types/workstations/workstation_config_readiness_check.rs");
        include!("types/workstations/workstation_iam_binding_condition.rs");
        include!("types/workstations/workstation_iam_member_condition.rs");
    }
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-gcp {
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
