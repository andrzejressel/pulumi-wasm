#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ImageArgs {
    /// The Docker build context
    #[builder(into, default)]
    pub build: pulumi_wasm_rust::Output<Option<super::types::DockerBuild>>,
    /// A flag to build an image on preview
    #[builder(into, default)]
    pub build_on_preview: pulumi_wasm_rust::Output<Option<bool>>,
    /// The image name, of the format repository[:tag], e.g. `docker.io/username/demo-image:v1`.
    /// This reference is not unique to each build and push.For the unique manifest SHA of a pushed docker image, or the local image ID, please use `repoDigest`.
    #[builder(into)]
    pub image_name: pulumi_wasm_rust::Output<String>,
    /// The registry to push the image to
    #[builder(into, default)]
    pub registry: pulumi_wasm_rust::Output<Option<super::types::Registry>>,
    /// A flag to skip a registry push.
    #[builder(into, default)]
    pub skip_push: pulumi_wasm_rust::Output<Option<bool>>,
}
pub struct ImageResult {
    /// The fully qualified image name that was pushed to the registry.
    pub base_image_name: pulumi_wasm_rust::Output<String>,
    /// The path to the build context to use.
    pub context: pulumi_wasm_rust::Output<String>,
    /// The location of the Dockerfile relative to the docker build context.
    pub dockerfile: pulumi_wasm_rust::Output<String>,
    /// The fully qualified image name
    pub image_name: pulumi_wasm_rust::Output<String>,
    /// The image's architecture and OS
    pub platform: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the registry server hosting the image.
    pub registry_server: pulumi_wasm_rust::Output<String>,
    /// **For pushed images:**
    /// The manifest digest of an image pushed to a registry, of the format repository@<algorithm>:<hash>, e.g. `username/demo-image@sha256:a6ae6dd8d39c5bb02320e41abf00cd4cb35905fec540e37d306c878be8d38bd3`.
    /// This reference is unique per image build and push.
    /// Only available for images pushed to a registry.
    /// Use when passing a reference to a pushed image to container management resources.
    ///
    /// **Local-only images**For local images, this field is the image ID of the built local image, of the format <algorithm>:<hash>, e.g `sha256:826a130323165bb0ccb0374ae774f885c067a951b51a6ee133577f4e5dbc4119`
    pub repo_digest: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: ImageArgs) -> ImageResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let build_binding = args.build.get_inner();
    let build_on_preview_binding = args.build_on_preview.get_inner();
    let image_name_binding = args.image_name.get_inner();
    let registry_binding = args.registry.get_inner();
    let skip_push_binding = args.skip_push.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "docker:index/image:Image".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "build".into(),
                value: &build_binding,
            },
            register_interface::ObjectField {
                name: "buildOnPreview".into(),
                value: &build_on_preview_binding,
            },
            register_interface::ObjectField {
                name: "imageName".into(),
                value: &image_name_binding,
            },
            register_interface::ObjectField {
                name: "registry".into(),
                value: &registry_binding,
            },
            register_interface::ObjectField {
                name: "skipPush".into(),
                value: &skip_push_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "baseImageName".into() },
            register_interface::ResultField { name : "context".into() },
            register_interface::ResultField { name : "dockerfile".into() },
            register_interface::ResultField { name : "imageName".into() },
            register_interface::ResultField { name : "platform".into() },
            register_interface::ResultField { name : "registryServer".into() },
            register_interface::ResultField { name : "repoDigest".into() },
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
    ImageResult {
        base_image_name: into_domain(hashmap.remove("baseImageName").unwrap()),
        context: into_domain(hashmap.remove("context").unwrap()),
        dockerfile: into_domain(hashmap.remove("dockerfile").unwrap()),
        image_name: into_domain(hashmap.remove("imageName").unwrap()),
        platform: into_domain(hashmap.remove("platform").unwrap()),
        registry_server: into_domain(hashmap.remove("registryServer").unwrap()),
        repo_digest: into_domain(hashmap.remove("repoDigest").unwrap()),
    }
}
