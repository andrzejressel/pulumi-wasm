/// Manages an Azure Endpoint within a Traffic Manager Profile.
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
///       name: example-public-ip
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       allocationMethod: Static
///       domainNameLabel: example-public-ip
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
///   exampleTrafficManagerAzureEndpoint:
///     type: azure:network:TrafficManagerAzureEndpoint
///     name: example
///     properties:
///       name: example-endpoint
///       profileId: ${exampleTrafficManagerProfile.id}
///       alwaysServeEnabled: true
///       weight: 100
///       targetResourceId: ${examplePublicIp.id}
/// ```
///
/// ## Import
///
/// Azure Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/trafficManagerAzureEndpoint:TrafficManagerAzureEndpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-resources/providers/Microsoft.Network/trafficManagerProfiles/example-profile/AzureEndpoints/example-endpoint
/// ```
///
pub mod traffic_manager_azure_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrafficManagerAzureEndpointArgs {
        /// If Always Serve is enabled, probing for endpoint health will be disabled and endpoints will be included in the traffic routing method. Defaults to `false`.
        #[builder(into, default)]
        pub always_serve_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `custom_header` blocks as defined below.
        #[builder(into, default)]
        pub custom_headers: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::TrafficManagerAzureEndpointCustomHeader,
                >,
            >,
        >,
        /// Is the endpoint enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of Geographic Regions used to distribute traffic, such as `WORLD`, `UK` or `DE`. The same location can't be specified in two endpoints. [See the Geographic Hierarchies documentation for more information](https://docs.microsoft.com/rest/api/trafficmanager/geographichierarchies/getdefault).
        #[builder(into, default)]
        pub geo_mappings: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the Azure Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the priority of this Endpoint, this must be specified for Profiles using the `Priority` traffic routing method. Supports values between 1 and 1000, with no Endpoints sharing the same value. If omitted the value will be computed in order of creation.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the Traffic Manager Profile that this Azure Endpoint should be created within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub profile_id: pulumi_wasm_rust::Output<String>,
        /// One or more `subnet` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub subnets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::TrafficManagerAzureEndpointSubnet>>,
        >,
        /// The ID of the Azure Resource which should be used as a target.
        #[builder(into)]
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
        /// Specifies how much traffic should be distributed to this endpoint, this must be specified for Profiles using the Weighted traffic routing method. Valid values are between `1` and `1000`. Defaults to `1`.
        #[builder(into, default)]
        pub weight: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct TrafficManagerAzureEndpointResult {
        /// If Always Serve is enabled, probing for endpoint health will be disabled and endpoints will be included in the traffic routing method. Defaults to `false`.
        pub always_serve_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `custom_header` blocks as defined below.
        pub custom_headers: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::TrafficManagerAzureEndpointCustomHeader,
                >,
            >,
        >,
        /// Is the endpoint enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of Geographic Regions used to distribute traffic, such as `WORLD`, `UK` or `DE`. The same location can't be specified in two endpoints. [See the Geographic Hierarchies documentation for more information](https://docs.microsoft.com/rest/api/trafficmanager/geographichierarchies/getdefault).
        pub geo_mappings: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the Azure Endpoint. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the priority of this Endpoint, this must be specified for Profiles using the `Priority` traffic routing method. Supports values between 1 and 1000, with no Endpoints sharing the same value. If omitted the value will be computed in order of creation.
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// The ID of the Traffic Manager Profile that this Azure Endpoint should be created within. Changing this forces a new resource to be created.
        pub profile_id: pulumi_wasm_rust::Output<String>,
        /// One or more `subnet` blocks as defined below. Changing this forces a new resource to be created.
        pub subnets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::TrafficManagerAzureEndpointSubnet>>,
        >,
        /// The ID of the Azure Resource which should be used as a target.
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
        /// Specifies how much traffic should be distributed to this endpoint, this must be specified for Profiles using the Weighted traffic routing method. Valid values are between `1` and `1000`. Defaults to `1`.
        pub weight: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: TrafficManagerAzureEndpointArgs,
    ) -> TrafficManagerAzureEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let always_serve_enabled_binding = args.always_serve_enabled.get_inner();
        let custom_headers_binding = args.custom_headers.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let geo_mappings_binding = args.geo_mappings.get_inner();
        let name_binding = args.name.get_inner();
        let priority_binding = args.priority.get_inner();
        let profile_id_binding = args.profile_id.get_inner();
        let subnets_binding = args.subnets.get_inner();
        let target_resource_id_binding = args.target_resource_id.get_inner();
        let weight_binding = args.weight.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/trafficManagerAzureEndpoint:TrafficManagerAzureEndpoint"
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
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding,
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
                    name: "targetResourceId".into(),
                },
                register_interface::ResultField {
                    name: "weight".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TrafficManagerAzureEndpointResult {
            always_serve_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alwaysServeEnabled").unwrap(),
            ),
            custom_headers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customHeaders").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
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
            target_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetResourceId").unwrap(),
            ),
            weight: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("weight").unwrap(),
            ),
        }
    }
}
