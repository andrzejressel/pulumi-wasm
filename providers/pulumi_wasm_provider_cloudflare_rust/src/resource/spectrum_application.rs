//! Provides a Cloudflare Spectrum Application. You can extend the power
//! of Cloudflare's DDoS, TLS, and IP Firewall to your other TCP-based
//! services.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:SpectrumApplication
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       protocol: tcp/22
//!       trafficType: direct
//!       dns:
//!         type: CNAME
//!         name: ssh.example.com
//!       originDirects:
//!         - tcp://192.0.2.1:22
//!       edgeIps:
//!         type: static
//!         ips:
//!           - 203.0.113.1
//!           - 203.0.113.2
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/spectrumApplication:SpectrumApplication example <zone_id>/<spectrum_application_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct SpectrumApplicationArgs {
    /// Enables Argo Smart Routing.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub argo_smart_routing: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name and type of DNS record for the Spectrum application.
    #[builder(into)]
    pub dns: pulumi_wasm_rust::Output<crate::types::SpectrumApplicationDns>,
    /// The anycast edge IP configuration for the hostname of this application.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub edge_ips: pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationEdgeIps>>,
    /// Enables the IP Firewall for this application.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ip_firewall: pulumi_wasm_rust::Output<Option<bool>>,
    /// A list of destination addresses to the origin. e.g. `tcp://192.0.2.1:22`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub origin_directs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// A destination DNS addresses to the origin.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub origin_dns: pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginDns>>,
    /// Origin port to proxy traffice to. Conflicts with `origin_port_range`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub origin_port: pulumi_wasm_rust::Output<Option<i32>>,
    /// Origin port range to proxy traffice to. When using a range, the protocol field must also specify a range, e.g. `tcp/22-23`. Conflicts with `origin_port`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub origin_port_range: pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginPortRange>>,
    /// The port configuration at Cloudflare's edge. e.g. `tcp/22`.
    #[builder(into)]
    pub protocol: pulumi_wasm_rust::Output<String>,
    /// Enables a proxy protocol to the origin. Available values: `off`, `v1`, `v2`, `simple`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub proxy_protocol: pulumi_wasm_rust::Output<Option<String>>,
    /// TLS configuration option for Cloudflare to connect to your origin. Available values: `off`, `flexible`, `full`, `strict`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub tls: pulumi_wasm_rust::Output<Option<String>>,
    /// Sets application type. Available values: `direct`, `http`, `https`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub traffic_type: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct SpectrumApplicationResult {
    /// Enables Argo Smart Routing.
    pub argo_smart_routing: pulumi_wasm_rust::Output<bool>,
    /// The name and type of DNS record for the Spectrum application.
    pub dns: pulumi_wasm_rust::Output<crate::types::SpectrumApplicationDns>,
    /// The anycast edge IP configuration for the hostname of this application.
    pub edge_ips: pulumi_wasm_rust::Output<crate::types::SpectrumApplicationEdgeIps>,
    /// Enables the IP Firewall for this application.
    pub ip_firewall: pulumi_wasm_rust::Output<bool>,
    /// A list of destination addresses to the origin. e.g. `tcp://192.0.2.1:22`.
    pub origin_directs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// A destination DNS addresses to the origin.
    pub origin_dns: pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginDns>>,
    /// Origin port to proxy traffice to. Conflicts with `origin_port_range`.
    pub origin_port: pulumi_wasm_rust::Output<Option<i32>>,
    /// Origin port range to proxy traffice to. When using a range, the protocol field must also specify a range, e.g. `tcp/22-23`. Conflicts with `origin_port`.
    pub origin_port_range: pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginPortRange>>,
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
pub fn create(name: &str, args: SpectrumApplicationArgs) -> SpectrumApplicationResult {

    let result = crate::bindings::pulumi::cloudflare::spectrum_application::invoke(name, &crate::bindings::pulumi::cloudflare::spectrum_application::Args {
        argo_smart_routing: &args.argo_smart_routing.get_inner(),
        dns: &args.dns.get_inner(),
        edge_ips: &args.edge_ips.get_inner(),
        ip_firewall: &args.ip_firewall.get_inner(),
        origin_directs: &args.origin_directs.get_inner(),
        origin_dns: &args.origin_dns.get_inner(),
        origin_port: &args.origin_port.get_inner(),
        origin_port_range: &args.origin_port_range.get_inner(),
        protocol: &args.protocol.get_inner(),
        proxy_protocol: &args.proxy_protocol.get_inner(),
        tls: &args.tls.get_inner(),
        traffic_type: &args.traffic_type.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    SpectrumApplicationResult {
        argo_smart_routing: crate::into_domain(result.argo_smart_routing),
        dns: crate::into_domain(result.dns),
        edge_ips: crate::into_domain(result.edge_ips),
        ip_firewall: crate::into_domain(result.ip_firewall),
        origin_directs: crate::into_domain(result.origin_directs),
        origin_dns: crate::into_domain(result.origin_dns),
        origin_port: crate::into_domain(result.origin_port),
        origin_port_range: crate::into_domain(result.origin_port_range),
        protocol: crate::into_domain(result.protocol),
        proxy_protocol: crate::into_domain(result.proxy_protocol),
        tls: crate::into_domain(result.tls),
        traffic_type: crate::into_domain(result.traffic_type),
        zone_id: crate::into_domain(result.zone_id),
    }
}
