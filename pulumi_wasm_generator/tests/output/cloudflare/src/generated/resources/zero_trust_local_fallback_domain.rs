/// Provides a Cloudflare Fallback Domain resource. Fallback domains are
/// used to ignore DNS requests to a given list of domains. These DNS
/// requests will be passed back to other DNS servers configured on
/// existing network interfaces on the device.
pub mod zero_trust_local_fallback_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustLocalFallbackDomainArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub domains: pulumi_wasm_rust::Output<
            Vec<super::types::ZeroTrustLocalFallbackDomainDomain>,
        >,
        /// The settings policy for which to configure this fallback domain policy.
        #[builder(into, default)]
        pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustLocalFallbackDomainResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub domains: pulumi_wasm_rust::Output<
            Vec<super::types::ZeroTrustLocalFallbackDomainDomain>,
        >,
        /// The settings policy for which to configure this fallback domain policy.
        pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ZeroTrustLocalFallbackDomainArgs,
    ) -> ZeroTrustLocalFallbackDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let domains_binding = args.domains.get_inner();
        let policy_id_binding = args.policy_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustLocalFallbackDomain:ZeroTrustLocalFallbackDomain"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "domains".into(),
                    value: &domains_binding,
                },
                register_interface::ObjectField {
                    name: "policyId".into(),
                    value: &policy_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "domains".into(),
                },
                register_interface::ResultField {
                    name: "policyId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ZeroTrustLocalFallbackDomainResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            domains: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domains").unwrap(),
            ),
            policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyId").unwrap(),
            ),
        }
    }
}
