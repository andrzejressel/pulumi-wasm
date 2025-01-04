/// Manages an Attestation Provider.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleProvider:
///     type: azure:attestation:Provider
///     name: example
///     properties:
///       name: exampleprovider
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       policySigningCertificateData:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: ./example/cert.pem
///           return: result
/// ```
///
/// ## Import
///
/// Attestation Providers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:attestation/provider:Provider example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Attestation/attestationProviders/provider1
/// ```
///
pub mod provider {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProviderArgs {
        /// The Azure Region where the Attestation Provider should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Attestation Provider. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        #[builder(into, default)]
        pub open_enclave_policy_base64: pulumi_wasm_rust::Output<Option<String>>,
        /// A valid X.509 certificate (Section 4 of [RFC4648](https://tools.ietf.org/html/rfc4648)). Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If the `policy_signing_certificate_data` argument contains more than one valid X.509 certificate only the first certificate will be used.
        #[builder(into, default)]
        pub policy_signing_certificate_data: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the attestation provider should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        ///
        /// > [More information on the JWT Policies can be found in this article on `learn.microsoft.com`](https://learn.microsoft.com/azure/attestation/author-sign-policy).
        #[builder(into, default)]
        pub sev_snp_policy_base64: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        #[builder(into, default)]
        pub sgx_enclave_policy_base64: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Attestation Provider.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        #[builder(into, default)]
        pub tpm_policy_base64: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProviderResult {
        /// The URI of the Attestation Service.
        pub attestation_uri: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Attestation Provider should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Attestation Provider. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        pub open_enclave_policy_base64: pulumi_wasm_rust::Output<Option<String>>,
        /// A valid X.509 certificate (Section 4 of [RFC4648](https://tools.ietf.org/html/rfc4648)). Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If the `policy_signing_certificate_data` argument contains more than one valid X.509 certificate only the first certificate will be used.
        pub policy_signing_certificate_data: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the attestation provider should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        ///
        /// > [More information on the JWT Policies can be found in this article on `learn.microsoft.com`](https://learn.microsoft.com/azure/attestation/author-sign-policy).
        pub sev_snp_policy_base64: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        pub sgx_enclave_policy_base64: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Attestation Provider.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        pub tpm_policy_base64: pulumi_wasm_rust::Output<Option<String>>,
        /// Trust model used for the Attestation Service.
        pub trust_model: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ProviderArgs) -> ProviderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let open_enclave_policy_base64_binding = args
            .open_enclave_policy_base64
            .get_inner();
        let policy_signing_certificate_data_binding = args
            .policy_signing_certificate_data
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sev_snp_policy_base64_binding = args.sev_snp_policy_base64.get_inner();
        let sgx_enclave_policy_base64_binding = args
            .sgx_enclave_policy_base64
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let tpm_policy_base64_binding = args.tpm_policy_base64.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:attestation/provider:Provider".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "openEnclavePolicyBase64".into(),
                    value: &open_enclave_policy_base64_binding,
                },
                register_interface::ObjectField {
                    name: "policySigningCertificateData".into(),
                    value: &policy_signing_certificate_data_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sevSnpPolicyBase64".into(),
                    value: &sev_snp_policy_base64_binding,
                },
                register_interface::ObjectField {
                    name: "sgxEnclavePolicyBase64".into(),
                    value: &sgx_enclave_policy_base64_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tpmPolicyBase64".into(),
                    value: &tpm_policy_base64_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "attestationUri".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "openEnclavePolicyBase64".into(),
                },
                register_interface::ResultField {
                    name: "policySigningCertificateData".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sevSnpPolicyBase64".into(),
                },
                register_interface::ResultField {
                    name: "sgxEnclavePolicyBase64".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tpmPolicyBase64".into(),
                },
                register_interface::ResultField {
                    name: "trustModel".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProviderResult {
            attestation_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attestationUri").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            open_enclave_policy_base64: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("openEnclavePolicyBase64").unwrap(),
            ),
            policy_signing_certificate_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policySigningCertificateData").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sev_snp_policy_base64: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sevSnpPolicyBase64").unwrap(),
            ),
            sgx_enclave_policy_base64: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sgxEnclavePolicyBase64").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tpm_policy_base64: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tpmPolicyBase64").unwrap(),
            ),
            trust_model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustModel").unwrap(),
            ),
        }
    }
}
