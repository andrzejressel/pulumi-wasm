#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_local_rulestack {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocalRulestackArgs {
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLocalRulestackResult {
        pub anti_spyware_profile: pulumi_gestalt_rust::Output<String>,
        pub anti_virus_profile: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub dns_subscription: pulumi_gestalt_rust::Output<String>,
        pub file_blocking_profile: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub outbound_trust_certificate: pulumi_gestalt_rust::Output<String>,
        pub outbound_untrust_certificate: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub url_filtering_profile: pulumi_gestalt_rust::Output<String>,
        pub vulnerability_profile: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLocalRulestackArgs,
    ) -> GetLocalRulestackResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:paloalto/getLocalRulestack:getLocalRulestack".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLocalRulestackResult {
            anti_spyware_profile: o.get_field("antiSpywareProfile"),
            anti_virus_profile: o.get_field("antiVirusProfile"),
            description: o.get_field("description"),
            dns_subscription: o.get_field("dnsSubscription"),
            file_blocking_profile: o.get_field("fileBlockingProfile"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            outbound_trust_certificate: o.get_field("outboundTrustCertificate"),
            outbound_untrust_certificate: o.get_field("outboundUntrustCertificate"),
            resource_group_name: o.get_field("resourceGroupName"),
            url_filtering_profile: o.get_field("urlFilteringProfile"),
            vulnerability_profile: o.get_field("vulnerabilityProfile"),
        }
    }
}
