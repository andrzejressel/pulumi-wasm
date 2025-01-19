/// Manages a Front Door (standard/premium) Route.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-cdn-frontdoor
///       location: West Europe
///   exampleZone:
///     type: azure:dns:Zone
///     name: example
///     properties:
///       name: example.com
///       resourceGroupName: ${example.name}
///   exampleFrontdoorProfile:
///     type: azure:cdn:FrontdoorProfile
///     name: example
///     properties:
///       name: example-profile
///       resourceGroupName: ${example.name}
///       skuName: Standard_AzureFrontDoor
///   exampleFrontdoorOriginGroup:
///     type: azure:cdn:FrontdoorOriginGroup
///     name: example
///     properties:
///       name: example-originGroup
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///       loadBalancing:
///         additionalLatencyInMilliseconds: 0
///         sampleSize: 16
///         successfulSamplesRequired: 3
///   exampleFrontdoorOrigin:
///     type: azure:cdn:FrontdoorOrigin
///     name: example
///     properties:
///       name: example-origin
///       cdnFrontdoorOriginGroupId: ${exampleFrontdoorOriginGroup.id}
///       enabled: true
///       certificateNameCheckEnabled: false
///       hostName: contoso.com
///       httpPort: 80
///       httpsPort: 443
///       originHostHeader: www.contoso.com
///       priority: 1
///       weight: 1
///   exampleFrontdoorEndpoint:
///     type: azure:cdn:FrontdoorEndpoint
///     name: example
///     properties:
///       name: example-endpoint
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///   exampleFrontdoorRuleSet:
///     type: azure:cdn:FrontdoorRuleSet
///     name: example
///     properties:
///       name: ExampleRuleSet
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///   contoso:
///     type: azure:cdn:FrontdoorCustomDomain
///     properties:
///       name: contoso-custom-domain
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///       dnsZoneId: ${exampleZone.id}
///       hostName:
///         fn::invoke:
///           function: std:join
///           arguments:
///             separator: .
///             input:
///               - contoso
///               - ${exampleZone.name}
///           return: result
///       tls:
///         certificateType: ManagedCertificate
///         minimumTlsVersion: TLS12
///   fabrikam:
///     type: azure:cdn:FrontdoorCustomDomain
///     properties:
///       name: fabrikam-custom-domain
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///       dnsZoneId: ${exampleZone.id}
///       hostName:
///         fn::invoke:
///           function: std:join
///           arguments:
///             separator: .
///             input:
///               - fabrikam
///               - ${exampleZone.name}
///           return: result
///       tls:
///         certificateType: ManagedCertificate
///         minimumTlsVersion: TLS12
///   exampleFrontdoorRoute:
///     type: azure:cdn:FrontdoorRoute
///     name: example
///     properties:
///       name: example-route
///       cdnFrontdoorEndpointId: ${exampleFrontdoorEndpoint.id}
///       cdnFrontdoorOriginGroupId: ${exampleFrontdoorOriginGroup.id}
///       cdnFrontdoorOriginIds:
///         - ${exampleFrontdoorOrigin.id}
///       cdnFrontdoorRuleSetIds:
///         - ${exampleFrontdoorRuleSet.id}
///       enabled: true
///       forwardingProtocol: HttpsOnly
///       httpsRedirectEnabled: true
///       patternsToMatches:
///         - /*
///       supportedProtocols:
///         - Http
///         - Https
///       cdnFrontdoorCustomDomainIds:
///         - ${contoso.id}
///         - ${fabrikam.id}
///       linkToDefaultDomain: false
///       cache:
///         queryStringCachingBehavior: IgnoreSpecifiedQueryStrings
///         queryStrings:
///           - account
///           - settings
///         compressionEnabled: true
///         contentTypesToCompresses:
///           - text/html
///           - text/javascript
///           - text/xml
///   contosoFrontdoorCustomDomainAssociation:
///     type: azure:cdn:FrontdoorCustomDomainAssociation
///     name: contoso
///     properties:
///       cdnFrontdoorCustomDomainId: ${contoso.id}
///       cdnFrontdoorRouteIds:
///         - ${exampleFrontdoorRoute.id}
///   fabrikamFrontdoorCustomDomainAssociation:
///     type: azure:cdn:FrontdoorCustomDomainAssociation
///     name: fabrikam
///     properties:
///       cdnFrontdoorCustomDomainId: ${fabrikam.id}
///       cdnFrontdoorRouteIds:
///         - ${exampleFrontdoorRoute.id}
/// ```
///
/// ## Import
///
/// Front Door Routes can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/frontdoorRoute:FrontdoorRoute example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Cdn/profiles/profile1/afdEndpoints/endpoint1/routes/route1
/// ```
///
pub mod frontdoor_route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrontdoorRouteArgs {
        /// A `cache` block as defined below.
        ///
        /// > **NOTE:** To disable caching, do not provide the `cache` block in the configuration file.
        #[builder(into, default)]
        pub cache: pulumi_wasm_rust::Output<
            Option<super::super::types::cdn::FrontdoorRouteCache>,
        >,
        /// The IDs of the Front Door Custom Domains which are associated with this Front Door Route.
        #[builder(into, default)]
        pub cdn_frontdoor_custom_domain_ids: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The resource ID of the Front Door Endpoint where this Front Door Route should exist. Changing this forces a new Front Door Route to be created.
        #[builder(into)]
        pub cdn_frontdoor_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Front Door Origin Group where this Front Door Route should be created.
        #[builder(into)]
        pub cdn_frontdoor_origin_group_id: pulumi_wasm_rust::Output<String>,
        /// One or more Front Door Origin resource IDs that this Front Door Route will link to.
        #[builder(into)]
        pub cdn_frontdoor_origin_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A directory path on the Front Door Origin that can be used to retrieve content (e.g. `contoso.cloudapp.net/originpath`).
        #[builder(into, default)]
        pub cdn_frontdoor_origin_path: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of the Front Door Rule Set IDs which should be assigned to this Front Door Route.
        #[builder(into, default)]
        pub cdn_frontdoor_rule_set_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Is this Front Door Route enabled? Possible values are `true` or `false`. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Protocol that will be use when forwarding traffic to backends. Possible values are `HttpOnly`, `HttpsOnly` or `MatchRequest`. Defaults to `MatchRequest`.
        #[builder(into, default)]
        pub forwarding_protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// Automatically redirect HTTP traffic to HTTPS traffic? Possible values are `true` or `false`. Defaults to `true`.
        ///
        /// > **NOTE:** The `https_redirect_enabled` rule is the first rule that will be executed.
        #[builder(into, default)]
        pub https_redirect_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should this Front Door Route be linked to the default endpoint? Possible values include `true` or `false`. Defaults to `true`.
        #[builder(into, default)]
        pub link_to_default_domain: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Front Door Route. Valid values must begin with a letter or number, end with a letter or number and may only contain letters, numbers and hyphens with a maximum length of 90 characters. Changing this forces a new Front Door Route to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The route patterns of the rule.
        #[builder(into)]
        pub patterns_to_matches: pulumi_wasm_rust::Output<Vec<String>>,
        /// One or more Protocols supported by this Front Door Route. Possible values are `Http` or `Https`.
        ///
        /// > **NOTE:** If `https_redirect_enabled` is set to `true` the `supported_protocols` field must contain both `Http` and `Https` values.
        #[builder(into)]
        pub supported_protocols: pulumi_wasm_rust::Output<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct FrontdoorRouteResult {
        /// A `cache` block as defined below.
        ///
        /// > **NOTE:** To disable caching, do not provide the `cache` block in the configuration file.
        pub cache: pulumi_wasm_rust::Output<
            Option<super::super::types::cdn::FrontdoorRouteCache>,
        >,
        /// The IDs of the Front Door Custom Domains which are associated with this Front Door Route.
        pub cdn_frontdoor_custom_domain_ids: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The resource ID of the Front Door Endpoint where this Front Door Route should exist. Changing this forces a new Front Door Route to be created.
        pub cdn_frontdoor_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Front Door Origin Group where this Front Door Route should be created.
        pub cdn_frontdoor_origin_group_id: pulumi_wasm_rust::Output<String>,
        /// One or more Front Door Origin resource IDs that this Front Door Route will link to.
        pub cdn_frontdoor_origin_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A directory path on the Front Door Origin that can be used to retrieve content (e.g. `contoso.cloudapp.net/originpath`).
        pub cdn_frontdoor_origin_path: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of the Front Door Rule Set IDs which should be assigned to this Front Door Route.
        pub cdn_frontdoor_rule_set_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Is this Front Door Route enabled? Possible values are `true` or `false`. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Protocol that will be use when forwarding traffic to backends. Possible values are `HttpOnly`, `HttpsOnly` or `MatchRequest`. Defaults to `MatchRequest`.
        pub forwarding_protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// Automatically redirect HTTP traffic to HTTPS traffic? Possible values are `true` or `false`. Defaults to `true`.
        ///
        /// > **NOTE:** The `https_redirect_enabled` rule is the first rule that will be executed.
        pub https_redirect_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should this Front Door Route be linked to the default endpoint? Possible values include `true` or `false`. Defaults to `true`.
        pub link_to_default_domain: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Front Door Route. Valid values must begin with a letter or number, end with a letter or number and may only contain letters, numbers and hyphens with a maximum length of 90 characters. Changing this forces a new Front Door Route to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The route patterns of the rule.
        pub patterns_to_matches: pulumi_wasm_rust::Output<Vec<String>>,
        /// One or more Protocols supported by this Front Door Route. Possible values are `Http` or `Https`.
        ///
        /// > **NOTE:** If `https_redirect_enabled` is set to `true` the `supported_protocols` field must contain both `Http` and `Https` values.
        pub supported_protocols: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FrontdoorRouteArgs) -> FrontdoorRouteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cache_binding = args.cache.get_inner();
        let cdn_frontdoor_custom_domain_ids_binding = args
            .cdn_frontdoor_custom_domain_ids
            .get_inner();
        let cdn_frontdoor_endpoint_id_binding = args
            .cdn_frontdoor_endpoint_id
            .get_inner();
        let cdn_frontdoor_origin_group_id_binding = args
            .cdn_frontdoor_origin_group_id
            .get_inner();
        let cdn_frontdoor_origin_ids_binding = args.cdn_frontdoor_origin_ids.get_inner();
        let cdn_frontdoor_origin_path_binding = args
            .cdn_frontdoor_origin_path
            .get_inner();
        let cdn_frontdoor_rule_set_ids_binding = args
            .cdn_frontdoor_rule_set_ids
            .get_inner();
        let enabled_binding = args.enabled.get_inner();
        let forwarding_protocol_binding = args.forwarding_protocol.get_inner();
        let https_redirect_enabled_binding = args.https_redirect_enabled.get_inner();
        let link_to_default_domain_binding = args.link_to_default_domain.get_inner();
        let name_binding = args.name.get_inner();
        let patterns_to_matches_binding = args.patterns_to_matches.get_inner();
        let supported_protocols_binding = args.supported_protocols.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cdn/frontdoorRoute:FrontdoorRoute".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cache".into(),
                    value: &cache_binding,
                },
                register_interface::ObjectField {
                    name: "cdnFrontdoorCustomDomainIds".into(),
                    value: &cdn_frontdoor_custom_domain_ids_binding,
                },
                register_interface::ObjectField {
                    name: "cdnFrontdoorEndpointId".into(),
                    value: &cdn_frontdoor_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "cdnFrontdoorOriginGroupId".into(),
                    value: &cdn_frontdoor_origin_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "cdnFrontdoorOriginIds".into(),
                    value: &cdn_frontdoor_origin_ids_binding,
                },
                register_interface::ObjectField {
                    name: "cdnFrontdoorOriginPath".into(),
                    value: &cdn_frontdoor_origin_path_binding,
                },
                register_interface::ObjectField {
                    name: "cdnFrontdoorRuleSetIds".into(),
                    value: &cdn_frontdoor_rule_set_ids_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "forwardingProtocol".into(),
                    value: &forwarding_protocol_binding,
                },
                register_interface::ObjectField {
                    name: "httpsRedirectEnabled".into(),
                    value: &https_redirect_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "linkToDefaultDomain".into(),
                    value: &link_to_default_domain_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "patternsToMatches".into(),
                    value: &patterns_to_matches_binding,
                },
                register_interface::ObjectField {
                    name: "supportedProtocols".into(),
                    value: &supported_protocols_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cache".into(),
                },
                register_interface::ResultField {
                    name: "cdnFrontdoorCustomDomainIds".into(),
                },
                register_interface::ResultField {
                    name: "cdnFrontdoorEndpointId".into(),
                },
                register_interface::ResultField {
                    name: "cdnFrontdoorOriginGroupId".into(),
                },
                register_interface::ResultField {
                    name: "cdnFrontdoorOriginIds".into(),
                },
                register_interface::ResultField {
                    name: "cdnFrontdoorOriginPath".into(),
                },
                register_interface::ResultField {
                    name: "cdnFrontdoorRuleSetIds".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "forwardingProtocol".into(),
                },
                register_interface::ResultField {
                    name: "httpsRedirectEnabled".into(),
                },
                register_interface::ResultField {
                    name: "linkToDefaultDomain".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "patternsToMatches".into(),
                },
                register_interface::ResultField {
                    name: "supportedProtocols".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FrontdoorRouteResult {
            cache: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cache").unwrap(),
            ),
            cdn_frontdoor_custom_domain_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cdnFrontdoorCustomDomainIds").unwrap(),
            ),
            cdn_frontdoor_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cdnFrontdoorEndpointId").unwrap(),
            ),
            cdn_frontdoor_origin_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cdnFrontdoorOriginGroupId").unwrap(),
            ),
            cdn_frontdoor_origin_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cdnFrontdoorOriginIds").unwrap(),
            ),
            cdn_frontdoor_origin_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cdnFrontdoorOriginPath").unwrap(),
            ),
            cdn_frontdoor_rule_set_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cdnFrontdoorRuleSetIds").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            forwarding_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forwardingProtocol").unwrap(),
            ),
            https_redirect_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsRedirectEnabled").unwrap(),
            ),
            link_to_default_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkToDefaultDomain").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            patterns_to_matches: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("patternsToMatches").unwrap(),
            ),
            supported_protocols: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedProtocols").unwrap(),
            ),
        }
    }
}
