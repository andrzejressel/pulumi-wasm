//! The [Cloudflare Ruleset Engine](https://developers.cloudflare.com/firewall/cf-rulesets)
//! allows you to create and deploy rules and rulesets.
//!
//! The engine syntax, inspired by the Wireshark Display Filter language, is the
//! same syntax used in custom Firewall Rules. Cloudflare uses the Ruleset Engine
//! in different products, allowing you to configure several products using the same
//! basic syntax.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Java
//! ```java
//! package generated_program;
//!
//! import com.pulumi.Context;
//! import com.pulumi.Pulumi;
//! import com.pulumi.core.Output;
//! import com.pulumi.cloudflare.Ruleset;
//! import com.pulumi.cloudflare.RulesetArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersOverridesArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersUriArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersUriPathArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersUriQueryArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleRatelimitArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersOriginArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersBrowserTtlArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersCacheKeyArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersCacheKeyCustomKeyArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersCacheKeyCustomKeyCookieArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersCacheKeyCustomKeyHeaderArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersCacheKeyCustomKeyHostArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersCacheKeyCustomKeyQueryStringArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersCacheKeyCustomKeyUserArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersEdgeTtlArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersServeStaleArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersFromListArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersFromValueArgs;
//! import com.pulumi.cloudflare.inputs.RulesetRuleActionParametersFromValueTargetUrlArgs;
//! import java.util.List;
//! import java.util.ArrayList;
//! import java.util.Map;
//! import java.io.File;
//! import java.nio.file.Files;
//! import java.nio.file.Paths;
//!
//! public class App {
//!     public static void main(String[] args) {
//!         Pulumi.run(App::stack);
//!     }
//!
//!     public static void stack(Context ctx) {
//!         // Magic Transit
//!         var magicTransitExample = new Ruleset("magicTransitExample", RulesetArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .description("example magic transit ruleset description")
//!             .kind("root")
//!             .name("account magic transit")
//!             .phase("magic_transit")
//!             .rules(RulesetRuleArgs.builder()
//!                 .action("allow")
//!                 .description("Allow TCP Ephemeral Ports")
//!                 .expression("tcp.dstport in { 32768..65535 }")
//!                 .build())
//!             .build());
//!
//!         // Zone-level WAF Managed Ruleset
//!         var zoneLevelManagedWaf = new Ruleset("zoneLevelManagedWaf", RulesetArgs.builder()        
//!             .description("managed WAF ruleset description")
//!             .kind("zone")
//!             .name("managed WAF")
//!             .phase("http_request_firewall_managed")
//!             .rules(RulesetRuleArgs.builder()
//!                 .action("execute")
//!                 .actionParameters(RulesetRuleActionParametersArgs.builder()
//!                     .id("efb7b8c949ac4650a09736fc376e9aee")
//!                     .build())
//!                 .description("Execute Cloudflare Managed Ruleset on my zone-level phase entry point ruleset")
//!                 .enabled(true)
//!                 .expression("(http.host eq \"example.host.com\")")
//!                 .build())
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!         // Zone-level WAF with tag-based overrides
//!         var zoneLevelManagedWafWithCategoryBasedOverrides = new Ruleset("zoneLevelManagedWafWithCategoryBasedOverrides", RulesetArgs.builder()        
//!             .description("managed WAF with tag-based overrides ruleset description")
//!             .kind("zone")
//!             .name("managed WAF with tag-based overrides")
//!             .phase("http_request_firewall_managed")
//!             .rules(RulesetRuleArgs.builder()
//!                 .action("execute")
//!                 .actionParameters(RulesetRuleActionParametersArgs.builder()
//!                     .id("efb7b8c949ac4650a09736fc376e9aee")
//!                     .overrides(RulesetRuleActionParametersOverridesArgs.builder()
//!                         .categories(                        
//!                             RulesetRuleActionParametersOverridesCategoryArgs.builder()
//!                                 .action("block")
//!                                 .category("wordpress")
//!                                 .enabled(true)
//!                                 .build(),
//!                             RulesetRuleActionParametersOverridesCategoryArgs.builder()
//!                                 .action("block")
//!                                 .category("joomla")
//!                                 .enabled(true)
//!                                 .build())
//!                         .build())
//!                     .build())
//!                 .description("overrides to only enable wordpress rules to block")
//!                 .enabled(false)
//!                 .expression("(http.host eq \"example.host.com\")")
//!                 .build())
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!         // Rewrite the URI path component to a static path
//!         var transformUriRulePath = new Ruleset("transformUriRulePath", RulesetArgs.builder()        
//!             .description("change the URI path to a new static path")
//!             .kind("zone")
//!             .name("transform rule for URI path")
//!             .phase("http_request_transform")
//!             .rules(RulesetRuleArgs.builder()
//!                 .action("rewrite")
//!                 .actionParameters(RulesetRuleActionParametersArgs.builder()
//!                     .uri(RulesetRuleActionParametersUriArgs.builder()
//!                         .path(RulesetRuleActionParametersUriPathArgs.builder()
//!                             .value("/my-new-route")
//!                             .build())
//!                         .build())
//!                     .build())
//!                 .description("example URI path transform rule")
//!                 .enabled(true)
//!                 .expression("(http.host eq \"example.com\" and http.request.uri.path eq \"/old-path\")")
//!                 .build())
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!         // Rewrite the URI query component to a static query
//!         var transformUriRuleQuery = new Ruleset("transformUriRuleQuery", RulesetArgs.builder()        
//!             .description("change the URI query to a new static query")
//!             .kind("zone")
//!             .name("transform rule for URI query parameter")
//!             .phase("http_request_transform")
//!             .rules(RulesetRuleArgs.builder()
//!                 .action("rewrite")
//!                 .actionParameters(RulesetRuleActionParametersArgs.builder()
//!                     .uri(RulesetRuleActionParametersUriArgs.builder()
//!                         .query(RulesetRuleActionParametersUriQueryArgs.builder()
//!                             .value("old=new_again")
//!                             .build())
//!                         .build())
//!                     .build())
//!                 .description("URI transformation query example")
//!                 .enabled(true)
//!                 .expression("(http.host eq \"example.host.com\")")
//!                 .build())
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!         // Rewrite HTTP headers to a modified values
//!         var transformUriHttpHeaders = new Ruleset("transformUriHttpHeaders", RulesetArgs.builder()        
//!             .description("modify HTTP headers before reaching origin")
//!             .kind("zone")
//!             .name("transform rule for HTTP headers")
//!             .phase("http_request_late_transform")
//!             .rules(RulesetRuleArgs.builder()
//!                 .action("rewrite")
//!                 .actionParameters(RulesetRuleActionParametersArgs.builder()
//!                     .headers(                    
//!                         RulesetRuleActionParametersHeaderArgs.builder()
//!                             .name("example-http-header-1")
//!                             .operation("set")
//!                             .value("my-http-header-value-1")
//!                             .build(),
//!                         RulesetRuleActionParametersHeaderArgs.builder()
//!                             .expression("cf.zone.name")
//!                             .name("example-http-header-2")
//!                             .operation("set")
//!                             .build(),
//!                         RulesetRuleActionParametersHeaderArgs.builder()
//!                             .name("example-http-header-3-to-remove")
//!                             .operation("remove")
//!                             .build())
//!                     .build())
//!                 .description("example request header transform rule")
//!                 .enabled(false)
//!                 .expression("(http.host eq \"example.host.com\")")
//!                 .build())
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!         // HTTP rate limit for an API route
//!         var rateLimitingExample = new Ruleset("rateLimitingExample", RulesetArgs.builder()        
//!             .description("apply HTTP rate limiting for a route")
//!             .kind("zone")
//!             .name("restrict API requests count")
//!             .phase("http_ratelimit")
//!             .rules(RulesetRuleArgs.builder()
//!                 .action("block")
//!                 .description("rate limit for API")
//!                 .enabled(true)
//!                 .expression("(http.request.uri.path matches \"^/api/\")")
//!                 .ratelimit(RulesetRuleRatelimitArgs.builder()
//!                     .characteristics(                    
//!                         "cf.colo.id",
//!                         "ip.src")
//!                     .mitigationTimeout(600)
//!                     .period(60)
//!                     .requestsPerPeriod(100)
//!                     .build())
//!                 .build())
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!         // Change origin for an API route
//!         var httpOriginExample = new Ruleset("httpOriginExample", RulesetArgs.builder()        
//!             .description("Change origin for a route")
//!             .kind("zone")
//!             .name("Change to some origin")
//!             .phase("http_request_origin")
//!             .rules(RulesetRuleArgs.builder()
//!                 .action("route")
//!                 .actionParameters(RulesetRuleActionParametersArgs.builder()
//!                     .hostHeader("some.host")
//!                     .origin(RulesetRuleActionParametersOriginArgs.builder()
//!                         .host("some.host")
//!                         .port(80)
//!                         .build())
//!                     .build())
//!                 .description("change origin to some.host")
//!                 .enabled(true)
//!                 .expression("(http.request.uri.path matches \"^/api/\")")
//!                 .build())
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!         // Custom fields logging
//!         var customFieldsLoggingExample = new Ruleset("customFieldsLoggingExample", RulesetArgs.builder()        
//!             .description("add custom fields to logging")
//!             .kind("zone")
//!             .name("log custom fields")
//!             .phase("http_log_custom_fields")
//!             .rules(RulesetRuleArgs.builder()
//!                 .action("log_custom_field")
//!                 .actionParameters(RulesetRuleActionParametersArgs.builder()
//!                     .cookieFields(                    
//!                         "__ga",
//!                         "accountNumber",
//!                         "__cfruid")
//!                     .requestFields(                    
//!                         "content-type",
//!                         "x-forwarded-for",
//!                         "host")
//!                     .responseFields(                    
//!                         "server",
//!                         "content-type",
//!                         "allow")
//!                     .build())
//!                 .description("log custom fields rule")
//!                 .enabled(true)
//!                 .expression("(http.host eq \"example.host.com\")")
//!                 .build())
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!         // Custom cache keys + settings
//!         var cacheSettingsExample = new Ruleset("cacheSettingsExample", RulesetArgs.builder()        
//!             .description("set cache settings for the request")
//!             .kind("zone")
//!             .name("set cache settings")
//!             .phase("http_request_cache_settings")
//!             .rules(RulesetRuleArgs.builder()
//!                 .action("set_cache_settings")
//!                 .actionParameters(RulesetRuleActionParametersArgs.builder()
//!                     .browserTtl(RulesetRuleActionParametersBrowserTtlArgs.builder()
//!                         .mode("respect_origin")
//!                         .build())
//!                     .cacheKey(RulesetRuleActionParametersCacheKeyArgs.builder()
//!                         .cacheDeceptionArmor(true)
//!                         .customKey(RulesetRuleActionParametersCacheKeyCustomKeyArgs.builder()
//!                             .cookie(RulesetRuleActionParametersCacheKeyCustomKeyCookieArgs.builder()
//!                                 .checkPresence(                                
//!                                     "cabc_t",
//!                                     "cdef_t")
//!                                 .include(                                
//!                                     "cabc",
//!                                     "cdef")
//!                                 .build())
//!                             .header(RulesetRuleActionParametersCacheKeyCustomKeyHeaderArgs.builder()
//!                                 .checkPresence(                                
//!                                     "habc_t",
//!                                     "hdef_t")
//!                                 .excludeOrigin(true)
//!                                 .include(                                
//!                                     "habc",
//!                                     "hdef")
//!                                 .build())
//!                             .host(RulesetRuleActionParametersCacheKeyCustomKeyHostArgs.builder()
//!                                 .resolved(true)
//!                                 .build())
//!                             .queryString(RulesetRuleActionParametersCacheKeyCustomKeyQueryStringArgs.builder()
//!                                 .exclude("*")
//!                                 .build())
//!                             .user(RulesetRuleActionParametersCacheKeyCustomKeyUserArgs.builder()
//!                                 .deviceType(true)
//!                                 .geo(false)
//!                                 .build())
//!                             .build())
//!                         .ignoreQueryStringsOrder(false)
//!                         .build())
//!                     .edgeTtl(RulesetRuleActionParametersEdgeTtlArgs.builder()
//!                         .default_(60)
//!                         .mode("override_origin")
//!                         .statusCodeTtl(                        
//!                             %!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference),
//!                             %!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference))
//!                         .build())
//!                     .originErrorPagePassthru(false)
//!                     .respectStrongEtags(true)
//!                     .serveStale(RulesetRuleActionParametersServeStaleArgs.builder()
//!                         .disableStaleWhileUpdating(true)
//!                         .build())
//!                     .build())
//!                 .description("set cache settings rule")
//!                 .enabled(true)
//!                 .expression("(http.host eq \"example.host.com\")")
//!                 .build())
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!         // Redirects based on a List resource
//!         var redirectFromListExample = new Ruleset("redirectFromListExample", RulesetArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .description("Redirect ruleset")
//!             .kind("root")
//!             .name("redirects")
//!             .phase("http_request_redirect")
//!             .rules(RulesetRuleArgs.builder()
//!                 .action("redirect")
//!                 .actionParameters(RulesetRuleActionParametersArgs.builder()
//!                     .fromList(RulesetRuleActionParametersFromListArgs.builder()
//!                         .key("http.request.full_uri")
//!                         .name("redirect_list")
//!                         .build())
//!                     .build())
//!                 .description("Apply redirects from redirect_list")
//!                 .enabled(true)
//!                 .expression("http.request.full_uri in $redirect_list")
//!                 .build())
//!             .build());
//!
//!         // Dynamic Redirects from value resource
//!         var redirectFromValueExample = new Ruleset("redirectFromValueExample", RulesetArgs.builder()        
//!             .description("Redirect ruleset")
//!             .kind("zone")
//!             .name("redirects")
//!             .phase("http_request_dynamic_redirect")
//!             .rules(RulesetRuleArgs.builder()
//!                 .action("redirect")
//!                 .actionParameters(RulesetRuleActionParametersArgs.builder()
//!                     .fromValue(RulesetRuleActionParametersFromValueArgs.builder()
//!                         .preserveQueryString(true)
//!                         .statusCode(301)
//!                         .targetUrl(RulesetRuleActionParametersFromValueTargetUrlArgs.builder()
//!                             .value("some_host.com")
//!                             .build())
//!                         .build())
//!                     .build())
//!                 .description("Apply redirect from value")
//!                 .enabled(true)
//!                 .expression("(http.request.uri.path matches \"^/api/\")")
//!                 .build())
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!         // Serve some custom error response
//!         var httpCustomErrorExample = new Ruleset("httpCustomErrorExample", RulesetArgs.builder()        
//!             .description("Serve some error response")
//!             .kind("zone")
//!             .name("Serve some error response")
//!             .phase("http_custom_errors")
//!             .rules(RulesetRuleArgs.builder()
//!                 .action("serve_error")
//!                 .actionParameters(RulesetRuleActionParametersArgs.builder()
//!                     .content("some error html")
//!                     .contentType("text/html")
//!                     .statusCode("530")
//!                     .build())
//!                 .description("serve some error response")
//!                 .enabled(true)
//!                 .expression("(http.request.uri.path matches \"^/api/\")")
//!                 .build())
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!         // Set Configuration Rules for an API route
//!         var httpConfigRulesExample = new Ruleset("httpConfigRulesExample", RulesetArgs.builder()        
//!             .description("set config rules for request")
//!             .kind("zone")
//!             .name("set config rules")
//!             .phase("http_config_settings")
//!             .rules(RulesetRuleArgs.builder()
//!                 .action("set_config")
//!                 .actionParameters(RulesetRuleActionParametersArgs.builder()
//!                     .bic(true)
//!                     .emailObfuscation(true)
//!                     .build())
//!                 .description("set config rules for matching request")
//!                 .enabled(true)
//!                 .expression("(http.request.uri.path matches \"^/api/\")")
//!                 .build())
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!         // Set compress algorithm for response.
//!         var responseCompressBrotliHtml = new Ruleset("responseCompressBrotliHtml", RulesetArgs.builder()        
//!             .description("Response compression ruleset")
//!             .kind("zone")
//!             .name("Brotli response compression for HTML")
//!             .phase("http_response_compression")
//!             .rules(RulesetRuleArgs.builder()
//!                 .action("compress_response")
//!                 .actionParameters(RulesetRuleActionParametersArgs.builder()
//!                     .algorithms(                    
//!                         RulesetRuleActionParametersAlgorithmArgs.builder()
//!                             .name("brotli")
//!                             .build(),
//!                         RulesetRuleActionParametersAlgorithmArgs.builder()
//!                             .name("auto")
//!                             .build())
//!                     .build())
//!                 .description("Prefer brotli compression for HTML")
//!                 .enabled(true)
//!                 .expression("http.response.content_type.media_type == \"text/html\"")
//!                 .build())
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Magic Transit
//!   magicTransitExample:
//!     type: cloudflare:Ruleset
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       description: example magic transit ruleset description
//!       kind: root
//!       name: account magic transit
//!       phase: magic_transit
//!       rules:
//!         - action: allow
//!           description: Allow TCP Ephemeral Ports
//!           expression: tcp.dstport in { 32768..65535 }
//!   # Zone-level WAF Managed Ruleset
//!   zoneLevelManagedWaf:
//!     type: cloudflare:Ruleset
//!     properties:
//!       description: managed WAF ruleset description
//!       kind: zone
//!       name: managed WAF
//!       phase: http_request_firewall_managed
//!       rules:
//!         - action: execute
//!           actionParameters:
//!             id: efb7b8c949ac4650a09736fc376e9aee
//!           description: Execute Cloudflare Managed Ruleset on my zone-level phase entry point ruleset
//!           enabled: true
//!           expression: (http.host eq "example.host.com")
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!   # Zone-level WAF with tag-based overrides
//!   zoneLevelManagedWafWithCategoryBasedOverrides:
//!     type: cloudflare:Ruleset
//!     properties:
//!       description: managed WAF with tag-based overrides ruleset description
//!       kind: zone
//!       name: managed WAF with tag-based overrides
//!       phase: http_request_firewall_managed
//!       rules:
//!         - action: execute
//!           actionParameters:
//!             id: efb7b8c949ac4650a09736fc376e9aee
//!             overrides:
//!               categories:
//!                 - action: block
//!                   category: wordpress
//!                   enabled: true
//!                 - action: block
//!                   category: joomla
//!                   enabled: true
//!           description: overrides to only enable wordpress rules to block
//!           enabled: false
//!           expression: (http.host eq "example.host.com")
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!   # Rewrite the URI path component to a static path
//!   transformUriRulePath:
//!     type: cloudflare:Ruleset
//!     properties:
//!       description: change the URI path to a new static path
//!       kind: zone
//!       name: transform rule for URI path
//!       phase: http_request_transform
//!       rules:
//!         - action: rewrite
//!           actionParameters:
//!             uri:
//!               path:
//!                 value: /my-new-route
//!           description: example URI path transform rule
//!           enabled: true
//!           expression: (http.host eq "example.com" and http.request.uri.path eq "/old-path")
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!   # Rewrite the URI query component to a static query
//!   transformUriRuleQuery:
//!     type: cloudflare:Ruleset
//!     properties:
//!       description: change the URI query to a new static query
//!       kind: zone
//!       name: transform rule for URI query parameter
//!       phase: http_request_transform
//!       rules:
//!         - action: rewrite
//!           actionParameters:
//!             uri:
//!               query:
//!                 value: old=new_again
//!           description: URI transformation query example
//!           enabled: true
//!           expression: (http.host eq "example.host.com")
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!   # Rewrite HTTP headers to a modified values
//!   transformUriHttpHeaders:
//!     type: cloudflare:Ruleset
//!     properties:
//!       description: modify HTTP headers before reaching origin
//!       kind: zone
//!       name: transform rule for HTTP headers
//!       phase: http_request_late_transform
//!       rules:
//!         - action: rewrite
//!           actionParameters:
//!             headers:
//!               - name: example-http-header-1
//!                 operation: set
//!                 value: my-http-header-value-1
//!               - expression: cf.zone.name
//!                 name: example-http-header-2
//!                 operation: set
//!               - name: example-http-header-3-to-remove
//!                 operation: remove
//!           description: example request header transform rule
//!           enabled: false
//!           expression: (http.host eq "example.host.com")
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!   # HTTP rate limit for an API route
//!   rateLimitingExample:
//!     type: cloudflare:Ruleset
//!     properties:
//!       description: apply HTTP rate limiting for a route
//!       kind: zone
//!       name: restrict API requests count
//!       phase: http_ratelimit
//!       rules:
//!         - action: block
//!           description: rate limit for API
//!           enabled: true
//!           expression: (http.request.uri.path matches "^/api/")
//!           ratelimit:
//!             characteristics:
//!               - cf.colo.id
//!               - ip.src
//!             mitigationTimeout: 600
//!             period: 60
//!             requestsPerPeriod: 100
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!   # Change origin for an API route
//!   httpOriginExample:
//!     type: cloudflare:Ruleset
//!     properties:
//!       description: Change origin for a route
//!       kind: zone
//!       name: Change to some origin
//!       phase: http_request_origin
//!       rules:
//!         - action: route
//!           actionParameters:
//!             hostHeader: some.host
//!             origin:
//!               host: some.host
//!               port: 80
//!           description: change origin to some.host
//!           enabled: true
//!           expression: (http.request.uri.path matches "^/api/")
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!   # Custom fields logging
//!   customFieldsLoggingExample:
//!     type: cloudflare:Ruleset
//!     properties:
//!       description: add custom fields to logging
//!       kind: zone
//!       name: log custom fields
//!       phase: http_log_custom_fields
//!       rules:
//!         - action: log_custom_field
//!           actionParameters:
//!             cookieFields:
//!               - __ga
//!               - accountNumber
//!               - __cfruid
//!             requestFields:
//!               - content-type
//!               - x-forwarded-for
//!               - host
//!             responseFields:
//!               - server
//!               - content-type
//!               - allow
//!           description: log custom fields rule
//!           enabled: true
//!           expression: (http.host eq "example.host.com")
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!   # Custom cache keys + settings
//!   cacheSettingsExample:
//!     type: cloudflare:Ruleset
//!     properties:
//!       description: set cache settings for the request
//!       kind: zone
//!       name: set cache settings
//!       phase: http_request_cache_settings
//!       rules:
//!         - action: set_cache_settings
//!           actionParameters:
//!             browserTtl:
//!               mode: respect_origin
//!             cacheKey:
//!               cacheDeceptionArmor: true
//!               customKey:
//!                 cookie:
//!                   checkPresence:
//!                     - cabc_t
//!                     - cdef_t
//!                   include:
//!                     - cabc
//!                     - cdef
//!                 header:
//!                   checkPresence:
//!                     - habc_t
//!                     - hdef_t
//!                   excludeOrigin: true
//!                   include:
//!                     - habc
//!                     - hdef
//!                 host:
//!                   resolved: true
//!                 queryString:
//!                   exclude:
//!                     - '*'
//!                 user:
//!                   deviceType: true
//!                   geo: false
//!               ignoreQueryStringsOrder: false
//!             edgeTtl:
//!               default: 60
//!               mode: override_origin
//!               statusCodeTtl:
//!                 - statusCode: 200
//!                   value: 50
//!                 - statusCodeRange:
//!                     - from: 201
//!                       to: 300
//!                   value: 30
//!             originErrorPagePassthru: false
//!             respectStrongEtags: true
//!             serveStale:
//!               disableStaleWhileUpdating: true
//!           description: set cache settings rule
//!           enabled: true
//!           expression: (http.host eq "example.host.com")
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!   # Redirects based on a List resource
//!   redirectFromListExample:
//!     type: cloudflare:Ruleset
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       description: Redirect ruleset
//!       kind: root
//!       name: redirects
//!       phase: http_request_redirect
//!       rules:
//!         - action: redirect
//!           actionParameters:
//!             fromList:
//!               key: http.request.full_uri
//!               name: redirect_list
//!           description: Apply redirects from redirect_list
//!           enabled: true
//!           expression: http.request.full_uri in $redirect_list
//!   # Dynamic Redirects from value resource
//!   redirectFromValueExample:
//!     type: cloudflare:Ruleset
//!     properties:
//!       description: Redirect ruleset
//!       kind: zone
//!       name: redirects
//!       phase: http_request_dynamic_redirect
//!       rules:
//!         - action: redirect
//!           actionParameters:
//!             fromValue:
//!               preserveQueryString: true
//!               statusCode: 301
//!               targetUrl:
//!                 value: some_host.com
//!           description: Apply redirect from value
//!           enabled: true
//!           expression: (http.request.uri.path matches "^/api/")
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!   # Serve some custom error response
//!   httpCustomErrorExample:
//!     type: cloudflare:Ruleset
//!     properties:
//!       description: Serve some error response
//!       kind: zone
//!       name: Serve some error response
//!       phase: http_custom_errors
//!       rules:
//!         - action: serve_error
//!           actionParameters:
//!             content: some error html
//!             contentType: text/html
//!             statusCode: '530'
//!           description: serve some error response
//!           enabled: true
//!           expression: (http.request.uri.path matches "^/api/")
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!   # Set Configuration Rules for an API route
//!   httpConfigRulesExample:
//!     type: cloudflare:Ruleset
//!     properties:
//!       description: set config rules for request
//!       kind: zone
//!       name: set config rules
//!       phase: http_config_settings
//!       rules:
//!         - action: set_config
//!           actionParameters:
//!             bic: true
//!             emailObfuscation: true
//!           description: set config rules for matching request
//!           enabled: true
//!           expression: (http.request.uri.path matches "^/api/")
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!   # Set compress algorithm for response.
//!   responseCompressBrotliHtml:
//!     type: cloudflare:Ruleset
//!     properties:
//!       description: Response compression ruleset
//!       kind: zone
//!       name: Brotli response compression for HTML
//!       phase: http_response_compression
//!       rules:
//!         - action: compress_response
//!           actionParameters:
//!             algorithms:
//!               - name: brotli
//!               - name: auto
//!           description: Prefer brotli compression for HTML
//!           enabled: true
//!           expression: http.response.content_type.media_type == "text/html"
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! Import an account scoped Ruleset configuration.
//!
//! ```sh
//! $ pulumi import cloudflare:index/ruleset:Ruleset example account/<account_id>/<ruleset_id>
//! ```
//!
//! Import a zone scoped Ruleset configuration.
//!
//! ```sh
//! $ pulumi import cloudflare:index/ruleset:Ruleset example zone/<zone_id>/<ruleset_id>
//! ```
//!

pub struct RulesetArgs {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Brief summary of the ruleset rule and its intended use.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Type of Ruleset to create. Available values: `custom`, `managed`, `root`, `zone`.
    pub kind: pulumi_wasm_rust::Output<String>,
    /// Name of the compression algorithm to use. Available values: `gzip`, `brotli`, `auto`, `default`, `none`
    pub name: pulumi_wasm_rust::Output<String>,
    /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`.
    pub phase: pulumi_wasm_rust::Output<String>,
    /// List of rule-based overrides.
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::RulesetRule>>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct RulesetResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Brief summary of the ruleset rule and its intended use.
    pub description: pulumi_wasm_rust::Output<String>,
    /// Type of Ruleset to create. Available values: `custom`, `managed`, `root`, `zone`.
    pub kind: pulumi_wasm_rust::Output<String>,
    /// Name of the compression algorithm to use. Available values: `gzip`, `brotli`, `auto`, `default`, `none`
    pub name: pulumi_wasm_rust::Output<String>,
    /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`.
    pub phase: pulumi_wasm_rust::Output<String>,
    /// List of rule-based overrides.
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::RulesetRule>>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RulesetArgs) -> RulesetResult {
    let result = crate::bindings::pulumi::cloudflare::ruleset::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::ruleset::Args {
            account_id: args.account_id.get_inner(),
            description: args.description.get_inner(),
            kind: args.kind.get_inner(),
            name: args.name.get_inner(),
            phase: args.phase.get_inner(),
            rules: args.rules.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    RulesetResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        kind: crate::into_domain(result.kind),
        name: crate::into_domain(result.name),
        phase: crate::into_domain(result.phase),
        rules: crate::into_domain(result.rules),
        zone_id: crate::into_domain(result.zone_id),
    }
}
