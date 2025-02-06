pub mod get_local_rulestack {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocalRulestackArgs {
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetLocalRulestackArgs,
    ) -> GetLocalRulestackResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:paloalto/getLocalRulestack:getLocalRulestack".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLocalRulestackResult {
            anti_spyware_profile: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("antiSpywareProfile"),
            ),
            anti_virus_profile: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("antiVirusProfile"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            dns_subscription: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsSubscription"),
            ),
            file_blocking_profile: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fileBlockingProfile"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            outbound_trust_certificate: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outboundTrustCertificate"),
            ),
            outbound_untrust_certificate: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outboundUntrustCertificate"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            url_filtering_profile: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("urlFilteringProfile"),
            ),
            vulnerability_profile: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vulnerabilityProfile"),
            ),
        }
    }
}
