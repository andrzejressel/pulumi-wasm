/// Manages a Palo Alto Networks Rulestack Certificate.
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
pub mod local_rulestack_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalRulestackCertificateArgs {
        /// The comment for Audit purposes.
        #[builder(into, default)]
        pub audit_comment: pulumi_wasm_rust::Output<Option<String>>,
        /// The description for the Certificate.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The `versionles_id` of the Key Vault Certificate to use. Changing this forces a new Palo Alto Networks Rulestack Certificate to be created.
        #[builder(into, default)]
        pub key_vault_certificate_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Palo Alto Networks Rulestack Certificate.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the TODO. Changing this forces a new Palo Alto Networks Rulestack Certificate to be created.
        #[builder(into)]
        pub rulestack_id: pulumi_wasm_rust::Output<String>,
        /// Should a Self Signed Certificate be used. Defaults to `false`. Changing this forces a new Palo Alto Networks Rulestack Certificate to be created.
        ///
        /// > **Note:** One and only one of `self_signed` or `key_vault_certificate_id` must be specified.
        #[builder(into, default)]
        pub self_signed: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct LocalRulestackCertificateResult {
        /// The comment for Audit purposes.
        pub audit_comment: pulumi_wasm_rust::Output<Option<String>>,
        /// The description for the Certificate.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The `versionles_id` of the Key Vault Certificate to use. Changing this forces a new Palo Alto Networks Rulestack Certificate to be created.
        pub key_vault_certificate_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Palo Alto Networks Rulestack Certificate.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the TODO. Changing this forces a new Palo Alto Networks Rulestack Certificate to be created.
        pub rulestack_id: pulumi_wasm_rust::Output<String>,
        /// Should a Self Signed Certificate be used. Defaults to `false`. Changing this forces a new Palo Alto Networks Rulestack Certificate to be created.
        ///
        /// > **Note:** One and only one of `self_signed` or `key_vault_certificate_id` must be specified.
        pub self_signed: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: LocalRulestackCertificateArgs,
    ) -> LocalRulestackCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let audit_comment_binding = args.audit_comment.get_inner();
        let description_binding = args.description.get_inner();
        let key_vault_certificate_id_binding = args.key_vault_certificate_id.get_inner();
        let name_binding = args.name.get_inner();
        let rulestack_id_binding = args.rulestack_id.get_inner();
        let self_signed_binding = args.self_signed.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:paloalto/localRulestackCertificate:LocalRulestackCertificate"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "auditComment".into(),
                    value: &audit_comment_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultCertificateId".into(),
                    value: &key_vault_certificate_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "rulestackId".into(),
                    value: &rulestack_id_binding,
                },
                register_interface::ObjectField {
                    name: "selfSigned".into(),
                    value: &self_signed_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "auditComment".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultCertificateId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "rulestackId".into(),
                },
                register_interface::ResultField {
                    name: "selfSigned".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LocalRulestackCertificateResult {
            audit_comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("auditComment").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            key_vault_certificate_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultCertificateId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            rulestack_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rulestackId").unwrap(),
            ),
            self_signed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfSigned").unwrap(),
            ),
        }
    }
}
