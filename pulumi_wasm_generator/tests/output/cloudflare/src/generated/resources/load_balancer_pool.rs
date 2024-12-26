#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct LoadBalancerPoolArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A list of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api).
    #[builder(into, default)]
    pub check_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Free text description.
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to enable (the default) this pool. Disabled pools will not receive traffic and are excluded from health checks. Disabling a pool will cause any load balancers using it to failover to the next pool (if any). Defaults to `true`.
    #[builder(into, default)]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The latitude this pool is physically located at; used for proximity steering.
    #[builder(into, default)]
    pub latitude: pulumi_wasm_rust::Output<Option<f64>>,
    /// Setting for controlling load shedding for this pool.
    #[builder(into, default)]
    pub load_sheddings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::LoadBalancerPoolLoadShedding>>,
    >,
    /// The longitude this pool is physically located at; used for proximity steering.
    #[builder(into, default)]
    pub longitude: pulumi_wasm_rust::Output<Option<f64>>,
    /// The minimum number of origins that must be healthy for this pool to serve traffic. If the number of healthy origins falls below this number, the pool will be marked unhealthy and we will failover to the next available pool. Defaults to `1`.
    #[builder(into, default)]
    pub minimum_origins: pulumi_wasm_rust::Output<Option<i32>>,
    /// The ID of the Monitor to use for health checking origins within this pool.
    #[builder(into, default)]
    pub monitor: pulumi_wasm_rust::Output<Option<String>>,
    /// A short name (tag) for the pool.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The email address to send health status notifications to. This can be an individual mailbox or a mailing list. Multiple emails can be supplied as a comma delimited list.
    #[builder(into, default)]
    pub notification_email: pulumi_wasm_rust::Output<Option<String>>,
    /// Set an origin steering policy to control origin selection within a pool.
    #[builder(into, default)]
    pub origin_steerings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::LoadBalancerPoolOriginSteering>>,
    >,
    /// The list of origins within this pool. Traffic directed at this pool is balanced across all currently healthy origins, provided the pool itself is healthy.
    #[builder(into)]
    pub origins: pulumi_wasm_rust::Output<Vec<super::types::LoadBalancerPoolOrigin>>,
}
pub struct LoadBalancerPoolResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A list of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api).
    pub check_regions: pulumi_wasm_rust::Output<Vec<String>>,
    /// The RFC3339 timestamp of when the load balancer was created.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// Free text description.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to enable (the default) this pool. Disabled pools will not receive traffic and are excluded from health checks. Disabling a pool will cause any load balancers using it to failover to the next pool (if any). Defaults to `true`.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The latitude this pool is physically located at; used for proximity steering.
    pub latitude: pulumi_wasm_rust::Output<Option<f64>>,
    /// Setting for controlling load shedding for this pool.
    pub load_sheddings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::LoadBalancerPoolLoadShedding>>,
    >,
    /// The longitude this pool is physically located at; used for proximity steering.
    pub longitude: pulumi_wasm_rust::Output<Option<f64>>,
    /// The minimum number of origins that must be healthy for this pool to serve traffic. If the number of healthy origins falls below this number, the pool will be marked unhealthy and we will failover to the next available pool. Defaults to `1`.
    pub minimum_origins: pulumi_wasm_rust::Output<Option<i32>>,
    /// The RFC3339 timestamp of when the load balancer was last modified.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// The ID of the Monitor to use for health checking origins within this pool.
    pub monitor: pulumi_wasm_rust::Output<Option<String>>,
    /// A short name (tag) for the pool.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The email address to send health status notifications to. This can be an individual mailbox or a mailing list. Multiple emails can be supplied as a comma delimited list.
    pub notification_email: pulumi_wasm_rust::Output<Option<String>>,
    /// Set an origin steering policy to control origin selection within a pool.
    pub origin_steerings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::LoadBalancerPoolOriginSteering>>,
    >,
    /// The list of origins within this pool. Traffic directed at this pool is balanced across all currently healthy origins, provided the pool itself is healthy.
    pub origins: pulumi_wasm_rust::Output<Vec<super::types::LoadBalancerPoolOrigin>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: LoadBalancerPoolArgs) -> LoadBalancerPoolResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let check_regions_binding = args.check_regions.get_inner();
    let description_binding = args.description.get_inner();
    let enabled_binding = args.enabled.get_inner();
    let latitude_binding = args.latitude.get_inner();
    let load_sheddings_binding = args.load_sheddings.get_inner();
    let longitude_binding = args.longitude.get_inner();
    let minimum_origins_binding = args.minimum_origins.get_inner();
    let monitor_binding = args.monitor.get_inner();
    let name_binding = args.name.get_inner();
    let notification_email_binding = args.notification_email.get_inner();
    let origin_steerings_binding = args.origin_steerings.get_inner();
    let origins_binding = args.origins.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/loadBalancerPool:LoadBalancerPool".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "checkRegions".into(),
                value: &check_regions_binding,
            },
            register_interface::ObjectField {
                name: "description".into(),
                value: &description_binding,
            },
            register_interface::ObjectField {
                name: "enabled".into(),
                value: &enabled_binding,
            },
            register_interface::ObjectField {
                name: "latitude".into(),
                value: &latitude_binding,
            },
            register_interface::ObjectField {
                name: "loadSheddings".into(),
                value: &load_sheddings_binding,
            },
            register_interface::ObjectField {
                name: "longitude".into(),
                value: &longitude_binding,
            },
            register_interface::ObjectField {
                name: "minimumOrigins".into(),
                value: &minimum_origins_binding,
            },
            register_interface::ObjectField {
                name: "monitor".into(),
                value: &monitor_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "notificationEmail".into(),
                value: &notification_email_binding,
            },
            register_interface::ObjectField {
                name: "originSteerings".into(),
                value: &origin_steerings_binding,
            },
            register_interface::ObjectField {
                name: "origins".into(),
                value: &origins_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "checkRegions".into() },
            register_interface::ResultField { name : "createdOn".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "enabled".into() },
            register_interface::ResultField { name : "latitude".into() },
            register_interface::ResultField { name : "loadSheddings".into() },
            register_interface::ResultField { name : "longitude".into() },
            register_interface::ResultField { name : "minimumOrigins".into() },
            register_interface::ResultField { name : "modifiedOn".into() },
            register_interface::ResultField { name : "monitor".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "notificationEmail".into() },
            register_interface::ResultField { name : "originSteerings".into() },
            register_interface::ResultField { name : "origins".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    LoadBalancerPoolResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        check_regions: into_domain(hashmap.remove("checkRegions").unwrap()),
        created_on: into_domain(hashmap.remove("createdOn").unwrap()),
        description: into_domain(hashmap.remove("description").unwrap()),
        enabled: into_domain(hashmap.remove("enabled").unwrap()),
        latitude: into_domain(hashmap.remove("latitude").unwrap()),
        load_sheddings: into_domain(hashmap.remove("loadSheddings").unwrap()),
        longitude: into_domain(hashmap.remove("longitude").unwrap()),
        minimum_origins: into_domain(hashmap.remove("minimumOrigins").unwrap()),
        modified_on: into_domain(hashmap.remove("modifiedOn").unwrap()),
        monitor: into_domain(hashmap.remove("monitor").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        notification_email: into_domain(hashmap.remove("notificationEmail").unwrap()),
        origin_steerings: into_domain(hashmap.remove("originSteerings").unwrap()),
        origins: into_domain(hashmap.remove("origins").unwrap()),
    }
}
