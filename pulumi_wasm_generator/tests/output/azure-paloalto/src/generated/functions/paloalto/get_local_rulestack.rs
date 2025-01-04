pub mod get_local_rulestack {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocalRulestackArgs {
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetLocalRulestackResult {
        pub anti_spyware_profile: pulumi_wasm_rust::Output<String>,
        pub anti_virus_profile: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub dns_subscription: pulumi_wasm_rust::Output<String>,
        pub file_blocking_profile: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub outbound_trust_certificate: pulumi_wasm_rust::Output<String>,
        pub outbound_untrust_certificate: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub url_filtering_profile: pulumi_wasm_rust::Output<String>,
        pub vulnerability_profile: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetLocalRulestackArgs) -> GetLocalRulestackResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:paloalto/getLocalRulestack:getLocalRulestack".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "antiSpywareProfile".into(),
                },
                register_interface::ResultField {
                    name: "antiVirusProfile".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "dnsSubscription".into(),
                },
                register_interface::ResultField {
                    name: "fileBlockingProfile".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outboundTrustCertificate".into(),
                },
                register_interface::ResultField {
                    name: "outboundUntrustCertificate".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "urlFilteringProfile".into(),
                },
                register_interface::ResultField {
                    name: "vulnerabilityProfile".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLocalRulestackResult {
            anti_spyware_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("antiSpywareProfile").unwrap(),
            ),
            anti_virus_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("antiVirusProfile").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            dns_subscription: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsSubscription").unwrap(),
            ),
            file_blocking_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileBlockingProfile").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            outbound_trust_certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outboundTrustCertificate").unwrap(),
            ),
            outbound_untrust_certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outboundUntrustCertificate").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            url_filtering_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("urlFilteringProfile").unwrap(),
            ),
            vulnerability_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vulnerabilityProfile").unwrap(),
            ),
        }
    }
}
