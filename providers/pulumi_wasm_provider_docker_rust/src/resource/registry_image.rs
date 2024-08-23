pub struct RegistryImageArgs {
    pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
    pub keep_remotely: pulumi_wasm_rust::Output<Option<bool>>,
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
}

pub struct RegistryImageResult {
    pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
    pub keep_remotely: pulumi_wasm_rust::Output<Option<bool>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub sha256_digest: pulumi_wasm_rust::Output<String>,
    pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
}

pub fn create(name: &str, args: RegistryImageArgs) -> RegistryImageResult {
    let result = crate::bindings::pulumi::docker::registry_image::invoke(
        name,
        &crate::bindings::pulumi::docker::registry_image::Args {
            insecure_skip_verify: args.insecure_skip_verify.get_inner(),
            keep_remotely: args.keep_remotely.get_inner(),
            name: args.name.get_inner(),
            triggers: args.triggers.get_inner(),
        },
    );

    RegistryImageResult {
        insecure_skip_verify: crate::into_domain(result.insecure_skip_verify),
        keep_remotely: crate::into_domain(result.keep_remotely),
        name: crate::into_domain(result.name),
        sha256_digest: crate::into_domain(result.sha256_digest),
        triggers: crate::into_domain(result.triggers),
    }
}
