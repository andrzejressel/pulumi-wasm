pub mod healthcare {
    include!("resources/healthcare/dicom_service.rs");
    include!("resources/healthcare/fhir_service.rs");
    include!("resources/healthcare/medtech_service.rs");
    include!("resources/healthcare/medtech_service_fhir_destination.rs");
    include!("resources/healthcare/service.rs");
    include!("resources/healthcare/workspace.rs");
}
pub mod functions {
    pub mod healthcare {
        include!("functions/healthcare/get_dicom_service.rs");
        include!("functions/healthcare/get_fhir_service.rs");
        include!("functions/healthcare/get_medtech_service.rs");
        include!("functions/healthcare/get_service.rs");
        include!("functions/healthcare/get_workspace.rs");
    }
}
pub mod types {
    pub mod healthcare {
        include!("types/healthcare/dicom_service_authentication.rs");
        include!("types/healthcare/dicom_service_identity.rs");
        include!("types/healthcare/dicom_service_private_endpoint.rs");
        include!("types/healthcare/fhir_service_authentication.rs");
        include!("types/healthcare/fhir_service_cors.rs");
        include!("types/healthcare/fhir_service_identity.rs");
        include!("types/healthcare/fhir_service_oci_artifact.rs");
        include!("types/healthcare/medtech_service_identity.rs");
        include!("types/healthcare/service_authentication_configuration.rs");
        include!("types/healthcare/service_cors_configuration.rs");
        include!("types/healthcare/service_identity.rs");
        include!("types/healthcare/workspace_private_endpoint_connection.rs");
        include!("types/healthcare/get_dicom_service_authentication.rs");
        include!("types/healthcare/get_dicom_service_identity.rs");
        include!("types/healthcare/get_dicom_service_private_endpoint.rs");
        include!("types/healthcare/get_fhir_service_authentication.rs");
        include!("types/healthcare/get_fhir_service_cor.rs");
        include!("types/healthcare/get_fhir_service_identity.rs");
        include!("types/healthcare/get_medtech_service_identity.rs");
        include!("types/healthcare/get_service_authentication_configuration.rs");
        include!("types/healthcare/get_service_cors_configuration.rs");
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
