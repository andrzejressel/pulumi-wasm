pub struct RemoteImageArgs {
    pub build: pulumi_wasm_rust::Output<Option<crate::types::RemoteImageBuild>>,
    pub force_remove: pulumi_wasm_rust::Output<Option<bool>>,
    pub keep_locally: pulumi_wasm_rust::Output<Option<bool>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub platform: pulumi_wasm_rust::Output<Option<String>>,
    pub pull_triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
}

pub struct RemoteImageResult {
    pub build: pulumi_wasm_rust::Output<Option<crate::types::RemoteImageBuild>>,
    pub force_remove: pulumi_wasm_rust::Output<Option<bool>>,
    pub image_id: pulumi_wasm_rust::Output<String>,
    pub keep_locally: pulumi_wasm_rust::Output<Option<bool>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub platform: pulumi_wasm_rust::Output<Option<String>>,
    pub pull_triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub repo_digest: pulumi_wasm_rust::Output<String>,
    pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
}

pub fn create(name: &str, args: RemoteImageArgs) -> RemoteImageResult {
    let result = crate::bindings::pulumi::docker::remote_image::invoke(
        name,
        &crate::bindings::pulumi::docker::remote_image::Args {
            build: args.build.get_inner(),
            force_remove: args.force_remove.get_inner(),
            keep_locally: args.keep_locally.get_inner(),
            name: args.name.get_inner(),
            platform: args.platform.get_inner(),
            pull_triggers: args.pull_triggers.get_inner(),
            triggers: args.triggers.get_inner(),
        },
    );

    RemoteImageResult {
        build: crate::into_domain(result.build),
        force_remove: crate::into_domain(result.force_remove),
        image_id: crate::into_domain(result.image_id),
        keep_locally: crate::into_domain(result.keep_locally),
        name: crate::into_domain(result.name),
        platform: crate::into_domain(result.platform),
        pull_triggers: crate::into_domain(result.pull_triggers),
        repo_digest: crate::into_domain(result.repo_digest),
        triggers: crate::into_domain(result.triggers),
    }
}
