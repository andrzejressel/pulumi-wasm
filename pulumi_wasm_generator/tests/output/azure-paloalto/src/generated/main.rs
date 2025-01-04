pub mod paloalto {
    include!("resources/paloalto/local_rulestack.rs");
    include!("resources/paloalto/local_rulestack_certificate.rs");
    include!("resources/paloalto/local_rulestack_fqdn_list.rs");
    include!(
        "resources/paloalto/local_rulestack_outbound_trust_certificate_association.rs"
    );
    include!(
        "resources/paloalto/local_rulestack_outbound_untrust_certificate_association.rs"
    );
    include!("resources/paloalto/local_rulestack_prefix_list.rs");
    include!("resources/paloalto/local_rulestack_rule.rs");
    include!(
        "resources/paloalto/next_generation_firewall_virtual_hub_local_rulestack.rs"
    );
    include!("resources/paloalto/next_generation_firewall_virtual_hub_panorama.rs");
    include!(
        "resources/paloalto/next_generation_firewall_virtual_network_local_rulestack.rs"
    );
    include!("resources/paloalto/next_generation_firewall_virtual_network_panorama.rs");
    include!("resources/paloalto/virtual_network_appliance.rs");
}
pub mod functions {
    pub mod paloalto {
        include!("functions/paloalto/get_local_rulestack.rs");
    }
}
pub mod types {
    pub mod paloalto {
        include!("types/paloalto/local_rulestack_rule_category.rs");
        include!("types/paloalto/local_rulestack_rule_destination.rs");
        include!("types/paloalto/local_rulestack_rule_source.rs");
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_local_rulestack_destination_nat.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_local_rulestack_destination_nat_backend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_local_rulestack_destination_nat_frontend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_local_rulestack_dns_settings.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_local_rulestack_network_profile.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_panorama_destination_nat.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_panorama_destination_nat_backend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_panorama_destination_nat_frontend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_panorama_dns_settings.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_panorama_network_profile.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_hub_panorama_panorama.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_local_rulestack_destination_nat.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_local_rulestack_destination_nat_backend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_local_rulestack_destination_nat_frontend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_local_rulestack_dns_settings.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_local_rulestack_network_profile.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_local_rulestack_network_profile_vnet_configuration.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_panorama_destination_nat.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_panorama_destination_nat_backend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_panorama_destination_nat_frontend_config.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_panorama_dns_settings.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_panorama_network_profile.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_panorama_network_profile_vnet_configuration.rs"
        );
        include!(
            "types/paloalto/next_generation_firewall_virtual_network_panorama_panorama.rs"
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
