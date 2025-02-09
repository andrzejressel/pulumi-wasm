#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_global_forwarding_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGlobalForwardingRuleArgs {
        /// The name of the global forwarding rule.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetGlobalForwardingRuleResult {
        pub allow_psc_global_access: pulumi_gestalt_rust::Output<bool>,
        pub base_forwarding_rule: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub forwarding_rule_id: pulumi_gestalt_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        pub ip_protocol: pulumi_gestalt_rust::Output<String>,
        pub ip_version: pulumi_gestalt_rust::Output<String>,
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub load_balancing_scheme: pulumi_gestalt_rust::Output<String>,
        pub metadata_filters: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetGlobalForwardingRuleMetadataFilter,
            >,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network: pulumi_gestalt_rust::Output<String>,
        pub network_tier: pulumi_gestalt_rust::Output<String>,
        pub no_automate_dns_zone: pulumi_gestalt_rust::Output<bool>,
        pub port_range: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub psc_connection_id: pulumi_gestalt_rust::Output<String>,
        pub psc_connection_status: pulumi_gestalt_rust::Output<String>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub self_link: pulumi_gestalt_rust::Output<String>,
        pub service_directory_registrations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetGlobalForwardingRuleServiceDirectoryRegistration,
            >,
        >,
        pub source_ip_ranges: pulumi_gestalt_rust::Output<Vec<String>>,
        pub subnetwork: pulumi_gestalt_rust::Output<String>,
        pub target: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetGlobalForwardingRuleArgs,
    ) -> GetGlobalForwardingRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getGlobalForwardingRule:getGlobalForwardingRule".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetGlobalForwardingRuleResult {
            allow_psc_global_access: o.get_field("allowPscGlobalAccess"),
            base_forwarding_rule: o.get_field("baseForwardingRule"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            forwarding_rule_id: o.get_field("forwardingRuleId"),
            id: o.get_field("id"),
            ip_address: o.get_field("ipAddress"),
            ip_protocol: o.get_field("ipProtocol"),
            ip_version: o.get_field("ipVersion"),
            label_fingerprint: o.get_field("labelFingerprint"),
            labels: o.get_field("labels"),
            load_balancing_scheme: o.get_field("loadBalancingScheme"),
            metadata_filters: o.get_field("metadataFilters"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            network_tier: o.get_field("networkTier"),
            no_automate_dns_zone: o.get_field("noAutomateDnsZone"),
            port_range: o.get_field("portRange"),
            project: o.get_field("project"),
            psc_connection_id: o.get_field("pscConnectionId"),
            psc_connection_status: o.get_field("pscConnectionStatus"),
            pulumi_labels: o.get_field("pulumiLabels"),
            self_link: o.get_field("selfLink"),
            service_directory_registrations: o
                .get_field("serviceDirectoryRegistrations"),
            source_ip_ranges: o.get_field("sourceIpRanges"),
            subnetwork: o.get_field("subnetwork"),
            target: o.get_field("target"),
        }
    }
}
