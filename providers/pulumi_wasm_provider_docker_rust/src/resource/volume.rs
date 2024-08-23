pub struct VolumeArgs {
    pub driver: pulumi_wasm_rust::Output<Option<String>>,
    pub driver_opts: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::VolumeLabel>>>,
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct VolumeResult {
    pub driver: pulumi_wasm_rust::Output<String>,
    pub driver_opts: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::VolumeLabel>>>,
    pub mountpoint: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: VolumeArgs) -> VolumeResult {
    let result = crate::bindings::pulumi::docker::volume::invoke(
        name,
        &crate::bindings::pulumi::docker::volume::Args {
            driver: args.driver.get_inner(),
            driver_opts: args.driver_opts.get_inner(),
            labels: args.labels.get_inner(),
            name: args.name.get_inner(),
        },
    );

    VolumeResult {
        driver: crate::into_domain(result.driver),
        driver_opts: crate::into_domain(result.driver_opts),
        labels: crate::into_domain(result.labels),
        mountpoint: crate::into_domain(result.mountpoint),
        name: crate::into_domain(result.name),
    }
}
