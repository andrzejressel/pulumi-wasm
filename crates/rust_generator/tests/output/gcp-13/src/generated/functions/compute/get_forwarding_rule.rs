#[allow(clippy::doc_lazy_continuation)]
pub mod get_forwarding_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetForwardingRuleArgs {
        /// The name of the forwarding rule.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region in which the resource belongs. If it
        /// is not provided, the project region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetForwardingRuleResult {
        pub all_ports: pulumi_gestalt_rust::Output<bool>,
        pub allow_global_access: pulumi_gestalt_rust::Output<bool>,
        pub allow_psc_global_access: pulumi_gestalt_rust::Output<bool>,
        pub backend_service: pulumi_gestalt_rust::Output<String>,
        pub base_forwarding_rule: pulumi_gestalt_rust::Output<String>,
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
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
        pub is_mirroring_collector: pulumi_gestalt_rust::Output<bool>,
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub load_balancing_scheme: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network: pulumi_gestalt_rust::Output<String>,
        pub network_tier: pulumi_gestalt_rust::Output<String>,
        pub no_automate_dns_zone: pulumi_gestalt_rust::Output<bool>,
        pub port_range: pulumi_gestalt_rust::Output<String>,
        pub ports: pulumi_gestalt_rust::Output<Vec<String>>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub psc_connection_id: pulumi_gestalt_rust::Output<String>,
        pub psc_connection_status: pulumi_gestalt_rust::Output<String>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub recreate_closed_psc: pulumi_gestalt_rust::Output<bool>,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        pub self_link: pulumi_gestalt_rust::Output<String>,
        pub service_directory_registrations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetForwardingRuleServiceDirectoryRegistration,
            >,
        >,
        pub service_label: pulumi_gestalt_rust::Output<String>,
        pub service_name: pulumi_gestalt_rust::Output<String>,
        pub source_ip_ranges: pulumi_gestalt_rust::Output<Vec<String>>,
        pub subnetwork: pulumi_gestalt_rust::Output<String>,
        pub target: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetForwardingRuleArgs,
    ) -> GetForwardingRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getForwardingRule:getForwardingRule".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetForwardingRuleResult {
            all_ports: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allPorts"),
            ),
            allow_global_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowGlobalAccess"),
            ),
            allow_psc_global_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowPscGlobalAccess"),
            ),
            backend_service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendService"),
            ),
            base_forwarding_rule: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("baseForwardingRule"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            forwarding_rule_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forwardingRuleId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddress"),
            ),
            ip_protocol: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipProtocol"),
            ),
            ip_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipVersion"),
            ),
            is_mirroring_collector: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isMirroringCollector"),
            ),
            label_fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labelFingerprint"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            load_balancing_scheme: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loadBalancingScheme"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            network_tier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkTier"),
            ),
            no_automate_dns_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("noAutomateDnsZone"),
            ),
            port_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("portRange"),
            ),
            ports: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ports")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            psc_connection_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pscConnectionId"),
            ),
            psc_connection_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pscConnectionStatus"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            recreate_closed_psc: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recreateClosedPsc"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            service_directory_registrations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceDirectoryRegistrations"),
            ),
            service_label: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceLabel"),
            ),
            service_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceName"),
            ),
            source_ip_ranges: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceIpRanges"),
            ),
            subnetwork: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetwork"),
            ),
            target: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("target"),
            ),
        }
    }
}
