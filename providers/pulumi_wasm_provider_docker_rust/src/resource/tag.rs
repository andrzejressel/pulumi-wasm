//! Creates a docker tag. It has the exact same functionality as the `docker tag` command. Deleting the resource will neither delete the source nor target images. The source image must exist on the machine running the docker daemon.

pub struct TagArgs {
    /// Name of the source image.
    pub source_image: pulumi_wasm_rust::Output<String>,
    /// Name of the target image.
    pub target_image: pulumi_wasm_rust::Output<String>,
}

pub struct TagResult {
    /// Name of the source image.
    pub source_image: pulumi_wasm_rust::Output<String>,
    /// ImageID of the source image in the format of `sha256:<<ID>>`
    pub source_image_id: pulumi_wasm_rust::Output<String>,
    /// Name of the target image.
    pub target_image: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TagArgs) -> TagResult {

    let result = crate::bindings::pulumi::docker::tag::invoke(name, &crate::bindings::pulumi::docker::tag::Args {
        source_image: args.source_image.get_inner(),
        target_image: args.target_image.get_inner(),
    });

    TagResult {
        source_image: crate::into_domain(result.source_image),
        source_image_id: crate::into_domain(result.source_image_id),
        target_image: crate::into_domain(result.target_image),
    }
}
