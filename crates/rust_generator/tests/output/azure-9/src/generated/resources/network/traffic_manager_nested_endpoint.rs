/// Manages a Nested Endpoint within a Traffic Manager Profile.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: example-publicip
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       allocationMethod: Static
///       domainNameLabel: example-pip
///   parent:
///     type: azure:network:TrafficManagerProfile
///     properties:
///       name: parent-profile
///       resourceGroupName: ${example.name}
///       trafficRoutingMethod: Weighted
///       dnsConfig:
///         relativeName: parent-profile
///         ttl: 100
///       monitorConfig:
///         protocol: HTTP
///         port: 80
///         path: /
///         intervalInSeconds: 30
///         timeoutInSeconds: 9
///         toleratedNumberOfFailures: 3
///       tags:
///         environment: Production
///   nested:
///     type: azure:network:TrafficManagerProfile
///     properties:
///       name: nested-profile
///       resourceGroupName: ${example.name}
///       trafficRoutingMethod: Priority
///       dnsConfig:
///         relativeName: nested-profile
///         ttl: 30
///       monitorConfig:
///         protocol: HTTP
///         port: 443
///         path: /
///   exampleTrafficManagerNestedEndpoint:
///     type: azure:network:TrafficManagerNestedEndpoint
///     name: example
///     properties:
///       name: example-endpoint
///       targetResourceId: ${nested.id}
///       priority: 1
///       profileId: ${parent.id}
///       minimumChildEndpoints: 9
///       weight: 5
/// ```
///
/// ## Import
///
/// Nested Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/trafficManagerNestedEndpoint:TrafficManagerNestedEndpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-resources/providers/Microsoft.Network/trafficManagerProfiles/example-profile/NestedEndpoints/example-endpoint
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod traffic_manager_nested_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrafficManagerNestedEndpointArgs {
        /// One or more `custom_header` blocks as defined below.
        #[builder(into, default)]
        pub custom_headers: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::TrafficManagerNestedEndpointCustomHeader,
                >,
            >,
        >,
        /// Is the endpoint enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the Azure location of the Endpoint, this must be specified for Profiles using the `Performance` routing method.
        #[builder(into, default)]
        pub endpoint_location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of Geographic Regions used to distribute traffic, such as `WORLD`, `UK` or `DE`. The same location can't be specified in two endpoints. [See the Geographic Hierarchies documentation for more information](https://docs.microsoft.com/rest/api/trafficmanager/geographichierarchies/getdefault).
        #[builder(into, default)]
        pub geo_mappings: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// This argument specifies the minimum number of endpoints that must be ‘online’ in the child profile in order for the parent profile to direct traffic to any of the endpoints in that child profile. This value must be larger than `0`.
        ///
        /// ~>**NOTE:** If `min_child_endpoints` is less than either `minimum_required_child_endpoints_ipv4` or `minimum_required_child_endpoints_ipv6`, then it won't have any effect.
        #[builder(into)]
        pub minimum_child_endpoints: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// This argument specifies the minimum number of IPv4 (DNS record type A) endpoints that must be ‘online’ in the child profile in order for the parent profile to direct traffic to any of the endpoints in that child profile. This argument only applies to Endpoints of type `nestedEndpoints` and
        #[builder(into, default)]
        pub minimum_required_child_endpoints_ipv4: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// This argument specifies the minimum number of IPv6 (DNS record type AAAA) endpoints that must be ‘online’ in the child profile in order for the parent profile to direct traffic to any of the endpoints in that child profile. This argument only applies to Endpoints of type `nestedEndpoints` and
        #[builder(into, default)]
        pub minimum_required_child_endpoints_ipv6: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The name of the External Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the priority of this Endpoint, this must be specified for Profiles using the `Priority` traffic routing method. Supports values between 1 and 1000, with no Endpoints sharing the same value. If omitted the value will be computed in order of creation. Defaults to `1`.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the Traffic Manager Profile that this External Endpoint should be created within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub profile_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `subnet` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub subnets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::network::TrafficManagerNestedEndpointSubnet>>,
        >,
        /// The resource id of an Azure resource to target.
        #[builder(into)]
        pub target_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies how much traffic should be distributed to this endpoint, this must be specified for Profiles using the Weighted traffic routing method. Valid values are between `1` and `1000`. Defaults to `1`.
        #[builder(into, default)]
        pub weight: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct TrafficManagerNestedEndpointResult {
        /// One or more `custom_header` blocks as defined below.
        pub custom_headers: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::network::TrafficManagerNestedEndpointCustomHeader,
                >,
            >,
        >,
        /// Is the endpoint enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the Azure location of the Endpoint, this must be specified for Profiles using the `Performance` routing method.
        pub endpoint_location: pulumi_gestalt_rust::Output<String>,
        /// A list of Geographic Regions used to distribute traffic, such as `WORLD`, `UK` or `DE`. The same location can't be specified in two endpoints. [See the Geographic Hierarchies documentation for more information](https://docs.microsoft.com/rest/api/trafficmanager/geographichierarchies/getdefault).
        pub geo_mappings: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// This argument specifies the minimum number of endpoints that must be ‘online’ in the child profile in order for the parent profile to direct traffic to any of the endpoints in that child profile. This value must be larger than `0`.
        ///
        /// ~>**NOTE:** If `min_child_endpoints` is less than either `minimum_required_child_endpoints_ipv4` or `minimum_required_child_endpoints_ipv6`, then it won't have any effect.
        pub minimum_child_endpoints: pulumi_gestalt_rust::Output<i32>,
        /// This argument specifies the minimum number of IPv4 (DNS record type A) endpoints that must be ‘online’ in the child profile in order for the parent profile to direct traffic to any of the endpoints in that child profile. This argument only applies to Endpoints of type `nestedEndpoints` and
        pub minimum_required_child_endpoints_ipv4: pulumi_gestalt_rust::Output<
            Option<i32>,
        >,
        /// This argument specifies the minimum number of IPv6 (DNS record type AAAA) endpoints that must be ‘online’ in the child profile in order for the parent profile to direct traffic to any of the endpoints in that child profile. This argument only applies to Endpoints of type `nestedEndpoints` and
        pub minimum_required_child_endpoints_ipv6: pulumi_gestalt_rust::Output<
            Option<i32>,
        >,
        /// The name of the External Endpoint. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the priority of this Endpoint, this must be specified for Profiles using the `Priority` traffic routing method. Supports values between 1 and 1000, with no Endpoints sharing the same value. If omitted the value will be computed in order of creation. Defaults to `1`.
        pub priority: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the Traffic Manager Profile that this External Endpoint should be created within. Changing this forces a new resource to be created.
        pub profile_id: pulumi_gestalt_rust::Output<String>,
        /// One or more `subnet` blocks as defined below. Changing this forces a new resource to be created.
        pub subnets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::network::TrafficManagerNestedEndpointSubnet>>,
        >,
        /// The resource id of an Azure resource to target.
        pub target_resource_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies how much traffic should be distributed to this endpoint, this must be specified for Profiles using the Weighted traffic routing method. Valid values are between `1` and `1000`. Defaults to `1`.
        pub weight: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TrafficManagerNestedEndpointArgs,
    ) -> TrafficManagerNestedEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let custom_headers_binding_1 = args.custom_headers.get_output(context);
        let custom_headers_binding = custom_headers_binding_1.get_inner();
        let enabled_binding_1 = args.enabled.get_output(context);
        let enabled_binding = enabled_binding_1.get_inner();
        let endpoint_location_binding_1 = args.endpoint_location.get_output(context);
        let endpoint_location_binding = endpoint_location_binding_1.get_inner();
        let geo_mappings_binding_1 = args.geo_mappings.get_output(context);
        let geo_mappings_binding = geo_mappings_binding_1.get_inner();
        let minimum_child_endpoints_binding_1 = args
            .minimum_child_endpoints
            .get_output(context);
        let minimum_child_endpoints_binding = minimum_child_endpoints_binding_1
            .get_inner();
        let minimum_required_child_endpoints_ipv4_binding_1 = args
            .minimum_required_child_endpoints_ipv4
            .get_output(context);
        let minimum_required_child_endpoints_ipv4_binding = minimum_required_child_endpoints_ipv4_binding_1
            .get_inner();
        let minimum_required_child_endpoints_ipv6_binding_1 = args
            .minimum_required_child_endpoints_ipv6
            .get_output(context);
        let minimum_required_child_endpoints_ipv6_binding = minimum_required_child_endpoints_ipv6_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let priority_binding_1 = args.priority.get_output(context);
        let priority_binding = priority_binding_1.get_inner();
        let profile_id_binding_1 = args.profile_id.get_output(context);
        let profile_id_binding = profile_id_binding_1.get_inner();
        let subnets_binding_1 = args.subnets.get_output(context);
        let subnets_binding = subnets_binding_1.get_inner();
        let target_resource_id_binding_1 = args.target_resource_id.get_output(context);
        let target_resource_id_binding = target_resource_id_binding_1.get_inner();
        let weight_binding_1 = args.weight.get_output(context);
        let weight_binding = weight_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/trafficManagerNestedEndpoint:TrafficManagerNestedEndpoint"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customHeaders".into(),
                    value: &custom_headers_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "endpointLocation".into(),
                    value: &endpoint_location_binding,
                },
                register_interface::ObjectField {
                    name: "geoMappings".into(),
                    value: &geo_mappings_binding,
                },
                register_interface::ObjectField {
                    name: "minimumChildEndpoints".into(),
                    value: &minimum_child_endpoints_binding,
                },
                register_interface::ObjectField {
                    name: "minimumRequiredChildEndpointsIpv4".into(),
                    value: &minimum_required_child_endpoints_ipv4_binding,
                },
                register_interface::ObjectField {
                    name: "minimumRequiredChildEndpointsIpv6".into(),
                    value: &minimum_required_child_endpoints_ipv6_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "profileId".into(),
                    value: &profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "subnets".into(),
                    value: &subnets_binding,
                },
                register_interface::ObjectField {
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "weight".into(),
                    value: &weight_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TrafficManagerNestedEndpointResult {
            custom_headers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customHeaders"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            endpoint_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointLocation"),
            ),
            geo_mappings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("geoMappings"),
            ),
            minimum_child_endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minimumChildEndpoints"),
            ),
            minimum_required_child_endpoints_ipv4: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minimumRequiredChildEndpointsIpv4"),
            ),
            minimum_required_child_endpoints_ipv6: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minimumRequiredChildEndpointsIpv6"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            priority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            profile_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("profileId"),
            ),
            subnets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnets"),
            ),
            target_resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetResourceId"),
            ),
            weight: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("weight"),
            ),
        }
    }
}
