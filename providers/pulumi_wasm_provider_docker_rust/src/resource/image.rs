pub struct ImageArgs {
    pub build: pulumi_wasm_rust::Output<Option<crate::types::DockerBuild>>,
    pub build_on_preview: pulumi_wasm_rust::Output<Option<bool>>,
    pub image_name: pulumi_wasm_rust::Output<String>,
    pub registry: pulumi_wasm_rust::Output<Option<crate::types::Registry>>,
    pub skip_push: pulumi_wasm_rust::Output<Option<bool>>,
}

pub struct ImageResult {
    pub base_image_name: pulumi_wasm_rust::Output<String>,
    pub context: pulumi_wasm_rust::Output<String>,
    pub dockerfile: pulumi_wasm_rust::Output<String>,
    pub image_name: pulumi_wasm_rust::Output<String>,
    pub platform: pulumi_wasm_rust::Output<Option<String>>,
    pub registry_server: pulumi_wasm_rust::Output<String>,
    pub repo_digest: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ImageArgs) -> ImageResult {
    let result = crate::bindings::pulumi::docker::image::invoke(
        name,
        &crate::bindings::pulumi::docker::image::Args {
            build: args.build.get_inner(),
            build_on_preview: args.build_on_preview.get_inner(),
            image_name: args.image_name.get_inner(),
            registry: args.registry.get_inner(),
            skip_push: args.skip_push.get_inner(),
        },
    );

    ImageResult {
        base_image_name: crate::into_domain(result.base_image_name),
        context: crate::into_domain(result.context),
        dockerfile: crate::into_domain(result.dockerfile),
        image_name: crate::into_domain(result.image_name),
        platform: crate::into_domain(result.platform),
        registry_server: crate::into_domain(result.registry_server),
        repo_digest: crate::into_domain(result.repo_digest),
    }
}
