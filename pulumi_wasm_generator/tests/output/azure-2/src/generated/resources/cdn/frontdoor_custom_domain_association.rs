/// Manages the association between a Front Door (standard/premium) Custom Domain and one or more Front Door (standard/premium) Routes.
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
///       name: domain.com
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
///       name: example-origin-group
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///       sessionAffinityEnabled: true
///       restoreTrafficTimeToHealedOrNewEndpointInMinutes: 10
///       healthProbe:
///         intervalInSeconds: 240
///         path: /healthProbe
///         protocol: Https
///         requestType: HEAD
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
///         - ${exampleFrontdoorCustomDomain.id}
///       linkToDefaultDomain: false
///   exampleFrontdoorCustomDomain:
///     type: azure:cdn:FrontdoorCustomDomain
///     name: example
///     properties:
///       name: example-customDomain
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
///   exampleFrontdoorCustomDomainAssociation:
///     type: azure:cdn:FrontdoorCustomDomainAssociation
///     name: example
///     properties:
///       cdnFrontdoorCustomDomainId: ${exampleFrontdoorCustomDomain.id}
///       cdnFrontdoorRouteIds:
///         - ${exampleFrontdoorRoute.id}
/// ```
///
/// ## Import
///
/// Front Door Custom Domain Associations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/frontdoorCustomDomainAssociation:FrontdoorCustomDomainAssociation example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Cdn/profiles/profile1/associations/assoc1
/// ```
///
pub mod frontdoor_custom_domain_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrontdoorCustomDomainAssociationArgs {
        /// The ID of the Front Door Custom Domain that should be managed by the association resource. Changing this forces a new association resource to be created.
        #[builder(into)]
        pub cdn_frontdoor_custom_domain_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// One or more IDs of the Front Door Route to which the Front Door Custom Domain is associated with.
        ///
        /// > **NOTE:** This should include all of the Front Door Route resources that the Front Door Custom Domain is associated with. If the list of Front Door Routes is not complete you will receive the service side error `This resource is still associated with a route. Please delete the association with the route first before deleting this resource` when you attempt to `destroy`/`delete` your Front Door Custom Domain.
        #[builder(into)]
        pub cdn_frontdoor_route_ids: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct FrontdoorCustomDomainAssociationResult {
        /// The ID of the Front Door Custom Domain that should be managed by the association resource. Changing this forces a new association resource to be created.
        pub cdn_frontdoor_custom_domain_id: pulumi_wasm_rust::Output<String>,
        /// One or more IDs of the Front Door Route to which the Front Door Custom Domain is associated with.
        ///
        /// > **NOTE:** This should include all of the Front Door Route resources that the Front Door Custom Domain is associated with. If the list of Front Door Routes is not complete you will receive the service side error `This resource is still associated with a route. Please delete the association with the route first before deleting this resource` when you attempt to `destroy`/`delete` your Front Door Custom Domain.
        pub cdn_frontdoor_route_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FrontdoorCustomDomainAssociationArgs,
    ) -> FrontdoorCustomDomainAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cdn_frontdoor_custom_domain_id_binding = args
            .cdn_frontdoor_custom_domain_id
            .get_output(context)
            .get_inner();
        let cdn_frontdoor_route_ids_binding = args
            .cdn_frontdoor_route_ids
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cdn/frontdoorCustomDomainAssociation:FrontdoorCustomDomainAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cdnFrontdoorCustomDomainId".into(),
                    value: &cdn_frontdoor_custom_domain_id_binding,
                },
                register_interface::ObjectField {
                    name: "cdnFrontdoorRouteIds".into(),
                    value: &cdn_frontdoor_route_ids_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FrontdoorCustomDomainAssociationResult {
            cdn_frontdoor_custom_domain_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cdnFrontdoorCustomDomainId"),
            ),
            cdn_frontdoor_route_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cdnFrontdoorRouteIds"),
            ),
        }
    }
}
