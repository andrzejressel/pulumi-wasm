//! Standalone Health Checks provide a way to monitor origin servers
//! without needing a Cloudflare Load Balancer.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let httpHealthCheck = healthcheck::create(
//!         "httpHealthCheck",
//!         HealthcheckArgs::builder()
//!             .address("example.com")
//!             .allow_insecure(false)
//!             .check_regions(vec!["WEU", "EEU",])
//!             .consecutive_fails(3)
//!             .consecutive_successes(2)
//!             .description("example http health check")
//!             .expected_body("alive")
//!             .expected_codes(vec!["2xx", "301",])
//!             .follow_redirects(true)
//!             .headers(
//!                 vec![
//!                     HealthcheckHeader::builder().header("Host")
//!                     .values(vec!["example.com",]).build_struct(),
//!                 ],
//!             )
//!             .interval(60)
//!             .method("GET")
//!             .name("http-health-check")
//!             .path("/health")
//!             .port(443)
//!             .retries(2)
//!             .suspended(false)
//!             .timeout(10)
//!             .type_("HTTPS")
//!             .zone_id("${cloudflareZoneId}")
//!             .build_struct(),
//!     );
//!     let tcpHealthCheck = healthcheck::create(
//!         "tcpHealthCheck",
//!         HealthcheckArgs::builder()
//!             .address("example.com")
//!             .check_regions(vec!["WEU", "EEU",])
//!             .consecutive_fails(3)
//!             .consecutive_successes(2)
//!             .description("example tcp health check")
//!             .interval(60)
//!             .method("connection_established")
//!             .name("tcp-health-check")
//!             .port(22)
//!             .retries(2)
//!             .suspended(false)
//!             .timeout(10)
//!             .type_("TCP")
//!             .zone_id("${cloudflareZoneId}")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! Use the Zone ID and Healthcheck ID to import.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/healthcheck:Healthcheck example <zone_id>/<healthcheck_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct HealthcheckArgs {
    /// The hostname or IP address of the origin server to run health checks on.
    #[builder(into)]
    pub address: pulumi_wasm_rust::Output<String>,
    /// Do not validate the certificate when the health check uses HTTPS. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub allow_insecure: pulumi_wasm_rust::Output<Option<bool>>,
    /// A list of regions from which to run health checks. If not set, Cloudflare will pick a default region. Available values: `WNAM`, `ENAM`, `WEU`, `EEU`, `NSAM`, `SSAM`, `OC`, `ME`, `NAF`, `SAF`, `IN`, `SEAS`, `NEAS`, `ALL_REGIONS`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub check_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The number of consecutive fails required from a health check before changing the health to unhealthy. Defaults to `1`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub consecutive_fails: pulumi_wasm_rust::Output<Option<i32>>,
    /// The number of consecutive successes required from a health check before changing the health to healthy. Defaults to `1`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub consecutive_successes: pulumi_wasm_rust::Output<Option<i32>>,
    /// A human-readable description of the health check.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// A case-insensitive sub-string to look for in the response body. If this string is not found the origin will be marked as unhealthy.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub expected_body: pulumi_wasm_rust::Output<Option<String>>,
    /// The expected HTTP response codes (e.g. '200') or code ranges (e.g. '2xx' for all codes starting with 2) of the health check.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub expected_codes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Follow redirects if the origin returns a 3xx status code. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub follow_redirects: pulumi_wasm_rust::Output<Option<bool>>,
    /// The HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::HealthcheckHeader>>>,
    /// The interval between each health check. Shorter intervals may give quicker notifications if the origin status changes, but will increase the load on the origin as we check from multiple locations. Defaults to `60`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub interval: pulumi_wasm_rust::Output<Option<i32>>,
    /// The HTTP method to use for the health check. Available values: `connection_established`, `GET`, `HEAD`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub method: pulumi_wasm_rust::Output<Option<String>>,
    /// A short name to identify the health check. Only alphanumeric characters, hyphens, and underscores are allowed.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The endpoint path to health check against. Defaults to `/`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub path: pulumi_wasm_rust::Output<Option<String>>,
    /// Port number to connect to for the health check. Defaults to `80`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub port: pulumi_wasm_rust::Output<Option<i32>>,
    /// The number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub retries: pulumi_wasm_rust::Output<Option<i32>>,
    /// If suspended, no health checks are sent to the origin. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
    /// The timeout (in seconds) before marking the health check as failed. Defaults to `5`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
    /// The protocol to use for the health check. Available values: `TCP`, `HTTP`, `HTTPS`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct HealthcheckResult {
    /// The hostname or IP address of the origin server to run health checks on.
    pub address: pulumi_wasm_rust::Output<String>,
    /// Do not validate the certificate when the health check uses HTTPS. Defaults to `false`.
    pub allow_insecure: pulumi_wasm_rust::Output<Option<bool>>,
    /// A list of regions from which to run health checks. If not set, Cloudflare will pick a default region. Available values: `WNAM`, `ENAM`, `WEU`, `EEU`, `NSAM`, `SSAM`, `OC`, `ME`, `NAF`, `SAF`, `IN`, `SEAS`, `NEAS`, `ALL_REGIONS`.
    pub check_regions: pulumi_wasm_rust::Output<Vec<String>>,
    /// The number of consecutive fails required from a health check before changing the health to unhealthy. Defaults to `1`.
    pub consecutive_fails: pulumi_wasm_rust::Output<Option<i32>>,
    /// The number of consecutive successes required from a health check before changing the health to healthy. Defaults to `1`.
    pub consecutive_successes: pulumi_wasm_rust::Output<Option<i32>>,
    /// Creation time.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// A human-readable description of the health check.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// A case-insensitive sub-string to look for in the response body. If this string is not found the origin will be marked as unhealthy.
    pub expected_body: pulumi_wasm_rust::Output<Option<String>>,
    /// The expected HTTP response codes (e.g. '200') or code ranges (e.g. '2xx' for all codes starting with 2) of the health check.
    pub expected_codes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Follow redirects if the origin returns a 3xx status code. Defaults to `false`.
    pub follow_redirects: pulumi_wasm_rust::Output<Option<bool>>,
    /// The HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden.
    pub headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::HealthcheckHeader>>>,
    /// The interval between each health check. Shorter intervals may give quicker notifications if the origin status changes, but will increase the load on the origin as we check from multiple locations. Defaults to `60`.
    pub interval: pulumi_wasm_rust::Output<Option<i32>>,
    /// The HTTP method to use for the health check. Available values: `connection_established`, `GET`, `HEAD`.
    pub method: pulumi_wasm_rust::Output<String>,
    /// Last modified time.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// A short name to identify the health check. Only alphanumeric characters, hyphens, and underscores are allowed.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The endpoint path to health check against. Defaults to `/`.
    pub path: pulumi_wasm_rust::Output<Option<String>>,
    /// Port number to connect to for the health check. Defaults to `80`.
    pub port: pulumi_wasm_rust::Output<Option<i32>>,
    /// The number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`.
    pub retries: pulumi_wasm_rust::Output<Option<i32>>,
    /// If suspended, no health checks are sent to the origin. Defaults to `false`.
    pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
    /// The timeout (in seconds) before marking the health check as failed. Defaults to `5`.
    pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
    /// The protocol to use for the health check. Available values: `TCP`, `HTTP`, `HTTPS`.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: HealthcheckArgs) -> HealthcheckResult {

    let result = crate::bindings::pulumi::cloudflare::healthcheck::invoke(name, &crate::bindings::pulumi::cloudflare::healthcheck::Args {
        address: &args.address.get_inner(),
        allow_insecure: &args.allow_insecure.get_inner(),
        check_regions: &args.check_regions.get_inner(),
        consecutive_fails: &args.consecutive_fails.get_inner(),
        consecutive_successes: &args.consecutive_successes.get_inner(),
        description: &args.description.get_inner(),
        expected_body: &args.expected_body.get_inner(),
        expected_codes: &args.expected_codes.get_inner(),
        follow_redirects: &args.follow_redirects.get_inner(),
        headers: &args.headers.get_inner(),
        interval: &args.interval.get_inner(),
        method: &args.method.get_inner(),
        name: &args.name.get_inner(),
        path: &args.path.get_inner(),
        port: &args.port.get_inner(),
        retries: &args.retries.get_inner(),
        suspended: &args.suspended.get_inner(),
        timeout: &args.timeout.get_inner(),
        type_: &args.type_.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    HealthcheckResult {
        address: crate::into_domain(result.address),
        allow_insecure: crate::into_domain(result.allow_insecure),
        check_regions: crate::into_domain(result.check_regions),
        consecutive_fails: crate::into_domain(result.consecutive_fails),
        consecutive_successes: crate::into_domain(result.consecutive_successes),
        created_on: crate::into_domain(result.created_on),
        description: crate::into_domain(result.description),
        expected_body: crate::into_domain(result.expected_body),
        expected_codes: crate::into_domain(result.expected_codes),
        follow_redirects: crate::into_domain(result.follow_redirects),
        headers: crate::into_domain(result.headers),
        interval: crate::into_domain(result.interval),
        method: crate::into_domain(result.method),
        modified_on: crate::into_domain(result.modified_on),
        name: crate::into_domain(result.name),
        path: crate::into_domain(result.path),
        port: crate::into_domain(result.port),
        retries: crate::into_domain(result.retries),
        suspended: crate::into_domain(result.suspended),
        timeout: crate::into_domain(result.timeout),
        type_: crate::into_domain(result.type_),
        zone_id: crate::into_domain(result.zone_id),
    }
}
