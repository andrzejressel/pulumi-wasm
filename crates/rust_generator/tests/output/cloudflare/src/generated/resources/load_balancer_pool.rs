/// Provides a Cloudflare Load Balancer pool resource. This provides a
/// pool of origins that can be used by a Cloudflare Load Balancer.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = load_balancer_pool::create(
///         "example",
///         LoadBalancerPoolArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .description("example load balancer pool")
///             .enabled(false)
///             .latitude(55)
///             .load_sheddings(
///                 vec![
///                     LoadBalancerPoolLoadShedding::builder().defaultPercent(55)
///                     .defaultPolicy("random").sessionPercent(12).sessionPolicy("hash")
///                     .build_struct(),
///                 ],
///             )
///             .longitude(-12)
///             .minimum_origins(1)
///             .name("example-pool")
///             .notification_email("someone@example.com")
///             .origin_steerings(
///                 vec![
///                     LoadBalancerPoolOriginSteering::builder().policy("random")
///                     .build_struct(),
///                 ],
///             )
///             .origins(
///                 vec![
///                     LoadBalancerPoolOrigin::builder().address("192.0.2.1").enabled(false)
///                     .headers(vec![LoadBalancerPoolOriginHeader::builder().header("Host")
///                     .values(vec!["example-1",]).build_struct(),]).name("example-1")
///                     .build_struct(), LoadBalancerPoolOrigin::builder()
///                     .address("192.0.2.2")
///                     .headers(vec![LoadBalancerPoolOriginHeader::builder().header("Host")
///                     .values(vec!["example-2",]).build_struct(),]).name("example-2")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/loadBalancerPool:LoadBalancerPool example <account_id>/<load_balancer_pool_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod load_balancer_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoadBalancerPoolArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api).
        #[builder(into, default)]
        pub check_regions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Free text description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to enable (the default) this pool. Disabled pools will not receive traffic and are excluded from health checks. Disabling a pool will cause any load balancers using it to failover to the next pool (if any). Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The latitude this pool is physically located at; used for proximity steering.
        #[builder(into, default)]
        pub latitude: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// Setting for controlling load shedding for this pool.
        #[builder(into, default)]
        pub load_sheddings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::LoadBalancerPoolLoadShedding>>,
        >,
        /// The longitude this pool is physically located at; used for proximity steering.
        #[builder(into, default)]
        pub longitude: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// The minimum number of origins that must be healthy for this pool to serve traffic. If the number of healthy origins falls below this number, the pool will be marked unhealthy and we will failover to the next available pool. Defaults to `1`.
        #[builder(into, default)]
        pub minimum_origins: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the Monitor to use for health checking origins within this pool.
        #[builder(into, default)]
        pub monitor: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A short name (tag) for the pool.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The email address to send health status notifications to. This can be an individual mailbox or a mailing list. Multiple emails can be supplied as a comma delimited list.
        #[builder(into, default)]
        pub notification_email: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set an origin steering policy to control origin selection within a pool.
        #[builder(into, default)]
        pub origin_steerings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::LoadBalancerPoolOriginSteering>>,
        >,
        /// The list of origins within this pool. Traffic directed at this pool is balanced across all currently healthy origins, provided the pool itself is healthy.
        #[builder(into)]
        pub origins: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::types::LoadBalancerPoolOrigin>,
        >,
    }
    #[allow(dead_code)]
    pub struct LoadBalancerPoolResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// A list of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api).
        pub check_regions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The RFC3339 timestamp of when the load balancer was created.
        pub created_on: pulumi_gestalt_rust::Output<String>,
        /// Free text description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to enable (the default) this pool. Disabled pools will not receive traffic and are excluded from health checks. Disabling a pool will cause any load balancers using it to failover to the next pool (if any). Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The latitude this pool is physically located at; used for proximity steering.
        pub latitude: pulumi_gestalt_rust::Output<Option<f64>>,
        /// Setting for controlling load shedding for this pool.
        pub load_sheddings: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::LoadBalancerPoolLoadShedding>>,
        >,
        /// The longitude this pool is physically located at; used for proximity steering.
        pub longitude: pulumi_gestalt_rust::Output<Option<f64>>,
        /// The minimum number of origins that must be healthy for this pool to serve traffic. If the number of healthy origins falls below this number, the pool will be marked unhealthy and we will failover to the next available pool. Defaults to `1`.
        pub minimum_origins: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The RFC3339 timestamp of when the load balancer was last modified.
        pub modified_on: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Monitor to use for health checking origins within this pool.
        pub monitor: pulumi_gestalt_rust::Output<Option<String>>,
        /// A short name (tag) for the pool.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The email address to send health status notifications to. This can be an individual mailbox or a mailing list. Multiple emails can be supplied as a comma delimited list.
        pub notification_email: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set an origin steering policy to control origin selection within a pool.
        pub origin_steerings: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::LoadBalancerPoolOriginSteering>>,
        >,
        /// The list of origins within this pool. Traffic directed at this pool is balanced across all currently healthy origins, provided the pool itself is healthy.
        pub origins: pulumi_gestalt_rust::Output<
            Vec<super::types::LoadBalancerPoolOrigin>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LoadBalancerPoolArgs,
    ) -> LoadBalancerPoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let check_regions_binding = args.check_regions.get_output(context);
        let description_binding = args.description.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let latitude_binding = args.latitude.get_output(context);
        let load_sheddings_binding = args.load_sheddings.get_output(context);
        let longitude_binding = args.longitude.get_output(context);
        let minimum_origins_binding = args.minimum_origins.get_output(context);
        let monitor_binding = args.monitor.get_output(context);
        let name_binding = args.name.get_output(context);
        let notification_email_binding = args.notification_email.get_output(context);
        let origin_steerings_binding = args.origin_steerings.get_output(context);
        let origins_binding = args.origins.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/loadBalancerPool:LoadBalancerPool".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "checkRegions".into(),
                    value: check_regions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "latitude".into(),
                    value: latitude_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadSheddings".into(),
                    value: load_sheddings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "longitude".into(),
                    value: longitude_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minimumOrigins".into(),
                    value: minimum_origins_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitor".into(),
                    value: monitor_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationEmail".into(),
                    value: notification_email_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "originSteerings".into(),
                    value: origin_steerings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "origins".into(),
                    value: origins_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LoadBalancerPoolResult {
            account_id: o.get_field("accountId"),
            check_regions: o.get_field("checkRegions"),
            created_on: o.get_field("createdOn"),
            description: o.get_field("description"),
            enabled: o.get_field("enabled"),
            latitude: o.get_field("latitude"),
            load_sheddings: o.get_field("loadSheddings"),
            longitude: o.get_field("longitude"),
            minimum_origins: o.get_field("minimumOrigins"),
            modified_on: o.get_field("modifiedOn"),
            monitor: o.get_field("monitor"),
            name: o.get_field("name"),
            notification_email: o.get_field("notificationEmail"),
            origin_steerings: o.get_field("originSteerings"),
            origins: o.get_field("origins"),
        }
    }
}
