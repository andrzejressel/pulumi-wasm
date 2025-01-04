pub mod kusto {
    include!("resources/kusto/attached_database_configuration.rs");
    include!("resources/kusto/cluster.rs");
    include!("resources/kusto/cluster_customer_managed_key.rs");
    include!("resources/kusto/cluster_managed_private_endpoint.rs");
    include!("resources/kusto/cluster_principal_assignment.rs");
    include!("resources/kusto/cosmosdb_data_connection.rs");
    include!("resources/kusto/database.rs");
    include!("resources/kusto/database_principal_assignment.rs");
    include!("resources/kusto/event_grid_data_connection.rs");
    include!("resources/kusto/eventhub_data_connection.rs");
    include!("resources/kusto/iot_hub_data_connection.rs");
    include!("resources/kusto/script.rs");
}
pub mod functions {
    pub mod kusto {
        include!("functions/kusto/get_cluster.rs");
        include!("functions/kusto/get_database.rs");
    }
}
pub mod types {
    pub mod kusto {
        include!("types/kusto/attached_database_configuration_sharing.rs");
        include!("types/kusto/cluster_identity.rs");
        include!("types/kusto/cluster_language_extension.rs");
        include!("types/kusto/cluster_optimized_auto_scale.rs");
        include!("types/kusto/cluster_sku.rs");
        include!("types/kusto/cluster_virtual_network_configuration.rs");
        include!("types/kusto/get_cluster_identity.rs");
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
