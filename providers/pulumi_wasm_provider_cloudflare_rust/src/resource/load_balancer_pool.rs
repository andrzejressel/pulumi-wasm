pub struct LoadBalancerPoolArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub check_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub latitude: pulumi_wasm_rust::Output<Option<f64>>,
    pub load_sheddings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolLoadShedding>>>,
    pub longitude: pulumi_wasm_rust::Output<Option<f64>>,
    pub minimum_origins: pulumi_wasm_rust::Output<Option<i32>>,
    pub monitor: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub notification_email: pulumi_wasm_rust::Output<Option<String>>,
    pub origin_steerings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolOriginSteering>>>,
    pub origins: pulumi_wasm_rust::Output<Vec<crate::types::LoadBalancerPoolOrigin>>,
}

pub struct LoadBalancerPoolResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub check_regions: pulumi_wasm_rust::Output<Vec<String>>,
    pub created_on: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub latitude: pulumi_wasm_rust::Output<Option<f64>>,
    pub load_sheddings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolLoadShedding>>>,
    pub longitude: pulumi_wasm_rust::Output<Option<f64>>,
    pub minimum_origins: pulumi_wasm_rust::Output<Option<i32>>,
    pub modified_on: pulumi_wasm_rust::Output<String>,
    pub monitor: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub notification_email: pulumi_wasm_rust::Output<Option<String>>,
    pub origin_steerings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolOriginSteering>>>,
    pub origins: pulumi_wasm_rust::Output<Vec<crate::types::LoadBalancerPoolOrigin>>,
}

pub fn create(name: &str, args: LoadBalancerPoolArgs) -> LoadBalancerPoolResult {
    let result = crate::bindings::pulumi::cloudflare::load_balancer_pool::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::load_balancer_pool::Args {
            account_id: args.account_id.get_inner(),
            check_regions: args.check_regions.get_inner(),
            description: args.description.get_inner(),
            enabled: args.enabled.get_inner(),
            latitude: args.latitude.get_inner(),
            load_sheddings: args.load_sheddings.get_inner(),
            longitude: args.longitude.get_inner(),
            minimum_origins: args.minimum_origins.get_inner(),
            monitor: args.monitor.get_inner(),
            name: args.name.get_inner(),
            notification_email: args.notification_email.get_inner(),
            origin_steerings: args.origin_steerings.get_inner(),
            origins: args.origins.get_inner(),
        },
    );

    LoadBalancerPoolResult {
        account_id: crate::into_domain(result.account_id),
        check_regions: crate::into_domain(result.check_regions),
        created_on: crate::into_domain(result.created_on),
        description: crate::into_domain(result.description),
        enabled: crate::into_domain(result.enabled),
        latitude: crate::into_domain(result.latitude),
        load_sheddings: crate::into_domain(result.load_sheddings),
        longitude: crate::into_domain(result.longitude),
        minimum_origins: crate::into_domain(result.minimum_origins),
        modified_on: crate::into_domain(result.modified_on),
        monitor: crate::into_domain(result.monitor),
        name: crate::into_domain(result.name),
        notification_email: crate::into_domain(result.notification_email),
        origin_steerings: crate::into_domain(result.origin_steerings),
        origins: crate::into_domain(result.origins),
    }
}
