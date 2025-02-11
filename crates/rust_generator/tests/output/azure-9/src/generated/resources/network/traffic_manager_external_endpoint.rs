/// Manages an External Endpoint within a Traffic Manager Profile.
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
///   exampleTrafficManagerProfile:
///     type: azure:network:TrafficManagerProfile
///     name: example
///     properties:
///       name: example-profile
///       resourceGroupName: ${example.name}
///       trafficRoutingMethod: Weighted
///       dnsConfig:
///         relativeName: example-profile
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
///   exampleTrafficManagerExternalEndpoint:
///     type: azure:network:TrafficManagerExternalEndpoint
///     name: example
///     properties:
///       name: example-endpoint
///       profileId: ${exampleTrafficManagerProfile.id}
///       alwaysServeEnabled: true
///       weight: 100
///       target: www.example.com
/// ```
///
/// ## Import
///
/// External Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/trafficManagerExternalEndpoint:TrafficManagerExternalEndpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-group/providers/Microsoft.Network/trafficManagerProfiles/example-profile/ExternalEndpoints/example-endpoint
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod traffic_manager_external_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrafficManagerExternalEndpointArgs {
        /// If Always Serve is enabled, probing for endpoint health will be disabled and endpoints will be included in the traffic routing method. Defaults to `false`.
        #[builder(into, default)]
        pub always_serve_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// One or more `custom_header` blocks as defined below.
        #[builder(into, default)]
        pub custom_headers: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::TrafficManagerExternalEndpointCustomHeader,
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
        /// The name of the External Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the priority of this Endpoint, this must be specified for Profiles using the `Priority` traffic routing method. Supports values between 1 and 1000, with no Endpoints sharing the same value. If omitted the value will be computed in order of creation.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the Traffic Manager Profile that this External Endpoint should be created within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub profile_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `subnet` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub subnets: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::network::TrafficManagerExternalEndpointSubnet>,
            >,
        >,
        /// The FQDN DNS name of the target.
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies how much traffic should be distributed to this endpoint, this must be specified for Profiles using the Weighted traffic routing method. Valid values are between `1` and `1000`. Defaults to `1`.
        #[builder(into, default)]
        pub weight: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct TrafficManagerExternalEndpointResult {
        /// If Always Serve is enabled, probing for endpoint health will be disabled and endpoints will be included in the traffic routing method. Defaults to `false`.
        pub always_serve_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// One or more `custom_header` blocks as defined below.
        pub custom_headers: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::network::TrafficManagerExternalEndpointCustomHeader,
                >,
            >,
        >,
        /// Is the endpoint enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the Azure location of the Endpoint, this must be specified for Profiles using the `Performance` routing method.
        pub endpoint_location: pulumi_gestalt_rust::Output<String>,
        /// A list of Geographic Regions used to distribute traffic, such as `WORLD`, `UK` or `DE`. The same location can't be specified in two endpoints. [See the Geographic Hierarchies documentation for more information](https://docs.microsoft.com/rest/api/trafficmanager/geographichierarchies/getdefault).
        pub geo_mappings: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name of the External Endpoint. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the priority of this Endpoint, this must be specified for Profiles using the `Priority` traffic routing method. Supports values between 1 and 1000, with no Endpoints sharing the same value. If omitted the value will be computed in order of creation.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the Traffic Manager Profile that this External Endpoint should be created within. Changing this forces a new resource to be created.
        pub profile_id: pulumi_gestalt_rust::Output<String>,
        /// One or more `subnet` blocks as defined below. Changing this forces a new resource to be created.
        pub subnets: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::network::TrafficManagerExternalEndpointSubnet>,
            >,
        >,
        /// The FQDN DNS name of the target.
        pub target: pulumi_gestalt_rust::Output<String>,
        /// Specifies how much traffic should be distributed to this endpoint, this must be specified for Profiles using the Weighted traffic routing method. Valid values are between `1` and `1000`. Defaults to `1`.
        pub weight: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrafficManagerExternalEndpointArgs,
    ) -> TrafficManagerExternalEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let always_serve_enabled_binding = args.always_serve_enabled.get_output(context);
        let custom_headers_binding = args.custom_headers.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let endpoint_location_binding = args.endpoint_location.get_output(context);
        let geo_mappings_binding = args.geo_mappings.get_output(context);
        let name_binding = args.name.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let profile_id_binding = args.profile_id.get_output(context);
        let subnets_binding = args.subnets.get_output(context);
        let target_binding = args.target.get_output(context);
        let weight_binding = args.weight.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/trafficManagerExternalEndpoint:TrafficManagerExternalEndpoint"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alwaysServeEnabled".into(),
                    value: &always_serve_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customHeaders".into(),
                    value: &custom_headers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointLocation".into(),
                    value: &endpoint_location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "geoMappings".into(),
                    value: &geo_mappings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: &priority_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "profileId".into(),
                    value: &profile_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnets".into(),
                    value: &subnets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "target".into(),
                    value: &target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "weight".into(),
                    value: &weight_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TrafficManagerExternalEndpointResult {
            always_serve_enabled: o.get_field("alwaysServeEnabled"),
            custom_headers: o.get_field("customHeaders"),
            enabled: o.get_field("enabled"),
            endpoint_location: o.get_field("endpointLocation"),
            geo_mappings: o.get_field("geoMappings"),
            name: o.get_field("name"),
            priority: o.get_field("priority"),
            profile_id: o.get_field("profileId"),
            subnets: o.get_field("subnets"),
            target: o.get_field("target"),
            weight: o.get_field("weight"),
        }
    }
}
