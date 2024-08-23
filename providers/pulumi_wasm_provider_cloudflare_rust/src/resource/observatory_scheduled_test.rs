pub struct ObservatoryScheduledTestArgs {
    pub frequency: pulumi_wasm_rust::Output<String>,
    pub region: pulumi_wasm_rust::Output<String>,
    pub url: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ObservatoryScheduledTestResult {
    pub frequency: pulumi_wasm_rust::Output<String>,
    pub region: pulumi_wasm_rust::Output<String>,
    pub url: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ObservatoryScheduledTestArgs) -> ObservatoryScheduledTestResult {
    let result = crate::bindings::pulumi::cloudflare::observatory_scheduled_test::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::observatory_scheduled_test::Args {
            frequency: args.frequency.get_inner(),
            region: args.region.get_inner(),
            url: args.url.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    ObservatoryScheduledTestResult {
        frequency: crate::into_domain(result.frequency),
        region: crate::into_domain(result.region),
        url: crate::into_domain(result.url),
        zone_id: crate::into_domain(result.zone_id),
    }
}
