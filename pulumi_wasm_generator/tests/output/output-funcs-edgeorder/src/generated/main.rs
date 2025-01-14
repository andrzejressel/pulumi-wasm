pub mod functions {
    include!("functions/list_configurations.rs");
    include!("functions/list_product_families.rs");
}
pub mod types {
    include!("types/availability_information_response.rs");
    include!("types/billing_meter_details_response.rs");
    include!("types/configuration_filters.rs");
    include!("types/configuration_response.rs");
    include!("types/cost_information_response.rs");
    include!("types/customer_subscription_details.rs");
    include!("types/customer_subscription_registered_features.rs");
    include!("types/description_response.rs");
    include!("types/dimensions_response.rs");
    include!("types/filterable_property.rs");
    include!("types/filterable_property_response.rs");
    include!("types/hierarchy_information.rs");
    include!("types/hierarchy_information_response.rs");
    include!("types/image_information_response.rs");
    include!("types/link_response.rs");
    include!("types/pav_2_meter_details_response.rs");
    include!("types/product_family_response.rs");
    include!("types/product_line_response.rs");
    include!("types/product_response.rs");
    include!("types/purchase_meter_details_response.rs");
    include!("types/specification_response.rs");
    include!("types/supported_filter_types.rs");
}
#[doc(hidden)]
pub mod constants {
    pulumi_wasm_rust::__private::constant::generate_string_const!(
        ConstStringPav2, "Pav2"
    );
    pulumi_wasm_rust::__private::constant::generate_string_const!(
        ConstStringPurchase, "Purchase"
    );
}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-myedgeorder {
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
#[link_section = "pulumi_wasm_provider::myedgeorder"]
#[no_mangle]
pub static PULUMI_WASM_PROVIDER_myedgeorder: [u8; 5] = *b"0.0.1";
