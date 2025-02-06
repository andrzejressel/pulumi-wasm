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
    pulumi_gestalt_rust::__private::constant::generate_string_const!(
        ConstStringPav2, "Pav2"
    );
    pulumi_gestalt_rust::__private::constant::generate_string_const!(
        ConstStringPurchase, "Purchase"
    );
}
mod bindings {
    pulumi_gestalt_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-gestalt@0.0.0-DEV;

world world-myedgeorder {
    import output-interface;
}

interface pulumi-engine {
    resource engine {
        constructor(in-preview: bool);
    }
}

interface output-interface {
    use pulumi-engine.{engine};

    resource output {
        constructor(engine: borrow<engine>, value: string, secret: bool);
        map: func(function-name: string) -> output;
    }
    combine: func(outputs: list<borrow<output>>) -> output;

    resource register-output {
        extract-field: func(field-name: string) -> output;
    }
}

interface register-interface {
    use pulumi-engine.{engine};
    use output-interface.{output, register-output};

    record object-field {
        name: string,
        value: borrow<output>
    }

    record register-resource-request {
        %type: string,
        name: string,
        version: string,
        object: list<object-field>,
    }

    register: func(engine: borrow<engine>, request: register-resource-request) -> register-output;

    record resource-invoke-request {
        token: string,
        version: string,
        object: list<object-field>,
    }

    invoke: func(engine: borrow<engine>, request: resource-invoke-request) -> register-output;
}",
        with : { "component:pulumi-gestalt/output-interface@0.0.0-DEV" :
        pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::output_interface
        } }
    );
}
#[link_section = "pulumi_gestalt_provider::myedgeorder"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_MYEDGEORDER: [u8; 44] = *b"{\"version\":\"0.0.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "0.0.1".to_string()
}
