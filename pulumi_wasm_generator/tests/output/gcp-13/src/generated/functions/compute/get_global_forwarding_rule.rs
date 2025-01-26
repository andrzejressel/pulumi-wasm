pub mod get_global_forwarding_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGlobalForwardingRuleArgs {
        /// The name of the global forwarding rule.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetGlobalForwardingRuleResult {
        pub allow_psc_global_access: pulumi_wasm_rust::Output<bool>,
        pub base_forwarding_rule: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub forwarding_rule_id: pulumi_wasm_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub ip_address: pulumi_wasm_rust::Output<String>,
        pub ip_protocol: pulumi_wasm_rust::Output<String>,
        pub ip_version: pulumi_wasm_rust::Output<String>,
        pub label_fingerprint: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub load_balancing_scheme: pulumi_wasm_rust::Output<String>,
        pub metadata_filters: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetGlobalForwardingRuleMetadataFilter,
            >,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub network: pulumi_wasm_rust::Output<String>,
        pub network_tier: pulumi_wasm_rust::Output<String>,
        pub no_automate_dns_zone: pulumi_wasm_rust::Output<bool>,
        pub port_range: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub psc_connection_id: pulumi_wasm_rust::Output<String>,
        pub psc_connection_status: pulumi_wasm_rust::Output<String>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub self_link: pulumi_wasm_rust::Output<String>,
        pub service_directory_registrations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetGlobalForwardingRuleServiceDirectoryRegistration,
            >,
        >,
        pub source_ip_ranges: pulumi_wasm_rust::Output<Vec<String>>,
        pub subnetwork: pulumi_wasm_rust::Output<String>,
        pub target: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetGlobalForwardingRuleArgs,
    ) -> GetGlobalForwardingRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getGlobalForwardingRule:getGlobalForwardingRule".into(),
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
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetGlobalForwardingRuleResult {
            allow_psc_global_access: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowPscGlobalAccess"),
            ),
            base_forwarding_rule: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("baseForwardingRule"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            forwarding_rule_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forwardingRuleId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipAddress"),
            ),
            ip_protocol: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipProtocol"),
            ),
            ip_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipVersion"),
            ),
            label_fingerprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("labelFingerprint"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            load_balancing_scheme: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loadBalancingScheme"),
            ),
            metadata_filters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metadataFilters"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            network_tier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkTier"),
            ),
            no_automate_dns_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("noAutomateDnsZone"),
            ),
            port_range: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("portRange"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            psc_connection_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pscConnectionId"),
            ),
            psc_connection_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pscConnectionStatus"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            service_directory_registrations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceDirectoryRegistrations"),
            ),
            source_ip_ranges: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceIpRanges"),
            ),
            subnetwork: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetwork"),
            ),
            target: pulumi_wasm_rust::__private::into_domain(o.extract_field("target")),
        }
    }
}
