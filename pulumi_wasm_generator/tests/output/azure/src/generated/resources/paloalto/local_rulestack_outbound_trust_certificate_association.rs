/// Manages a Palo Alto Networks Rulestack Outbound Trust Certificate Association.
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
///             .name("rg-example")
///             .build_struct(),
///     );
///     let exampleLocalRulestack = local_rulestack::create(
///         "exampleLocalRulestack",
///         LocalRulestackArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLocalRulestackCertificate = local_rulestack_certificate::create(
///         "exampleLocalRulestackCertificate",
///         LocalRulestackCertificateArgs::builder()
///             .name("example")
///             .rulestack_id("${exampleLocalRulestack.id}")
///             .self_signed(true)
///             .build_struct(),
///     );
///     let exampleLocalRulestackOutboundTrustCertificateAssociation = local_rulestack_outbound_trust_certificate_association::create(
///         "exampleLocalRulestackOutboundTrustCertificateAssociation",
///         LocalRulestackOutboundTrustCertificateAssociationArgs::builder()
///             .certificate_id("${exampleLocalRulestackCertificate.id}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod local_rulestack_outbound_trust_certificate_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalRulestackOutboundTrustCertificateAssociationArgs {
        /// The ID of the Certificate to use as the Outbound Trust Certificate. Changing this forces a new Palo Alto Networks Rulestack Outbound Trust Certificate Association to be created.
        #[builder(into)]
        pub certificate_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LocalRulestackOutboundTrustCertificateAssociationResult {
        /// The ID of the Certificate to use as the Outbound Trust Certificate. Changing this forces a new Palo Alto Networks Rulestack Outbound Trust Certificate Association to be created.
        pub certificate_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: LocalRulestackOutboundTrustCertificateAssociationArgs,
    ) -> LocalRulestackOutboundTrustCertificateAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_id_binding = args.certificate_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:paloalto/localRulestackOutboundTrustCertificateAssociation:LocalRulestackOutboundTrustCertificateAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateId".into(),
                    value: &certificate_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificateId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LocalRulestackOutboundTrustCertificateAssociationResult {
            certificate_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateId").unwrap(),
            ),
        }
    }
}