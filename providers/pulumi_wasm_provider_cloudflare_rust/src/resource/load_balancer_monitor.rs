//! If Cloudflare's Load Balancing to load-balance across multiple
//! origin servers or data centers, you configure one of these Monitors
//! to actively check the availability of those servers over HTTP(S) or
//! TCP.
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/loadBalancerMonitor:LoadBalancerMonitor example <account_id>/<load_balancer_monitor_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct LoadBalancerMonitorArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Do not validate the certificate when monitor use HTTPS.  Only valid if `type` is "http" or "https".
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub allow_insecure: pulumi_wasm_rust::Output<Option<bool>>,
    /// To be marked unhealthy the monitored origin must fail this healthcheck N consecutive times. Defaults to `0`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub consecutive_down: pulumi_wasm_rust::Output<Option<i32>>,
    /// To be marked healthy the monitored origin must pass this healthcheck N consecutive times. Defaults to `0`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub consecutive_up: pulumi_wasm_rust::Output<Option<i32>>,
    /// Free text description.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// A case-insensitive sub-string to look for in the response body. If this string is not found, the origin will be marked as unhealthy. Only valid if `type` is "http" or "https".
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub expected_body: pulumi_wasm_rust::Output<Option<String>>,
    /// The expected HTTP response code or code range of the health check. Eg `2xx`. Only valid and required if `type` is "http" or "https".
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub expected_codes: pulumi_wasm_rust::Output<Option<String>>,
    /// Follow redirects if returned by the origin. Only valid if `type` is "http" or "https".
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub follow_redirects: pulumi_wasm_rust::Output<Option<bool>>,
    /// The HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerMonitorHeader>>>,
    /// The interval between each health check. Shorter intervals may improve failover time, but will increase load on the origins as we check from multiple locations. Defaults to `60`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub interval: pulumi_wasm_rust::Output<Option<i32>>,
    /// The method to use for the health check.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub method: pulumi_wasm_rust::Output<Option<String>>,
    /// The endpoint path to health check against.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub path: pulumi_wasm_rust::Output<Option<String>>,
    /// The port number to use for the healthcheck, required when creating a TCP monitor.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub port: pulumi_wasm_rust::Output<Option<i32>>,
    /// Assign this monitor to emulate the specified zone while probing. Only valid if `type` is "http" or "https".
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub probe_zone: pulumi_wasm_rust::Output<Option<String>>,
    /// The number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub retries: pulumi_wasm_rust::Output<Option<i32>>,
    /// The timeout (in seconds) before marking the health check as failed. Defaults to `5`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
    /// The protocol to use for the healthcheck. Available values: `http`, `https`, `tcp`, `udp_icmp`, `icmp_ping`, `smtp`. Defaults to `http`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct LoadBalancerMonitorResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Do not validate the certificate when monitor use HTTPS.  Only valid if `type` is "http" or "https".
    pub allow_insecure: pulumi_wasm_rust::Output<Option<bool>>,
    /// To be marked unhealthy the monitored origin must fail this healthcheck N consecutive times. Defaults to `0`.
    pub consecutive_down: pulumi_wasm_rust::Output<Option<i32>>,
    /// To be marked healthy the monitored origin must pass this healthcheck N consecutive times. Defaults to `0`.
    pub consecutive_up: pulumi_wasm_rust::Output<Option<i32>>,
    /// The RFC3339 timestamp of when the load balancer monitor was created.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// Free text description.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// A case-insensitive sub-string to look for in the response body. If this string is not found, the origin will be marked as unhealthy. Only valid if `type` is "http" or "https".
    pub expected_body: pulumi_wasm_rust::Output<Option<String>>,
    /// The expected HTTP response code or code range of the health check. Eg `2xx`. Only valid and required if `type` is "http" or "https".
    pub expected_codes: pulumi_wasm_rust::Output<Option<String>>,
    /// Follow redirects if returned by the origin. Only valid if `type` is "http" or "https".
    pub follow_redirects: pulumi_wasm_rust::Output<Option<bool>>,
    /// The HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden.
    pub headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerMonitorHeader>>>,
    /// The interval between each health check. Shorter intervals may improve failover time, but will increase load on the origins as we check from multiple locations. Defaults to `60`.
    pub interval: pulumi_wasm_rust::Output<Option<i32>>,
    /// The method to use for the health check.
    pub method: pulumi_wasm_rust::Output<String>,
    /// The RFC3339 timestamp of when the load balancer monitor was last modified.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// The endpoint path to health check against.
    pub path: pulumi_wasm_rust::Output<String>,
    /// The port number to use for the healthcheck, required when creating a TCP monitor.
    pub port: pulumi_wasm_rust::Output<Option<i32>>,
    /// Assign this monitor to emulate the specified zone while probing. Only valid if `type` is "http" or "https".
    pub probe_zone: pulumi_wasm_rust::Output<Option<String>>,
    /// The number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`.
    pub retries: pulumi_wasm_rust::Output<Option<i32>>,
    /// The timeout (in seconds) before marking the health check as failed. Defaults to `5`.
    pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
    /// The protocol to use for the healthcheck. Available values: `http`, `https`, `tcp`, `udp_icmp`, `icmp_ping`, `smtp`. Defaults to `http`.
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: LoadBalancerMonitorArgs) -> LoadBalancerMonitorResult {

    let result = crate::bindings::pulumi::cloudflare::load_balancer_monitor::invoke(name, &crate::bindings::pulumi::cloudflare::load_balancer_monitor::Args {
        account_id: &args.account_id.get_inner(),
        allow_insecure: &args.allow_insecure.get_inner(),
        consecutive_down: &args.consecutive_down.get_inner(),
        consecutive_up: &args.consecutive_up.get_inner(),
        description: &args.description.get_inner(),
        expected_body: &args.expected_body.get_inner(),
        expected_codes: &args.expected_codes.get_inner(),
        follow_redirects: &args.follow_redirects.get_inner(),
        headers: &args.headers.get_inner(),
        interval: &args.interval.get_inner(),
        method: &args.method.get_inner(),
        path: &args.path.get_inner(),
        port: &args.port.get_inner(),
        probe_zone: &args.probe_zone.get_inner(),
        retries: &args.retries.get_inner(),
        timeout: &args.timeout.get_inner(),
        type_: &args.type_.get_inner(),
    });

    LoadBalancerMonitorResult {
        account_id: crate::into_domain(result.account_id),
        allow_insecure: crate::into_domain(result.allow_insecure),
        consecutive_down: crate::into_domain(result.consecutive_down),
        consecutive_up: crate::into_domain(result.consecutive_up),
        created_on: crate::into_domain(result.created_on),
        description: crate::into_domain(result.description),
        expected_body: crate::into_domain(result.expected_body),
        expected_codes: crate::into_domain(result.expected_codes),
        follow_redirects: crate::into_domain(result.follow_redirects),
        headers: crate::into_domain(result.headers),
        interval: crate::into_domain(result.interval),
        method: crate::into_domain(result.method),
        modified_on: crate::into_domain(result.modified_on),
        path: crate::into_domain(result.path),
        port: crate::into_domain(result.port),
        probe_zone: crate::into_domain(result.probe_zone),
        retries: crate::into_domain(result.retries),
        timeout: crate::into_domain(result.timeout),
        type_: crate::into_domain(result.type_),
    }
}
