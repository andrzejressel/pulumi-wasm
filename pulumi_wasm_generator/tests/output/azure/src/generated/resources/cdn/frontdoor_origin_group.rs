/// Manages a Front Door (standard/premium) Origin Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-cdn-frontdoor")
///             .build_struct(),
///     );
///     let exampleFrontdoorOriginGroup = frontdoor_origin_group::create(
///         "exampleFrontdoorOriginGroup",
///         FrontdoorOriginGroupArgs::builder()
///             .cdn_frontdoor_profile_id("${exampleFrontdoorProfile.id}")
///             .health_probe(
///                 FrontdoorOriginGroupHealthProbe::builder()
///                     .intervalInSeconds(240)
///                     .path("/healthProbe")
///                     .protocol("Https")
///                     .requestType("HEAD")
///                     .build_struct(),
///             )
///             .load_balancing(
///                 FrontdoorOriginGroupLoadBalancing::builder()
///                     .additionalLatencyInMilliseconds(0)
///                     .sampleSize(16)
///                     .successfulSamplesRequired(3)
///                     .build_struct(),
///             )
///             .name("example-origin-group")
///             .restore_traffic_time_to_healed_or_new_endpoint_in_minutes(10)
///             .session_affinity_enabled(true)
///             .build_struct(),
///     );
///     let exampleFrontdoorProfile = frontdoor_profile::create(
///         "exampleFrontdoorProfile",
///         FrontdoorProfileArgs::builder()
///             .name("example-profile")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard_AzureFrontDoor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Front Door Origin Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/frontdoorOriginGroup:FrontdoorOriginGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Cdn/profiles/profile1/originGroups/originGroup1
/// ```
///
pub mod frontdoor_origin_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrontdoorOriginGroupArgs {
        /// The ID of the Front Door Profile within which this Front Door Origin Group should exist. Changing this forces a new Front Door Origin Group to be created.
        #[builder(into)]
        pub cdn_frontdoor_profile_id: pulumi_wasm_rust::Output<String>,
        /// A `health_probe` block as defined below.
        #[builder(into, default)]
        pub health_probe: pulumi_wasm_rust::Output<
            Option<super::super::types::cdn::FrontdoorOriginGroupHealthProbe>,
        >,
        /// A `load_balancing` block as defined below.
        #[builder(into)]
        pub load_balancing: pulumi_wasm_rust::Output<
            super::super::types::cdn::FrontdoorOriginGroupLoadBalancing,
        >,
        /// The name which should be used for this Front Door Origin Group. Changing this forces a new Front Door Origin Group to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the amount of time which should elapse before shifting traffic to another endpoint when a healthy endpoint becomes unhealthy or a new endpoint is added. Possible values are between `0` and `50` minutes (inclusive). Default is `10` minutes.
        ///
        /// > **NOTE:** This property is currently not used, but will be in the near future.
        #[builder(into, default)]
        pub restore_traffic_time_to_healed_or_new_endpoint_in_minutes: pulumi_wasm_rust::Output<
            Option<i32>,
        >,
        /// Specifies whether session affinity should be enabled on this host. Defaults to `true`.
        #[builder(into, default)]
        pub session_affinity_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct FrontdoorOriginGroupResult {
        /// The ID of the Front Door Profile within which this Front Door Origin Group should exist. Changing this forces a new Front Door Origin Group to be created.
        pub cdn_frontdoor_profile_id: pulumi_wasm_rust::Output<String>,
        /// A `health_probe` block as defined below.
        pub health_probe: pulumi_wasm_rust::Output<
            Option<super::super::types::cdn::FrontdoorOriginGroupHealthProbe>,
        >,
        /// A `load_balancing` block as defined below.
        pub load_balancing: pulumi_wasm_rust::Output<
            super::super::types::cdn::FrontdoorOriginGroupLoadBalancing,
        >,
        /// The name which should be used for this Front Door Origin Group. Changing this forces a new Front Door Origin Group to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the amount of time which should elapse before shifting traffic to another endpoint when a healthy endpoint becomes unhealthy or a new endpoint is added. Possible values are between `0` and `50` minutes (inclusive). Default is `10` minutes.
        ///
        /// > **NOTE:** This property is currently not used, but will be in the near future.
        pub restore_traffic_time_to_healed_or_new_endpoint_in_minutes: pulumi_wasm_rust::Output<
            Option<i32>,
        >,
        /// Specifies whether session affinity should be enabled on this host. Defaults to `true`.
        pub session_affinity_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: FrontdoorOriginGroupArgs,
    ) -> FrontdoorOriginGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cdn_frontdoor_profile_id_binding = args.cdn_frontdoor_profile_id.get_inner();
        let health_probe_binding = args.health_probe.get_inner();
        let load_balancing_binding = args.load_balancing.get_inner();
        let name_binding = args.name.get_inner();
        let restore_traffic_time_to_healed_or_new_endpoint_in_minutes_binding = args
            .restore_traffic_time_to_healed_or_new_endpoint_in_minutes
            .get_inner();
        let session_affinity_enabled_binding = args.session_affinity_enabled.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cdn/frontdoorOriginGroup:FrontdoorOriginGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cdnFrontdoorProfileId".into(),
                    value: &cdn_frontdoor_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "healthProbe".into(),
                    value: &health_probe_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancing".into(),
                    value: &load_balancing_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "restoreTrafficTimeToHealedOrNewEndpointInMinutes".into(),
                    value: &restore_traffic_time_to_healed_or_new_endpoint_in_minutes_binding,
                },
                register_interface::ObjectField {
                    name: "sessionAffinityEnabled".into(),
                    value: &session_affinity_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cdnFrontdoorProfileId".into(),
                },
                register_interface::ResultField {
                    name: "healthProbe".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancing".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "restoreTrafficTimeToHealedOrNewEndpointInMinutes".into(),
                },
                register_interface::ResultField {
                    name: "sessionAffinityEnabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FrontdoorOriginGroupResult {
            cdn_frontdoor_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cdnFrontdoorProfileId").unwrap(),
            ),
            health_probe: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthProbe").unwrap(),
            ),
            load_balancing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancing").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            restore_traffic_time_to_healed_or_new_endpoint_in_minutes: pulumi_wasm_rust::__private::into_domain(
                hashmap
                    .remove("restoreTrafficTimeToHealedOrNewEndpointInMinutes")
                    .unwrap(),
            ),
            session_affinity_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionAffinityEnabled").unwrap(),
            ),
        }
    }
}