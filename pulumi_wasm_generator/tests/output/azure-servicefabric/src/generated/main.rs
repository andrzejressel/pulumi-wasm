pub mod servicefabric {
    include!("resources/servicefabric/cluster.rs");
    include!("resources/servicefabric/managed_cluster.rs");
}
pub mod functions {}
pub mod types {
    pub mod servicefabric {
        include!("types/servicefabric/cluster_azure_active_directory.rs");
        include!("types/servicefabric/cluster_certificate.rs");
        include!("types/servicefabric/cluster_certificate_common_names.rs");
        include!("types/servicefabric/cluster_certificate_common_names_common_name.rs");
        include!("types/servicefabric/cluster_client_certificate_common_name.rs");
        include!("types/servicefabric/cluster_client_certificate_thumbprint.rs");
        include!("types/servicefabric/cluster_diagnostics_config.rs");
        include!("types/servicefabric/cluster_fabric_setting.rs");
        include!("types/servicefabric/cluster_node_type.rs");
        include!("types/servicefabric/cluster_node_type_application_ports.rs");
        include!("types/servicefabric/cluster_node_type_ephemeral_ports.rs");
        include!("types/servicefabric/cluster_reverse_proxy_certificate.rs");
        include!(
            "types/servicefabric/cluster_reverse_proxy_certificate_common_names.rs"
        );
        include!(
            "types/servicefabric/cluster_reverse_proxy_certificate_common_names_common_name.rs"
        );
        include!("types/servicefabric/cluster_upgrade_policy.rs");
        include!("types/servicefabric/cluster_upgrade_policy_delta_health_policy.rs");
        include!("types/servicefabric/cluster_upgrade_policy_health_policy.rs");
        include!("types/servicefabric/managed_cluster_authentication.rs");
        include!(
            "types/servicefabric/managed_cluster_authentication_active_directory.rs"
        );
        include!("types/servicefabric/managed_cluster_authentication_certificate.rs");
        include!("types/servicefabric/managed_cluster_custom_fabric_setting.rs");
        include!("types/servicefabric/managed_cluster_lb_rule.rs");
        include!("types/servicefabric/managed_cluster_node_type.rs");
        include!("types/servicefabric/managed_cluster_node_type_vm_secret.rs");
        include!(
            "types/servicefabric/managed_cluster_node_type_vm_secret_certificate.rs"
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
