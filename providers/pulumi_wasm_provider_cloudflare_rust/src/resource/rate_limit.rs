//! Provides a Cloudflare rate limit resource for a given zone. This can
//! be used to limit the traffic you receive zone-wide, or matching more
//! specific types of requests/responses.
//! 
//! > `cloudflare.RateLimit` is in a deprecation phase until January 15th, 2025.
//!   During this time period, this resource is still
//!   fully supported but you are strongly advised to move to the
//!   `cloudflare.Ruleset` resource. Full details can be found in the
//!   developer documentation.
//! 
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:RateLimit
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       threshold: 2000
//!       period: 2
//!       match:
//!         request:
//!           urlPattern: ${cloudflareZone}/*
//!           schemes:
//!             - HTTP
//!             - HTTPS
//!           methods:
//!             - GET
//!             - POST
//!             - PUT
//!             - DELETE
//!             - PATCH
//!             - HEAD
//!         response:
//!           statuses:
//!             - 200
//!             - 201
//!             - 202
//!             - 301
//!             - 429
//!           originTraffic: false
//!           headers:
//!             - name: Host
//!               op: eq
//!               value: localhost
//!             - name: X-Example
//!               op: ne
//!               value: my-example
//!       action:
//!         mode: simulate
//!         timeout: 43200
//!         response:
//!           contentType: text/plain
//!           body: custom response body
//!       correlate:
//!         by: nat
//!       disabled: false
//!       description: example rate limit for a zone
//!       bypassUrlPatterns:
//!         - example.com/bypass1
//!         - example.com/bypass2
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/rateLimit:RateLimit example <zone_id>/<rate_limit_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct RateLimitArgs {
    /// The action to be performed when the threshold of matched traffic within the period defined is exceeded.
    #[builder(into)]
    pub action: pulumi_wasm_rust::Output<crate::types::RateLimitAction>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub bypass_url_patterns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Determines how rate limiting is applied. By default if not specified, rate limiting applies to the clients IP address.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub correlate: pulumi_wasm_rust::Output<Option<crate::types::RateLimitCorrelate>>,
    /// A note that you can use to describe the reason for a rate limit. This value is sanitized and all tags are removed.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether this ratelimit is currently disabled. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Determines which traffic the rate limit counts towards the threshold. By default matches all traffic in the zone.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub match_: pulumi_wasm_rust::Output<Option<crate::types::RateLimitMatch>>,
    /// The time in seconds to count matching traffic. If the count exceeds threshold within this period the action will be performed.
    #[builder(into)]
    pub period: pulumi_wasm_rust::Output<i32>,
    /// The threshold that triggers the rate limit mitigations, combine with period.
    #[builder(into)]
    pub threshold: pulumi_wasm_rust::Output<i32>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct RateLimitResult {
    /// The action to be performed when the threshold of matched traffic within the period defined is exceeded.
    pub action: pulumi_wasm_rust::Output<crate::types::RateLimitAction>,
    pub bypass_url_patterns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Determines how rate limiting is applied. By default if not specified, rate limiting applies to the clients IP address.
    pub correlate: pulumi_wasm_rust::Output<Option<crate::types::RateLimitCorrelate>>,
    /// A note that you can use to describe the reason for a rate limit. This value is sanitized and all tags are removed.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether this ratelimit is currently disabled. Defaults to `false`.
    pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Determines which traffic the rate limit counts towards the threshold. By default matches all traffic in the zone.
    pub match_: pulumi_wasm_rust::Output<crate::types::RateLimitMatch>,
    /// The time in seconds to count matching traffic. If the count exceeds threshold within this period the action will be performed.
    pub period: pulumi_wasm_rust::Output<i32>,
    /// The threshold that triggers the rate limit mitigations, combine with period.
    pub threshold: pulumi_wasm_rust::Output<i32>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RateLimitArgs) -> RateLimitResult {

    let result = crate::bindings::pulumi::cloudflare::rate_limit::invoke(name, &crate::bindings::pulumi::cloudflare::rate_limit::Args {
        action: &args.action.get_inner(),
        bypass_url_patterns: &args.bypass_url_patterns.get_inner(),
        correlate: &args.correlate.get_inner(),
        description: &args.description.get_inner(),
        disabled: &args.disabled.get_inner(),
        match_: &args.match_.get_inner(),
        period: &args.period.get_inner(),
        threshold: &args.threshold.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    RateLimitResult {
        action: crate::into_domain(result.action),
        bypass_url_patterns: crate::into_domain(result.bypass_url_patterns),
        correlate: crate::into_domain(result.correlate),
        description: crate::into_domain(result.description),
        disabled: crate::into_domain(result.disabled),
        match_: crate::into_domain(result.match_),
        period: crate::into_domain(result.period),
        threshold: crate::into_domain(result.threshold),
        zone_id: crate::into_domain(result.zone_id),
    }
}
