pub struct TagArgs {
    pub source_image: pulumi_wasm_rust::Output<String>,
    pub target_image: pulumi_wasm_rust::Output<String>,
}

pub struct TagResult {
    pub source_image: pulumi_wasm_rust::Output<String>,
    pub source_image_id: pulumi_wasm_rust::Output<String>,
    pub target_image: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: TagArgs) -> TagResult {
    let result = crate::bindings::pulumi::docker::tag::invoke(
        name,
        &crate::bindings::pulumi::docker::tag::Args {
            source_image: args.source_image.get_inner(),
            target_image: args.target_image.get_inner(),
        },
    );

    TagResult {
        source_image: crate::into_domain(result.source_image),
        source_image_id: crate::into_domain(result.source_image_id),
        target_image: crate::into_domain(result.target_image),
    }
}
