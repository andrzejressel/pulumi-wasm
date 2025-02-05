/// Manages a Traffic Manager Profile to which multiple endpoints can be attached.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   server:
///     type: random:RandomId
///     properties:
///       keepers:
///         azi_id: 1
///       byteLength: 8
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: trafficmanagerProfile
///       location: West Europe
///   exampleTrafficManagerProfile:
///     type: azure:network:TrafficManagerProfile
///     name: example
///     properties:
///       name: ${server.hex}
///       resourceGroupName: ${example.name}
///       trafficRoutingMethod: Weighted
///       dnsConfig:
///         relativeName: ${server.hex}
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
/// ```
///
/// ## Import
///
/// Traffic Manager Profiles can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:trafficmanager/profile:Profile exampleProfile /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/trafficManagerProfiles/mytrafficmanagerprofile1
/// ```
///
pub mod profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProfileArgs {
        /// This block specifies the DNS configuration of the Profile. One `dns_config` block as defined below.
        #[builder(into)]
        pub dns_config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::trafficmanager::ProfileDnsConfig,
        >,
        /// The amount of endpoints to return for DNS queries to this Profile. Possible values range from `1` to `8`.
        ///
        /// > **NOTE:** `max_return` must be set when the `traffic_routing_method` is `MultiValue`.
        #[builder(into, default)]
        pub max_return: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// This block specifies the Endpoint monitoring configuration for the Profile. One `monitor_config` block as defined below.
        #[builder(into)]
        pub monitor_config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::trafficmanager::ProfileMonitorConfig,
        >,
        /// The name of the Traffic Manager profile. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The status of the profile, can be set to either `Enabled` or `Disabled`. Defaults to `Enabled`.
        #[builder(into, default)]
        pub profile_status: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Traffic Manager profile. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the algorithm used to route traffic. Possible values are `Geographic`, `Weighted`, `Performance`, `Priority`, `Subnet` and `MultiValue`.
        /// * `Geographic` - Traffic is routed based on Geographic regions specified in the Endpoint.
        /// * `MultiValue` - All healthy Endpoints are returned.  MultiValue routing method works only if all the endpoints of type `External` and are specified as IPv4 or IPv6 addresses.
        /// * `Performance` - Traffic is routed via the User's closest Endpoint
        /// * `Priority` - Traffic is routed to the Endpoint with the lowest `priority` value.
        /// * `Subnet` - Traffic is routed based on a mapping of sets of end-user IP address ranges to a specific Endpoint within a Traffic Manager profile.
        /// * `Weighted` - Traffic is spread across Endpoints proportional to their `weight` value.
        #[builder(into)]
        pub traffic_routing_method: pulumi_wasm_rust::InputOrOutput<String>,
        /// Indicates whether Traffic View is enabled for the Traffic Manager profile.
        #[builder(into, default)]
        pub traffic_view_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ProfileResult {
        /// This block specifies the DNS configuration of the Profile. One `dns_config` block as defined below.
        pub dns_config: pulumi_wasm_rust::Output<
            super::super::types::trafficmanager::ProfileDnsConfig,
        >,
        /// The FQDN of the created Profile.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// The amount of endpoints to return for DNS queries to this Profile. Possible values range from `1` to `8`.
        ///
        /// > **NOTE:** `max_return` must be set when the `traffic_routing_method` is `MultiValue`.
        pub max_return: pulumi_wasm_rust::Output<Option<i32>>,
        /// This block specifies the Endpoint monitoring configuration for the Profile. One `monitor_config` block as defined below.
        pub monitor_config: pulumi_wasm_rust::Output<
            super::super::types::trafficmanager::ProfileMonitorConfig,
        >,
        /// The name of the Traffic Manager profile. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The status of the profile, can be set to either `Enabled` or `Disabled`. Defaults to `Enabled`.
        pub profile_status: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Traffic Manager profile. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the algorithm used to route traffic. Possible values are `Geographic`, `Weighted`, `Performance`, `Priority`, `Subnet` and `MultiValue`.
        /// * `Geographic` - Traffic is routed based on Geographic regions specified in the Endpoint.
        /// * `MultiValue` - All healthy Endpoints are returned.  MultiValue routing method works only if all the endpoints of type `External` and are specified as IPv4 or IPv6 addresses.
        /// * `Performance` - Traffic is routed via the User's closest Endpoint
        /// * `Priority` - Traffic is routed to the Endpoint with the lowest `priority` value.
        /// * `Subnet` - Traffic is routed based on a mapping of sets of end-user IP address ranges to a specific Endpoint within a Traffic Manager profile.
        /// * `Weighted` - Traffic is spread across Endpoints proportional to their `weight` value.
        pub traffic_routing_method: pulumi_wasm_rust::Output<String>,
        /// Indicates whether Traffic View is enabled for the Traffic Manager profile.
        pub traffic_view_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ProfileArgs,
    ) -> ProfileResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dns_config_binding = args.dns_config.get_output(context).get_inner();
        let max_return_binding = args.max_return.get_output(context).get_inner();
        let monitor_config_binding = args.monitor_config.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let profile_status_binding = args.profile_status.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let traffic_routing_method_binding = args
            .traffic_routing_method
            .get_output(context)
            .get_inner();
        let traffic_view_enabled_binding = args
            .traffic_view_enabled
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:trafficmanager/profile:Profile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dnsConfig".into(),
                    value: &dns_config_binding,
                },
                register_interface::ObjectField {
                    name: "maxReturn".into(),
                    value: &max_return_binding,
                },
                register_interface::ObjectField {
                    name: "monitorConfig".into(),
                    value: &monitor_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "profileStatus".into(),
                    value: &profile_status_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "trafficRoutingMethod".into(),
                    value: &traffic_routing_method_binding,
                },
                register_interface::ObjectField {
                    name: "trafficViewEnabled".into(),
                    value: &traffic_view_enabled_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProfileResult {
            dns_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsConfig"),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(o.extract_field("fqdn")),
            max_return: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maxReturn"),
            ),
            monitor_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("monitorConfig"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            profile_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("profileStatus"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            traffic_routing_method: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trafficRoutingMethod"),
            ),
            traffic_view_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trafficViewEnabled"),
            ),
        }
    }
}
