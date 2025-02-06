/// Manages a Palo Alto Networks Rulestack.
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
/// }
/// ```
///
/// ## Import
///
/// Palo Alto Networks Rulestacks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:paloalto/localRulestack:LocalRulestack example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/PaloAltoNetworks.Cloudngfw/localRulestacks/myLocalRulestack
/// ```
///
pub mod local_rulestack {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalRulestackArgs {
        /// The setting to use for Anti-Spyware. Possible values include `BestPractice`, and `Custom`.
        #[builder(into, default)]
        pub anti_spyware_profile: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The setting to use for Anti-Virus. Possible values include `BestPractice`, and `Custom`.
        #[builder(into, default)]
        pub anti_virus_profile: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description for this Local Rulestack.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// TThe setting to use for DNS Subscription. Possible values include `BestPractice`, and `Custom`.
        #[builder(into, default)]
        pub dns_subscription: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The setting to use for the File Blocking Profile. Possible values include `BestPractice`, and `Custom`.
        #[builder(into, default)]
        pub file_blocking_profile: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure Region where the Palo Alto Networks Rulestack should exist. Changing this forces a new Palo Alto Networks Rulestack to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Palo Alto Networks Rulestack. Changing this forces a new Palo Alto Networks Rulestack to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Palo Alto Networks Rulestack should exist. Changing this forces a new Palo Alto Networks Rulestack to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The setting to use for the URL Filtering Profile. Possible values include `BestPractice`, and `Custom`.
        #[builder(into, default)]
        pub url_filtering_profile: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The setting to use for the Vulnerability Profile. Possible values include `BestPractice`, and `Custom`.
        #[builder(into, default)]
        pub vulnerability_profile: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LocalRulestackResult {
        /// The setting to use for Anti-Spyware. Possible values include `BestPractice`, and `Custom`.
        pub anti_spyware_profile: pulumi_gestalt_rust::Output<Option<String>>,
        /// The setting to use for Anti-Virus. Possible values include `BestPractice`, and `Custom`.
        pub anti_virus_profile: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description for this Local Rulestack.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// TThe setting to use for DNS Subscription. Possible values include `BestPractice`, and `Custom`.
        pub dns_subscription: pulumi_gestalt_rust::Output<Option<String>>,
        /// The setting to use for the File Blocking Profile. Possible values include `BestPractice`, and `Custom`.
        pub file_blocking_profile: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Azure Region where the Palo Alto Networks Rulestack should exist. Changing this forces a new Palo Alto Networks Rulestack to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Palo Alto Networks Rulestack. Changing this forces a new Palo Alto Networks Rulestack to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Palo Alto Networks Rulestack should exist. Changing this forces a new Palo Alto Networks Rulestack to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The setting to use for the URL Filtering Profile. Possible values include `BestPractice`, and `Custom`.
        pub url_filtering_profile: pulumi_gestalt_rust::Output<Option<String>>,
        /// The setting to use for the Vulnerability Profile. Possible values include `BestPractice`, and `Custom`.
        pub vulnerability_profile: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LocalRulestackArgs,
    ) -> LocalRulestackResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let anti_spyware_profile_binding = args
            .anti_spyware_profile
            .get_output(context)
            .get_inner();
        let anti_virus_profile_binding = args
            .anti_virus_profile
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let dns_subscription_binding = args
            .dns_subscription
            .get_output(context)
            .get_inner();
        let file_blocking_profile_binding = args
            .file_blocking_profile
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let url_filtering_profile_binding = args
            .url_filtering_profile
            .get_output(context)
            .get_inner();
        let vulnerability_profile_binding = args
            .vulnerability_profile
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:paloalto/localRulestack:LocalRulestack".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "antiSpywareProfile".into(),
                    value: &anti_spyware_profile_binding,
                },
                register_interface::ObjectField {
                    name: "antiVirusProfile".into(),
                    value: &anti_virus_profile_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "dnsSubscription".into(),
                    value: &dns_subscription_binding,
                },
                register_interface::ObjectField {
                    name: "fileBlockingProfile".into(),
                    value: &file_blocking_profile_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "urlFilteringProfile".into(),
                    value: &url_filtering_profile_binding,
                },
                register_interface::ObjectField {
                    name: "vulnerabilityProfile".into(),
                    value: &vulnerability_profile_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LocalRulestackResult {
            anti_spyware_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("antiSpywareProfile"),
            ),
            anti_virus_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("antiVirusProfile"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            dns_subscription: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsSubscription"),
            ),
            file_blocking_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fileBlockingProfile"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            url_filtering_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("urlFilteringProfile"),
            ),
            vulnerability_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vulnerabilityProfile"),
            ),
        }
    }
}
