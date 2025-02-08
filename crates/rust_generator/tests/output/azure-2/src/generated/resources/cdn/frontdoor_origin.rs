/// Manages a Front Door (standard/premium) Origin.
///
/// !>**IMPORTANT:** If you are attempting to implement an Origin that uses its own Private Link Service with a Load Balancer the Profile resource in your configuration file **must** have a `depends_on` meta-argument which references the `azure.privatedns.LinkService`, see `Example Usage With Private Link Service` below.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleFrontdoorOrigin = frontdoor_origin::create(
///         "exampleFrontdoorOrigin",
///         FrontdoorOriginArgs::builder()
///             .cdn_frontdoor_origin_group_id("${exampleFrontdoorOriginGroup.id}")
///             .certificate_name_check_enabled(false)
///             .enabled(true)
///             .host_name("contoso.com")
///             .http_port(80)
///             .https_port(443)
///             .name("example-origin")
///             .origin_host_header("www.contoso.com")
///             .priority(1)
///             .weight(1)
///             .build_struct(),
///     );
///     let exampleFrontdoorOriginGroup = frontdoor_origin_group::create(
///         "exampleFrontdoorOriginGroup",
///         FrontdoorOriginGroupArgs::builder()
///             .cdn_frontdoor_profile_id("${exampleFrontdoorProfile.id}")
///             .load_balancing(FrontdoorOriginGroupLoadBalancing::builder().build_struct())
///             .name("example-origingroup")
///             .build_struct(),
///     );
///     let exampleFrontdoorProfile = frontdoor_profile::create(
///         "exampleFrontdoorProfile",
///         FrontdoorProfileArgs::builder()
///             .name("example-profile")
///             .resource_group_name("${example.name}")
///             .sku_name("Premium_AzureFrontDoor")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### With Private Link
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestoracc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Premium
///       accountReplicationType: LRS
///       allowNestedItemsToBePublic: false
///       networkRules:
///         defaultAction: Deny
///       tags:
///         environment: Example
///   exampleFrontdoorProfile:
///     type: azure:cdn:FrontdoorProfile
///     name: example
///     properties:
///       name: example-profile
///       resourceGroupName: ${example.name}
///       skuName: Premium_AzureFrontDoor
///   exampleFrontdoorOriginGroup:
///     type: azure:cdn:FrontdoorOriginGroup
///     name: example
///     properties:
///       name: example-origin-group
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///       loadBalancing: {}
///   exampleFrontdoorOrigin:
///     type: azure:cdn:FrontdoorOrigin
///     name: example
///     properties:
///       name: example-origin
///       cdnFrontdoorOriginGroupId: ${exampleFrontdoorOriginGroup.id}
///       enabled: true
///       certificateNameCheckEnabled: true
///       hostName: ${exampleAccount.primaryBlobHost}
///       originHostHeader: ${exampleAccount.primaryBlobHost}
///       priority: 1
///       weight: 500
///       privateLink:
///         requestMessage: Request access for Private Link Origin CDN Frontdoor
///         targetType: blob
///         location: ${exampleAccount.location}
///         privateLinkTargetId: ${exampleAccount.id}
/// ```
///
///
/// ### With Private Link Service
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleFrontdoorProfile:
///     type: azure:cdn:FrontdoorProfile
///     name: example
///     properties:
///       name: profile-example
///       resourceGroupName: ${example.name}
///       skuName: Premium_AzureFrontDoor
///     options:
///       dependsOn:
///         - ${exampleLinkService}
///   exampleFrontdoorOrigin:
///     type: azure:cdn:FrontdoorOrigin
///     name: example
///     properties:
///       name: origin-example
///       cdnFrontdoorOriginGroupId: ${exampleFrontdoorOriginGroup.id}
///       enabled: true
///       hostName: example.com
///       originHostHeader: example.com
///       priority: 1
///       weight: 1000
///       certificateNameCheckEnabled: false
///       privateLink:
///         requestMessage: Request access for Private Link Origin CDN Frontdoor
///         location: ${example.location}
///         privateLinkTargetId: ${exampleLinkService.id}
///   exampleFrontdoorOriginGroup:
///     type: azure:cdn:FrontdoorOriginGroup
///     name: example
///     properties:
///       name: group-example
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///       loadBalancing:
///         additionalLatencyInMilliseconds: 0
///         sampleSize: 16
///         successfulSamplesRequired: 3
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: vn-example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       addressSpaces:
///         - 10.5.0.0/16
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: sn-example
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.5.1.0/24
///       privateLinkServiceNetworkPoliciesEnabled: false
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: ip-example
///       sku: Standard
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       allocationMethod: Static
///   exampleLoadBalancer:
///     type: azure:lb:LoadBalancer
///     name: example
///     properties:
///       name: lb-example
///       sku: Standard
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       frontendIpConfigurations:
///         - name: ${examplePublicIp.name}
///           publicIpAddressId: ${examplePublicIp.id}
///   exampleLinkService:
///     type: azure:privatedns:LinkService
///     name: example
///     properties:
///       name: pls-example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       visibilitySubscriptionIds:
///         - ${current.subscriptionId}
///       loadBalancerFrontendIpConfigurationIds:
///         - ${exampleLoadBalancer.frontendIpConfigurations[0].id}
///       natIpConfigurations:
///         - name: primary
///           privateIpAddress: 10.5.1.17
///           privateIpAddressVersion: IPv4
///           subnetId: ${exampleSubnet.id}
///           primary: true
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Example HCL Configurations
///
/// * Private Link Origin with Storage Account Blob
/// * Private Link Origin with Storage Account Static Web Site
/// * Private Link Origin with Linux Web Application
/// * Private Link Origin with Internal Load Balancer
///
/// ## Import
///
/// Front Door Origins can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/frontdoorOrigin:FrontdoorOrigin example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Cdn/profiles/profile1/originGroups/originGroup1/origins/origin1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod frontdoor_origin {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrontdoorOriginArgs {
        /// The ID of the Front Door Origin Group within which this Front Door Origin should exist. Changing this forces a new Front Door Origin to be created.
        #[builder(into)]
        pub cdn_frontdoor_origin_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies whether certificate name checks are enabled for this origin.
        #[builder(into)]
        pub certificate_name_check_enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Should the origin be enabled? Possible values are `true` or `false`. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The IPv4 address, IPv6 address or Domain name of the Origin.
        ///
        /// !> **IMPORTANT:** This must be unique across all Front Door Origins within a Front Door Endpoint.
        #[builder(into)]
        pub host_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The value of the HTTP port. Must be between `1` and `65535`. Defaults to `80`.
        #[builder(into, default)]
        pub http_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The value of the HTTPS port. Must be between `1` and `65535`. Defaults to `443`.
        #[builder(into, default)]
        pub https_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name which should be used for this Front Door Origin. Changing this forces a new Front Door Origin to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The host header value (an IPv4 address, IPv6 address or Domain name) which is sent to the origin with each request. If unspecified the hostname from the request will be used.
        ///
        /// > Azure Front Door Origins, such as Web Apps, Blob Storage, and Cloud Services require this host header value to match the origin's hostname. This field's value overrides the host header defined in the Front Door Endpoint. For more information on how to properly set the origin host header value please see the [product documentation](https://docs.microsoft.com/azure/frontdoor/origin?pivots=front-door-standard-premium#origin-host-header).
        #[builder(into, default)]
        pub origin_host_header: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Priority of origin in given origin group for load balancing. Higher priorities will not be used for load balancing if any lower priority origin is healthy. Must be between `1` and `5` (inclusive). Defaults to `1`.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A `private_link` block as defined below.
        ///
        /// > **NOTE:** Private Link requires that the Front Door Profile this Origin is hosted within is using the SKU `Premium_AzureFrontDoor` and that the `certificate_name_check_enabled` field is set to `true`.
        #[builder(into, default)]
        pub private_link: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cdn::FrontdoorOriginPrivateLink>,
        >,
        /// The weight of the origin in a given origin group for load balancing. Must be between `1` and `1000`. Defaults to `500`.
        #[builder(into, default)]
        pub weight: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct FrontdoorOriginResult {
        /// The ID of the Front Door Origin Group within which this Front Door Origin should exist. Changing this forces a new Front Door Origin to be created.
        pub cdn_frontdoor_origin_group_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether certificate name checks are enabled for this origin.
        pub certificate_name_check_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Should the origin be enabled? Possible values are `true` or `false`. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The IPv4 address, IPv6 address or Domain name of the Origin.
        ///
        /// !> **IMPORTANT:** This must be unique across all Front Door Origins within a Front Door Endpoint.
        pub host_name: pulumi_gestalt_rust::Output<String>,
        /// The value of the HTTP port. Must be between `1` and `65535`. Defaults to `80`.
        pub http_port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The value of the HTTPS port. Must be between `1` and `65535`. Defaults to `443`.
        pub https_port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name which should be used for this Front Door Origin. Changing this forces a new Front Door Origin to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The host header value (an IPv4 address, IPv6 address or Domain name) which is sent to the origin with each request. If unspecified the hostname from the request will be used.
        ///
        /// > Azure Front Door Origins, such as Web Apps, Blob Storage, and Cloud Services require this host header value to match the origin's hostname. This field's value overrides the host header defined in the Front Door Endpoint. For more information on how to properly set the origin host header value please see the [product documentation](https://docs.microsoft.com/azure/frontdoor/origin?pivots=front-door-standard-premium#origin-host-header).
        pub origin_host_header: pulumi_gestalt_rust::Output<Option<String>>,
        /// Priority of origin in given origin group for load balancing. Higher priorities will not be used for load balancing if any lower priority origin is healthy. Must be between `1` and `5` (inclusive). Defaults to `1`.
        pub priority: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A `private_link` block as defined below.
        ///
        /// > **NOTE:** Private Link requires that the Front Door Profile this Origin is hosted within is using the SKU `Premium_AzureFrontDoor` and that the `certificate_name_check_enabled` field is set to `true`.
        pub private_link: pulumi_gestalt_rust::Output<
            Option<super::super::types::cdn::FrontdoorOriginPrivateLink>,
        >,
        /// The weight of the origin in a given origin group for load balancing. Must be between `1` and `1000`. Defaults to `500`.
        pub weight: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FrontdoorOriginArgs,
    ) -> FrontdoorOriginResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cdn_frontdoor_origin_group_id_binding = args
            .cdn_frontdoor_origin_group_id
            .get_output(context)
            .get_inner();
        let certificate_name_check_enabled_binding = args
            .certificate_name_check_enabled
            .get_output(context)
            .get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let host_name_binding = args.host_name.get_output(context).get_inner();
        let http_port_binding = args.http_port.get_output(context).get_inner();
        let https_port_binding = args.https_port.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let origin_host_header_binding = args
            .origin_host_header
            .get_output(context)
            .get_inner();
        let priority_binding = args.priority.get_output(context).get_inner();
        let private_link_binding = args.private_link.get_output(context).get_inner();
        let weight_binding = args.weight.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cdn/frontdoorOrigin:FrontdoorOrigin".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cdnFrontdoorOriginGroupId".into(),
                    value: &cdn_frontdoor_origin_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "certificateNameCheckEnabled".into(),
                    value: &certificate_name_check_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "hostName".into(),
                    value: &host_name_binding,
                },
                register_interface::ObjectField {
                    name: "httpPort".into(),
                    value: &http_port_binding,
                },
                register_interface::ObjectField {
                    name: "httpsPort".into(),
                    value: &https_port_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "originHostHeader".into(),
                    value: &origin_host_header_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "privateLink".into(),
                    value: &private_link_binding,
                },
                register_interface::ObjectField {
                    name: "weight".into(),
                    value: &weight_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FrontdoorOriginResult {
            cdn_frontdoor_origin_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cdnFrontdoorOriginGroupId"),
            ),
            certificate_name_check_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateNameCheckEnabled"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            host_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostName"),
            ),
            http_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpPort"),
            ),
            https_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpsPort"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            origin_host_header: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("originHostHeader"),
            ),
            priority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            private_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateLink"),
            ),
            weight: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("weight"),
            ),
        }
    }
}
