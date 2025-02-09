/// Manages a Palo Alto Local Rulestack FQDN List.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod local_rulestack_fqdn_list {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalRulestackFqdnListArgs {
        /// The comment for Audit purposes.
        #[builder(into, default)]
        pub audit_comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description for the FQDN List.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of Fully Qualified Domain Names.
        #[builder(into)]
        pub fully_qualified_domain_names: pulumi_gestalt_rust::InputOrOutput<
            Vec<String>,
        >,
        /// The name which should be used for this Palo Alto Local Rulestack FQDN List.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the TODO. Changing this forces a new Palo Alto Local Rulestack FQDN List to be created.
        #[builder(into)]
        pub rulestack_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LocalRulestackFqdnListResult {
        /// The comment for Audit purposes.
        pub audit_comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description for the FQDN List.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies a list of Fully Qualified Domain Names.
        pub fully_qualified_domain_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name which should be used for this Palo Alto Local Rulestack FQDN List.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the TODO. Changing this forces a new Palo Alto Local Rulestack FQDN List to be created.
        pub rulestack_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocalRulestackFqdnListArgs,
    ) -> LocalRulestackFqdnListResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let audit_comment_binding = args.audit_comment.get_output(context);
        let description_binding = args.description.get_output(context);
        let fully_qualified_domain_names_binding = args
            .fully_qualified_domain_names
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let rulestack_id_binding = args.rulestack_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:paloalto/localRulestackFqdnList:LocalRulestackFqdnList".into(),
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
                    name: "fullyQualifiedDomainNames".into(),
                    value: fully_qualified_domain_names_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rulestackId".into(),
                    value: rulestack_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LocalRulestackFqdnListResult {
            audit_comment: o.get_field("auditComment"),
            description: o.get_field("description"),
            fully_qualified_domain_names: o.get_field("fullyQualifiedDomainNames"),
            name: o.get_field("name"),
            rulestack_id: o.get_field("rulestackId"),
        }
    }
}
