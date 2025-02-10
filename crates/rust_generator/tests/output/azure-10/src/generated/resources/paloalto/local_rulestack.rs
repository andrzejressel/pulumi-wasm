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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocalRulestackArgs,
    ) -> LocalRulestackResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let anti_spyware_profile_binding = args.anti_spyware_profile.get_output(context);
        let anti_virus_profile_binding = args.anti_virus_profile.get_output(context);
        let description_binding = args.description.get_output(context);
        let dns_subscription_binding = args.dns_subscription.get_output(context);
        let file_blocking_profile_binding = args
            .file_blocking_profile
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let url_filtering_profile_binding = args
            .url_filtering_profile
            .get_output(context);
        let vulnerability_profile_binding = args
            .vulnerability_profile
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:paloalto/localRulestack:LocalRulestack".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "antiSpywareProfile".into(),
                    value: anti_spyware_profile_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "antiVirusProfile".into(),
                    value: anti_virus_profile_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsSubscription".into(),
                    value: dns_subscription_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fileBlockingProfile".into(),
                    value: file_blocking_profile_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "urlFilteringProfile".into(),
                    value: url_filtering_profile_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vulnerabilityProfile".into(),
                    value: vulnerability_profile_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LocalRulestackResult {
            anti_spyware_profile: o.get_field("antiSpywareProfile"),
            anti_virus_profile: o.get_field("antiVirusProfile"),
            description: o.get_field("description"),
            dns_subscription: o.get_field("dnsSubscription"),
            file_blocking_profile: o.get_field("fileBlockingProfile"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            url_filtering_profile: o.get_field("urlFilteringProfile"),
            vulnerability_profile: o.get_field("vulnerabilityProfile"),
        }
    }
}
