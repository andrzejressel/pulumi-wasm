pub mod appconfiguration {
    include!("resources/appconfiguration/configuration_feature.rs");
    include!("resources/appconfiguration/configuration_key.rs");
    include!("resources/appconfiguration/configuration_store.rs");
    include!("resources/appconfiguration/lication_load_balancer.rs");
    include!("resources/appconfiguration/lication_load_balancer_frontend.rs");
    include!("resources/appconfiguration/lication_load_balancer_subnet_association.rs");
}
pub mod functions {
    pub mod appconfiguration {
        include!("functions/appconfiguration/get_configuration_key.rs");
        include!("functions/appconfiguration/get_configuration_keys.rs");
        include!("functions/appconfiguration/get_configuration_store.rs");
    }
}
pub mod types {
    pub mod appconfiguration {
        include!("types/appconfiguration/configuration_feature_targeting_filter.rs");
        include!(
            "types/appconfiguration/configuration_feature_targeting_filter_group.rs"
        );
        include!("types/appconfiguration/configuration_feature_timewindow_filter.rs");
        include!("types/appconfiguration/configuration_store_encryption.rs");
        include!("types/appconfiguration/configuration_store_identity.rs");
        include!("types/appconfiguration/configuration_store_primary_read_key.rs");
        include!("types/appconfiguration/configuration_store_primary_write_key.rs");
        include!("types/appconfiguration/configuration_store_replica.rs");
        include!("types/appconfiguration/configuration_store_secondary_read_key.rs");
        include!("types/appconfiguration/configuration_store_secondary_write_key.rs");
        include!("types/appconfiguration/get_configuration_keys_item.rs");
        include!("types/appconfiguration/get_configuration_store_encryption.rs");
        include!("types/appconfiguration/get_configuration_store_identity.rs");
        include!("types/appconfiguration/get_configuration_store_primary_read_key.rs");
        include!("types/appconfiguration/get_configuration_store_primary_write_key.rs");
        include!("types/appconfiguration/get_configuration_store_replica.rs");
        include!("types/appconfiguration/get_configuration_store_secondary_read_key.rs");
        include!(
            "types/appconfiguration/get_configuration_store_secondary_write_key.rs"
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
