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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod provider {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProviderArgs {
        /// The Azure Region where the Attestation Provider should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Attestation Provider. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        #[builder(into, default)]
        pub open_enclave_policy_base64: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A valid X.509 certificate (Section 4 of [RFC4648](https://tools.ietf.org/html/rfc4648)). Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If the `policy_signing_certificate_data` argument contains more than one valid X.509 certificate only the first certificate will be used.
        #[builder(into, default)]
        pub policy_signing_certificate_data: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the Resource Group where the attestation provider should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        ///
        /// > [More information on the JWT Policies can be found in this article on `learn.microsoft.com`](https://learn.microsoft.com/azure/attestation/author-sign-policy).
        #[builder(into, default)]
        pub sev_snp_policy_base64: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        #[builder(into, default)]
        pub sgx_enclave_policy_base64: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A mapping of tags which should be assigned to the Attestation Provider.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        #[builder(into, default)]
        pub tpm_policy_base64: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProviderResult {
        /// The URI of the Attestation Service.
        pub attestation_uri: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Attestation Provider should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Attestation Provider. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        pub open_enclave_policy_base64: pulumi_gestalt_rust::Output<Option<String>>,
        /// A valid X.509 certificate (Section 4 of [RFC4648](https://tools.ietf.org/html/rfc4648)). Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If the `policy_signing_certificate_data` argument contains more than one valid X.509 certificate only the first certificate will be used.
        pub policy_signing_certificate_data: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group where the attestation provider should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        ///
        /// > [More information on the JWT Policies can be found in this article on `learn.microsoft.com`](https://learn.microsoft.com/azure/attestation/author-sign-policy).
        pub sev_snp_policy_base64: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        pub sgx_enclave_policy_base64: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Attestation Provider.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the base64 URI Encoded RFC 7519 JWT that should be used for the Attestation Policy.
        pub tpm_policy_base64: pulumi_gestalt_rust::Output<Option<String>>,
        /// Trust model used for the Attestation Service.
        pub trust_model: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProviderArgs,
    ) -> ProviderResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let open_enclave_policy_base64_binding = args
            .open_enclave_policy_base64
            .get_output(context);
        let policy_signing_certificate_data_binding = args
            .policy_signing_certificate_data
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sev_snp_policy_base64_binding = args
            .sev_snp_policy_base64
            .get_output(context);
        let sgx_enclave_policy_base64_binding = args
            .sgx_enclave_policy_base64
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tpm_policy_base64_binding = args.tpm_policy_base64.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:attestation/provider:Provider".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "openEnclavePolicyBase64".into(),
                    value: open_enclave_policy_base64_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policySigningCertificateData".into(),
                    value: policy_signing_certificate_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sevSnpPolicyBase64".into(),
                    value: sev_snp_policy_base64_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sgxEnclavePolicyBase64".into(),
                    value: sgx_enclave_policy_base64_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tpmPolicyBase64".into(),
                    value: tpm_policy_base64_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProviderResult {
            attestation_uri: o.get_field("attestationUri"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            open_enclave_policy_base64: o.get_field("openEnclavePolicyBase64"),
            policy_signing_certificate_data: o.get_field("policySigningCertificateData"),
            resource_group_name: o.get_field("resourceGroupName"),
            sev_snp_policy_base64: o.get_field("sevSnpPolicyBase64"),
            sgx_enclave_policy_base64: o.get_field("sgxEnclavePolicyBase64"),
            tags: o.get_field("tags"),
            tpm_policy_base64: o.get_field("tpmPolicyBase64"),
            trust_model: o.get_field("trustModel"),
        }
    }
}
