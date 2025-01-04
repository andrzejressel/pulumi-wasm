pub mod workloadssap {
    include!("resources/workloadssap/discovery_virtual_instance.rs");
    include!("resources/workloadssap/single_node_virtual_instance.rs");
    include!("resources/workloadssap/three_tier_virtual_instance.rs");
}
pub mod functions {}
pub mod types {
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
