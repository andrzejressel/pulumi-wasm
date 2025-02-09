#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod next_generation_firewall_virtual_hub_local_rulestack {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NextGenerationFirewallVirtualHubLocalRulestackArgs {
        #[builder(into, default)]
        pub destination_nats: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackDestinationNat,
                >,
            >,
        >,
        #[builder(into, default)]
        pub dns_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackDnsSettings,
            >,
        >,
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub network_profile: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackNetworkProfile,
        >,
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub rulestack_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NextGenerationFirewallVirtualHubLocalRulestackResult {
        pub destination_nats: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackDestinationNat,
                >,
            >,
        >,
        pub dns_settings: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackDnsSettings,
            >,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network_profile: pulumi_gestalt_rust::Output<
            super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackNetworkProfile,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub rulestack_id: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NextGenerationFirewallVirtualHubLocalRulestackArgs,
    ) -> NextGenerationFirewallVirtualHubLocalRulestackResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let destination_nats_binding_1 = args.destination_nats.get_output(context);
        let destination_nats_binding = destination_nats_binding_1.get_inner();
        let dns_settings_binding_1 = args.dns_settings.get_output(context);
        let dns_settings_binding = dns_settings_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let network_profile_binding_1 = args.network_profile.get_output(context);
        let network_profile_binding = network_profile_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let rulestack_id_binding_1 = args.rulestack_id.get_output(context);
        let rulestack_id_binding = rulestack_id_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:paloalto/nextGenerationFirewallVirtualHubLocalRulestack:NextGenerationFirewallVirtualHubLocalRulestack"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationNats".into(),
                    value: &destination_nats_binding,
                },
                register_interface::ObjectField {
                    name: "dnsSettings".into(),
                    value: &dns_settings_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkProfile".into(),
                    value: &network_profile_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "rulestackId".into(),
                    value: &rulestack_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NextGenerationFirewallVirtualHubLocalRulestackResult {
            destination_nats: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationNats"),
            ),
            dns_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsSettings"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkProfile"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            rulestack_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rulestackId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
