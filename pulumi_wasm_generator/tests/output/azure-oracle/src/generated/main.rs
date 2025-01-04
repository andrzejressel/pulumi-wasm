pub mod oracle {
    include!("resources/oracle/autonomous_database.rs");
    include!("resources/oracle/cloud_vm_cluster.rs");
    include!("resources/oracle/exadata_infrastructure.rs");
}
pub mod functions {
    pub mod oracle {
        include!("functions/oracle/get_adbs_character_sets.rs");
        include!("functions/oracle/get_adbs_national_character_sets.rs");
        include!("functions/oracle/get_autonomous_database.rs");
        include!("functions/oracle/get_cloud_vm_cluster.rs");
        include!("functions/oracle/get_db_nodes.rs");
        include!("functions/oracle/get_db_servers.rs");
        include!("functions/oracle/get_db_system_shapes.rs");
        include!("functions/oracle/get_exadata_infrastructure.rs");
        include!("functions/oracle/get_gi_versions.rs");
    }
}
pub mod types {
    pub mod oracle {
        include!("types/oracle/cloud_vm_cluster_data_collection_options.rs");
        include!("types/oracle/exadata_infrastructure_maintenance_window.rs");
        include!("types/oracle/get_adbs_character_sets_character_set.rs");
        include!("types/oracle/get_adbs_national_character_sets_character_set.rs");
        include!("types/oracle/get_cloud_vm_cluster_data_collection_option.rs");
        include!("types/oracle/get_cloud_vm_cluster_iorm_config_cach.rs");
        include!("types/oracle/get_cloud_vm_cluster_iorm_config_cach_db_plan.rs");
        include!("types/oracle/get_db_nodes_db_node.rs");
        include!("types/oracle/get_db_servers_db_server.rs");
        include!("types/oracle/get_db_system_shapes_db_system_shape.rs");
        include!("types/oracle/get_exadata_infrastructure_estimated_patching_time.rs");
        include!("types/oracle/get_exadata_infrastructure_maintenance_window.rs");
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
