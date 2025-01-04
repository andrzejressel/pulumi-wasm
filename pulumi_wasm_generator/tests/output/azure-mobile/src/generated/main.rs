pub mod mobile {
    include!("resources/mobile/network.rs");
    include!("resources/mobile/network_attached_data_network.rs");
    include!("resources/mobile/network_data_network.rs");
    include!("resources/mobile/network_packet_core_control_plane.rs");
    include!("resources/mobile/network_packet_core_data_plane.rs");
    include!("resources/mobile/network_service.rs");
    include!("resources/mobile/network_sim.rs");
    include!("resources/mobile/network_sim_group.rs");
    include!("resources/mobile/network_sim_policy.rs");
    include!("resources/mobile/network_site.rs");
    include!("resources/mobile/network_slice.rs");
}
pub mod functions {
    pub mod mobile {
        include!("functions/mobile/get_network.rs");
        include!("functions/mobile/get_network_attached_data_network.rs");
        include!("functions/mobile/get_network_data_network.rs");
        include!("functions/mobile/get_network_packet_core_control_plane.rs");
        include!("functions/mobile/get_network_packet_core_data_plane.rs");
        include!("functions/mobile/get_network_service.rs");
        include!("functions/mobile/get_network_sim.rs");
        include!("functions/mobile/get_network_sim_group.rs");
        include!("functions/mobile/get_network_sim_policy.rs");
        include!("functions/mobile/get_network_site.rs");
        include!("functions/mobile/get_network_slice.rs");
    }
}
pub mod types {
    pub mod mobile {
        include!(
            "types/mobile/network_attached_data_network_network_address_port_translation.rs"
        );
        include!(
            "types/mobile/network_attached_data_network_network_address_port_translation_port_range.rs"
        );
        include!("types/mobile/network_packet_core_control_plane_identity.rs");
        include!(
            "types/mobile/network_packet_core_control_plane_local_diagnostics_access.rs"
        );
        include!("types/mobile/network_packet_core_control_plane_platform.rs");
        include!("types/mobile/network_service_pcc_rule.rs");
        include!("types/mobile/network_service_pcc_rule_qos_policy.rs");
        include!(
            "types/mobile/network_service_pcc_rule_qos_policy_guaranteed_bit_rate.rs"
        );
        include!("types/mobile/network_service_pcc_rule_qos_policy_maximum_bit_rate.rs");
        include!("types/mobile/network_service_pcc_rule_service_data_flow_template.rs");
        include!("types/mobile/network_service_service_qos_policy.rs");
        include!("types/mobile/network_service_service_qos_policy_maximum_bit_rate.rs");
        include!("types/mobile/network_sim_group_identity.rs");
        include!("types/mobile/network_sim_policy_slice.rs");
        include!("types/mobile/network_sim_policy_slice_data_network.rs");
        include!(
            "types/mobile/network_sim_policy_slice_data_network_session_aggregate_maximum_bit_rate.rs"
        );
        include!(
            "types/mobile/network_sim_policy_user_equipment_aggregate_maximum_bit_rate.rs"
        );
        include!("types/mobile/network_sim_static_ip_configuration.rs");
        include!(
            "types/mobile/network_slice_single_network_slice_selection_assistance_information.rs"
        );
        include!(
            "types/mobile/get_network_attached_data_network_network_address_port_translation.rs"
        );
        include!(
            "types/mobile/get_network_attached_data_network_network_address_port_translation_port_range.rs"
        );
        include!("types/mobile/get_network_packet_core_control_plane_identity.rs");
        include!(
            "types/mobile/get_network_packet_core_control_plane_local_diagnostics_access.rs"
        );
        include!("types/mobile/get_network_packet_core_control_plane_platform.rs");
        include!("types/mobile/get_network_service_pcc_rule.rs");
        include!("types/mobile/get_network_service_pcc_rule_qos_policy.rs");
        include!(
            "types/mobile/get_network_service_pcc_rule_qos_policy_guaranteed_bit_rate.rs"
        );
        include!(
            "types/mobile/get_network_service_pcc_rule_qos_policy_maximum_bit_rate.rs"
        );
        include!(
            "types/mobile/get_network_service_pcc_rule_service_data_flow_template.rs"
        );
        include!("types/mobile/get_network_service_service_qos_policy.rs");
        include!(
            "types/mobile/get_network_service_service_qos_policy_maximum_bit_rate.rs"
        );
        include!("types/mobile/get_network_sim_group_identity.rs");
        include!("types/mobile/get_network_sim_policy_slice.rs");
        include!("types/mobile/get_network_sim_policy_slice_data_network.rs");
        include!(
            "types/mobile/get_network_sim_policy_slice_data_network_session_aggregate_maximum_bit_rate.rs"
        );
        include!(
            "types/mobile/get_network_sim_policy_user_equipment_aggregate_maximum_bit_rate.rs"
        );
        include!("types/mobile/get_network_sim_static_ip_configuration.rs");
        include!(
            "types/mobile/get_network_slice_single_network_slice_selection_assistance_information.rs"
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
