/// Manages a Palo Alto Networks Rulestack Certificate.
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
/// }
/// ```
///
/// ## Import
///
/// Palo Alto Networks Rulestack Certificates can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:paloalto/localRulestackCertificate:LocalRulestackCertificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/PaloAltoNetworks.Cloudngfw/localRulestacks/myLocalRulestack/certificates/myCertificate
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod local_rulestack_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalRulestackCertificateArgs {
        /// The comment for Audit purposes.
        #[builder(into, default)]
        pub audit_comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description for the Certificate.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The `versionles_id` of the Key Vault Certificate to use. Changing this forces a new Palo Alto Networks Rulestack Certificate to be created.
        #[builder(into, default)]
        pub key_vault_certificate_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Palo Alto Networks Rulestack Certificate.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the TODO. Changing this forces a new Palo Alto Networks Rulestack Certificate to be created.
        #[builder(into)]
        pub rulestack_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Should a Self Signed Certificate be used. Defaults to `false`. Changing this forces a new Palo Alto Networks Rulestack Certificate to be created.
        ///
        /// > **Note:** One and only one of `self_signed` or `key_vault_certificate_id` must be specified.
        #[builder(into, default)]
        pub self_signed: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct LocalRulestackCertificateResult {
        /// The comment for Audit purposes.
        pub audit_comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description for the Certificate.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The `versionles_id` of the Key Vault Certificate to use. Changing this forces a new Palo Alto Networks Rulestack Certificate to be created.
        pub key_vault_certificate_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Palo Alto Networks Rulestack Certificate.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the TODO. Changing this forces a new Palo Alto Networks Rulestack Certificate to be created.
        pub rulestack_id: pulumi_gestalt_rust::Output<String>,
        /// Should a Self Signed Certificate be used. Defaults to `false`. Changing this forces a new Palo Alto Networks Rulestack Certificate to be created.
        ///
        /// > **Note:** One and only one of `self_signed` or `key_vault_certificate_id` must be specified.
        pub self_signed: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocalRulestackCertificateArgs,
    ) -> LocalRulestackCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let audit_comment_binding = args.audit_comment.get_output(context);
        let description_binding = args.description.get_output(context);
        let key_vault_certificate_id_binding = args
            .key_vault_certificate_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let rulestack_id_binding = args.rulestack_id.get_output(context);
        let self_signed_binding = args.self_signed.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:paloalto/localRulestackCertificate:LocalRulestackCertificate"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "auditComment".into(),
                    value: audit_comment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultCertificateId".into(),
                    value: key_vault_certificate_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rulestackId".into(),
                    value: rulestack_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selfSigned".into(),
                    value: self_signed_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LocalRulestackCertificateResult {
            audit_comment: o.get_field("auditComment"),
            description: o.get_field("description"),
            key_vault_certificate_id: o.get_field("keyVaultCertificateId"),
            name: o.get_field("name"),
            rulestack_id: o.get_field("rulestackId"),
            self_signed: o.get_field("selfSigned"),
        }
    }
}
