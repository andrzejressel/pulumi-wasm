pub mod redis {
    include!("resources/redis/cache.rs");
    include!("resources/redis/cache_access_policy.rs");
    include!("resources/redis/cache_access_policy_assignment.rs");
    include!("resources/redis/enterprise_cluster.rs");
    include!("resources/redis/enterprise_database.rs");
    include!("resources/redis/firewall_rule.rs");
    include!("resources/redis/linked_server.rs");
}
pub mod functions {
    pub mod redis {
        include!("functions/redis/get_cache.rs");
        include!("functions/redis/get_enterprise_database.rs");
    }
}
pub mod types {
    pub mod redis {
        include!("types/redis/cache_identity.rs");
        include!("types/redis/cache_patch_schedule.rs");
        include!("types/redis/cache_redis_configuration.rs");
        include!("types/redis/enterprise_database_module.rs");
        include!("types/redis/get_cache_patch_schedule.rs");
        include!("types/redis/get_cache_redis_configuration.rs");
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
