/// Manages a Logic App Integration Account Certificate.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleIntegrationAccount = integration_account::create(
///         "exampleIntegrationAccount",
///         IntegrationAccountArgs::builder()
///             .location("${example.location}")
///             .name("example-ia")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard")
///             .build_struct(),
///     );
///     let exampleIntegrationAccountCertificate = integration_account_certificate::create(
///         "exampleIntegrationAccountCertificate",
///         IntegrationAccountCertificateArgs::builder()
///             .integration_account_name("${exampleIntegrationAccount.name}")
///             .name("example-iac")
///             .public_certificate(
///                 "MIIDbzCCAlegAwIBAgIJAIzjRD36sIbbMA0GCSqGSIb3DQEBCwUAME0xCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApTb21lLVN0YXRlMRIwEAYDVQQKDAl0ZXJyYWZvcm0xFTATBgNVBAMMDHRlcnJhZm9ybS5pbzAgFw0xNzA0MjEyMDA1MjdaGA8yMTE3MDMyODIwMDUyN1owTTELMAkGA1UEBhMCVVMxEzARBgNVBAgMClNvbWUtU3RhdGUxEjAQBgNVBAoMCXRlcnJhZm9ybTEVMBMGA1UEAwwMdGVycmFmb3JtLmlvMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA3L9L5szT4+FLykTFNyyPjy/k3BQTYAfRQzP2dhnsuUKm3cdPC0NyZ+wEXIUGhoDO2YG6EYChOl8fsDqDOjloSUGKqYw++nlpHIuUgJx8IxxG2XkALCjFU7EmF+w7kn76d0ezpEIYxnLP+KG2DVornoEt1aLhv1MLmpgEZZPhDbMSLhSYWeTVRMayXLwqtfgnDumQSB+8d/1JuJqrSI4pD12JozVThzb6hsjfb6RMX4epPmrGn0PbTPEEA6awmsxBCXB0s13nNQt/O0hLM2agwvAyozilQV+s616Ckgk6DJoUkqZhDy7vPYMIRSr98fBws6zkrV6tTLjmD8xAvobePQIDAQABo1AwTjAdBgNVHQ4EFgQUXIqO421zMMmbcRRX9wctZFCQuPIwHwYDVR0jBBgwFoAUXIqO421zMMmbcRRX9wctZFCQuPIwDAYDVR0TBAUwAwEB/zANBgkqhkiG9w0BAQsFAAOCAQEAr82NeT3BYJOKLlUL6Om5LjUF66ewcJjG9ltdvyQwVneMcq7t5UAPxgChzqNRVk4da8PzkXpjBJyWezHupdJNX3XqeUk2kSxqQ6/gmhqvfI3y7djrwoO6jvMEY26WqtkTNORWDP3THJJVimC3zV+KMU5UBVrEzhOVhHSU709lBP75o0BBn3xGsPqSq9k8IotIFfyAc6a+XP3+ZMpvh7wqAUml7vWa5wlcXExCx39h1balfDSLGNC4swWPCp9AMnQR0p+vMay9hNP1Eh+9QYUai14d5KS3cFV+KxE1cJR5HD/iLltnnOEbpMsB0eVOZWkFvE7Y5lW0oVSAfin5TwTJMQ==",
///             )
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Logic App Integration Account Certificates can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/integrationAccountCertificate:IntegrationAccountCertificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Logic/integrationAccounts/account1/certificates/certificate1
/// ```
///
pub mod integration_account_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationAccountCertificateArgs {
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Certificate to be created.
        #[builder(into)]
        pub integration_account_name: pulumi_wasm_rust::Output<String>,
        /// A `key_vault_key` block as documented below.
        #[builder(into, default)]
        pub key_vault_key: pulumi_wasm_rust::Output<
            Option<
                super::super::types::logicapps::IntegrationAccountCertificateKeyVaultKey,
            >,
        >,
        /// A JSON mapping of any Metadata for this Logic App Integration Account Certificate.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Logic App Integration Account Certificate. Changing this forces a new Logic App Integration Account Certificate to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The public certificate for the Logic App Integration Account Certificate.
        #[builder(into, default)]
        pub public_certificate: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Logic App Integration Account Certificate should exist. Changing this forces a new Logic App Integration Account Certificate to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationAccountCertificateResult {
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Certificate to be created.
        pub integration_account_name: pulumi_wasm_rust::Output<String>,
        /// A `key_vault_key` block as documented below.
        pub key_vault_key: pulumi_wasm_rust::Output<
            Option<
                super::super::types::logicapps::IntegrationAccountCertificateKeyVaultKey,
            >,
        >,
        /// A JSON mapping of any Metadata for this Logic App Integration Account Certificate.
        pub metadata: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Logic App Integration Account Certificate. Changing this forces a new Logic App Integration Account Certificate to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The public certificate for the Logic App Integration Account Certificate.
        pub public_certificate: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Logic App Integration Account Certificate should exist. Changing this forces a new Logic App Integration Account Certificate to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: IntegrationAccountCertificateArgs,
    ) -> IntegrationAccountCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let integration_account_name_binding = args.integration_account_name.get_inner();
        let key_vault_key_binding = args.key_vault_key.get_inner();
        let metadata_binding = args.metadata.get_inner();
        let name_binding = args.name.get_inner();
        let public_certificate_binding = args.public_certificate.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:logicapps/integrationAccountCertificate:IntegrationAccountCertificate"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "integrationAccountName".into(),
                    value: &integration_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultKey".into(),
                    value: &key_vault_key_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicCertificate".into(),
                    value: &public_certificate_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "integrationAccountName".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultKey".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicCertificate".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IntegrationAccountCertificateResult {
            integration_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationAccountName").unwrap(),
            ),
            key_vault_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultKey").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicCertificate").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}