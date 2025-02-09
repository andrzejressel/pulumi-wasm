/// Manages a Palo Alto Networks Rulestack Outbound Trust Certificate Association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod local_rulestack_outbound_trust_certificate_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalRulestackOutboundTrustCertificateAssociationArgs {
        /// The ID of the Certificate to use as the Outbound Trust Certificate. Changing this forces a new Palo Alto Networks Rulestack Outbound Trust Certificate Association to be created.
        #[builder(into)]
        pub certificate_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LocalRulestackOutboundTrustCertificateAssociationResult {
        /// The ID of the Certificate to use as the Outbound Trust Certificate. Changing this forces a new Palo Alto Networks Rulestack Outbound Trust Certificate Association to be created.
        pub certificate_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LocalRulestackOutboundTrustCertificateAssociationArgs,
    ) -> LocalRulestackOutboundTrustCertificateAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let certificate_id_binding_1 = args.certificate_id.get_output(context);
        let certificate_id_binding = certificate_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:paloalto/localRulestackOutboundTrustCertificateAssociation:LocalRulestackOutboundTrustCertificateAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateId".into(),
                    value: &certificate_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LocalRulestackOutboundTrustCertificateAssociationResult {
            certificate_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateId"),
            ),
        }
    }
}
