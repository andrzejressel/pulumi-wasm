pub struct ServiceConfigArgs {
    pub data: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct ServiceConfigResult {
    pub data: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ServiceConfigArgs) -> ServiceConfigResult {
    let result = crate::bindings::pulumi::docker::service_config::invoke(
        name,
        &crate::bindings::pulumi::docker::service_config::Args {
            data: args.data.get_inner(),
            name: args.name.get_inner(),
        },
    );

    ServiceConfigResult {
        data: crate::into_domain(result.data),
        name: crate::into_domain(result.name),
    }
}
