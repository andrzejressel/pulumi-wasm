/// Provides a Cloudflare rate limit resource for a given zone. This can
/// be used to limit the traffic you receive zone-wide, or matching more
/// specific types of requests/responses.
///
/// > `cloudflare.RateLimit` is in a deprecation phase until January 15th, 2025.
///   During this time period, this resource is still
///   fully supported but you are strongly advised to move to the
///   `cloudflare.Ruleset` resource. Full details can be found in the
///   developer documentation.
///
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: cloudflare:RateLimit
///     properties:
///       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
///       threshold: 2000
///       period: 2
///       match:
///         request:
///           urlPattern: ${cloudflareZone}/*
///           schemes:
///             - HTTP
///             - HTTPS
///           methods:
///             - GET
///             - POST
///             - PUT
///             - DELETE
///             - PATCH
///             - HEAD
///         response:
///           statuses:
///             - 200
///             - 201
///             - 202
///             - 301
///             - 429
///           originTraffic: false
///           headers:
///             - name: Host
///               op: eq
///               value: localhost
///             - name: X-Example
///               op: ne
///               value: my-example
///       action:
///         mode: simulate
///         timeout: 43200
///         response:
///           contentType: text/plain
///           body: custom response body
///       correlate:
///         by: nat
///       disabled: false
///       description: example rate limit for a zone
///       bypassUrlPatterns:
///         - example.com/bypass1
///         - example.com/bypass2
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/rateLimit:RateLimit example <zone_id>/<rate_limit_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod rate_limit {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RateLimitArgs {
        /// The action to be performed when the threshold of matched traffic within the period defined is exceeded.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<super::types::RateLimitAction>,
        #[builder(into, default)]
        pub bypass_url_patterns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Determines how rate limiting is applied. By default if not specified, rate limiting applies to the clients IP address.
        #[builder(into, default)]
        pub correlate: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::RateLimitCorrelate>,
        >,
        /// A note that you can use to describe the reason for a rate limit. This value is sanitized and all tags are removed.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether this ratelimit is currently disabled. Defaults to `false`.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Determines which traffic the rate limit counts towards the threshold. By default matches all traffic in the zone.
        #[builder(into, default)]
        pub match_: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::RateLimitMatch>,
        >,
        /// The time in seconds to count matching traffic. If the count exceeds threshold within this period the action will be performed.
        #[builder(into)]
        pub period: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The threshold that triggers the rate limit mitigations, combine with period.
        #[builder(into)]
        pub threshold: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RateLimitResult {
        /// The action to be performed when the threshold of matched traffic within the period defined is exceeded.
        pub action: pulumi_gestalt_rust::Output<super::types::RateLimitAction>,
        pub bypass_url_patterns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Determines how rate limiting is applied. By default if not specified, rate limiting applies to the clients IP address.
        pub correlate: pulumi_gestalt_rust::Output<
            Option<super::types::RateLimitCorrelate>,
        >,
        /// A note that you can use to describe the reason for a rate limit. This value is sanitized and all tags are removed.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether this ratelimit is currently disabled. Defaults to `false`.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Determines which traffic the rate limit counts towards the threshold. By default matches all traffic in the zone.
        pub match_: pulumi_gestalt_rust::Output<super::types::RateLimitMatch>,
        /// The time in seconds to count matching traffic. If the count exceeds threshold within this period the action will be performed.
        pub period: pulumi_gestalt_rust::Output<i32>,
        /// The threshold that triggers the rate limit mitigations, combine with period.
        pub threshold: pulumi_gestalt_rust::Output<i32>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RateLimitArgs,
    ) -> RateLimitResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let action_binding_1 = args.action.get_output(context);
        let action_binding = action_binding_1.get_inner();
        let bypass_url_patterns_binding_1 = args.bypass_url_patterns.get_output(context);
        let bypass_url_patterns_binding = bypass_url_patterns_binding_1.get_inner();
        let correlate_binding_1 = args.correlate.get_output(context);
        let correlate_binding = correlate_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let disabled_binding_1 = args.disabled.get_output(context);
        let disabled_binding = disabled_binding_1.get_inner();
        let match__binding_1 = args.match_.get_output(context);
        let match__binding = match__binding_1.get_inner();
        let period_binding_1 = args.period.get_output(context);
        let period_binding = period_binding_1.get_inner();
        let threshold_binding_1 = args.threshold.get_output(context);
        let threshold_binding = threshold_binding_1.get_inner();
        let zone_id_binding_1 = args.zone_id.get_output(context);
        let zone_id_binding = zone_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/rateLimit:RateLimit".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "bypassUrlPatterns".into(),
                    value: &bypass_url_patterns_binding,
                },
                register_interface::ObjectField {
                    name: "correlate".into(),
                    value: &correlate_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
                },
                register_interface::ObjectField {
                    name: "match".into(),
                    value: &match__binding,
                },
                register_interface::ObjectField {
                    name: "period".into(),
                    value: &period_binding,
                },
                register_interface::ObjectField {
                    name: "threshold".into(),
                    value: &threshold_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RateLimitResult {
            action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("action"),
            ),
            bypass_url_patterns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bypassUrlPatterns"),
            ),
            correlate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("correlate"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            disabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disabled"),
            ),
            match_: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("match"),
            ),
            period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("period"),
            ),
            threshold: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("threshold"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
