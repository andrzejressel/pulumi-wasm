pub mod databricks {
    include!("resources/databricks/access_connector.rs");
    include!("resources/databricks/virtual_network_peering.rs");
    include!("resources/databricks/workspace.rs");
    include!("resources/databricks/workspace_customer_managed_key.rs");
    include!("resources/databricks/workspace_root_dbfs_customer_managed_key.rs");
}
pub mod functions {
    pub mod databricks {
        include!("functions/databricks/get_access_connector.rs");
        include!("functions/databricks/get_workspace.rs");
        include!("functions/databricks/get_workspace_private_endpoint_connection.rs");
    }
}
pub mod types {
    pub mod databricks {
        include!("types/databricks/access_connector_identity.rs");
        include!("types/databricks/workspace_custom_parameters.rs");
        include!("types/databricks/workspace_enhanced_security_compliance.rs");
        include!("types/databricks/workspace_managed_disk_identity.rs");
        include!("types/databricks/workspace_storage_account_identity.rs");
        include!("types/databricks/get_access_connector_identity.rs");
        include!("types/databricks/get_workspace_enhanced_security_compliance.rs");
        include!("types/databricks/get_workspace_managed_disk_identity.rs");
        include!(
            "types/databricks/get_workspace_private_endpoint_connection_connection.rs"
        );
        include!("types/databricks/get_workspace_storage_account_identity.rs");
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