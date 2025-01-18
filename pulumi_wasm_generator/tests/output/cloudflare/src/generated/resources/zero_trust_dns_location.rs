/// Provides a Cloudflare Teams Location resource. Teams Locations are
/// referenced when creating secure web gateway policies.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zero_trust_dns_location::create(
///         "example",
///         ZeroTrustDnsLocationArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .client_default(true)
///             .ecs_support(false)
///             .name("office")
///             .networks(
///                 vec![
///                     ZeroTrustDnsLocationNetwork::builder().network("203.0.113.1/32")
///                     .build_struct(), ZeroTrustDnsLocationNetwork::builder()
///                     .network("203.0.113.2/32").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustDnsLocation:ZeroTrustDnsLocation example <account_id>/<teams_location_id>
/// ```
///
pub mod zero_trust_dns_location {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustDnsLocationArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Indicator that this is the default location.
        #[builder(into, default)]
        pub client_default: pulumi_wasm_rust::Output<Option<bool>>,
        /// Indicator that this location needs to resolve EDNS queries.
        #[builder(into, default)]
        pub ecs_support: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the teams location.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The networks CIDRs that comprise the location.
        #[builder(into, default)]
        pub networks: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ZeroTrustDnsLocationNetwork>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustDnsLocationResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Indicator that anonymized logs are enabled.
        pub anonymized_logs_enabled: pulumi_wasm_rust::Output<bool>,
        /// Indicator that this is the default location.
        pub client_default: pulumi_wasm_rust::Output<Option<bool>>,
        /// The FQDN that DoH clients should be pointed at.
        pub doh_subdomain: pulumi_wasm_rust::Output<String>,
        /// Indicator that this location needs to resolve EDNS queries.
        pub ecs_support: pulumi_wasm_rust::Output<Option<bool>>,
        /// Client IP address.
        pub ip: pulumi_wasm_rust::Output<String>,
        /// IP to direct all IPv4 DNS queries to.
        pub ipv4_destination: pulumi_wasm_rust::Output<String>,
        /// Name of the teams location.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The networks CIDRs that comprise the location.
        pub networks: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ZeroTrustDnsLocationNetwork>>,
        >,
        pub policy_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ZeroTrustDnsLocationArgs,
    ) -> ZeroTrustDnsLocationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let client_default_binding = args.client_default.get_inner();
        let ecs_support_binding = args.ecs_support.get_inner();
        let name_binding = args.name.get_inner();
        let networks_binding = args.networks.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustDnsLocation:ZeroTrustDnsLocation".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "clientDefault".into(),
                    value: &client_default_binding,
                },
                register_interface::ObjectField {
                    name: "ecsSupport".into(),
                    value: &ecs_support_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networks".into(),
                    value: &networks_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "anonymizedLogsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "clientDefault".into(),
                },
                register_interface::ResultField {
                    name: "dohSubdomain".into(),
                },
                register_interface::ResultField {
                    name: "ecsSupport".into(),
                },
                register_interface::ResultField {
                    name: "ip".into(),
                },
                register_interface::ResultField {
                    name: "ipv4Destination".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networks".into(),
                },
                register_interface::ResultField {
                    name: "policyIds".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ZeroTrustDnsLocationResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            anonymized_logs_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("anonymizedLogsEnabled").unwrap(),
            ),
            client_default: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientDefault").unwrap(),
            ),
            doh_subdomain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dohSubdomain").unwrap(),
            ),
            ecs_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ecsSupport").unwrap(),
            ),
            ip: pulumi_wasm_rust::__private::into_domain(hashmap.remove("ip").unwrap()),
            ipv4_destination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv4Destination").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            networks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networks").unwrap(),
            ),
            policy_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyIds").unwrap(),
            ),
        }
    }
}
