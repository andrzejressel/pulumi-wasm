/// Manages a Palo Alto Local Rulestack FQDN List.
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
///             .resource_group_name("${exampleAzurermResrouceGroup.name}")
///             .build_struct(),
///     );
///     let exampleLocalRulestackFqdnList = local_rulestack_fqdn_list::create(
///         "exampleLocalRulestackFqdnList",
///         LocalRulestackFqdnListArgs::builder()
///             .fully_qualified_domain_names(vec!["contoso.com",])
///             .name("example")
///             .rulestack_id("${exampleLocalRulestack.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Palo Alto Local Rulestack FQDN Lists can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:paloalto/localRulestackFqdnList:LocalRulestackFqdnList example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/PaloAltoNetworks.Cloudngfw/localRulestacks/myLocalRulestack/fqdnLists/myFQDNList1
/// ```
///
pub mod local_rulestack_fqdn_list {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalRulestackFqdnListArgs {
        /// The comment for Audit purposes.
        #[builder(into, default)]
        pub audit_comment: pulumi_wasm_rust::Output<Option<String>>,
        /// The description for the FQDN List.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a list of Fully Qualified Domain Names.
        #[builder(into)]
        pub fully_qualified_domain_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name which should be used for this Palo Alto Local Rulestack FQDN List.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the TODO. Changing this forces a new Palo Alto Local Rulestack FQDN List to be created.
        #[builder(into)]
        pub rulestack_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LocalRulestackFqdnListResult {
        /// The comment for Audit purposes.
        pub audit_comment: pulumi_wasm_rust::Output<Option<String>>,
        /// The description for the FQDN List.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a list of Fully Qualified Domain Names.
        pub fully_qualified_domain_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name which should be used for this Palo Alto Local Rulestack FQDN List.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the TODO. Changing this forces a new Palo Alto Local Rulestack FQDN List to be created.
        pub rulestack_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: LocalRulestackFqdnListArgs,
    ) -> LocalRulestackFqdnListResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let audit_comment_binding = args.audit_comment.get_inner();
        let description_binding = args.description.get_inner();
        let fully_qualified_domain_names_binding = args
            .fully_qualified_domain_names
            .get_inner();
        let name_binding = args.name.get_inner();
        let rulestack_id_binding = args.rulestack_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:paloalto/localRulestackFqdnList:LocalRulestackFqdnList".into(),
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
                    name: "fullyQualifiedDomainNames".into(),
                    value: &fully_qualified_domain_names_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "rulestackId".into(),
                    value: &rulestack_id_binding,
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
                    name: "fullyQualifiedDomainNames".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "rulestackId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LocalRulestackFqdnListResult {
            audit_comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("auditComment").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            fully_qualified_domain_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fullyQualifiedDomainNames").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            rulestack_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rulestackId").unwrap(),
            ),
        }
    }
}