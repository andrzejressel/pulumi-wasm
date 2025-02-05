/// Manages a Palo Alto Local Rulestack Prefix List.
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
///     let exampleLocalRulestackPrefixList = local_rulestack_prefix_list::create(
///         "exampleLocalRulestackPrefixList",
///         LocalRulestackPrefixListArgs::builder()
///             .name("example")
///             .prefix_lists(vec!["10.0.1.0/24",])
///             .rulestack_id("${exampleLocalRulestack.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Palo Alto Local Rulestack Prefix Lists can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:paloalto/localRulestackPrefixList:LocalRulestackPrefixList example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/PaloAltoNetworks.Cloudngfw/localRulestacks/myLocalRulestack/prefixLists/myFQDNList1
/// ```
///
pub mod local_rulestack_prefix_list {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalRulestackPrefixListArgs {
        /// The comment for Audit purposes.
        #[builder(into, default)]
        pub audit_comment: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The description for the Prefix List.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Palo Alto Local Rulestack Prefix List.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of Prefixes.
        #[builder(into)]
        pub prefix_lists: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The ID of the Local Rulestack on which to create this Prefix List. Changing this forces a new Palo Alto Local Rulestack Prefix List to be created.
        #[builder(into)]
        pub rulestack_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LocalRulestackPrefixListResult {
        /// The comment for Audit purposes.
        pub audit_comment: pulumi_wasm_rust::Output<Option<String>>,
        /// The description for the Prefix List.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Palo Alto Local Rulestack Prefix List.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of Prefixes.
        pub prefix_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the Local Rulestack on which to create this Prefix List. Changing this forces a new Palo Alto Local Rulestack Prefix List to be created.
        pub rulestack_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LocalRulestackPrefixListArgs,
    ) -> LocalRulestackPrefixListResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let audit_comment_binding = args.audit_comment.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let prefix_lists_binding = args.prefix_lists.get_output(context).get_inner();
        let rulestack_id_binding = args.rulestack_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:paloalto/localRulestackPrefixList:LocalRulestackPrefixList"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "prefixLists".into(),
                    value: &prefix_lists_binding,
                },
                register_interface::ObjectField {
                    name: "rulestackId".into(),
                    value: &rulestack_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LocalRulestackPrefixListResult {
            audit_comment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("auditComment"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            prefix_lists: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("prefixLists"),
            ),
            rulestack_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rulestackId"),
            ),
        }
    }
}
