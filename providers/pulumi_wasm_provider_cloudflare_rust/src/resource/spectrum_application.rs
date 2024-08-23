pub struct SpectrumApplicationArgs {
    pub argo_smart_routing: pulumi_wasm_rust::Output<Option<bool>>,
    pub dns: pulumi_wasm_rust::Output<crate::types::SpectrumApplicationDns>,
    pub edge_ips: pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationEdgeIps>>,
    pub ip_firewall: pulumi_wasm_rust::Output<Option<bool>>,
    pub origin_directs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub origin_dns: pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginDns>>,
    pub origin_port: pulumi_wasm_rust::Output<Option<i32>>,
    pub origin_port_range:
        pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginPortRange>>,
    pub protocol: pulumi_wasm_rust::Output<String>,
    pub proxy_protocol: pulumi_wasm_rust::Output<Option<String>>,
    pub tls: pulumi_wasm_rust::Output<Option<String>>,
    pub traffic_type: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct SpectrumApplicationResult {
    pub argo_smart_routing: pulumi_wasm_rust::Output<bool>,
    pub dns: pulumi_wasm_rust::Output<crate::types::SpectrumApplicationDns>,
    pub edge_ips: pulumi_wasm_rust::Output<crate::types::SpectrumApplicationEdgeIps>,
    pub ip_firewall: pulumi_wasm_rust::Output<bool>,
    pub origin_directs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub origin_dns: pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginDns>>,
    pub origin_port: pulumi_wasm_rust::Output<Option<i32>>,
    pub origin_port_range:
        pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginPortRange>>,
    pub protocol: pulumi_wasm_rust::Output<String>,
    pub proxy_protocol: pulumi_wasm_rust::Output<String>,
    pub tls: pulumi_wasm_rust::Output<String>,
    pub traffic_type: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: SpectrumApplicationArgs) -> SpectrumApplicationResult {
    let result = crate::bindings::pulumi::cloudflare::spectrum_application::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::spectrum_application::Args {
            argo_smart_routing: args.argo_smart_routing.get_inner(),
            dns: args.dns.get_inner(),
            edge_ips: args.edge_ips.get_inner(),
            ip_firewall: args.ip_firewall.get_inner(),
            origin_directs: args.origin_directs.get_inner(),
            origin_dns: args.origin_dns.get_inner(),
            origin_port: args.origin_port.get_inner(),
            origin_port_range: args.origin_port_range.get_inner(),
            protocol: args.protocol.get_inner(),
            proxy_protocol: args.proxy_protocol.get_inner(),
            tls: args.tls.get_inner(),
            traffic_type: args.traffic_type.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

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
