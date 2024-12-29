/// Provides an CloudSearch domain service access policy resource.
///
/// The provider waits for the domain service access policy to become `Active` when applying a configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["cloudsearch:search", "cloudsearch:document",])
///                     .conditions(vec![GetPolicyDocumentStatementCondition::builder()
///                     .test("IpAddress").values(vec!["192.0.2.0/32",])
///                     .variable("aws:SourceIp").build_struct(),]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["*",]). type ("*").build_struct(),])
///                     .sid("search_only").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleDomain = domain::create(
///         "exampleDomain",
///         DomainArgs::builder().name("example-domain").build_struct(),
///     );
///     let exampleDomainServiceAccessPolicy = domain_service_access_policy::create(
///         "exampleDomainServiceAccessPolicy",
///         DomainServiceAccessPolicyArgs::builder()
///             .access_policy("${example.json}")
///             .domain_name("${exampleDomain.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudSearch domain service access policies using the domain name. For example:
///
/// ```sh
/// $ pulumi import aws:cloudsearch/domainServiceAccessPolicy:DomainServiceAccessPolicy example example-domain
/// ```
pub mod domain_service_access_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainServiceAccessPolicyArgs {
        /// The access rules you want to configure. These rules replace any existing rules. See the [AWS documentation](https://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-access.html) for details.
        #[builder(into)]
        pub access_policy: pulumi_wasm_rust::Output<String>,
        /// The CloudSearch domain name the policy applies to.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DomainServiceAccessPolicyResult {
        /// The access rules you want to configure. These rules replace any existing rules. See the [AWS documentation](https://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-access.html) for details.
        pub access_policy: pulumi_wasm_rust::Output<String>,
        /// The CloudSearch domain name the policy applies to.
        pub domain_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DomainServiceAccessPolicyArgs,
    ) -> DomainServiceAccessPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_policy_binding = args.access_policy.get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudsearch/domainServiceAccessPolicy:DomainServiceAccessPolicy"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessPolicy".into(),
                    value: &access_policy_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessPolicy".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainServiceAccessPolicyResult {
            access_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessPolicy").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
        }
    }
}
