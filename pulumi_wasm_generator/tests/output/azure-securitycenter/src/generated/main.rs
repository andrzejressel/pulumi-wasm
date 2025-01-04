pub mod securitycenter {
    include!("resources/securitycenter/advanced_threat_protection.rs");
    include!("resources/securitycenter/assessment.rs");
    include!("resources/securitycenter/assessment_policy.rs");
    include!("resources/securitycenter/auto_provisioning.rs");
    include!("resources/securitycenter/automation.rs");
    include!("resources/securitycenter/contact.rs");
    include!(
        "resources/securitycenter/server_vulnerability_assessment_virtual_machine.rs"
    );
    include!("resources/securitycenter/server_vulnerability_assessments_setting.rs");
    include!("resources/securitycenter/setting.rs");
    include!("resources/securitycenter/storage_defender.rs");
    include!("resources/securitycenter/subscription_pricing.rs");
    include!("resources/securitycenter/workspace.rs");
}
pub mod functions {}
pub mod types {
    pub mod securitycenter {
        include!("types/securitycenter/assessment_status.rs");
        include!("types/securitycenter/automation_action.rs");
        include!("types/securitycenter/automation_source.rs");
        include!("types/securitycenter/automation_source_rule_set.rs");
        include!("types/securitycenter/automation_source_rule_set_rule.rs");
        include!("types/securitycenter/subscription_pricing_extension.rs");
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
