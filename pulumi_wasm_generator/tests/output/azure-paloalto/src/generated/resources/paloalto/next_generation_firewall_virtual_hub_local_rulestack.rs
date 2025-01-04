pub mod next_generation_firewall_virtual_hub_local_rulestack {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NextGenerationFirewallVirtualHubLocalRulestackArgs {
        #[builder(into, default)]
        pub destination_nats: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackDestinationNat,
                >,
            >,
        >,
        #[builder(into, default)]
        pub dns_settings: pulumi_wasm_rust::Output<
            Option<
                super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackDnsSettings,
            >,
        >,
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into)]
        pub network_profile: pulumi_wasm_rust::Output<
            super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackNetworkProfile,
        >,
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub rulestack_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NextGenerationFirewallVirtualHubLocalRulestackResult {
        pub destination_nats: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackDestinationNat,
                >,
            >,
        >,
        pub dns_settings: pulumi_wasm_rust::Output<
            Option<
                super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackDnsSettings,
            >,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub network_profile: pulumi_wasm_rust::Output<
            super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackNetworkProfile,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub rulestack_id: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NextGenerationFirewallVirtualHubLocalRulestackArgs,
    ) -> NextGenerationFirewallVirtualHubLocalRulestackResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_nats_binding = args.destination_nats.get_inner();
        let dns_settings_binding = args.dns_settings.get_inner();
        let name_binding = args.name.get_inner();
        let network_profile_binding = args.network_profile.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let rulestack_id_binding = args.rulestack_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:paloalto/nextGenerationFirewallVirtualHubLocalRulestack:NextGenerationFirewallVirtualHubLocalRulestack"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "destinationNats".into(),
                },
                register_interface::ResultField {
                    name: "dnsSettings".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkProfile".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "rulestackId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NextGenerationFirewallVirtualHubLocalRulestackResult {
            destination_nats: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationNats").unwrap(),
            ),
            dns_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsSettings").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkProfile").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            rulestack_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rulestackId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
