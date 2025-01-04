pub mod appinsights {
    include!("resources/appinsights/analytics_item.rs");
    include!("resources/appinsights/api_key.rs");
    include!("resources/appinsights/insights.rs");
    include!("resources/appinsights/smart_detection_rule.rs");
    include!("resources/appinsights/standard_web_test.rs");
    include!("resources/appinsights/web_test.rs");
    include!("resources/appinsights/workbook.rs");
    include!("resources/appinsights/workbook_template.rs");
}
pub mod functions {
    pub mod appinsights {
        include!("functions/appinsights/get_insights.rs");
    }
}
pub mod types {
    pub mod appinsights {
        include!("types/appinsights/standard_web_test_request.rs");
        include!("types/appinsights/standard_web_test_request_header.rs");
        include!("types/appinsights/standard_web_test_validation_rules.rs");
        include!("types/appinsights/standard_web_test_validation_rules_content.rs");
        include!("types/appinsights/workbook_identity.rs");
        include!("types/appinsights/workbook_template_gallery.rs");
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
