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
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowPscGlobalAccess".into(),
                },
                register_interface::ResultField {
                    name: "baseForwardingRule".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "forwardingRuleId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipAddress".into(),
                },
                register_interface::ResultField {
                    name: "ipProtocol".into(),
                },
                register_interface::ResultField {
                    name: "ipVersion".into(),
                },
                register_interface::ResultField {
                    name: "labelFingerprint".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancingScheme".into(),
                },
                register_interface::ResultField {
                    name: "metadataFilters".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "networkTier".into(),
                },
                register_interface::ResultField {
                    name: "noAutomateDnsZone".into(),
                },
                register_interface::ResultField {
                    name: "portRange".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pscConnectionId".into(),
                },
                register_interface::ResultField {
                    name: "pscConnectionStatus".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "serviceDirectoryRegistrations".into(),
                },
                register_interface::ResultField {
                    name: "sourceIpRanges".into(),
                },
                register_interface::ResultField {
                    name: "subnetwork".into(),
                },
                register_interface::ResultField {
                    name: "target".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetGlobalForwardingRuleResult {
            allow_psc_global_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowPscGlobalAccess").unwrap(),
            ),
            base_forwarding_rule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseForwardingRule").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            forwarding_rule_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forwardingRuleId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddress").unwrap(),
            ),
            ip_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipProtocol").unwrap(),
            ),
            ip_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipVersion").unwrap(),
            ),
            label_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labelFingerprint").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            load_balancing_scheme: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancingScheme").unwrap(),
            ),
            metadata_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadataFilters").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            network_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkTier").unwrap(),
            ),
            no_automate_dns_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("noAutomateDnsZone").unwrap(),
            ),
            port_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("portRange").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            psc_connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pscConnectionId").unwrap(),
            ),
            psc_connection_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pscConnectionStatus").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            service_directory_registrations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceDirectoryRegistrations").unwrap(),
            ),
            source_ip_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceIpRanges").unwrap(),
            ),
            subnetwork: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetwork").unwrap(),
            ),
            target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("target").unwrap(),
            ),
        }
    }
}
