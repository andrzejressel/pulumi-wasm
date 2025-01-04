pub mod keyvault {
    include!("resources/keyvault/access_policy.rs");
    include!("resources/keyvault/certifiate.rs");
    include!("resources/keyvault/certificate.rs");
    include!("resources/keyvault/certificate_contacts.rs");
    include!("resources/keyvault/certificate_issuer.rs");
    include!("resources/keyvault/key.rs");
    include!("resources/keyvault/key_vault.rs");
    include!("resources/keyvault/managed_hardware_security_module.rs");
    include!("resources/keyvault/managed_hardware_security_module_key.rs");
    include!(
        "resources/keyvault/managed_hardware_security_module_key_rotation_policy.rs"
    );
    include!("resources/keyvault/managed_hardware_security_module_role_assignment.rs");
    include!("resources/keyvault/managed_hardware_security_module_role_definition.rs");
    include!("resources/keyvault/managed_storage_account.rs");
    include!("resources/keyvault/managed_storage_account_sas_token_definition.rs");
    include!("resources/keyvault/secret.rs");
}
pub mod functions {
    pub mod keyvault {
        include!("functions/keyvault/get_access_policy.rs");
        include!("functions/keyvault/get_certificate.rs");
        include!("functions/keyvault/get_certificate_data.rs");
        include!("functions/keyvault/get_certificate_issuer.rs");
        include!("functions/keyvault/get_certificates.rs");
        include!("functions/keyvault/get_encrypted_value.rs");
        include!("functions/keyvault/get_key.rs");
        include!("functions/keyvault/get_key_vault.rs");
        include!("functions/keyvault/get_managed_hardware_security_module.rs");
        include!("functions/keyvault/get_managed_hardware_security_module_key.rs");
        include!(
            "functions/keyvault/get_managed_hardware_security_module_role_definition.rs"
        );
        include!("functions/keyvault/get_secret.rs");
        include!("functions/keyvault/get_secrets.rs");
    }
}
pub mod types {
    pub mod keyvault {
        include!("types/keyvault/certifiate_certificate.rs");
        include!("types/keyvault/certifiate_certificate_attribute.rs");
        include!("types/keyvault/certifiate_certificate_policy.rs");
        include!("types/keyvault/certifiate_certificate_policy_issuer_parameters.rs");
        include!("types/keyvault/certifiate_certificate_policy_key_properties.rs");
        include!("types/keyvault/certifiate_certificate_policy_lifetime_action.rs");
        include!(
            "types/keyvault/certifiate_certificate_policy_lifetime_action_action.rs"
        );
        include!(
            "types/keyvault/certifiate_certificate_policy_lifetime_action_trigger.rs"
        );
        include!("types/keyvault/certifiate_certificate_policy_secret_properties.rs");
        include!(
            "types/keyvault/certifiate_certificate_policy_x_509_certificate_properties.rs"
        );
        include!(
            "types/keyvault/certifiate_certificate_policy_x_509_certificate_properties_subject_alternative_names.rs"
        );
        include!("types/keyvault/certificate_certificate.rs");
        include!("types/keyvault/certificate_certificate_attribute.rs");
        include!("types/keyvault/certificate_certificate_policy.rs");
        include!("types/keyvault/certificate_certificate_policy_issuer_parameters.rs");
        include!("types/keyvault/certificate_certificate_policy_key_properties.rs");
        include!("types/keyvault/certificate_certificate_policy_lifetime_action.rs");
        include!(
            "types/keyvault/certificate_certificate_policy_lifetime_action_action.rs"
        );
        include!(
            "types/keyvault/certificate_certificate_policy_lifetime_action_trigger.rs"
        );
        include!("types/keyvault/certificate_certificate_policy_secret_properties.rs");
        include!(
            "types/keyvault/certificate_certificate_policy_x_509_certificate_properties.rs"
        );
        include!(
            "types/keyvault/certificate_certificate_policy_x_509_certificate_properties_subject_alternative_names.rs"
        );
        include!("types/keyvault/certificate_contacts_contact.rs");
        include!("types/keyvault/certificate_issuer_admin.rs");
        include!("types/keyvault/key_rotation_policy.rs");
        include!("types/keyvault/key_rotation_policy_automatic.rs");
        include!("types/keyvault/key_vault_access_policy.rs");
        include!("types/keyvault/key_vault_contact.rs");
        include!("types/keyvault/key_vault_network_acls.rs");
        include!("types/keyvault/managed_hardware_security_module_network_acls.rs");
        include!(
            "types/keyvault/managed_hardware_security_module_role_definition_permission.rs"
        );
        include!("types/keyvault/get_certificate_certificate_policy.rs");
        include!(
            "types/keyvault/get_certificate_certificate_policy_issuer_parameter.rs"
        );
        include!("types/keyvault/get_certificate_certificate_policy_key_property.rs");
        include!("types/keyvault/get_certificate_certificate_policy_lifetime_action.rs");
        include!(
            "types/keyvault/get_certificate_certificate_policy_lifetime_action_action.rs"
        );
        include!(
            "types/keyvault/get_certificate_certificate_policy_lifetime_action_trigger.rs"
        );
        include!("types/keyvault/get_certificate_certificate_policy_secret_property.rs");
        include!(
            "types/keyvault/get_certificate_certificate_policy_x_509_certificate_property.rs"
        );
        include!(
            "types/keyvault/get_certificate_certificate_policy_x_509_certificate_property_subject_alternative_name.rs"
        );
        include!("types/keyvault/get_certificate_issuer_admin.rs");
        include!("types/keyvault/get_certificates_certificate.rs");
        include!("types/keyvault/get_key_vault_access_policy.rs");
        include!("types/keyvault/get_key_vault_network_acl.rs");
        include!(
            "types/keyvault/get_managed_hardware_security_module_role_definition_permission.rs"
        );
        include!("types/keyvault/get_secrets_secret.rs");
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
