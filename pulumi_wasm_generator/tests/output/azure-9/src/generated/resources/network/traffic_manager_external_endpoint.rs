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
pub mod traffic_manager_external_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrafficManagerExternalEndpointArgs {
        /// If Always Serve is enabled, probing for endpoint health will be disabled and endpoints will be included in the traffic routing method. Defaults to `false`.
        #[builder(into, default)]
        pub always_serve_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// One or more `custom_header` blocks as defined below.
        #[builder(into, default)]
        pub custom_headers: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::TrafficManagerExternalEndpointCustomHeader,
                >,
            >,
        >,
        /// Is the endpoint enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the Azure location of the Endpoint, this must be specified for Profiles using the `Performance` routing method.
        #[builder(into, default)]
        pub endpoint_location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of Geographic Regions used to distribute traffic, such as `WORLD`, `UK` or `DE`. The same location can't be specified in two endpoints. [See the Geographic Hierarchies documentation for more information](https://docs.microsoft.com/rest/api/trafficmanager/geographichierarchies/getdefault).
        #[builder(into, default)]
        pub geo_mappings: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the External Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the priority of this Endpoint, this must be specified for Profiles using the `Priority` traffic routing method. Supports values between 1 and 1000, with no Endpoints sharing the same value. If omitted the value will be computed in order of creation.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The ID of the Traffic Manager Profile that this External Endpoint should be created within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub profile_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// One or more `subnet` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub subnets: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::types::network::TrafficManagerExternalEndpointSubnet>,
            >,
        >,
        /// The FQDN DNS name of the target.
        #[builder(into)]
        pub target: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies how much traffic should be distributed to this endpoint, this must be specified for Profiles using the Weighted traffic routing method. Valid values are between `1` and `1000`. Defaults to `1`.
        #[builder(into, default)]
        pub weight: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct TrafficManagerExternalEndpointResult {
        /// If Always Serve is enabled, probing for endpoint health will be disabled and endpoints will be included in the traffic routing method. Defaults to `false`.
        pub always_serve_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `custom_header` blocks as defined below.
        pub custom_headers: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::TrafficManagerExternalEndpointCustomHeader,
                >,
            >,
        >,
        /// Is the endpoint enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the Azure location of the Endpoint, this must be specified for Profiles using the `Performance` routing method.
        pub endpoint_location: pulumi_wasm_rust::Output<String>,
        /// A list of Geographic Regions used to distribute traffic, such as `WORLD`, `UK` or `DE`. The same location can't be specified in two endpoints. [See the Geographic Hierarchies documentation for more information](https://docs.microsoft.com/rest/api/trafficmanager/geographichierarchies/getdefault).
        pub geo_mappings: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the External Endpoint. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the priority of this Endpoint, this must be specified for Profiles using the `Priority` traffic routing method. Supports values between 1 and 1000, with no Endpoints sharing the same value. If omitted the value will be computed in order of creation.
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// The ID of the Traffic Manager Profile that this External Endpoint should be created within. Changing this forces a new resource to be created.
        pub profile_id: pulumi_wasm_rust::Output<String>,
        /// One or more `subnet` blocks as defined below. Changing this forces a new resource to be created.
        pub subnets: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::network::TrafficManagerExternalEndpointSubnet>,
            >,
        >,
        /// The FQDN DNS name of the target.
        pub target: pulumi_wasm_rust::Output<String>,
        /// Specifies how much traffic should be distributed to this endpoint, this must be specified for Profiles using the Weighted traffic routing method. Valid values are between `1` and `1000`. Defaults to `1`.
        pub weight: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TrafficManagerExternalEndpointArgs,
    ) -> TrafficManagerExternalEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let always_serve_enabled_binding = args
            .always_serve_enabled
            .get_output(context)
            .get_inner();
        let custom_headers_binding = args.custom_headers.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let endpoint_location_binding = args
            .endpoint_location
            .get_output(context)
            .get_inner();
        let geo_mappings_binding = args.geo_mappings.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let priority_binding = args.priority.get_output(context).get_inner();
        let profile_id_binding = args.profile_id.get_output(context).get_inner();
        let subnets_binding = args.subnets.get_output(context).get_inner();
        let target_binding = args.target.get_output(context).get_inner();
        let weight_binding = args.weight.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/trafficManagerExternalEndpoint:TrafficManagerExternalEndpoint"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alwaysServeEnabled".into(),
                    value: &always_serve_enabled_binding,
                },
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
                    name: "target".into(),
                    value: &target_binding,
                },
                register_interface::ObjectField {
                    name: "weight".into(),
                    value: &weight_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alwaysServeEnabled".into(),
                },
                register_interface::ResultField {
                    name: "customHeaders".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "endpointLocation".into(),
                },
                register_interface::ResultField {
                    name: "geoMappings".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "profileId".into(),
                },
                register_interface::ResultField {
                    name: "subnets".into(),
                },
                register_interface::ResultField {
                    name: "target".into(),
                },
                register_interface::ResultField {
                    name: "weight".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TrafficManagerExternalEndpointResult {
            always_serve_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alwaysServeEnabled").unwrap(),
            ),
            custom_headers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customHeaders").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            endpoint_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointLocation").unwrap(),
            ),
            geo_mappings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("geoMappings").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("profileId").unwrap(),
            ),
            subnets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnets").unwrap(),
            ),
            target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("target").unwrap(),
            ),
            weight: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("weight").unwrap(),
            ),
        }
    }
}
