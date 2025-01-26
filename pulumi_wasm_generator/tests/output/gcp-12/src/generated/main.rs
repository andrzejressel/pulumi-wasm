pub mod vmwareengine {
    include!("resources/vmwareengine/cluster.rs");
    include!("resources/vmwareengine/external_access_rule.rs");
    include!("resources/vmwareengine/external_address.rs");
    include!("resources/vmwareengine/network.rs");
    include!("resources/vmwareengine/network_peering.rs");
    include!("resources/vmwareengine/network_policy.rs");
    include!("resources/vmwareengine/private_cloud.rs");
    include!("resources/vmwareengine/subnet.rs");
}
pub mod vpcaccess {
    include!("resources/vpcaccess/connector.rs");
}
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
    pub mod vmwareengine {
        include!("functions/vmwareengine/get_cluster.rs");
        include!("functions/vmwareengine/get_external_access_rule.rs");
        include!("functions/vmwareengine/get_external_address.rs");
        include!("functions/vmwareengine/get_network.rs");
        include!("functions/vmwareengine/get_network_peering.rs");
        include!("functions/vmwareengine/get_network_policy.rs");
        include!("functions/vmwareengine/get_nsx_credentials.rs");
        include!("functions/vmwareengine/get_private_cloud.rs");
        include!("functions/vmwareengine/get_subnet.rs");
        include!("functions/vmwareengine/get_vcenter_credentials.rs");
    }
    pub mod vpcaccess {
        include!("functions/vpcaccess/get_connector.rs");
    }
    pub mod workbench {
        include!("functions/workbench/get_instance_iam_policy.rs");
    }
    pub mod workstations {
        include!("functions/workstations/get_workstation_config_iam_policy.rs");
        include!("functions/workstations/get_workstation_iam_policy.rs");
    }
}
pub mod types {
    pub mod vmwareengine {
        include!("types/vmwareengine/cluster_autoscaling_settings.rs");
        include!(
            "types/vmwareengine/cluster_autoscaling_settings_autoscaling_policy.rs"
        );
        include!(
            "types/vmwareengine/cluster_autoscaling_settings_autoscaling_policy_consumed_memory_thresholds.rs"
        );
        include!(
            "types/vmwareengine/cluster_autoscaling_settings_autoscaling_policy_cpu_thresholds.rs"
        );
        include!(
            "types/vmwareengine/cluster_autoscaling_settings_autoscaling_policy_storage_thresholds.rs"
        );
        include!("types/vmwareengine/cluster_node_type_config.rs");
        include!("types/vmwareengine/external_access_rule_destination_ip_range.rs");
        include!("types/vmwareengine/external_access_rule_source_ip_range.rs");
        include!("types/vmwareengine/network_policy_external_ip.rs");
        include!("types/vmwareengine/network_policy_internet_access.rs");
        include!("types/vmwareengine/network_vpc_network.rs");
        include!("types/vmwareengine/private_cloud_hcx.rs");
        include!("types/vmwareengine/private_cloud_management_cluster.rs");
        include!(
            "types/vmwareengine/private_cloud_management_cluster_autoscaling_settings.rs"
        );
        include!(
            "types/vmwareengine/private_cloud_management_cluster_autoscaling_settings_autoscaling_policy.rs"
        );
        include!(
            "types/vmwareengine/private_cloud_management_cluster_autoscaling_settings_autoscaling_policy_consumed_memory_thresholds.rs"
        );
        include!(
            "types/vmwareengine/private_cloud_management_cluster_autoscaling_settings_autoscaling_policy_cpu_thresholds.rs"
        );
        include!(
            "types/vmwareengine/private_cloud_management_cluster_autoscaling_settings_autoscaling_policy_storage_thresholds.rs"
        );
        include!(
            "types/vmwareengine/private_cloud_management_cluster_node_type_config.rs"
        );
        include!(
            "types/vmwareengine/private_cloud_management_cluster_stretched_cluster_config.rs"
        );
        include!("types/vmwareengine/private_cloud_network_config.rs");
        include!("types/vmwareengine/private_cloud_nsx.rs");
        include!("types/vmwareengine/private_cloud_vcenter.rs");
        include!("types/vmwareengine/subnet_dhcp_address_range.rs");
        include!("types/vmwareengine/get_cluster_autoscaling_setting.rs");
        include!(
            "types/vmwareengine/get_cluster_autoscaling_setting_autoscaling_policy.rs"
        );
        include!(
            "types/vmwareengine/get_cluster_autoscaling_setting_autoscaling_policy_consumed_memory_threshold.rs"
        );
        include!(
            "types/vmwareengine/get_cluster_autoscaling_setting_autoscaling_policy_cpu_threshold.rs"
        );
        include!(
            "types/vmwareengine/get_cluster_autoscaling_setting_autoscaling_policy_storage_threshold.rs"
        );
        include!("types/vmwareengine/get_cluster_node_type_config.rs");
        include!("types/vmwareengine/get_external_access_rule_destination_ip_range.rs");
        include!("types/vmwareengine/get_external_access_rule_source_ip_range.rs");
        include!("types/vmwareengine/get_network_policy_external_ip.rs");
        include!("types/vmwareengine/get_network_policy_internet_access.rs");
        include!("types/vmwareengine/get_network_vpc_network.rs");
        include!("types/vmwareengine/get_private_cloud_hcx.rs");
        include!("types/vmwareengine/get_private_cloud_management_cluster.rs");
        include!(
            "types/vmwareengine/get_private_cloud_management_cluster_autoscaling_setting.rs"
        );
        include!(
            "types/vmwareengine/get_private_cloud_management_cluster_autoscaling_setting_autoscaling_policy.rs"
        );
        include!(
            "types/vmwareengine/get_private_cloud_management_cluster_autoscaling_setting_autoscaling_policy_consumed_memory_threshold.rs"
        );
        include!(
            "types/vmwareengine/get_private_cloud_management_cluster_autoscaling_setting_autoscaling_policy_cpu_threshold.rs"
        );
        include!(
            "types/vmwareengine/get_private_cloud_management_cluster_autoscaling_setting_autoscaling_policy_storage_threshold.rs"
        );
        include!(
            "types/vmwareengine/get_private_cloud_management_cluster_node_type_config.rs"
        );
        include!(
            "types/vmwareengine/get_private_cloud_management_cluster_stretched_cluster_config.rs"
        );
        include!("types/vmwareengine/get_private_cloud_network_config.rs");
        include!("types/vmwareengine/get_private_cloud_nsx.rs");
        include!("types/vmwareengine/get_private_cloud_vcenter.rs");
        include!("types/vmwareengine/get_subnet_dhcp_address_range.rs");
    }
    pub mod vpcaccess {
        include!("types/vpcaccess/connector_subnet.rs");
        include!("types/vpcaccess/get_connector_subnet.rs");
    }
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
#[link_section = "pulumi_wasm_provider::gcp"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_GCP: [u8; 45] = *b"{\"version\":\"8.12.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "8.12.1".to_string()
}
