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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spectrum_application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpectrumApplicationArgs {
        /// Enables Argo Smart Routing.
        #[builder(into, default)]
        pub argo_smart_routing: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name and type of DNS record for the Spectrum application.
        #[builder(into)]
        pub dns: pulumi_gestalt_rust::InputOrOutput<
            super::types::SpectrumApplicationDns,
        >,
        /// The anycast edge IP configuration for the hostname of this application.
        #[builder(into, default)]
        pub edge_ips: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::SpectrumApplicationEdgeIps>,
        >,
        /// Enables the IP Firewall for this application.
        #[builder(into, default)]
        pub ip_firewall: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of destination addresses to the origin. e.g. `tcp://192.0.2.1:22`.
        #[builder(into, default)]
        pub origin_directs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A destination DNS addresses to the origin.
        #[builder(into, default)]
        pub origin_dns: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::SpectrumApplicationOriginDns>,
        >,
        /// Origin port to proxy traffice to. Conflicts with `origin_port_range`.
        #[builder(into, default)]
        pub origin_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Origin port range to proxy traffice to. When using a range, the protocol field must also specify a range, e.g. `tcp/22-23`. Conflicts with `origin_port`.
        #[builder(into, default)]
        pub origin_port_range: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::SpectrumApplicationOriginPortRange>,
        >,
        /// The port configuration at Cloudflare's edge. e.g. `tcp/22`.
        #[builder(into)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Enables a proxy protocol to the origin. Available values: `off`, `v1`, `v2`, `simple`.
        #[builder(into, default)]
        pub proxy_protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// TLS configuration option for Cloudflare to connect to your origin. Available values: `off`, `flexible`, `full`, `strict`.
        #[builder(into, default)]
        pub tls: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Sets application type. Available values: `direct`, `http`, `https`.
        #[builder(into, default)]
        pub traffic_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpectrumApplicationResult {
        /// Enables Argo Smart Routing.
        pub argo_smart_routing: pulumi_gestalt_rust::Output<bool>,
        /// The name and type of DNS record for the Spectrum application.
        pub dns: pulumi_gestalt_rust::Output<super::types::SpectrumApplicationDns>,
        /// The anycast edge IP configuration for the hostname of this application.
        pub edge_ips: pulumi_gestalt_rust::Output<
            super::types::SpectrumApplicationEdgeIps,
        >,
        /// Enables the IP Firewall for this application.
        pub ip_firewall: pulumi_gestalt_rust::Output<bool>,
        /// A list of destination addresses to the origin. e.g. `tcp://192.0.2.1:22`.
        pub origin_directs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A destination DNS addresses to the origin.
        pub origin_dns: pulumi_gestalt_rust::Output<
            Option<super::types::SpectrumApplicationOriginDns>,
        >,
        /// Origin port to proxy traffice to. Conflicts with `origin_port_range`.
        pub origin_port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Origin port range to proxy traffice to. When using a range, the protocol field must also specify a range, e.g. `tcp/22-23`. Conflicts with `origin_port`.
        pub origin_port_range: pulumi_gestalt_rust::Output<
            Option<super::types::SpectrumApplicationOriginPortRange>,
        >,
        /// The port configuration at Cloudflare's edge. e.g. `tcp/22`.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// Enables a proxy protocol to the origin. Available values: `off`, `v1`, `v2`, `simple`.
        pub proxy_protocol: pulumi_gestalt_rust::Output<String>,
        /// TLS configuration option for Cloudflare to connect to your origin. Available values: `off`, `flexible`, `full`, `strict`.
        pub tls: pulumi_gestalt_rust::Output<String>,
        /// Sets application type. Available values: `direct`, `http`, `https`.
        pub traffic_type: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpectrumApplicationArgs,
    ) -> SpectrumApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let argo_smart_routing_binding = args.argo_smart_routing.get_output(context);
        let dns_binding = args.dns.get_output(context);
        let edge_ips_binding = args.edge_ips.get_output(context);
        let ip_firewall_binding = args.ip_firewall.get_output(context);
        let origin_directs_binding = args.origin_directs.get_output(context);
        let origin_dns_binding = args.origin_dns.get_output(context);
        let origin_port_binding = args.origin_port.get_output(context);
        let origin_port_range_binding = args.origin_port_range.get_output(context);
        let protocol_binding = args.protocol.get_output(context);
        let proxy_protocol_binding = args.proxy_protocol.get_output(context);
        let tls_binding = args.tls.get_output(context);
        let traffic_type_binding = args.traffic_type.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/spectrumApplication:SpectrumApplication".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "argoSmartRouting".into(),
                    value: argo_smart_routing_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dns".into(),
                    value: dns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "edgeIps".into(),
                    value: edge_ips_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipFirewall".into(),
                    value: ip_firewall_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "originDirects".into(),
                    value: origin_directs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "originDns".into(),
                    value: origin_dns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "originPort".into(),
                    value: origin_port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "originPortRange".into(),
                    value: origin_port_range_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocol".into(),
                    value: protocol_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "proxyProtocol".into(),
                    value: proxy_protocol_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tls".into(),
                    value: tls_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficType".into(),
                    value: traffic_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpectrumApplicationResult {
            argo_smart_routing: o.get_field("argoSmartRouting"),
            dns: o.get_field("dns"),
            edge_ips: o.get_field("edgeIps"),
            ip_firewall: o.get_field("ipFirewall"),
            origin_directs: o.get_field("originDirects"),
            origin_dns: o.get_field("originDns"),
            origin_port: o.get_field("originPort"),
            origin_port_range: o.get_field("originPortRange"),
            protocol: o.get_field("protocol"),
            proxy_protocol: o.get_field("proxyProtocol"),
            tls: o.get_field("tls"),
            traffic_type: o.get_field("trafficType"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
