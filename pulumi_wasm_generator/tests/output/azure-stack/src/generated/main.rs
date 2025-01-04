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
pub mod functions {
    pub mod stack {
        include!("functions/stack/get_hci_cluster.rs");
    }
}
pub mod types {
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
