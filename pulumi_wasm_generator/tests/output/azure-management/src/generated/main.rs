pub mod management {
    include!("resources/management/group.rs");
    include!("resources/management/group_policy_assignment.rs");
    include!("resources/management/group_policy_exemption.rs");
    include!("resources/management/group_policy_remediation.rs");
    include!("resources/management/group_subscription_association.rs");
    include!("resources/management/group_template_deployment.rs");
    include!("resources/management/lock.rs");
    include!("resources/management/private_link.rs");
    include!("resources/management/private_link_association.rs");
}
pub mod functions {
    pub mod management {
        include!("functions/management/get_group.rs");
        include!("functions/management/get_group_template_deployment.rs");
    }
}
pub mod types {
    pub mod management {
        include!("types/management/group_policy_assignment_identity.rs");
        include!("types/management/group_policy_assignment_non_compliance_message.rs");
        include!("types/management/group_policy_assignment_override.rs");
        include!("types/management/group_policy_assignment_override_selector.rs");
        include!("types/management/group_policy_assignment_resource_selector.rs");
        include!(
            "types/management/group_policy_assignment_resource_selector_selector.rs"
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
