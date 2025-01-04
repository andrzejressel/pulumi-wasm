pub mod servicebus {
    include!("resources/servicebus/namespace.rs");
    include!("resources/servicebus/namespace_authorization_rule.rs");
    include!("resources/servicebus/namespace_disaster_recovery_config.rs");
    include!("resources/servicebus/queue.rs");
    include!("resources/servicebus/queue_authorization_rule.rs");
    include!("resources/servicebus/subscription.rs");
    include!("resources/servicebus/subscription_rule.rs");
    include!("resources/servicebus/topic.rs");
    include!("resources/servicebus/topic_authorization_rule.rs");
}
pub mod functions {
    pub mod servicebus {
        include!("functions/servicebus/get_namespace.rs");
        include!("functions/servicebus/get_namespace_authorization_rule.rs");
        include!("functions/servicebus/get_namespace_disaster_recovery_config.rs");
        include!("functions/servicebus/get_queue.rs");
        include!("functions/servicebus/get_queue_authorization_rule.rs");
        include!("functions/servicebus/get_subscription.rs");
        include!("functions/servicebus/get_topic.rs");
        include!("functions/servicebus/get_topic_authorization_rule.rs");
    }
}
pub mod types {
    pub mod servicebus {
        include!("types/servicebus/namespace_customer_managed_key.rs");
        include!("types/servicebus/namespace_identity.rs");
        include!("types/servicebus/namespace_network_rule_set.rs");
        include!("types/servicebus/namespace_network_rule_set_network_rule.rs");
        include!("types/servicebus/subscription_client_scoped_subscription.rs");
        include!("types/servicebus/subscription_rule_correlation_filter.rs");
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
