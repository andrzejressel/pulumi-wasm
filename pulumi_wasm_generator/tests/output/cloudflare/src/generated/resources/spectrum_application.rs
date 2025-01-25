/// Provides a Cloudflare Spectrum Application. You can extend the power
/// of Cloudflare's DDoS, TLS, and IP Firewall to your other TCP-based
/// services.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: cloudflare:SpectrumApplication
///     properties:
///       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
///       protocol: tcp/22
///       trafficType: direct
///       dns:
///         type: CNAME
///         name: ssh.example.com
///       originDirects:
///         - tcp://192.0.2.1:22
///       edgeIps:
///         type: static
///         ips:
///           - 203.0.113.1
///           - 203.0.113.2
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/spectrumApplication:SpectrumApplication example <zone_id>/<spectrum_application_id>
/// ```
///
pub mod spectrum_application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpectrumApplicationArgs {
        /// Enables Argo Smart Routing.
        #[builder(into, default)]
        pub argo_smart_routing: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name and type of DNS record for the Spectrum application.
        #[builder(into)]
        pub dns: pulumi_wasm_rust::InputOrOutput<super::types::SpectrumApplicationDns>,
        /// The anycast edge IP configuration for the hostname of this application.
        #[builder(into, default)]
        pub edge_ips: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::SpectrumApplicationEdgeIps>,
        >,
        /// Enables the IP Firewall for this application.
        #[builder(into, default)]
        pub ip_firewall: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A list of destination addresses to the origin. e.g. `tcp://192.0.2.1:22`.
        #[builder(into, default)]
        pub origin_directs: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A destination DNS addresses to the origin.
        #[builder(into, default)]
        pub origin_dns: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::SpectrumApplicationOriginDns>,
        >,
        /// Origin port to proxy traffice to. Conflicts with `origin_port_range`.
        #[builder(into, default)]
        pub origin_port: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Origin port range to proxy traffice to. When using a range, the protocol field must also specify a range, e.g. `tcp/22-23`. Conflicts with `origin_port`.
        #[builder(into, default)]
        pub origin_port_range: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::SpectrumApplicationOriginPortRange>,
        >,
        /// The port configuration at Cloudflare's edge. e.g. `tcp/22`.
        #[builder(into)]
        pub protocol: pulumi_wasm_rust::InputOrOutput<String>,
        /// Enables a proxy protocol to the origin. Available values: `off`, `v1`, `v2`, `simple`.
        #[builder(into, default)]
        pub proxy_protocol: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// TLS configuration option for Cloudflare to connect to your origin. Available values: `off`, `flexible`, `full`, `strict`.
        #[builder(into, default)]
        pub tls: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Sets application type. Available values: `direct`, `http`, `https`.
        #[builder(into, default)]
        pub traffic_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpectrumApplicationResult {
        /// Enables Argo Smart Routing.
        pub argo_smart_routing: pulumi_wasm_rust::Output<bool>,
        /// The name and type of DNS record for the Spectrum application.
        pub dns: pulumi_wasm_rust::Output<super::types::SpectrumApplicationDns>,
        /// The anycast edge IP configuration for the hostname of this application.
        pub edge_ips: pulumi_wasm_rust::Output<super::types::SpectrumApplicationEdgeIps>,
        /// Enables the IP Firewall for this application.
        pub ip_firewall: pulumi_wasm_rust::Output<bool>,
        /// A list of destination addresses to the origin. e.g. `tcp://192.0.2.1:22`.
        pub origin_directs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A destination DNS addresses to the origin.
        pub origin_dns: pulumi_wasm_rust::Output<
            Option<super::types::SpectrumApplicationOriginDns>,
        >,
        /// Origin port to proxy traffice to. Conflicts with `origin_port_range`.
        pub origin_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// Origin port range to proxy traffice to. When using a range, the protocol field must also specify a range, e.g. `tcp/22-23`. Conflicts with `origin_port`.
        pub origin_port_range: pulumi_wasm_rust::Output<
            Option<super::types::SpectrumApplicationOriginPortRange>,
        >,
        /// The port configuration at Cloudflare's edge. e.g. `tcp/22`.
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// Enables a proxy protocol to the origin. Available values: `off`, `v1`, `v2`, `simple`.
        pub proxy_protocol: pulumi_wasm_rust::Output<String>,
        /// TLS configuration option for Cloudflare to connect to your origin. Available values: `off`, `flexible`, `full`, `strict`.
        pub tls: pulumi_wasm_rust::Output<String>,
        /// Sets application type. Available values: `direct`, `http`, `https`.
        pub traffic_type: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SpectrumApplicationArgs,
    ) -> SpectrumApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let argo_smart_routing_binding = args
            .argo_smart_routing
            .get_output(context)
            .get_inner();
        let dns_binding = args.dns.get_output(context).get_inner();
        let edge_ips_binding = args.edge_ips.get_output(context).get_inner();
        let ip_firewall_binding = args.ip_firewall.get_output(context).get_inner();
        let origin_directs_binding = args.origin_directs.get_output(context).get_inner();
        let origin_dns_binding = args.origin_dns.get_output(context).get_inner();
        let origin_port_binding = args.origin_port.get_output(context).get_inner();
        let origin_port_range_binding = args
            .origin_port_range
            .get_output(context)
            .get_inner();
        let protocol_binding = args.protocol.get_output(context).get_inner();
        let proxy_protocol_binding = args.proxy_protocol.get_output(context).get_inner();
        let tls_binding = args.tls.get_output(context).get_inner();
        let traffic_type_binding = args.traffic_type.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/spectrumApplication:SpectrumApplication".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "argoSmartRouting".into(),
                    value: &argo_smart_routing_binding,
                },
                register_interface::ObjectField {
                    name: "dns".into(),
                    value: &dns_binding,
                },
                register_interface::ObjectField {
                    name: "edgeIps".into(),
                    value: &edge_ips_binding,
                },
                register_interface::ObjectField {
                    name: "ipFirewall".into(),
                    value: &ip_firewall_binding,
                },
                register_interface::ObjectField {
                    name: "originDirects".into(),
                    value: &origin_directs_binding,
                },
                register_interface::ObjectField {
                    name: "originDns".into(),
                    value: &origin_dns_binding,
                },
                register_interface::ObjectField {
                    name: "originPort".into(),
                    value: &origin_port_binding,
                },
                register_interface::ObjectField {
                    name: "originPortRange".into(),
                    value: &origin_port_range_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "proxyProtocol".into(),
                    value: &proxy_protocol_binding,
                },
                register_interface::ObjectField {
                    name: "tls".into(),
                    value: &tls_binding,
                },
                register_interface::ObjectField {
                    name: "trafficType".into(),
                    value: &traffic_type_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "argoSmartRouting".into(),
                },
                register_interface::ResultField {
                    name: "dns".into(),
                },
                register_interface::ResultField {
                    name: "edgeIps".into(),
                },
                register_interface::ResultField {
                    name: "ipFirewall".into(),
                },
                register_interface::ResultField {
                    name: "originDirects".into(),
                },
                register_interface::ResultField {
                    name: "originDns".into(),
                },
                register_interface::ResultField {
                    name: "originPort".into(),
                },
                register_interface::ResultField {
                    name: "originPortRange".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "proxyProtocol".into(),
                },
                register_interface::ResultField {
                    name: "tls".into(),
                },
                register_interface::ResultField {
                    name: "trafficType".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpectrumApplicationResult {
            argo_smart_routing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("argoSmartRouting").unwrap(),
            ),
            dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dns").unwrap(),
            ),
            edge_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeIps").unwrap(),
            ),
            ip_firewall: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipFirewall").unwrap(),
            ),
            origin_directs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originDirects").unwrap(),
            ),
            origin_dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originDns").unwrap(),
            ),
            origin_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originPort").unwrap(),
            ),
            origin_port_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originPortRange").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            proxy_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyProtocol").unwrap(),
            ),
            tls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tls").unwrap(),
            ),
            traffic_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficType").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
