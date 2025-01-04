pub mod get_forwarding_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetForwardingRuleArgs {
        /// The name of the forwarding rule.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region in which the resource belongs. If it
        /// is not provided, the project region is used.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetForwardingRuleResult {
        pub all_ports: pulumi_wasm_rust::Output<bool>,
        pub allow_global_access: pulumi_wasm_rust::Output<bool>,
        pub allow_psc_global_access: pulumi_wasm_rust::Output<bool>,
        pub backend_service: pulumi_wasm_rust::Output<String>,
        pub base_forwarding_rule: pulumi_wasm_rust::Output<String>,
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
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
        pub is_mirroring_collector: pulumi_wasm_rust::Output<bool>,
        pub label_fingerprint: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub load_balancing_scheme: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub network: pulumi_wasm_rust::Output<String>,
        pub network_tier: pulumi_wasm_rust::Output<String>,
        pub no_automate_dns_zone: pulumi_wasm_rust::Output<bool>,
        pub port_range: pulumi_wasm_rust::Output<String>,
        pub ports: pulumi_wasm_rust::Output<Vec<String>>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub psc_connection_id: pulumi_wasm_rust::Output<String>,
        pub psc_connection_status: pulumi_wasm_rust::Output<String>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub recreate_closed_psc: pulumi_wasm_rust::Output<bool>,
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        pub self_link: pulumi_wasm_rust::Output<String>,
        pub service_directory_registrations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetForwardingRuleServiceDirectoryRegistration,
            >,
        >,
        pub service_label: pulumi_wasm_rust::Output<String>,
        pub service_name: pulumi_wasm_rust::Output<String>,
        pub source_ip_ranges: pulumi_wasm_rust::Output<Vec<String>>,
        pub subnetwork: pulumi_wasm_rust::Output<String>,
        pub target: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetForwardingRuleArgs) -> GetForwardingRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getForwardingRule:getForwardingRule".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "allPorts".into(),
                },
                register_interface::ResultField {
                    name: "allowGlobalAccess".into(),
                },
                register_interface::ResultField {
                    name: "allowPscGlobalAccess".into(),
                },
                register_interface::ResultField {
                    name: "backendService".into(),
                },
                register_interface::ResultField {
                    name: "baseForwardingRule".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
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
                    name: "isMirroringCollector".into(),
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
                    name: "ports".into(),
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
                    name: "recreateClosedPsc".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "serviceDirectoryRegistrations".into(),
                },
                register_interface::ResultField {
                    name: "serviceLabel".into(),
                },
                register_interface::ResultField {
                    name: "serviceName".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetForwardingRuleResult {
            all_ports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allPorts").unwrap(),
            ),
            allow_global_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowGlobalAccess").unwrap(),
            ),
            allow_psc_global_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowPscGlobalAccess").unwrap(),
            ),
            backend_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendService").unwrap(),
            ),
            base_forwarding_rule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseForwardingRule").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
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
            is_mirroring_collector: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isMirroringCollector").unwrap(),
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
            ports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ports").unwrap(),
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
            recreate_closed_psc: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recreateClosedPsc").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            service_directory_registrations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceDirectoryRegistrations").unwrap(),
            ),
            service_label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceLabel").unwrap(),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceName").unwrap(),
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
