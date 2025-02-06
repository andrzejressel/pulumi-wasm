pub mod voice {
    include!("resources/voice/services_communications_gateway.rs");
    include!("resources/voice/services_communications_gateway_test_line.rs");
}
pub mod waf {
    include!("resources/waf/policy.rs");
}
pub mod webpubsub {
    include!("resources/webpubsub/custom_certificate.rs");
    include!("resources/webpubsub/custom_domain.rs");
    include!("resources/webpubsub/hub.rs");
    include!("resources/webpubsub/network_acl.rs");
    include!("resources/webpubsub/service.rs");
    include!("resources/webpubsub/shared_private_link_resource.rs");
}
pub mod workloadssap {
    include!("resources/workloadssap/discovery_virtual_instance.rs");
    include!("resources/workloadssap/single_node_virtual_instance.rs");
    include!("resources/workloadssap/three_tier_virtual_instance.rs");
}
pub mod functions {
    pub mod waf {
        include!("functions/waf/get_firewall_policy.rs");
    }
    pub mod webpubsub {
        include!("functions/webpubsub/get_private_link_resource.rs");
        include!("functions/webpubsub/get_service.rs");
    }
}
pub mod types {
    pub mod voice {
        include!("types/voice/services_communications_gateway_service_location.rs");
    }
    pub mod waf {
        include!("types/waf/policy_custom_rule.rs");
        include!("types/waf/policy_custom_rule_match_condition.rs");
        include!("types/waf/policy_custom_rule_match_condition_match_variable.rs");
        include!("types/waf/policy_managed_rules.rs");
        include!("types/waf/policy_managed_rules_exclusion.rs");
        include!("types/waf/policy_managed_rules_exclusion_excluded_rule_set.rs");
        include!(
            "types/waf/policy_managed_rules_exclusion_excluded_rule_set_rule_group.rs"
        );
        include!("types/waf/policy_managed_rules_managed_rule_set.rs");
        include!(
            "types/waf/policy_managed_rules_managed_rule_set_rule_group_override.rs"
        );
        include!(
            "types/waf/policy_managed_rules_managed_rule_set_rule_group_override_rule.rs"
        );
        include!("types/waf/policy_policy_settings.rs");
        include!("types/waf/policy_policy_settings_log_scrubbing.rs");
        include!("types/waf/policy_policy_settings_log_scrubbing_rule.rs");
    }
    pub mod webpubsub {
        include!("types/webpubsub/hub_event_handler.rs");
        include!("types/webpubsub/hub_event_handler_auth.rs");
        include!("types/webpubsub/hub_event_listener.rs");
        include!("types/webpubsub/network_acl_private_endpoint.rs");
        include!("types/webpubsub/network_acl_public_network.rs");
        include!("types/webpubsub/service_identity.rs");
        include!("types/webpubsub/service_live_trace.rs");
        include!(
            "types/webpubsub/get_private_link_resource_shared_private_link_resource_type.rs"
        );
    }
    pub mod workloadssap {
        include!("types/workloadssap/discovery_virtual_instance_identity.rs");
        include!("types/workloadssap/single_node_virtual_instance_identity.rs");
        include!(
            "types/workloadssap/single_node_virtual_instance_single_server_configuration.rs"
        );
        include!(
            "types/workloadssap/single_node_virtual_instance_single_server_configuration_disk_volume_configuration.rs"
        );
        include!(
            "types/workloadssap/single_node_virtual_instance_single_server_configuration_virtual_machine_configuration.rs"
        );
        include!(
            "types/workloadssap/single_node_virtual_instance_single_server_configuration_virtual_machine_configuration_image.rs"
        );
        include!(
            "types/workloadssap/single_node_virtual_instance_single_server_configuration_virtual_machine_configuration_os_profile.rs"
        );
        include!(
            "types/workloadssap/single_node_virtual_instance_single_server_configuration_virtual_machine_resource_names.rs"
        );
        include!(
            "types/workloadssap/single_node_virtual_instance_single_server_configuration_virtual_machine_resource_names_data_disk.rs"
        );
        include!("types/workloadssap/three_tier_virtual_instance_identity.rs");
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_application_server_configuration.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_application_server_configuration_virtual_machine_configuration.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_application_server_configuration_virtual_machine_configuration_image.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_application_server_configuration_virtual_machine_configuration_os_profile.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_central_server_configuration.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_central_server_configuration_virtual_machine_configuration.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_central_server_configuration_virtual_machine_configuration_image.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_central_server_configuration_virtual_machine_configuration_os_profile.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_database_server_configuration.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_database_server_configuration_disk_volume_configuration.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_database_server_configuration_virtual_machine_configuration.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_database_server_configuration_virtual_machine_configuration_image.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_database_server_configuration_virtual_machine_configuration_os_profile.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_resource_names.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_resource_names_application_server.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_resource_names_application_server_virtual_machine.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_resource_names_application_server_virtual_machine_data_disk.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_resource_names_central_server.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_resource_names_central_server_load_balancer.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_resource_names_central_server_virtual_machine.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_resource_names_central_server_virtual_machine_data_disk.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_resource_names_database_server.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_resource_names_database_server_load_balancer.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_resource_names_database_server_virtual_machine.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_resource_names_database_server_virtual_machine_data_disk.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_resource_names_shared_storage.rs"
        );
        include!(
            "types/workloadssap/three_tier_virtual_instance_three_tier_configuration_transport_create_and_mount.rs"
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
